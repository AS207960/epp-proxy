use super::{
    router as outer_router, ClientCertConf, LogoutRequest, RequestMessage, ServerFeatures,
};
use crate::proto;
use chrono::prelude::*;
use foreign_types_shared::ForeignType;
use futures::future::FutureExt;
use futures::stream::StreamExt;
use futures::SinkExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub mod balance;
pub mod contact;
pub mod domain;
pub mod eurid;
pub mod fee;
pub mod host;
pub mod isnic;
pub mod launch;
pub mod maintenance;
pub mod nominet;
pub mod poll;
pub mod rgp;
pub mod router;
pub mod traficom;
pub mod verisign;

use crate::proto::EPPServiceExtension;

fn recv_msg(data: String, host: &str) -> Result<proto::EPPMessage, ()> {
    let message: proto::EPPMessage = match xml_serde::from_str(&data) {
        Ok(m) => m,
        Err(err) => {
            error!("Invalid XML from {}: {}", host, err);
            return Err(());
        }
    };
    debug!("Decoded EPP message from {} to: {:#?}", host, message);
    Ok(message)
}

/// Main client struct for the EEP client
#[derive(Debug)]
pub struct EPPClient {
    log_dir: std::path::PathBuf,
    host: String,
    hostname: String,
    tag: String,
    password: String,
    new_password: Option<String>,
    tls_context: openssl::ssl::SslContext,
    server_id: String,
    pipelining: bool,
    is_awaiting_response: bool,
    is_closing: bool,
    /// Is the EPP server in a state to receive and process commands
    ready: bool,
    router: outer_router::Router<router::Router>,
    /// What features does the server support
    features: ServerFeatures,
    nominet_tag_list_subordinate: bool,
    nominet_tag_list_subordinate_client: Option<futures::channel::mpsc::Sender<RequestMessage>>,
}

lazy_static! {
    static ref TLS_CONNECT_LOCK: tokio::sync::Semaphore = tokio::sync::Semaphore::new(1);
}

impl EPPClient {
    /// Creates a new EPP client ready to be started
    ///
    /// # Arguments
    /// * `conf` - Configuration to use for this client
    pub async fn new<'a, C: Into<Option<&'a str>>, S: Into<Option<ClientCertConf<'a>>>>(
        conf: super::ClientConf<'a, C, S>,
        pkcs11_engine: Option<crate::P11Engine>,
    ) -> std::io::Result<Self> {
        let mut context_builder =
            openssl::ssl::SslContext::builder(openssl::ssl::SslMethod::tls_client())?;
        context_builder.set_min_proto_version(Some(openssl::ssl::SslVersion::TLS1_2))?;

        if conf.danger_accept_invalid_certs {
            context_builder.set_verify(openssl::ssl::SslVerifyMode::NONE);
        } else {
            context_builder.set_verify(openssl::ssl::SslVerifyMode::PEER);
        }

        let hostname = conf
            .host
            .rsplitn(2, ':')
            .collect::<Vec<_>>()
            .pop()
            .unwrap()
            .to_string();

        let mut cert_store = openssl::x509::store::X509StoreBuilder::new()?;
        if conf.root_certs.is_empty() {
            cert_store.set_default_paths()?;
        } else {
            for root_cert_path in conf.root_certs.iter() {
                let root_cert_bytes = tokio::fs::read(root_cert_path).await?;
                let root_cert = openssl::x509::X509::from_pem(&root_cert_bytes)?;
                cert_store.add_cert(root_cert)?;
            }
        }
        let cert_store = cert_store.build();
        context_builder.set_cert_store(cert_store);

        if !conf.danger_accept_invalid_hostname {
            context_builder
                .verify_param_mut()
                .set_hostflags(openssl::x509::verify::X509CheckFlags::NO_PARTIAL_WILDCARDS);
            context_builder.verify_param_mut().set_host(&hostname)?;
        }

        let mut priv_key = None;

        if let Some(client_cert) = conf.client_cert.into() {
            match client_cert {
                ClientCertConf::PKCS12(pkcs12_file) => {
                    let pkcs = tokio::fs::read(pkcs12_file).await?;
                    let identity = openssl::pkcs12::Pkcs12::from_der(&pkcs)?.parse("")?;
                    context_builder.set_certificate(&identity.cert)?;
                    context_builder.set_private_key(&identity.pkey)?;
                    for cert in identity.chain.into_iter().flatten() {
                        context_builder.add_extra_chain_cert(cert)?;
                    }
                }
                ClientCertConf::PKCS11 { key_id, cert_chain } => {
                    context_builder.set_certificate_chain_file(cert_chain)?;
                    priv_key.replace(key_id);
                }
            }
        }

        let context = context_builder.build();

        if let Some(key_id) = priv_key {
            let pkcs11_engine = match pkcs11_engine {
                Some(e) => e,
                None => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "PKCS#11 engine required to used PKCS#11 keys",
                    ));
                }
            };

            info!("Using PKCS#11 key ID {} for {} ", key_id, hostname);
            let engine_key_id = std::ffi::CString::new(key_id).unwrap();
            let ctx = context.clone();
            let h = hostname.clone();
            // This sometimes takes absolutely forever to run, so throw it in a thread type thing.
            tokio::task::spawn_blocking(move || -> std::io::Result<()> {
                unsafe {
                    trace!("Loading OpenSSL UI for {}", h);
                    let ui = crate::cvt_p(openssl_sys::UI_OpenSSL())?;
                    trace!("Loading private key for for {}", h);
                    let priv_key = crate::cvt_p(openssl_sys::ENGINE_load_private_key(
                        **pkcs11_engine.claim(),
                        engine_key_id.as_ptr(),
                        ui,
                        std::ptr::null_mut(),
                    ))?;
                    trace!("Setting private key for for {}", h);
                    openssl_sys::SSL_CTX_use_PrivateKey(ctx.as_ptr(), priv_key);
                    trace!("Freeing private key for {}", h);
                    openssl_sys::EVP_PKEY_free(priv_key);
                    Ok(())
                }
            })
            .await??;
        }

        Ok(Self {
            log_dir: conf.log_dir,
            host: conf.host.to_string(),
            hostname,
            tag: conf.tag.to_string(),
            password: conf.password.to_string(),
            tls_context: context,
            new_password: conf.new_password.into().map(|c| c.to_string()),
            pipelining: conf.pipelining,
            features: ServerFeatures {
                errata: conf.errata,
                ..Default::default()
            },
            server_id: String::new(),
            is_awaiting_response: false,
            is_closing: false,
            nominet_tag_list_subordinate: false,
            nominet_tag_list_subordinate_client: None,
            ready: false,
            router: outer_router::Router::default(),
        })
    }

    // Starts up the EPP server and returns the sending end of a tokio channel to inject
    // commands into the client to be processed
    pub fn start(mut self) -> futures::channel::mpsc::Sender<RequestMessage> {
        info!("EPP Client for {} starting...", &self.host);
        if self.nominet_tag_list_subordinate {
            info!("This is a Nominet Tag list subordinate client");
        }
        let (sender, receiver) = futures::channel::mpsc::channel::<RequestMessage>(16);
        tokio::spawn(async move {
            self._main_loop(receiver).await;
        });
        sender
    }

    async fn _main_loop(&mut self, receiver: futures::channel::mpsc::Receiver<RequestMessage>) {
        let mut receiver = receiver.fuse();
        loop {
            self.is_closing = false;
            self.is_awaiting_response = false;

            let mut sock = {
                trace!("Getting connection for {}", self.hostname);
                let connect_fut = self._connect().fuse();
                futures::pin_mut!(connect_fut);

                loop {
                    futures::select! {
                        x = receiver.next() => {
                            match x {
                                Some(x) => outer_router::Router::<router::Router>::reject_request(x),
                                None => {
                                    info!("All senders for {} dropped, exiting...", self.host);
                                    return
                                }
                            };
                        }
                        s = connect_fut => {
                            break s;
                        }
                    }
                }
            };
            trace!("Got connection for {}", self.hostname);

            {
                let exit_str = format!("All senders for {} dropped, exiting...", self.host);
                trace!("Setting up connection to {}", self.hostname);
                let setup_fut = self._setup_connection(&mut sock).fuse();
                futures::pin_mut!(setup_fut);
                match loop {
                    futures::select! {
                        x = receiver.next() => {
                            match x {
                                Some(x) => outer_router::Router::<router::Router>::reject_request(x),
                                None => {
                                    info!("{}", exit_str);
                                    return
                                }
                            };
                        }
                        s = setup_fut => {
                            break s;
                        }
                    }
                } {
                    Ok(_) => {}
                    Err(r) => {
                        if r {
                            break;
                        } else {
                            tokio::time::sleep(tokio::time::Duration::new(5, 0)).await;
                            continue;
                        }
                    }
                }
            }
            trace!("Connection setup to {}", self.hostname);

            let (sock_read, mut sock_write) = tokio::io::split(sock);
            let msg_receiver = super::epp_like::ClientReceiver {
                host: self.host.clone(),
                reader: sock_read,
                root: self.log_dir.clone(),
                decode_fn: recv_msg,
            };
            let mut message_channel = msg_receiver.run().fuse();
            let mut keepalive_interval = tokio::time::interval(tokio::time::Duration::new(120, 0));

            trace!("Entering event loop for {}", self.hostname);
            loop {
                if self.pipelining || !self.is_awaiting_response {
                    futures::select! {
                        r = receiver.next() => {
                            match r {
                                Some(r) => match self._handle_request(r, &mut sock_write).await {
                                    Ok(_) => {},
                                    Err(_) => {
                                        tokio::time::sleep(tokio::time::Duration::new(5, 0)).await;
                                        break;
                                    }
                                },
                                None => {
                                    info!("All senders for {} dropped, exiting...", self.host);
                                    return
                                }
                            };
                        }
                        m = message_channel.next() => {
                            match m {
                                Some(m) => match m {
                                    Ok(m) => match self._handle_response(m).await {
                                        Ok(c) => if c && self.is_closing {
                                            info!("Closing connection to {}...", self.host);
                                            return
                                        },
                                        Err(_) => break
                                    },
                                    Err(c) => if c && self.is_closing {
                                        info!("Closing connection to {}...", self.host);
                                        return
                                    } else {
                                        break
                                    }
                                },
                                None => break
                            }
                        }
                        _ = keepalive_interval.tick().fuse() => {
                            match self._send_keepalive(&mut sock_write).await {
                                Ok(_) => {},
                                Err(_) => {
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    let mut delay =
                        Box::pin(tokio::time::sleep(tokio::time::Duration::new(15, 0)).fuse());
                    let resp = futures::select! {
                        r = message_channel.next() => r,
                        _ = delay => {
                            warn!("Timeout awaiting response from {}", self.host);
                            break;
                        }
                    };
                    match resp {
                        Some(m) => match m {
                            Ok(m) => match self._handle_response(m).await {
                                Ok(c) => {
                                    if c && self.is_closing {
                                        info!("Closing connection to {}...", self.host);
                                        return;
                                    }
                                }
                                Err(_) => break,
                            },
                            Err(c) => {
                                if c && self.is_closing {
                                    info!("Closing connection to {}...", self.host);
                                    return;
                                } else {
                                    break;
                                }
                            }
                        },
                        None => break,
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::new(5, 0)).await;
        }
    }

    async fn _send_keepalive<W: std::marker::Unpin + tokio::io::AsyncWrite>(
        &mut self,
        sock_write: &mut W,
    ) -> Result<(), ()> {
        let message = proto::EPPMessage {
            message: proto::EPPMessageType::Hello {},
        };
        self.is_awaiting_response = true;
        let receiver = self._send_msg(&message, sock_write).fuse();
        let mut delay = Box::pin(tokio::time::sleep(tokio::time::Duration::new(15, 0)).fuse());
        futures::pin_mut!(receiver);
        let resp = futures::select! {
            r = receiver => r,
            _ = delay => {
                return Err(());
            }
        };
        match resp {
            Ok(_) => Ok(()),
            Err(_) => {
                error!("Failed to send hello keepalive command");
                Err(())
            }
        }
    }

    async fn _handle_request<W: std::marker::Unpin + tokio::io::AsyncWrite>(
        &mut self,
        req: outer_router::RequestMessage,
        sock_write: &mut W,
    ) -> Result<(), ()> {
        match (req, self.nominet_tag_list_subordinate) {
            (outer_router::RequestMessage::NominetTagList(t), false) => {
                let client = match &mut self.nominet_tag_list_subordinate_client {
                    Some(c) => c,
                    None => return Err(()),
                };
                match client
                    .send(outer_router::RequestMessage::NominetTagList(t))
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("Failed to send to subordinate server: {}", e);
                        Err(())
                    }
                }
            }
            (outer_router::RequestMessage::Logout(t), _) => {
                match &mut self.nominet_tag_list_subordinate_client {
                    Some(client) => {
                        let (sender, _) = futures::channel::oneshot::channel();
                        match client
                            .send(outer_router::RequestMessage::Logout(Box::new(
                                LogoutRequest {
                                    return_path: sender,
                                },
                            )))
                            .await
                        {
                            Ok(_) => {}
                            Err(e) => {
                                warn!("Failed to send to subordinate server: {}", e);
                                return Err(());
                            }
                        }
                    }
                    None => {}
                };
                self.is_closing = true;
                match self
                    .router
                    .handle_request(&self.features, outer_router::RequestMessage::Logout(t))
                {
                    Some(((command, extension), command_id)) => {
                        self.is_awaiting_response = true;
                        match self
                            ._send_command(command, extension, sock_write, command_id)
                            .await
                        {
                            Ok(_) => Ok(()),
                            Err(_) => Err(()),
                        }
                    }
                    None => Ok(()),
                }
            }
            (req, _) => match self.router.handle_request(&self.features, req) {
                Some(((command, extension), command_id)) => {
                    self.is_awaiting_response = true;
                    match self
                        ._send_command(command, extension, sock_write, command_id)
                        .await
                    {
                        Ok(_) => Ok(()),
                        Err(_) => Err(()),
                    }
                }
                None => Ok(()),
            },
        }
    }

    async fn _handle_response(&mut self, res: proto::EPPMessage) -> Result<bool, ()> {
        self.is_awaiting_response = false;
        match res.message {
            proto::EPPMessageType::Response(response) => {
                if !response.is_success() {
                    warn!(
                        "Received failure result from {}: {}",
                        self.server_id,
                        response.response_msg()
                    );
                }
                let transaction_id = match &response.transaction_id.client_transaction_id {
                    Some(i) => i,
                    None => {
                        error!(
                            "Received response without client transaction ID from {}",
                            self.server_id
                        );
                        return Err(());
                    }
                };
                let is_closing = response.is_closing();
                let transaction_id = match uuid::Uuid::parse_str(transaction_id) {
                    Ok(i) => i,
                    Err(e) => {
                        error!(
                            "Received response with invalid transaction UUID from {}: {}",
                            self.server_id, e
                        );
                        return Err(());
                    }
                };
                self.router.handle_response(&transaction_id, *response);
                Ok(is_closing)
            }
            proto::EPPMessageType::Greeting(greeting) => {
                if (greeting.server_date - Utc::now()).num_minutes() >= 5 {
                    warn!(
                        "Local time out by more than 5 minutes from time reported by {}",
                        greeting.server_id
                    );
                }
                Ok(false)
            }
            o => {
                warn!(
                    "Received unexpected response from {}: {:?}",
                    self.server_id, o
                );
                Ok(false)
            }
        }
    }

    async fn _setup_connection(
        &mut self,
        sock: &mut tokio_openssl::SslStream<TcpStream>,
    ) -> Result<(), bool> {
        let msg = match super::epp_like::recv_msg(sock, &self.host, &self.log_dir, recv_msg).await {
            Ok(m) => m,
            Err(_) => {
                info!("Restarting connection...");
                self._close(sock).await;
                return Err(false);
            }
        };

        if let proto::EPPMessageType::Greeting(greeting) = msg.message {
            self.server_id = greeting.server_id.clone();
            info!("Connection open with: {}", self.server_id);
            match self._process_greeting(greeting).await {
                Ok(_) => {}
                Err(_) => {
                    info!("Will not attempt to reopen connection");
                    self._close(sock).await;
                    return Err(true);
                }
            }

            match self._login(sock).await {
                Ok(_) => {}
                Err(_) => {
                    info!("Restarting connection...");
                    self._close(sock).await;
                    return Err(false);
                }
            }
            Ok(())
        } else {
            error!(
                "Didn't receive greeting as first message from {}",
                &self.host
            );
            info!("Restarting connection...");
            self._close(sock).await;
            Err(false)
        }
    }

    async fn _process_greeting(&mut self, greeting: proto::EPPGreeting) -> Result<(), ()> {
        if !greeting.service_menu.versions.contains(&"1.0".to_string()) {
            error!("No common supported version with {}", greeting.server_id);
            return Err(());
        }
        if greeting.service_menu.languages.contains(&"en".to_string()) {
            self.features.language = "en".to_string();
        } else if greeting
            .service_menu
            .languages
            .contains(&"en-US".to_string())
        {
            self.features.language = "en-US".to_string();
        } else {
            error!("No common supported language with {}", greeting.server_id);
            return Err(());
        }
        if (greeting.server_date - Utc::now()).num_minutes() >= 5 {
            warn!(
                "Local time out by more than 5 minutes from time reported by {}",
                greeting.server_id
            );
        }
        self.features.contact_supported = greeting
            .service_menu
            .supports("urn:ietf:params:xml:ns:contact-1.0");
        self.features.domain_supported = greeting
            .service_menu
            .supports("urn:ietf:params:xml:ns:domain-1.0");
        self.features.host_supported = greeting
            .service_menu
            .supports("urn:ietf:params:xml:ns:host-1.0");
        self.features.change_poll_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:changePoll-1.0");
        self.features.rgp_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:rgp-1.0");
        self.features.secdns_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:secDNS-1.1");
        self.features.nominet_notifications = greeting
            .service_menu
            .supports_ext("http://www.nominet.org.uk/epp/xml/std-notifications-1.2");
        self.features.nominet_tag_list = greeting
            .service_menu
            .supports("http://www.nominet.org.uk/epp/xml/nom-tag-1.0");
        self.features.nominet_contact_ext = greeting
            .service_menu
            .supports_ext("http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0");
        self.features.nominet_data_quality = greeting
            .service_menu
            .supports_ext("http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1");
        self.features.nominet_handshake = greeting
            .service_menu
            .supports_ext("http://www.nominet.org.uk/epp/xml/std-handshake-1.0");
        self.features.nominet_release = greeting
            .service_menu
            .supports_ext("http://www.nominet.org.uk/epp/xml/std-release-1.0");
        self.features.switch_balance = greeting
            .service_menu
            .supports_ext("https://www.nic.ch/epp/balance-1.0");
        self.features.verisign_balance = greeting
            .service_menu
            .supports("http://www.verisign.com/epp/balance-1.0");
        self.features.unitedtld_balance = greeting
            .service_menu
            .supports("http://www.unitedtld.com/epp/finance-1.0");
        self.features.unitedtld_charge = greeting
            .service_menu
            .supports_ext("http://www.unitedtld.com/epp/charge-1.0");
        self.features.verisign_low_balance = greeting
            .service_menu
            .supports_ext("http://www.verisign.com/epp/lowbalance-poll-1.0");
        self.features.verisign_whois_info = greeting
            .service_menu
            .supports_ext("http://www.verisign.com/epp/whoisInf-1.0");
        self.features.corenic_mark = greeting
            .service_menu
            .supports_ext("http://xmlns.corenic.net/epp/mark-ext-1.0");
        self.features.nsset_supported = greeting
            .service_menu
            .supports("urn:ietf:params:xml:ns:nsset-1.2");
        self.features.fee_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:epp:fee-1.0");
        self.features.launch_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:launch-1.0");
        self.features.fee_011_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:fee-0.11");
        self.features.fee_09_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:fee-0.9");
        self.features.fee_08_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:fee-0.8");
        self.features.fee_07_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:fee-0.7");
        self.features.fee_05_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:fee-0.5");
        self.features.unhandled_ns_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:epp:unhandled-namespaces-1.0");
        self.features.eai_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:epp:eai-0.2");
        self.features.maintenance_supported = greeting
            .service_menu
            .supports("urn:ietf:params:xml:ns:epp:maintenance-0.3");
        self.features.login_sec_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:epp:loginSec-1.0");
        self.features.login_sec_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:epp:loginSec-1.0");
        self.features.eurid_contact_support = greeting
            .service_menu
            .supports_ext("http://www.eurid.eu/xml/epp/contact-ext-1.3");
        self.features.eurid_domain_support = greeting
            .service_menu
            .supports_ext("http://www.eurid.eu/xml/epp/domain-ext-2.4");
        self.features.eurid_hit_points_supported = greeting
            .service_menu
            .supports("http://www.eurid.eu/xml/epp/registrarHitPoints-1.0");
        self.features.eurid_registration_limit_supported = greeting
            .service_menu
            .supports("http://www.eurid.eu/xml/epp/registrationLimit-1.1");
        self.features.eurid_finance_supported = greeting
            .service_menu
            .supports("http://www.eurid.eu/xml/epp/registrarFinance-1.0");
        self.features.eurid_dnssec_eligibility_support = greeting
            .service_menu
            .supports("http://www.eurid.eu/xml/epp/dnssecEligibility-1.0");
        self.features.eurid_dns_quality_support = greeting
            .service_menu
            .supports("http://www.eurid.eu/xml/epp/dnsQuality-2.0");
        self.features.qualified_lawyer_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:qualifiedLawyer-1.0");
        self.features.verisign_sync_supported = greeting
            .service_menu
            .supports_ext("http://www.verisign.com/epp/sync-1.0");
        self.features.isnic_domain_supported = greeting
            .service_menu
            .supports_ext("urn:is.isnic:xml:ns:is-ext-domain-1.0");
        self.features.isnic_contact_supported = greeting
            .service_menu
            .supports_ext("urn:is.isnic:xml:ns:is-ext-contact-1.0");
        self.features.isnic_host_supported = greeting
            .service_menu
            .supports_ext("urn:is.isnic:xml:ns:is-ext-host-1.0");
        self.features.isnic_account_supported = greeting
            .service_menu
            .supports_ext("urn:is.isnic:xml:ns:is-ext-account-1.0");

        if !(self.features.contact_supported
            | self.features.domain_supported
            | self.features.host_supported
            | self.features.nominet_tag_list
            | self.features.nsset_supported)
        {
            error!("No common supported objects with {}", greeting.server_id);
            return Err(());
        }
        Ok(())
    }

    async fn _login(&mut self, sock: &mut tokio_openssl::SslStream<TcpStream>) -> Result<(), ()> {
        let mut objects = vec![];
        let mut ext_objects = vec![];

        if self.nominet_tag_list_subordinate {
            objects.push("http://www.nominet.org.uk/epp/xml/nom-tag-1.0".to_string())
        } else {
            if self.features.contact_supported {
                objects.push("urn:ietf:params:xml:ns:contact-1.0".to_string())
            }
            if self.features.domain_supported {
                objects.push("urn:ietf:params:xml:ns:domain-1.0".to_string())
            }
            if self.features.host_supported {
                objects.push("urn:ietf:params:xml:ns:host-1.0".to_string())
            }
            if self.features.change_poll_supported {
                ext_objects.push("urn:ietf:params:xml:ns:changePoll-1.0".to_string())
            }
            if self.features.rgp_supported {
                ext_objects.push("urn:ietf:params:xml:ns:rgp-1.0".to_string())
            }
            if self.features.secdns_supported {
                ext_objects.push("urn:ietf:params:xml:ns:secDNS-1.1".to_string())
            }
            if self.features.nominet_notifications {
                ext_objects
                    .push("http://www.nominet.org.uk/epp/xml/std-notifications-1.2".to_string())
            }
            if self.features.nominet_contact_ext {
                ext_objects
                    .push("http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0".to_string())
            }
            if self.features.nominet_data_quality {
                ext_objects
                    .push("http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1".to_string())
            }
            if self.features.nominet_handshake {
                ext_objects.push("http://www.nominet.org.uk/epp/xml/std-handshake-1.0".to_string())
            }
            if self.features.nominet_release {
                ext_objects.push("http://www.nominet.org.uk/epp/xml/std-release-1.0".to_string())
            }
            if self.features.switch_balance {
                ext_objects.push("https://www.nic.ch/epp/balance-1.0".to_string())
            }
            if self.features.verisign_balance {
                objects.push("http://www.verisign.com/epp/balance-1.0".to_string())
            }
            if self.features.unitedtld_balance {
                objects.push("http://www.unitedtld.com/epp/finance-1.0".to_string())
            }
            if self.features.unitedtld_charge {
                ext_objects.push("http://www.unitedtld.com/epp/charge-1.0".to_string())
            }
            if self.features.verisign_low_balance {
                ext_objects.push("http://www.verisign.com/epp/lowbalance-poll-1.0".to_string())
            }
            if self.features.verisign_whois_info {
                ext_objects.push("http://www.verisign.com/epp/whoisInf-1.0".to_string())
            }
            if self.features.corenic_mark {
                ext_objects.push("http://xmlns.corenic.net/epp/mark-ext-1.0".to_string())
            }
            if self.features.nsset_supported {
                objects.push("urn:ietf:params:xml:ns:nsset-1.2".to_string())
            }
            if self.features.launch_supported {
                ext_objects.push("urn:ietf:params:xml:ns:launch-1.0".to_string())
            }
            if self.features.fee_supported {
                ext_objects.push("urn:ietf:params:xml:ns:epp:fee-1.0".to_string())
            } else if self.features.fee_011_supported {
                ext_objects.push("urn:ietf:params:xml:ns:fee-0.11".to_string())
            } else if self.features.fee_09_supported {
                ext_objects.push("urn:ietf:params:xml:ns:fee-0.9".to_string())
            } else if self.features.fee_08_supported {
                ext_objects.push("urn:ietf:params:xml:ns:fee-0.8".to_string())
            } else if self.features.fee_07_supported {
                ext_objects.push("urn:ietf:params:xml:ns:fee-0.7".to_string())
            } else if self.features.fee_05_supported {
                ext_objects.push("urn:ietf:params:xml:ns:fee-0.5".to_string())
            }
            if self.features.unhandled_ns_supported {
                ext_objects.push("urn:ietf:params:xml:ns:epp:unhandled-namespaces-1.0".to_string())
            }
            if self.features.eai_supported {
                ext_objects.push("urn:ietf:params:xml:ns:epp:eai-0.2".to_string())
            }
            if self.features.login_sec_supported {
                ext_objects.push("urn:ietf:params:xml:ns:epp:loginSec-1.0".to_string())
            }
            if self.features.maintenance_supported {
                objects.push("urn:ietf:params:xml:ns:epp:maintenance-0.3".to_string())
            }
            if self.features.eurid_hit_points_supported {
                objects.push("http://www.eurid.eu/xml/epp/registrarHitPoints-1.0".to_string())
            }
            if self.features.eurid_registration_limit_supported {
                objects.push("http://www.eurid.eu/xml/epp/registrationLimit-1.1".to_string())
            }
            if self.features.eurid_finance_supported {
                objects.push("http://www.eurid.eu/xml/epp/registrarFinance-1.0".to_string())
            }
            if self.features.eurid_dns_quality_support {
                objects.push("http://www.eurid.eu/xml/epp/dnsQuality-2.0".to_string())
            }
            if self.features.eurid_dnssec_eligibility_support {
                objects.push("http://www.eurid.eu/xml/epp/dnssecEligibility-1.0".to_string())
            }
            if self.features.eurid_domain_support {
                ext_objects.push("http://www.eurid.eu/xml/epp/contact-ext-1.3".to_string())
            }
            if self.features.eurid_contact_support {
                ext_objects.push("http://www.eurid.eu/xml/epp/domain-ext-2.4".to_string())
            }
            if self.features.qualified_lawyer_supported {
                ext_objects.push("urn:ietf:params:xml:ns:qualifiedLawyer-1.0".to_string())
            }
            if self.features.verisign_sync_supported {
                ext_objects.push("http://www.verisign.com/epp/sync-1.0".to_string())
            }
            if self.features.isnic_domain_supported {
                ext_objects.push("urn:is.isnic:xml:ns:is-ext-domain-1.0".to_string())
            }
            if self.features.isnic_contact_supported {
                ext_objects.push("urn:is.isnic:xml:ns:is-ext-contact-1.0".to_string())
            }
            if self.features.isnic_host_supported {
                ext_objects.push("urn:is.isnic:xml:ns:is-ext-host-1.0".to_string())
            }
            if self.features.isnic_account_supported {
                ext_objects.push("urn:is.isnic:xml:ns:is-ext-account-1.0".to_string())
            }
            if self.features.nominet_tag_list {
                let new_client = Self {
                    host: self.host.clone(),
                    hostname: self.hostname.clone(),
                    tag: self.tag.clone(),
                    password: self.password.clone(),
                    nominet_tag_list_subordinate: true,
                    log_dir: self.log_dir.clone(),
                    tls_context: self.tls_context.clone(),
                    new_password: None,
                    pipelining: self.pipelining,
                    features: ServerFeatures {
                        errata: self.features.errata.clone(),
                        ..Default::default()
                    },
                    server_id: String::new(),
                    is_awaiting_response: false,
                    is_closing: false,
                    nominet_tag_list_subordinate_client: None,
                    ready: false,
                    router: outer_router::Router::default(),
                };
                self.nominet_tag_list_subordinate_client = Some(new_client.start());
            }
        }

        if let Some(new_password) = &self.new_password {
            let new_password = new_password.clone();
            match self
                ._try_login(
                    self.password.clone(),
                    Some(new_password),
                    objects.clone(),
                    ext_objects.clone(),
                    sock,
                )
                .await
            {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if e {
                        return Err(());
                    }
                }
            }
        }
        match self
            ._try_login(self.password.clone(), None, objects, ext_objects, sock)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    async fn _try_login(
        &mut self,
        password: String,
        new_password: Option<String>,
        objects: Vec<String>,
        ext_objects: Vec<String>,
        sock: &mut tokio_openssl::SslStream<TcpStream>,
    ) -> Result<(), bool> {
        let mut command = proto::EPPLogin {
            client_id: self.tag.clone(),
            password: String::new(),
            new_password: None,
            options: proto::EPPLoginOptions {
                version: "1.0".to_string(),
                language: self.features.language.clone(),
            },
            services: proto::EPPLoginServices {
                objects,
                extension: if ext_objects.is_empty() {
                    None
                } else {
                    Some(EPPServiceExtension {
                        extensions: ext_objects,
                    })
                },
            },
        };

        let ext = if self.features.login_sec_supported {
            command.password = "[LOGIN-SECURITY]".to_string();
            if new_password.is_some() {
                command.new_password = Some("[LOGIN-SECURITY]".to_string());
            }

            Some(vec![proto::EPPCommandExtensionType::EPPLoginSecurity(
                proto::login_sec::EPPLoginSecurity {
                    password: Some(password),
                    new_password,
                    user_agent: Some(proto::login_sec::EPPLoginSecurityUserAgent {
                        tech: Some(crate::built_info::RUSTC_VERSION.to_string()),
                        app: Some(format!(
                            "epp-proxy {}",
                            crate::built_info::GIT_VERSION.unwrap_or("unknown")
                        )),
                        os: match (sys_info::os_type(), sys_info::os_release()) {
                            (Ok(t), Ok(r)) => Some(format!("{} {}", t, r)),
                            _ => None,
                        },
                    }),
                },
            )])
        } else {
            command.password = password;
            command.new_password = new_password;

            None
        };

        match self
            ._send_command(proto::EPPCommandType::Login(command), ext, sock, None)
            .await
        {
            Ok(i) => i,
            Err(_) => {
                error!("Failed to send login command");
                return Err(true);
            }
        };
        let msg = match super::epp_like::recv_msg(sock, &self.host, &self.log_dir, recv_msg).await {
            Ok(msg) => msg,
            Err(_) => {
                error!("Failed to receive login response");
                return Err(true);
            }
        };
        if let proto::EPPMessageType::Response(response) = msg.message {
            let login_sec_info = match &response.extension {
                Some(ext) => ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPLoginSecurityData(i) => Some(i),
                    _ => None,
                }),
                None => None,
            };
            if let Some(login_sec) = login_sec_info {
                for event in &login_sec.events {
                    let mut msg = format!("EPP Logic Security Event; {:?}", &event.event_type);
                    if let Some(name) = &event.event_name {
                        msg.push_str(&format!(" ({})", name));
                    }
                    if let Some(value) = &event.value {
                        msg.push_str(&format!(", value: \"{}\"", value));
                    }
                    if let Some(duration) = &event.duration {
                        msg.push_str(&format!(", duration: \"{}\"", duration));
                    }
                    if let Some(expiration) = &event.expiration_date {
                        msg.push_str(&format!(", expiration: {}", expiration));
                    }
                    if let Some(event_msg) = &event.msg {
                        msg.push_str(&format!(", message: \"{}\"", event_msg));
                    }
                    match &event.level {
                        proto::login_sec::EPPLoginSecurityEventLevel::Warning => warn!("{}", msg),
                        proto::login_sec::EPPLoginSecurityEventLevel::Error => error!("{}", msg),
                    }
                }
            }
            if !response.is_success() {
                error!(
                    "Login to {} failed with error: {}",
                    self.server_id,
                    response.response_msg()
                );
                Err(false)
            } else {
                info!("Successfully logged into {}", self.server_id);
                Ok(())
            }
        } else {
            error!(
                "Didn't receive response to login command from {}",
                self.server_id
            );
            Err(true)
        }
    }

    async fn _send_command<
        W: std::marker::Unpin + tokio::io::AsyncWrite,
        M: Into<Option<uuid::Uuid>>,
        E: Into<Option<Vec<proto::EPPCommandExtensionType>>>,
    >(
        &self,
        command: proto::EPPCommandType,
        extension: E,
        sock: &mut W,
        message_id: M,
    ) -> Result<uuid::Uuid, ()> {
        let message_id = match message_id.into() {
            Some(m) => m,
            None => uuid::Uuid::new_v4(),
        };
        let command = proto::EPPCommand {
            command,
            extension: extension
                .into()
                .map(|e| proto::EPPCommandExtension { value: e }),
            client_transaction_id: Some(message_id.to_hyphenated().to_string()),
        };
        let message = proto::EPPMessage {
            message: proto::EPPMessageType::Command(Box::new(command)),
        };
        match self._send_msg(&message, sock).await {
            Ok(_) => Ok(message_id),
            Err(_) => Err(()),
        }
    }

    async fn _send_msg<W: std::marker::Unpin + tokio::io::AsyncWrite>(
        &self,
        message: &proto::EPPMessage,
        sock: &mut W,
    ) -> Result<(), ()> {
        let encoded_msg = xml_serde::to_string(message).unwrap();
        debug!(
            "Sending EPP message to {} with contents: {}",
            self.hostname, encoded_msg
        );
        let msg_bytes = encoded_msg.as_bytes();
        let msg_len = msg_bytes.len() + 4;
        match sock.write_u32(msg_len as u32).await {
            Ok(_) => {}
            Err(err) => {
                error!("Error writing data unit length to {}: {}", &self.host, err);
                return Err(());
            }
        };
        match sock.write(msg_bytes).await {
            Ok(_) => {}
            Err(err) => {
                error!("Error writing data unit to {}: {}", &self.host, err);
                return Err(());
            }
        }
        match super::epp_like::write_msg_log(&encoded_msg, "send", &self.log_dir).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed writing sent message to message log: {}", e);
            }
        }
        Ok(())
    }

    async fn _close(&mut self, sock: &mut tokio_openssl::SslStream<TcpStream>) {
        self.router.drain();
        match sock.shutdown().await {
            Ok(_) => {
                info!("Connection to {} closed", &self.host);
            }
            Err(err) => {
                error!(
                    "Error closing connection to {}: {}, dropping anyway",
                    &self.host, err
                );
            }
        }
    }

    async fn _connect(&self) -> tokio_openssl::SslStream<TcpStream> {
        loop {
            match self._try_connect().await {
                Ok(s) => {
                    info!("Successfully connected to {}", &self.host);
                    return s;
                }
                Err(_) => {
                    tokio::time::sleep(tokio::time::Duration::new(5, 0)).await;
                }
            }
        }
    }

    async fn _try_connect(&self) -> Result<tokio_openssl::SslStream<TcpStream>, ()> {
        let addr = match tokio::net::lookup_host(&self.host).await {
            Ok(mut s) => match s.next() {
                Some(s) => s,
                None => {
                    error!("Resolving {} returned no records", self.host);
                    return Err(());
                }
            },
            Err(err) => {
                error!("Failed to resolve {}: {}", self.host, err);
                return Err(());
            }
        };

        // Many servers <drop the connection if no TLS data is sent within ~10 seconds.
        // We therefore have to make sure only one thread at a time connects TLS otherwise a thread
        // could open a TCP stream and then wait a while for another thread to negotiate TLS
        // (it shouldn't be blocking because async, but sometimes it is), by which time the server
        // has dropped the connection and it all has to start again.
        //
        // Please don't ask how this happens>, or how I found out, just don't try and fix this.
        trace!("Getting connect lock for {}", self.hostname);
        let lock = TLS_CONNECT_LOCK.acquire().await.unwrap();
        trace!("Setting up TLS stream for {}", self.hostname);

        trace!("Opening TCP connection to {}", self.hostname);
        let socket = match tokio::net::TcpStream::connect(&addr).await {
            Ok(s) => s,
            Err(err) => {
                error!("Unable to connect to {}: {}", self.host, err);
                return Err(());
            }
        };

        trace!("Creating TLS context for {}", self.hostname);
        let mut cx = match (move || -> std::io::Result<tokio_openssl::SslStream<TcpStream>> {
            let mut ssl = openssl::ssl::Ssl::new(&self.tls_context)?;
            ssl.set_hostname(&self.hostname)?;
            let cx = tokio_openssl::SslStream::new(ssl, socket)?;
            Ok(cx)
        })() {
            Ok(s) => Box::pin(s),
            Err(err) => {
                error!("Unable to create TLS context: {}", err);
                return Err(());
            }
        };
        // I know this is disgusting, but OpenSSL isn't actually async compatible when using
        // a HSM.
        trace!("Negotiating TLS connection to {}", self.hostname);
        let res = match tokio::task::spawn_blocking(
            move || -> Result<tokio_openssl::SslStream<TcpStream>, openssl::ssl::Error> {
                futures::executor::block_on(std::pin::Pin::as_mut(&mut cx).connect())?;
                Ok(*std::pin::Pin::into_inner(cx))
            },
        )
        .await
        {
            Ok(s) => match s {
                Ok(c) => Ok(c),
                Err(err) => {
                    error!("Unable to start TLS session to {}: {}", self.host, err);
                    return Err(());
                }
            },
            Err(err) => {
                error!("Unable to start TLS session to {}: {}", self.host, err);
                return Err(());
            }
        };
        trace!("Dropping connect lock for {}", self.hostname);
        std::mem::drop(lock);
        res
    }
}

pub fn handle_logout(
    _client: &super::ServerFeatures,
    _req: &LogoutRequest,
) -> router::HandleReqReturn<()> {
    Ok((proto::EPPCommandType::Logout {}, None))
}

pub fn handle_logout_response(_response: proto::EPPResponse) -> super::Response<()> {
    super::Response::Ok(())
}
