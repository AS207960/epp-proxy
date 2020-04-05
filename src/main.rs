#![recursion_limit = "1024"]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://as207960.net/img/logo.png")]

//! A proxy server for interacting with EPP servers over gRPC
//!
//! The server will listen for gRPC requests on `[::1]:50051`. See the proto/epp.proto file
//! for information on the gRPC ptotobufs used to communicate with the server.
//!
//! Server expects configuration in json files it the folder `./conf/` relative to the
//! programs current working directory on startup. JSON file should follow the structure of the
//! [`ConfigFile`] struct, where server is the TLS server to connect to in the form `domain:port`,
//! tag is the client login ID, password is the client login password, and zones is a list of DNS
//! zones said server is responsible for such as `ch`, `co.uk`, and `org.uk`.
//! Example config file:
//! ```text
//! {
//!  "server": "ote-epp.nominet.org.uk:700",
//!  "tag": "AS207960",
//!  "password": "supersecretpassword",
//!  "zones": [
//!    "uk"
//!  ]
//! }
//! ```

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod client;
mod grpc;
mod proto;
mod xml_ser;

#[derive(Debug, Deserialize)]
struct ConfigFile {
    /// Server host in the form `domain:port`
    server: String,
    /// Client ID to login to the server
    tag: String,
    /// Password to login to the server
    password: String,
    /// The zones the server is responsible for such as `co.uk` or `ch`
    zones: Vec<String>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut configs = vec![];
    let conf_dir = std::fs::read_dir("./conf").unwrap();
    for conf_file in conf_dir {
        let conf_file = conf_file.unwrap();
        let conf_file_type = conf_file.file_type().unwrap();
        if !conf_file_type.is_dir() {
            let file = std::fs::File::open(conf_file.path()).unwrap();
            let conf: ConfigFile = serde_json::from_reader(file).unwrap();
            configs.push(conf);
        }
    }

    let config = configs.first().unwrap();
    let epp_client = client::EPPClient::new(&config.server, &config.tag, &config.password);
    let epp_client_sender = epp_client.start();

    let addr = "[::1]:50051".parse().unwrap();
    let server = grpc::EPPProxy {
        client_sender: epp_client_sender,
    };
    tonic::transport::Server::builder()
        .add_service(grpc::epp_proto::epp_proxy_server::EppProxyServer::new(
            server,
        ))
        .serve(addr)
        .await
        .unwrap();
}
