#![recursion_limit = "1024"]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://as207960.net/assets/img/logo.svg")]

//! A proxy server for interacting with EPP servers over gRPC
//!
//! The server will listen for gRPC requests on `[::1]:50051` by deafault.
//! See the proto/epp.proto file for information on the gRPC protobufs used to communicate
//! with the server. Use `--help` to view more options.
//!
//! Server expects configuration in json files it the folder `./conf/` relative to the
//! programs current working directory on startup. JSON file should follow the structure of the
//! [`ConfigFile`] struct, where id is a unique ID for identifying the register in gRPC commands,
//! server is the TLS server to connect to in the form `domain:port`,
//! tag is the client login ID, password is the client login password, new_password is the optional
//! new EPP password if it is to be changed on login, zones is a list of DNS
//! zones said server is responsible for such as `ch`, `co.uk`, and `org.uk`, client_cert
//! is an optional TLS certificated bundle in PKCS12 format, pipelining defines support for multiple
//! in flight commands, errata defines server errata.
//!
//! Supported errata are:
//! * `traficom`
//! * `verisign-tv`
//! * `verisign-cc`
//! * `rrpproxy`
//!
//! Example config file:
//! ```text
//! {
//!  "id": "nominet",
//!  "server": "ote-epp.nominet.org.uk:700",
//!  "tag": "AS207960",
//!  "new_password": "supersecretpassword",
//!  "password": "oldpassword",
//!  "zones": [
//!    "uk"
//!  ],
//!  "client_cert": "priv/as207960-registrar.pfx",
//!  "root_certs": ["root/uniregistry.pem"],
//!  "pipelining": true,
//!  "errata": "traficom"
//! }
//! ```

#[macro_use]
extern crate log;

use warp::Filter;

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
        let logger = sentry::integrations::log::SentryLogger::with_dest(log_builder.build());
        log::set_boxed_logger(Box::new(logger)).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
    }
}

#[cfg(not(target_os = "linux"))]
fn setup_logging() {
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder.parse_filters(&std::env::var("RUST_LOG").unwrap_or_default());
    let logger = sentry::integrations::log::SentryLogger::with_dest(log_builder.build());
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}

#[derive(Copy, Clone)]
enum AuthMethod {
    OAuth,
    StaticKey,
}

impl clap::ValueEnum for AuthMethod {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::OAuth, Self::StaticKey]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::OAuth => clap::builder::PossibleValue::new("oauth"),
            Self::StaticKey => clap::builder::PossibleValue::new("static"),
        })
    }
}

#[tokio::main]
async fn main() {
    setup_logging();
    openssl::init();

    let matches = clap::Command::new("epp-proxy")
        .version(env!("CARGO_PKG_VERSION"))
        .about("gRPC to EPP proxy")
        .author("Q of AS207960 <q@as207960.net>")
        .arg(
            clap::Arg::new("listen")
                .short('l')
                .long("listen")
                .value_name("ADDR")
                .default_value("[::1]:50051")
                .value_parser(clap::value_parser!(std::net::SocketAddr))
                .help("Address for gRPC to listen on"),
        )
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
                .value_parser(clap::builder::EnumValueParser::<AuthMethod>::new())
                .default_value("oauth")
                .help("Authentication method to use, oauth or static API key"),
        )
        .get_matches();

    let auth: Box<dyn Auth + Send + Sync> = match matches.get_one::<AuthMethod>("auth").unwrap() {
        AuthMethod::OAuth => Box::new(epp_proxy::oauth_client()),
        AuthMethod::StaticKey => Box::new(StaticAuth::new()),
    };
    let identity = epp_proxy::server_identity().await;
    let pkcs11_engine =
        epp_proxy::setup_pkcs11_engine(matches.get_one::<String>("hsm_conf").map(|s| s.as_str()))
            .await;

    let conf_dir_path = matches.get_one::<String>("conf").unwrap();
    let mut configs = vec![];
    let conf_dir = match std::fs::read_dir(conf_dir_path) {
        Ok(r) => r,
        Err(e) => {
            error!("Can't list config directory: {}", e);
            return;
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
                    error!(
                        "Can't open config file {}: {}",
                        conf_file.path().to_string_lossy(),
                        e
                    );
                    return;
                }
            };
            let conf: epp_proxy::ConfigFile = match serde_json::from_reader(file) {
                Ok(c) => c,
                Err(e) => {
                    error!(
                        "Can't parse config file {}: {}",
                        conf_file.path().to_string_lossy(),
                        e
                    );
                    return;
                }
            };
            configs.push(conf);
        }
    }

    let storage: std::sync::Arc<Box<dyn epp_proxy::Storage>> =
        match matches.get_one::<String>("log_driver").unwrap().as_str() {
            "fs" => {
                let log_dir_path = matches.get_one::<std::path::PathBuf>("log").unwrap();
                match std::fs::create_dir_all(log_dir_path) {
                    Ok(()) => {}
                    Err(e) => {
                        error!("Can't create log directory: {}", e);
                        return;
                    }
                }
                std::sync::Arc::new(Box::new(epp_proxy::FSStorage::new(log_dir_path.to_owned())))
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
                std::sync::Arc::new(Box::new(epp_proxy::S3Storage::new(
                    endpoint, creds, region, bucket,
                )))
            }
            _ => unreachable!(),
        };

    let mut router = epp_proxy::Router::new();
    let mut clients = vec![];
    let metrics =
        std::sync::Arc::new(epp_proxy::metrics::Metrics::new().expect("create metrics registry"));
    for config in configs {
        let scoped_storage = epp_proxy::StorageScoped::new_arc(storage.clone(), &config.id);
        let metrics_registry = metrics.new_scope(config.id.clone());
        let epp_client = epp_proxy::create_client(
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
            futs.push(epp_proxy::client::logout(c));
        }
        for res in futures::future::join_all(futs).await {
            if let Err(err) = res {
                warn!("Failed to logout from server: {:?}", err);
            }
        }
        std::process::exit(0);
    });

    let server = epp_proxy::grpc::EPPProxy {
        client_router: router,
    };
    let addr = *matches.get_one::<std::net::SocketAddr>("listen").unwrap();
    let metrics_addr = *matches
        .get_one::<std::net::SocketAddr>("metrics_listen")
        .unwrap();

    let svc = epp_proxy::grpc::epp_proto::epp_proxy_server::EppProxyServer::new(server);
    let w_svc = AuthService {
        inner: svc,
        auth: std::sync::Arc::new(auth),
    };

    let reflection_svc = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(epp_proxy::grpc::epp_proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let metrics_route = warp::path!("metrics").and_then(metrics_handler);

    info!("Starting metrics server on {}", metrics_addr);
    tokio::task::spawn(async move {
        warp::serve(metrics_route).run(metrics_addr).await;
    });

    info!("Listening for gRPC commands on {}...", addr);
    tonic::transport::Server::builder()
        .tls_config(tonic::transport::ServerTlsConfig::new().identity(identity))
        .unwrap()
        .add_service(reflection_svc)
        .add_service(w_svc)
        .serve(addr)
        .await
        .unwrap();
}

async fn metrics_handler() -> Result<impl warp::Reply, warp::Rejection> {
    use prometheus::Encoder;

    let mut buffer = Vec::new();
    let encoder = prometheus::TextEncoder::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        eprintln!("could not encode custom metrics: {}", e);
    };

    let res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("custom metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };

    Ok(res)
}

#[tonic::async_trait]
trait Auth {
    async fn auth(&self, token: &str) -> bool;
}

#[tonic::async_trait]
impl Auth for rust_keycloak::oauth::OAuthClient {
    async fn auth(&self, token: &str) -> bool {
        self.verify_token(token, "access-epp").await.is_ok()
    }
}

#[derive(Clone)]
struct StaticAuth {
    token: String,
}

impl StaticAuth {
    fn new() -> Self {
        dotenv::dotenv().ok();

        let token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set");

        Self {
            token: token.trim().to_string(),
        }
    }
}

#[tonic::async_trait]
impl Auth for StaticAuth {
    async fn auth(&self, token: &str) -> bool {
        token == self.token
    }
}

#[derive(Clone)]
struct AuthService<T> {
    inner: T,
    auth: std::sync::Arc<Box<dyn Auth + Send + Sync>>,
}

impl<T> tower_service::Service<http::Request<tonic::transport::Body>> for AuthService<T>
where
    T: tower_service::Service<http::Request<tonic::transport::Body>> + Send + Clone + 'static,
    T::Future: Send + 'static,
    T::Error: 'static,
    T::Response: From<http::response::Response<tonic::body::BoxBody>> + 'static,
{
    type Response = T::Response;
    type Error = T::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: http::Request<tonic::transport::Body>) -> Self::Future {
        let headers = req.headers().to_owned();
        let auth = self.auth.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let res = match headers.get("authorization") {
                Some(t) => match t.to_str() {
                    Ok(t) => {
                        let auth_token_str = t.trim();
                        if let Some(auth_token) = auth_token_str.strip_prefix("Bearer ") {
                            if auth.auth(auth_token).await {
                                Ok(inner.call(req).await?)
                            } else {
                                Err("Invalid auth token")
                            }
                        } else {
                            Err("Invalid auth token")
                        }
                    }
                    Err(_) => Err("Invalid auth token"),
                },
                _ => Err("No valid auth token"),
            };

            match res {
                Ok(r) => Ok(r),
                Err(status) => {
                    let mut res = http::Response::new(());

                    *res.version_mut() = http::Version::HTTP_2;

                    let (mut parts, _body) = res.into_parts();

                    parts.headers.insert(
                        http::header::CONTENT_TYPE,
                        http::header::HeaderValue::from_static("application/grpc"),
                    );

                    parts
                        .headers
                        .insert("grpc-status", http::HeaderValue::from_static("16"));
                    if let Ok(v) = http::HeaderValue::from_str(status) {
                        parts.headers.insert("grpc-message", v);
                    }

                    Ok(http::Response::from_parts(parts, tonic::body::empty_body()).into())
                }
            }
        })
    }
}

impl<T> tonic::transport::server::NamedService for AuthService<T>
where
    T: tonic::transport::server::NamedService,
{
    const NAME: &'static str = T::NAME;
}
