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
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

mod client;
mod grpc;
mod proto;

#[allow(missing_docs)]
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ClientCertConfig {
    PKCS12(String),
    PKCS11 { key_id: String, cert_chain: String },
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    /// Unique registry ID
    id: String,
    /// Type/protocol of server being connected to
    #[serde(default)]
    server_type: ConfigServerType,
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
    client_cert: Option<ClientCertConfig>,
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

#[derive(Debug, Deserialize)]
enum ConfigServerType {
    #[serde(rename = "EPP")]
    Epp,
    #[serde(rename = "TMCH")]
    Tmch,
}

impl Default for ConfigServerType {
    fn default() -> Self {
        ConfigServerType::Epp
    }
}

#[derive(Debug, Deserialize)]
struct HSMConfigFile {
    pin: String,
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

    fn add_client(&mut self, epp_client: Box<dyn client::Client>, config: ConfigFile) {
        let epp_client_sender = epp_client.start();

        for zone in &config.zones {
            self.zone_to_client
                .insert(zone.clone(), (epp_client_sender.clone(), config.id.clone()));
        }
        self.id_to_client.insert(config.id, epp_client_sender);
    }

    /// Fetches client sender by registry ID
    pub fn client_by_id(&self, id: &str) -> Option<client::RequestSender> {
        self.id_to_client.get(id).cloned()
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

fn cvt(r: libc::c_int) -> Result<libc::c_int, openssl::error::ErrorStack> {
    if r <= 0 {
        Err(openssl::error::ErrorStack::get())
    } else {
        Ok(r)
    }
}

fn cvt_p<T>(r: *mut T) -> Result<*mut T, openssl::error::ErrorStack> {
    if r.is_null() {
        Err(openssl::error::ErrorStack::get())
    } else {
        Ok(r)
    }
}

struct P11EngineInner(*mut openssl_sys::ENGINE);

/// Holding type with drop for OpenSSL engine references
#[derive(Clone)]
pub struct P11Engine(std::sync::Arc<std::sync::Mutex<P11EngineInner>>);

impl Drop for P11EngineInner {
    fn drop(&mut self) {
        trace!("Dropping PKCS#11 engine");
        unsafe {
            cvt(openssl_sys::ENGINE_free(self.0)).unwrap();
        }
    }
}

// I think engine pointers can be shared between threads, but if it starts crashing, maybe I'm
// wrong and remove this.
unsafe impl Send for P11EngineInner {}

impl P11Engine {
    fn new(engine: *mut openssl_sys::ENGINE) -> Self {
        Self(std::sync::Arc::new(std::sync::Mutex::new(P11EngineInner(
            engine,
        ))))
    }

    fn claim(&self) -> std::sync::MutexGuard<'_, P11EngineInner> {
        self.0.lock().unwrap()
    }
}

impl std::ops::Deref for P11EngineInner {
    type Target = *mut openssl_sys::ENGINE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for P11EngineInner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
            clap::Arg::with_name("hsm_conf")
                .short("h")
                .long("hsm-conf")
                .takes_value(true)
                .help("Where to read the HSM config file from"),
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

    let mut pkcs11_engine: Option<P11Engine> = None;
    let hsm_conf_file = matches.value_of("hsm_conf");
    if let Some(hsm_conf_file) = hsm_conf_file {
        info!("Loading PKCS#11 module");
        let file = match std::fs::File::open(hsm_conf_file) {
            Ok(f) => f,
            Err(e) => {
                error!("Can't open config file {}: {}", hsm_conf_file, e);
                return;
            }
        };
        let conf: HSMConfigFile = match serde_json::from_reader(file) {
            Ok(c) => c,
            Err(e) => {
                error!("Can't parse config file {}: {}", hsm_conf_file, e);
                return;
            }
        };

        let engine_id = std::ffi::CString::new("pkcs11").unwrap();
        let engine_pin_ctrl = std::ffi::CString::new("PIN").unwrap();
        let engine_pin = std::ffi::CString::new(conf.pin).unwrap();

        let engine = match match tokio::task::spawn_blocking(
            move || -> Result<P11Engine, openssl::error::ErrorStack> {
                unsafe {
                    // Something here seems to be blocking, even though we shouldn't be talking to the HSM yet.
                    openssl_sys::ENGINE_load_builtin_engines();
                    let engine =
                        P11Engine::new(cvt_p(openssl_sys::ENGINE_by_id(engine_id.as_ptr()))?);
                    cvt(openssl_sys::ENGINE_init(**engine.claim()))?;
                    cvt(openssl_sys::ENGINE_ctrl_cmd_string(
                        **engine.claim(),
                        engine_pin_ctrl.as_ptr(),
                        engine_pin.as_ptr(),
                        1,
                    ))?;
                    info!("Loaded PKCS#11 engine");
                    Ok(engine)
                }
            },
        )
        .await
        {
            Ok(e) => e,
            Err(e) => {
                error!("Can't setup OpenSSL: {}", e);
                return;
            }
        } {
            Ok(e) => e,
            Err(e) => {
                error!("Can't setup OpenSSL: {}", e);
                return;
            }
        };
        pkcs11_engine.replace(engine);
    };

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
        let client_conf = client::ClientConf {
            host: &config.server,
            tag: &config.tag,
            password: &config.password,
            log_dir,
            client_cert: match &config.client_cert {
                Some(ClientCertConfig::PKCS12(s)) => Some(client::ClientCertConf::PKCS12(s)),
                Some(ClientCertConfig::PKCS11 { key_id, cert_chain }) => {
                    Some(client::ClientCertConf::PKCS11 { key_id, cert_chain })
                }
                _ => None,
            },
            root_certs: &match config.root_certs.as_ref() {
                Some(r) => r.iter().map(|c| c.as_str()).collect::<Vec<_>>(),
                None => vec![],
            },
            danger_accept_invalid_certs: config.danger_accept_invalid_certs.unwrap_or(false),
            danger_accept_invalid_hostname: config.danger_accept_invalid_hostnames.unwrap_or(false),
            new_password: config.new_password.as_deref(),
            pipelining: config.pipelining,
            errata: config.errata.clone(),
        };
        let epp_client = match match config.server_type {
            ConfigServerType::Epp => {
                client::epp::EPPClient::new(client_conf, pkcs11_engine.clone())
                    .await
                    .map(|c| Box::new(c) as Box<dyn client::Client>)
            }
            ConfigServerType::Tmch => {
                client::tmch_client::TMCHClient::new(client_conf, pkcs11_engine.clone())
                    .await
                    .map(|c| Box::new(c) as Box<dyn client::Client>)
            }
        } {
            Ok(c) => c,
            Err(e) => {
                error!("Can't create client: {}", e);
                return;
            }
        };
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
