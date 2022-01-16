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

#[tokio::main]
async fn main() {
    //sentry::integrations::panic::register_panic_handler();
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder.parse_filters(&std::env::var("RUST_LOG").unwrap_or_default());
    let logger = sentry::integrations::log::SentryLogger::with_dest(log_builder.build());
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    //let _guard =
    //    sentry::init("https://786e367376234c2b9bee2bb1984c2e84@o222429.ingest.sentry.io/5247736");
    openssl::init();

    let matches = clap::App::new("epp-proxy")
        .version(env!("CARGO_PKG_VERSION"))
        .about("gRPC to EPP proxy")
        .author("Q of AS207960 <q@as207960.net>")
        .arg(
            clap::Arg::new("listen")
                .short('l')
                .long("listen")
                .takes_value(true)
                .default_value("[::1]:50051")
                .validator(|s| {
                    let addr: Result<std::net::SocketAddr, _> = s.parse();
                    match addr {
                        Ok(_) => Ok(()),
                        Err(_) => Err("Invalid listen address format".to_string()),
                    }
                })
                .help("Which address for gRPC to listen on"),
        )
        .arg(
            clap::Arg::new("conf")
                .short('c')
                .long("conf")
                .takes_value(true)
                .default_value("./conf/")
                .help("Where to read config files from"),
        )
        .arg(
            clap::Arg::new("hsm_conf")
                .short('h')
                .long("hsm-conf")
                .takes_value(true)
                .help("Where to read the HSM config file from"),
        )
        .arg(
            clap::Arg::new("log")
                .long("log")
                .takes_value(true)
                .default_value("./log/")
                .help("Directory to write command logs to"),
        )
        .get_matches();

    let oauth_client = epp_proxy::oauth_client();
    let identity = epp_proxy::server_identity().await;
    let pkcs11_engine = epp_proxy::setup_pkcs11_engine(matches.value_of("hsm_conf")).await;

    let conf_dir_path = matches.value_of("conf").unwrap();
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

    let log_dir_path: &std::path::Path = matches.value_of("log").unwrap().as_ref();
    match std::fs::create_dir_all(&log_dir_path) {
        Ok(()) => {}
        Err(e) => {
            error!("Can't create log directory: {}", e);
            return;
        }
    }

    let mut router = epp_proxy::Router::new();
    let mut clients = vec![];
    for config in configs {
        let log_dir = log_dir_path.join(&config.id);
        match std::fs::create_dir_all(&log_dir) {
            Ok(()) => {}
            Err(e) => {
                error!("Can't create log directory for {}: {}", config.id, e);
                return;
            }
        }
        let epp_client = epp_proxy::create_client(log_dir, &config, &pkcs11_engine, true).await;
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
    let addr = matches.value_of("listen").unwrap().parse().unwrap();

    let svc = epp_proxy::grpc::epp_proto::epp_proxy_server::EppProxyServer::new(server);
    let w_svc = AuthService {
        inner: svc,
        oauth_client,
    };

    info!("Listening for gRPC commands on {}...", addr);
    tonic::transport::Server::builder()
        .tls_config(tonic::transport::ServerTlsConfig::new().identity(identity))
        .unwrap()
        .add_service(w_svc)
        .serve(addr)
        .await
        .unwrap();
}

#[derive(Clone)]
struct AuthService<T> {
    inner: T,
    oauth_client: rust_keycloak::oauth::OAuthClient,
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
        let client = self.oauth_client.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let res = match headers.get("authorization") {
                Some(t) => match t.to_str() {
                    Ok(t) => {
                        let auth_token_str = t.trim();
                        if let Some(auth_token) = auth_token_str.strip_prefix("Bearer ") {
                            match client.verify_token(auth_token, "access-epp").await {
                                Ok(_) => Ok(inner.call(req).await?),
                                Err(_) => Err("Invalid auth token"),
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
