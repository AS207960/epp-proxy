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
//! tag is the client login ID, password is the client login password, old_password is the optional
//! current/old EPP password if it is to be changed on login, zones is a list of DNS
//! zones said server is responsible for such as `ch`, `co.uk`, and `org.uk`, and client_cert
//! is an optional TLS certificated bundle in PKCS12 format.
//! Example config file:
//! ```text
//! {
//!  "id": "nominet",
//!  "server": "ote-epp.nominet.org.uk:700",
//!  "tag": "AS207960",
//!  "password": "supersecretpassword",
//!  "old_password": "oldpassword",
//!  "zones": [
//!    "uk"
//!  ],
//!  "client_cert": "priv/as207960-registrar.pfx"
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
mod xml_ser;

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
    /// Current password if the password is to be changed
    old_password: Option<String>,
    /// The zones the server is responsible for such as `co.uk` or `ch`
    zones: Vec<String>,
    /// PKCS12 file for TLS client auth
    client_cert: Option<String>,
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
        let epp_client = client::EPPClient::new(
            &config.server,
            &config.tag,
            &config.password,
            log_dir,
            config.client_cert.as_deref(),
            config.old_password.as_deref(),
        );
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
                    .join("."),
            ) {
                return Some(c.clone());
            }
            domain_parts.pop()?;
        }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

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
            let file = match std::fs::File::open(conf_file.path()) {
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
        for mut c in handles {
            client::logout(&mut c).await.unwrap();
        }
        std::process::exit(0);
    });

    let server = grpc::EPPProxy {
        client_router: router,
    };
    let addr = matches.value_of("listen").unwrap().parse().unwrap();
    info!("Listening for gRPC commands on {}...", addr);
    tonic::transport::Server::builder()
        .add_service(grpc::epp_proto::epp_proxy_server::EppProxyServer::new(
            server,
        ))
        .serve(addr)
        .await
        .unwrap();
}
