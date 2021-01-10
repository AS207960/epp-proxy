#![recursion_limit = "1024"]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://as207960.net/img/logo.png")]

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
//!  "new_wpassword": "supersecretpassword",
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
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

mod client;
mod grpc;
mod proto;

#[derive(Debug, Deserialize)]
struct ConfigFile {
    /// Unique registrar ID
    id: String,
    /// Server host in the form `domain:port`
    server: String,
    /// Client ID to login to the server
    tag: String,
    /// Password to login to the server
    password: String,
    /// New password if the password is to be changed
    new_password: Option<String>,
    /// The zones the server is responsible for such as `co.uk` or `ch`
    zones: Vec<String>,
    /// PKCS12 file for TLS client auth
    client_cert: Option<String>,
    /// Root certificates to trust on this connection
    root_certs: Option<Vec<String>>,
    /// Accept invalid TLS certs
    danger_accept_invalid_certs: Option<bool>,
    /// Accept TLS certs with a hostname that doesn't match the DNS label
    danger_accept_invalid_hostnames: Option<bool>,
    /// Does the server support pipelining?
    pipelining: bool,
    /// For naughty servers
    errata: Option<String>,
}

/// Route requests to the correct EPP client for the authoritative registry
#[derive(Debug, Default)]
pub struct Router {
    id_to_client: HashMap<String, client::RequestSender>,
    zone_to_client: HashMap<String, (client::RequestSender, String)>,
}

impl Router {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn add_config(&mut self, config: &ConfigFile, log_dir: std::path::PathBuf) {
        let epp_client = client::EPPClient::new(client::ClientConf {
            host: &config.server,
            tag: &config.tag,
            password: &config.password,
            log_dir,
            client_cert: config.client_cert.as_deref(),
            root_certs: &match config.root_certs.as_ref() {
                Some(r) => r.iter().map(|c| c.as_str()).collect::<Vec<_>>(),
                None => vec![]
            },
            danger_accept_invalid_certs: config.danger_accept_invalid_certs.unwrap_or(false),
            danger_accept_invalid_hostname: config.danger_accept_invalid_hostnames.unwrap_or(false),
            new_password: config.new_password.as_deref(),
            pipelining: config.pipelining,
            errata: config.errata.clone(),
        });
        let epp_client_sender = epp_client.start();

        for zone in &config.zones {
            self.zone_to_client
                .insert(zone.clone(), (epp_client_sender.clone(), config.id.clone()));
        }
        self.id_to_client
            .insert(config.id.clone(), epp_client_sender);
    }

    /// Fetches client sender by registry ID
    pub fn client_by_id(&self, id: &str) -> Option<client::RequestSender> {
        match self.id_to_client.get(id) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    /// Searches for client sender by a domain the client is authoritative for
    pub fn client_by_domain(&self, domain: &str) -> Option<(client::RequestSender, String)> {
        let mut domain_parts = domain.split('.').collect::<Vec<_>>();
        domain_parts.reverse();
        loop {
            if let Some(c) = self.zone_to_client.get(
                &domain_parts
                    .clone()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join(".")
                    .to_lowercase(),
            ) {
                return Some(c.clone());
            }
            domain_parts.pop()?;
        }
    }
}

fn oauth_client() -> rust_keycloak::oauth::OAuthClient {
    dotenv::dotenv().ok();

    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let well_known_url = std::env::var("OAUTH_WELL_KNOWN").unwrap_or_else(|_| {
        "https://sso.as207960.net/auth/realms/dev/.well-known/openid-configuration".to_string()
    });

    let config =
        rust_keycloak::oauth::OAuthClientConfig::new(&client_id, &client_secret, &well_known_url)
            .unwrap();

    rust_keycloak::oauth::OAuthClient::new(config)
}

async fn server_identity() -> tonic::transport::Identity {
    dotenv::dotenv().ok();

    let cert_file = std::env::var("TLS_CERT_FILE").expect("TLS_CERT_FILE must be set");
    let key_file = std::env::var("TLS_KEY_FILE").expect("TLS_KEY_FILE must be set");

    let cert = tokio::fs::read(cert_file)
        .await
        .expect("Can't read TLS cert");
    let key = tokio::fs::read(key_file).await.expect("Can't read TLS key");
    tonic::transport::Identity::from_pem(cert, key)
}

#[tokio::main]
async fn main() {
    //sentry::integrations::panic::register_panic_handler();
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder.parse_filters(&std::env::var("RUST_LOG").unwrap_or_default());
    let logger = sentry::integrations::log::SentryLogger::with_dest(log_builder.build());
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    let _guard =
        sentry::init("https://786e367376234c2b9bee2bb1984c2e84@o222429.ingest.sentry.io/5247736");

    let matches = clap::App::new("epp-proxy")
        .version("0.0.1")
        .about("gRPC to EPP proxy")
        .author("Q of AS207960 <q@as207960.net>")
        .arg(
            clap::Arg::with_name("listen")
                .short("l")
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
            clap::Arg::with_name("conf")
                .short("c")
                .long("conf")
                .takes_value(true)
                .default_value("./conf/")
                .help("Where to read config files from"),
        )
        .arg(
            clap::Arg::with_name("log")
                .long("log")
                .takes_value(true)
                .default_value("./log/")
                .help("Directory to write command logs to"),
        )
        .get_matches();

    let oauth_client = oauth_client();
    let identity = server_identity().await;

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
            let conf: ConfigFile = match serde_json::from_reader(file) {
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

    let mut router = Router::new();
    for config in configs {
        let log_dir = log_dir_path.join(&config.id);
        match std::fs::create_dir_all(&log_dir) {
            Ok(()) => {}
            Err(e) => {
                error!("Can't create log directory for {}: {}", config.id, e);
                return;
            }
        }
        router.add_config(&config, log_dir);
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
            futs.push(client::logout(c));
        }
        for res in futures::future::join_all(futs).await {
            if let Err(err) = res {
                warn!("Failed to logout from server: {:?}", err);
            }
        }
        std::process::exit(0);
    });

    let server = grpc::EPPProxy {
        client_router: router,
    };
    let addr = matches.value_of("listen").unwrap().parse().unwrap();

    let svc = grpc::epp_proto::epp_proxy_server::EppProxyServer::new(server);
    let w_svc = AuthService {
        inner: svc,
        oauth_client,
    };

    info!("Listening for gRPC commands on {}...", addr);
    tonic::transport::Server::builder()
        .tls_config(tonic::transport::ServerTlsConfig::new().identity(identity))
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

impl<T, B> tower_service::Service<http::Request<B>> for AuthService<T>
where
    T: tower_service::Service<http::Request<B>> + Send + Clone + 'static,
    T::Future: Send + 'static,
    T::Error: 'static,
    T::Response: From<http::response::Response<tonic::body::BoxBody>> + 'static,
    B: tonic::codegen::HttpBody + Send + Sync + 'static,
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

    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        let headers = req.headers().to_owned();
        let client = self.oauth_client.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let res = match headers.get("authorization") {
                Some(t) => match t.to_str() {
                    Ok(t) => {
                        let auth_token_str = t.trim();
                        if auth_token_str.starts_with("Bearer ") {
                            match client
                                .verify_token(&auth_token_str[7..], "access-epp")
                                .await
                            {
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

                    Ok(http::Response::from_parts(parts, tonic::body::BoxBody::empty()).into())
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
