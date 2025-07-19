use log::error;

#[tokio::main]
async fn main() {
    epp_proxy::cmd_common::init();

    let matches = epp_proxy::cmd_common::cmd_args!("rpp-proxy", "RPP to EPP proxy")
        .arg(
            clap::Arg::new("listen")
                .short('l')
                .long("listen")
                .value_name("ADDR")
                .default_value("[::1]:80")
                .value_parser(clap::value_parser!(std::net::SocketAddr))
                .help("Address for gRPC to listen on"),
        )
        .get_matches();

    let auth = epp_proxy::auth::create_auth_method(*matches.get_one::<epp_proxy::auth::AuthMethod>("auth").unwrap());
    let client_router = match epp_proxy::cmd_common::setup_router(&matches).await {
        Ok(router) => router,
        Err(e) => {
            error!("{}", e);
            return
        }
    };

    let server = epp_proxy::rpp::RPPProxy {
        auth,
        client_router,
    };
    let addr = *matches.get_one::<std::net::SocketAddr>("listen").unwrap();

    let metrics_addr = *matches
        .get_one::<std::net::SocketAddr>("metrics_listen")
        .unwrap();
    epp_proxy::metrics::start_metrics_server(metrics_addr);

    server.launch(addr).await;
}
