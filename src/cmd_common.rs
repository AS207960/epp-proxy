#[macro_export]
macro_rules! cmd_args {
    ($cmd:literal, $about:literal) => {
        clap::Command::new($cmd)
        .version(env!("CARGO_PKG_VERSION"))
        .about($about)
        .author("Q Misell, AS207960 Cyfyngedig <q@as207960.net>")
        .arg(
            clap::Arg::new("metrics_listen")
                .short('m')
                .long("metrics_listen")
                .value_name("ADDR")
                .default_value("[::1]:8000")
                .value_parser(clap::value_parser!(std::net::SocketAddr))
                .help("Address for Prometheus metrics to listen on"),
        )
        .arg(
            clap::Arg::new("conf")
                .short('c')
                .long("conf")
                .value_name("FILE")
                .default_value("./conf/")
                .help("Where to read config files from"),
        )
        .arg(
            clap::Arg::new("hsm_conf")
                .short('p')
                .long("hsm-conf")
                .value_name("FILE")
                .help("Where to read the HSM config file from"),
        )
        .arg(
            clap::Arg::new("log_driver")
                .long("log-driver")
                .value_name("DRIVER")
                .default_value("fs")
                .env("LOG_DRIVER")
                .value_parser(["fs", "s3"])
                .help("Which log driver to use, filesystem or s3"),
        )
        .arg(
            clap::Arg::new("log")
                .long("log")
                .value_name("DIR")
                .default_value("./log/")
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .help("Directory to write command logs to")
                .required_if_eq("log_driver", "fs"),
        )
        .arg(
            clap::Arg::new("s3_endpoint")
                .long("s3-endpoint")
                .value_name("URL")
                .env("S3_ENDPOINT")
                .help("S3 endpoint to use")
                .required_if_eq("log_driver", "s3"),
        )
        .arg(
            clap::Arg::new("s3_region")
                .long("s3-region")
                .value_name("REGION")
                .env("S3_REGION")
                .help("S3 region name")
                .required_if_eq("log_driver", "s3"),
        )
        .arg(
            clap::Arg::new("s3_bucket")
                .long("s3-bucket")
                .value_name("BUCKET")
                .env("S3_BUCKET")
                .help("S3 bucket name")
                .required_if_eq("log_driver", "s3"),
        )
        .arg(
            clap::Arg::new("s3_access_key_id")
                .long("s3-access-key-id")
                .value_name("KEY_ID")
                .env("S3_ACCESS_KEY_ID")
                .help("S3 access key ID")
                .required_if_eq("log_driver", "s3"),
        )
        .arg(
            clap::Arg::new("s3_secret_access_key")
                .long("s3-secret-access-key")
                .value_name("SECRET_KEY")
                .env("S3_SECRET_ACCESS_KEY")
                .help("S3 secret access key")
                .required_if_eq("log_driver", "s3"),
        )
        .arg(
            clap::Arg::new("auth")
                .long("auth")
                .short('a')
                .value_name("METHOD")
                .value_parser(clap::builder::EnumValueParser::<epp_proxy::auth::AuthMethod>::new())
                .default_value("oauth")
                .help("Authentication method to use, oauth or static API key"),
        )
    };
}

pub use cmd_args;

#[cfg(target_os = "linux")]
fn setup_logging() {
    if systemd_journal_logger::connected_to_journal() {
        systemd_journal_logger::JournalLog::new()
            .unwrap()
            .install()
            .unwrap();
        log::set_max_level(log::LevelFilter::Info);
    } else {
        let mut log_builder = pretty_env_logger::formatted_builder();
        log_builder.parse_filters(&std::env::var("RUST_LOG").unwrap_or_default());
        log::set_boxed_logger(Box::new(log_builder.build())).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
    }
}

#[cfg(not(target_os = "linux"))]
fn setup_logging() {
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder.parse_filters(&std::env::var("RUST_LOG").unwrap_or_default());
    log::set_boxed_logger(Box::new(log_builder.build())).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}

pub fn init() {
    setup_logging();
    openssl::init();
    rustls::crypto::ring::default_provider().install_default()
        .expect("failed to install ring crypto provider");
}

pub async fn setup_router(matches: &clap::ArgMatches) -> Result<crate::Router, String> {
    let pkcs11_engine =
        crate::setup_pkcs11_engine(matches.get_one::<String>("hsm_conf").map(|s| s.as_str()))
            .await;

    let conf_dir_path = matches.get_one::<String>("conf").unwrap();
    let mut configs = vec![];
    let conf_dir = match std::fs::read_dir(conf_dir_path) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Can't list config directory: {}", e));
        }
    };
    for conf_file in conf_dir {
        let conf_file = conf_file.unwrap();
        let conf_file_type = conf_file.file_type().unwrap();
        if !conf_file_type.is_dir() {
            let conf_file_path = conf_file.path();
            if conf_file_path.extension().unwrap_or_default() != "json" {
                continue;
            }
            let file = match std::fs::File::open(conf_file_path) {
                Ok(f) => f,
                Err(e) => {
                    return Err(format!(
                        "Can't open config file {}: {}",
                        conf_file.path().to_string_lossy(),
                        e
                    ));
                }
            };
            let conf: crate::ConfigFile = match serde_json::from_reader(file) {
                Ok(c) => c,
                Err(e) => {
                    return Err(format!(
                        "Can't parse config file {}: {}",
                        conf_file.path().to_string_lossy(),
                        e
                    ));
                }
            };
            configs.push(conf);
        }
    }

    let storage: std::sync::Arc<Box<dyn crate::Storage>> =
        match matches.get_one::<String>("log_driver").unwrap().as_str() {
            "fs" => {
                let log_dir_path = matches.get_one::<std::path::PathBuf>("log").unwrap();
                match std::fs::create_dir_all(log_dir_path) {
                    Ok(()) => {}
                    Err(e) => {
                        return Err(format!("Can't create log directory: {}", e));
                    }
                }
                std::sync::Arc::new(Box::new(crate::FSStorage::new(log_dir_path.to_owned())))
            }
            "s3" => {
                let endpoint = matches.get_one::<String>("s3_endpoint").unwrap();
                let region = aws_sdk_s3::config::Region::new(
                    matches.get_one::<String>("s3_region").unwrap().clone(),
                );
                let bucket = matches.get_one::<String>("s3_bucket").unwrap();
                let access_key_id = matches.get_one::<String>("s3_access_key_id").unwrap();
                let secret_access_key = matches.get_one::<String>("s3_secret_access_key").unwrap();

                let creds = aws_credential_types::Credentials::new(
                    access_key_id.to_string(),
                    secret_access_key.to_string(),
                    None,
                    None,
                    "epp-proxy",
                );
                std::sync::Arc::new(Box::new(crate::S3Storage::new(
                    endpoint, creds, region, bucket,
                )))
            }
            _ => unreachable!(),
        };

    let mut router = crate::Router::new();
    let mut clients = vec![];
    let metrics =
        std::sync::Arc::new(crate::metrics::PrometheusMetrics::new().expect("create metrics registry"));
    for config in configs {
        let scoped_storage = crate::StorageScoped::new_arc(storage.clone(), &config.id);
        let metrics_registry = metrics.new_scope(config.id.clone());
        let epp_client = crate::create_client(
            scoped_storage,
            &config,
            &pkcs11_engine,
            metrics_registry,
            true,
        )
            .await;
        clients.push((epp_client, config))
    }

    for (client, config) in clients {
        router.add_client(client, config)
    }

    let handles: Vec<_> = router.id_to_client.values().cloned().collect();
    tokio::spawn(async move {
        use futures::future::FutureExt;
        let mut term_stream =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();
        let mut int_stream =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt()).unwrap();
        let term_fut = term_stream.recv().fuse();
        let int_fut = int_stream.recv().fuse();
        futures::pin_mut!(term_fut);
        futures::pin_mut!(int_fut);
        futures::select! {
            _ = term_fut => {}
            _ = int_fut => {}
        }
        let mut futs = vec![];
        for c in handles {
            futs.push(crate::client::logout(None, c));
        }
        for res in futures::future::join_all(futs).await {
            if let Err(err) = res {
                warn!("Failed to logout from server: {:?}", err);
            }
        }
        std::process::exit(0);
    });

    Ok(router)
}