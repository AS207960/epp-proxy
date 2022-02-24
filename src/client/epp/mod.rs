use super::Client;
use super::{router as outer_router, BlankRequest, RequestMessage};
use crate::client::router::CommandTransactionID;
use crate::proto;
use chrono::prelude::*;
use futures::future::FutureExt;
use futures::stream::StreamExt;
use futures::SinkExt;

pub mod balance;
pub mod contact;
pub mod domain;
pub mod email_forward;
pub mod eurid;
pub mod fee;
pub mod host;
pub mod isnic;
pub mod launch;
pub mod maintenance;
pub mod mark;
pub mod nominet;
pub mod poll;
pub mod rgp;
pub mod router;
pub mod traficom;
pub mod verisign;
pub mod personal_registration;

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

fn send_msg(data: &proto::EPPMessage, host: &str) -> Result<String, ()> {
    let encoded_msg = match xml_serde::to_string(data) {
        Ok(m) => m,
        Err(err) => {
            error!("Error serialising message for {}: {}", host, err);
            return Err(());
        }
    };
    Ok(encoded_msg)
}

/// Features supported by the server
#[derive(Debug, Default)]
pub struct ServerFeatures {
    /// For naughty servers
    errata: Option<String>,
    language: String,
    /// RFC 5731 support
    domain_supported: bool,
    /// RFC 5732 support
    host_supported: bool,
    /// RFC 5733 support
    contact_supported: bool,
    /// RFC 8590 support
    change_poll_supported: bool,
    /// RFC 3915 support
    rgp_supported: bool,
    /// RFC 5910 support
    secdns_supported: bool,
    /// http://www.nominet.org.uk/epp/xml/std-notifications-1.2 support
    nominet_notifications: bool,
    /// http://www.nominet.org.uk/epp/xml/nom-tag-1.0 support
    nominet_tag_list: bool,
    /// http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0 support
    nominet_contact_ext: bool,
    /// http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1 support
    nominet_data_quality: bool,
    /// http://www.nominet.org.uk/epp/xml/std-handshake-1.0 support
    nominet_release: bool,
    /// http://www.nominet.org.uk/epp/xml/std-release-1.0 support
    nominet_handshake: bool,
    /// https://www.nic.ch/epp/balance-1.0 support
    switch_balance: bool,
    /// http://www.verisign.com/epp/balance-1.0 support
    verisign_balance: bool,
    /// http://www.unitedtld.com/epp/finance-1.0 support
    unitedtld_balance: bool,
    /// http://www.unitedtld.com/epp/charge-1.0 support
    unitedtld_charge: bool,
    /// http://www.verisign.com/epp/lowbalance-poll-1.0 support
    verisign_low_balance: bool,
    /// http://www.verisign.com/epp/whoisInf-1.0 support
    verisign_whois_info: bool,
    /// http://xmlns.corenic.net/epp/mark-ext-1.0 support
    corenic_mark: bool,
    /// urn:ietf:params:xml:ns:nsset-1.2 support (NOT AN ACTUAL IETF NAMESPACE)
    nsset_supported: bool,
    /// RFC 8748 support
    fee_supported: bool,
    /// RFC 8334 support
    launch_supported: bool,
    /// urn:ietf:params:xml:ns:fee-0.11 support
    fee_011_supported: bool,
    /// urn:ietf:params:xml:ns:fee-0.9 support
    fee_09_supported: bool,
    /// urn:ietf:params:xml:ns:fee-0.8 support
    fee_08_supported: bool,
    /// urn:ietf:params:xml:ns:fee-0.7 support
    fee_07_supported: bool,
    /// urn:ietf:params:xml:ns:fee-0.5 support
    fee_05_supported: bool,
    /// urn:ietf:params:xml:ns:epp:unhandled-namespaces-1.0 support
    unhandled_ns_supported: bool,
    /// urn:ietf:params:xml:ns:epp:eai-0.2 support
    eai_supported: bool,
    /// urn:ietf:params:xml:ns:epp:maintenance-0.3 support
    maintenance_supported: bool,
    /// RFC8807 support
    login_sec_supported: bool,
    /// http://www.eurid.eu/xml/epp/contact-ext-1.3 support
    eurid_contact_support: bool,
    /// http://www.eurid.eu/xml/epp/domain-ext-2.4 support
    eurid_domain_support: bool,
    /// http://www.eurid.eu/xml/epp/dnsQuality-2.0 support
    eurid_dns_quality_support: bool,
    /// http://www.eurid.eu/xml/epp/dnssecEligibility-1.0 support
    eurid_dnssec_eligibility_support: bool,
    /// http://www.eurid.eu/xml/epp/homoglyph-1.0 support
    eurid_homoglyph_supported: bool,
    /// http://www.eurid.eu/xml/epp/authInfo-1.1 support
    eurid_auth_info_supported: bool,
    /// http://www.eurid.eu/xml/epp/idn-1.0 support
    eurid_idn_supported: bool,
    /// http://www.eurid.eu/xml/epp/registrarFinance-1.0 support
    eurid_finance_supported: bool,
    /// http://www.eurid.eu/xml/epp/registrarHitPoints-1.0 support
    eurid_hit_points_supported: bool,
    /// http://www.eurid.eu/xml/epp/registrationLimit-1.1 support
    eurid_registration_limit_supported: bool,
    /// http://www.eurid.eu/xml/epp/poll-1.2 support
    eurid_poll_supported: bool,
    /// urn:ietf:params:xml:ns:qualifiedLawyer-1.0 support
    qualified_lawyer_supported: bool,
    /// http://www.verisign.com/epp/sync-1.0 support
    verisign_sync_supported: bool,
    /// urn:is.isnic:xml:ns:is-ext-domain-1.0 support
    isnic_domain_supported: bool,
    /// urn:is.isnic:xml:ns:is-ext-host-1.0 support
    isnic_host_supported: bool,
    /// urn:is.isnic:xml:ns:is-ext-contact-1.0 support
    isnic_contact_supported: bool,
    /// urn:is.isnic:xml:ns:is-ext-list-1.0 support
    isnic_list_supported: bool,
    /// urn:is.isnic:xml:ns:is-ext-account-1.0 support
    isnic_account_supported: bool,
    /// http://www.nic.name/epp/emailFwd-1.0 support
    email_forward_supported: bool,
    /// http://www.nic.name/epp/persReg-1.0 support
    personal_registration_supported: bool
}

impl ServerFeatures {
    fn has_erratum(&self, name: &str) -> bool {
        match &self.errata {
            Some(s) => s == name,
            None => false,
        }
    }
}

/// Main client struct for the EEP client
#[derive(Debug)]
pub struct EPPClient {
    log_dir: std::path::PathBuf,
    host: String,
    tag: String,
    password: String,
    new_password: Option<String>,
    server_id: String,
    pipelining: bool,
    keepalive: bool,
    is_awaiting_response: bool,
    is_closing: bool,
    router: outer_router::Router<router::Router, ServerFeatures>,
    /// What features does the server support
    features: ServerFeatures,
    nominet_tag_list_subordinate: bool,
    nominet_tag_list_subordinate_client: Option<futures::channel::mpsc::Sender<RequestMessage>>,
    nominet_dac_subordinate_client: Option<futures::channel::mpsc::Sender<RequestMessage>>,
    nominet_dac_client: Option<super::nominet_dac::DACClient>,
    tls_client: super::epp_like::tls_client::TLSClient,
}

impl super::Client for EPPClient {
    // Starts up the EPP client and returns the sending end of a tokio channel to inject
    // commands into the client to be processed
    fn start(
        mut self: Box<Self>,
    ) -> (
        futures::channel::mpsc::Sender<RequestMessage>,
        futures::channel::mpsc::UnboundedReceiver<CommandTransactionID>,
    ) {
        info!("EPP Client for {} starting...", &self.host);
        if self.nominet_tag_list_subordinate {
            info!("This is a Nominet Tag list subordinate client");
        }
        let (sender, receiver) = futures::channel::mpsc::channel::<RequestMessage>(16);
        let (ready_sender, ready_receiver) = futures::channel::mpsc::unbounded();

        if let Some(nominet_dac_client) = self.nominet_dac_client.take() {
            self.nominet_dac_subordinate_client
                .replace(Box::new(nominet_dac_client).start().0);
        }

        tokio::spawn(async move {
            self._main_loop(receiver, ready_sender).await;
        });

        (sender, ready_receiver)
    }
}

impl EPPClient {
    /// Creates a new EPP client ready to be started
    ///
    /// # Arguments
    /// * `conf` - Configuration to use for this client
    pub async fn new<'a, C: Into<Option<&'a str>>>(
        conf: super::ClientConf<'a, C>,
        pkcs11_engine: Option<crate::P11Engine>,
    ) -> std::io::Result<Self> {
        let nominet_dac_client = match conf.nominet_dac.as_ref().map(|nominet_dac_conf| {
            super::nominet_dac::DACClient::new(
                nominet_dac_conf.real_time,
                nominet_dac_conf.time_delay,
                conf.source_address,
            )
        }) {
            Some(c) => Some(c.await?),
            None => None,
        };

        let tls_client =
            super::epp_like::tls_client::TLSClient::new((&conf).into(), pkcs11_engine).await?;

        Ok(Self {
            log_dir: conf.log_dir,
            host: conf.host.to_string(),
            tag: conf.tag.to_string(),
            password: conf.password.to_string(),
            new_password: conf.new_password.into().map(|c| c.to_string()),
            pipelining: conf.pipelining,
            keepalive: conf.keepalive,
            features: ServerFeatures {
                errata: conf.errata,
                ..Default::default()
            },
            server_id: String::new(),
            is_awaiting_response: false,
            is_closing: false,
            nominet_tag_list_subordinate: false,
            nominet_tag_list_subordinate_client: None,
            nominet_dac_subordinate_client: None,
            nominet_dac_client,
            router: outer_router::Router::default(),
            tls_client,
        })
    }

    async fn _main_loop(
        &mut self,
        receiver: futures::channel::mpsc::Receiver<RequestMessage>,
        mut ready_sender: futures::channel::mpsc::UnboundedSender<CommandTransactionID>,
    ) {
        let mut receiver = receiver.fuse();
        loop {
            self.is_closing = false;
            self.is_awaiting_response = false;

            let mut sock = {
                trace!("Getting connection for {}", self.host);
                let connect_fut = self.tls_client.connect().fuse();
                futures::pin_mut!(connect_fut);

                loop {
                    futures::select! {
                        x = receiver.next() => {
                            match x {
                                Some(x) => outer_router::Router::<router::Router, ServerFeatures>::reject_request(x),
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
            trace!("Got connection for {}", self.host);

            let setup_res = {
                let exit_str = format!("All senders for {} dropped, exiting...", self.host);
                trace!("Setting up connection to {}", self.host);
                let setup_fut = self._setup_connection(&mut sock).fuse();
                futures::pin_mut!(setup_fut);
                match loop {
                    futures::select! {
                        x = receiver.next() => {
                            match x {
                                Some(x) => outer_router::Router::<router::Router, ServerFeatures>::reject_request(x),
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
                    Ok(s) => s,
                    Err(r) => {
                        if r {
                            break;
                        } else {
                            tokio::time::sleep(tokio::time::Duration::new(5, 0)).await;
                            continue;
                        }
                    }
                }
            };
            trace!("Connection setup to {}", self.host);
            let _ = ready_sender.send(setup_res).await;

            let (sock_read, mut sock_write) = tokio::io::split(sock);
            let msg_receiver = super::epp_like::ClientReceiver {
                host: self.host.clone(),
                reader: sock_read,
                root: self.log_dir.clone(),
                decode_fn: recv_msg,
            };
            let mut message_channel = msg_receiver.run().fuse();
            let mut keepalive_interval = tokio::time::interval(tokio::time::Duration::new(120, 0));

            trace!("Entering event loop for {}", self.host);
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
        if self.keepalive {
            let message = proto::EPPMessage {
                message: proto::EPPMessageType::Hello {},
            };
            self.is_awaiting_response = true;
            let receiver = super::epp_like::send_msg(
                &self.host,
                sock_write,
                &self.log_dir,
                send_msg,
                &message,
            )
            .fuse();
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
        } else {
            Ok(())
        }
    }

    async fn _handle_request<W: std::marker::Unpin + tokio::io::AsyncWrite>(
        &mut self,
        req: outer_router::RequestMessage,
        sock_write: &mut W,
    ) -> Result<(), ()> {
        match (
            req,
            self.nominet_tag_list_subordinate,
            &mut self.nominet_dac_subordinate_client,
        ) {
            (outer_router::RequestMessage::NominetTagList(t), false, _) => {
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
            (outer_router::RequestMessage::DomainCheck(t), _, Some(dac_client)) => match dac_client
                .send(outer_router::RequestMessage::DomainCheck(t))
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => {
                    warn!("Failed to send to subordinate DAC server: {}", e);
                    Err(())
                }
            },
            (outer_router::RequestMessage::DACDomain(t), _, Some(dac_client)) => {
                match dac_client
                    .send(outer_router::RequestMessage::DACDomain(t))
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("Failed to send to subordinate DAC server: {}", e);
                        Err(())
                    }
                }
            }
            (outer_router::RequestMessage::DACUsage(t), _, Some(dac_client)) => {
                match dac_client
                    .send(outer_router::RequestMessage::DACUsage(t))
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("Failed to send to subordinate DAC server: {}", e);
                        Err(())
                    }
                }
            }
            (outer_router::RequestMessage::DACLimits(t), _, Some(dac_client)) => {
                match dac_client
                    .send(outer_router::RequestMessage::DACLimits(t))
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("Failed to send to subordinate DAC server: {}", e);
                        Err(())
                    }
                }
            }
            (outer_router::RequestMessage::Hello(_), _, _) => {
                match &mut self.nominet_tag_list_subordinate_client {
                    Some(client) => {
                        let (sender, _) = futures::channel::oneshot::channel();
                        match client
                            .send(outer_router::RequestMessage::Hello(Box::new(
                                BlankRequest {
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
                let message = proto::EPPMessage {
                    message: proto::EPPMessageType::Hello {},
                };
                match super::epp_like::send_msg(
                    &self.host,
                    sock_write,
                    &self.log_dir,
                    send_msg,
                    &message,
                )
                .await
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                }
            }
            (outer_router::RequestMessage::Logout(t), _, _) => {
                match &mut self.nominet_tag_list_subordinate_client {
                    Some(client) => {
                        let (sender, _) = futures::channel::oneshot::channel();
                        match client
                            .send(outer_router::RequestMessage::Logout(Box::new(
                                BlankRequest {
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
                match &mut self.nominet_dac_subordinate_client {
                    Some(dac_client) => {
                        let (sender, _) = futures::channel::oneshot::channel();
                        match dac_client
                            .send(outer_router::RequestMessage::Logout(Box::new(
                                BlankRequest {
                                    return_path: sender,
                                },
                            )))
                            .await
                        {
                            Ok(_) => {}
                            Err(e) => {
                                warn!("Failed to send to subordinate DAC server: {}", e);
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
            (req, _, _) => match self.router.handle_request(&self.features, req) {
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
        sock: &mut super::epp_like::tls_client::TLSConnection,
    ) -> Result<CommandTransactionID, bool> {
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
                Ok(res) => Ok(CommandTransactionID {
                    client: res.client_transaction_id.unwrap_or_default(),
                    server: res.server_transaction_id.unwrap_or_default(),
                }),
                Err(_) => {
                    info!("Restarting connection...");
                    self._close(sock).await;
                    Err(false)
                }
            }
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
        self.features.eurid_poll_supported = greeting
            .service_menu
            .supports_ext("http://www.eurid.eu/xml/epp/poll-1.2");
        self.features.eurid_idn_supported = greeting
            .service_menu
            .supports_ext("http://www.eurid.eu/xml/epp/idn-1.0");
        self.features.eurid_homoglyph_supported = greeting
            .service_menu
            .supports_ext("http://www.eurid.eu/xml/epp/homoglyph-1.0");
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
        self.features.isnic_list_supported = greeting
            .service_menu
            .supports("urn:is.isnic:xml:ns:is-ext-list-1.0");
        self.features.email_forward_supported = greeting
            .service_menu
            .supports("http://www.nic.name/epp/emailFwd-1.0");
        self.features.personal_registration_supported = greeting
            .service_menu
            .supports_ext("http://www.nic.name/epp/persReg-1.0");

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

    async fn _login(
        &mut self,
        sock: &mut super::epp_like::tls_client::TLSConnection,
    ) -> Result<proto::EPPTransactionIdentifier, ()> {
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
            if self.features.eurid_poll_supported {
                ext_objects.push("http://www.eurid.eu/xml/epp/poll-1.2".to_string())
            }
            if self.features.eurid_homoglyph_supported {
                ext_objects.push("http://www.eurid.eu/xml/epp/homoglyph-1.0".to_string())
            }
            if self.features.eurid_idn_supported {
                ext_objects.push("http://www.eurid.eu/xml/epp/idn-1.0".to_string())
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
            if self.features.isnic_list_supported {
                objects.push("urn:is.isnic:xml:ns:is-ext-list-1.0".to_string())
            }
            if self.features.email_forward_supported {
                objects.push("http://www.nic.name/epp/emailFwd-1.0".to_string())
            }
            if self.features.personal_registration_supported {
                ext_objects.push("http://www.nic.name/epp/persReg-1.0".to_string())
            }
            if self.features.nominet_tag_list {
                let new_client = Self {
                    host: self.host.clone(),
                    tag: self.tag.clone(),
                    password: self.password.clone(),
                    nominet_tag_list_subordinate: true,
                    log_dir: self.log_dir.clone(),
                    new_password: None,
                    pipelining: self.pipelining,
                    keepalive: self.keepalive,
                    features: ServerFeatures {
                        errata: self.features.errata.clone(),
                        ..Default::default()
                    },
                    server_id: String::new(),
                    is_awaiting_response: false,
                    is_closing: false,
                    nominet_tag_list_subordinate_client: None,
                    nominet_dac_subordinate_client: None,
                    nominet_dac_client: None,
                    router: outer_router::Router::default(),
                    tls_client: self.tls_client.clone(),
                };
                self.nominet_tag_list_subordinate_client = Some(Box::new(new_client).start().0);
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
                Ok(r) => return Ok(r),
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
            Ok(r) => Ok(r),
            Err(_) => Err(()),
        }
    }

    async fn _try_login(
        &mut self,
        password: String,
        new_password: Option<String>,
        objects: Vec<String>,
        ext_objects: Vec<String>,
        sock: &mut super::epp_like::tls_client::TLSConnection,
    ) -> Result<proto::EPPTransactionIdentifier, bool> {
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
                Ok(response.transaction_id)
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
        match super::epp_like::send_msg(&self.host, sock, &self.log_dir, send_msg, &message).await {
            Ok(_) => Ok(message_id),
            Err(_) => Err(()),
        }
    }

    async fn _close(&mut self, sock: &mut super::epp_like::tls_client::TLSConnection) {
        self.router.drain();
        sock.close().await
    }
}

pub fn handle_logout(_client: &ServerFeatures, _req: &BlankRequest) -> router::HandleReqReturn<()> {
    Ok((proto::EPPCommandType::Logout {}, None))
}

pub fn handle_logout_response(_response: proto::EPPResponse) -> super::Response<()> {
    super::Response::Ok(())
}

impl From<&proto::EPPTransferStatus> for super::TransferStatus {
    fn from(from: &proto::EPPTransferStatus) -> Self {
        use proto::EPPTransferStatus;
        match from {
            EPPTransferStatus::ClientApproved => super::TransferStatus::ClientApproved,
            EPPTransferStatus::ClientCancelled => super::TransferStatus::ClientCancelled,
            EPPTransferStatus::ClientRejected => super::TransferStatus::ClientRejected,
            EPPTransferStatus::Pending => super::TransferStatus::Pending,
            EPPTransferStatus::ServerApproved => super::TransferStatus::ServerApproved,
            EPPTransferStatus::ServerCancelled => super::TransferStatus::ServerCancelled,
        }
    }
}
