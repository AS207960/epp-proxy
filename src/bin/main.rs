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
    epp_proxy::cmd_common::init();

    let matches = epp_proxy::cmd_common::cmd_args!("epp-proxy", "gRPC to EPP proxy")
        .arg(
            clap::Arg::new("listen")
                .short('l')
                .long("listen")
                .value_name("ADDR")
                .default_value("[::1]:50051")
                .value_parser(clap::value_parser!(std::net::SocketAddr))
                .help("Address for gRPC to listen on"),
        )
        .get_matches();

    let auth = epp_proxy::auth::create_auth_method(*matches.get_one::<epp_proxy::auth::AuthMethod>("auth").unwrap());

    let identity = epp_proxy::server_identity().await;
    let router = match epp_proxy::cmd_common::setup_router(&matches).await {
        Ok(router) => router,
        Err(e) => {
            error!("{}", e);
            return
        }
    };

    let server = epp_proxy::grpc::EPPProxy {
        client_router: router,
    };
    let addr = *matches.get_one::<std::net::SocketAddr>("listen").unwrap();

    let svc = epp_proxy::grpc::epp_proto::epp_proxy_server::EppProxyServer::new(server);
    let w_svc = AuthService {
        inner: svc,
        auth: std::sync::Arc::new(auth),
    };

    let reflection_svc = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(epp_proxy::grpc::epp_proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    let metrics_addr = *matches
        .get_one::<std::net::SocketAddr>("metrics_listen")
        .unwrap();
    epp_proxy::metrics::start_metrics_server(metrics_addr);

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

#[derive(Clone)]
struct AuthService<T> {
    inner: T,
    auth: std::sync::Arc<Box<dyn epp_proxy::auth::Auth + Send + Sync>>,
}

impl<T> tower_service::Service<tonic::codegen::http::Request<tonic::body::Body>> for AuthService<T>
where
    T: tower_service::Service<tonic::codegen::http::Request<tonic::body::Body>> + Send + Clone + 'static,
    T::Future: Send + 'static,
    T::Error: 'static,
    T::Response: From<tonic::codegen::http::Response<tonic::body::Body>> + 'static,
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

    fn call(&mut self, req: tonic::codegen::http::Request<tonic::body::Body>) -> Self::Future {
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
                    let mut res = tonic::codegen::http::Response::new(());

                    *res.version_mut() = tonic::codegen::http::Version::HTTP_2;

                    let (mut parts, _body) = res.into_parts();

                    parts.headers.insert(
                        tonic::codegen::http::header::CONTENT_TYPE,
                        tonic::codegen::http::header::HeaderValue::from_static("application/grpc"),
                    );

                    parts
                        .headers
                        .insert("grpc-status", tonic::codegen::http::HeaderValue::from_static("16"));
                    if let Ok(v) = tonic::codegen::http::HeaderValue::from_str(status) {
                        parts.headers.insert("grpc-message", v);
                    }

                    Ok(tonic::codegen::http::Response::from_parts(parts, tonic::body::Body::empty()).into())
                }
            }
        })
    }
}

impl<T> tonic::server::NamedService for AuthService<T>
where
    T: tonic::server::NamedService,
{
    const NAME: &'static str = T::NAME;
}
