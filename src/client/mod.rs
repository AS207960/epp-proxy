//! # Async/await EPP client.
//!
//! Messages should be injected into the server using the helper functions in subordinate modules such as
//! [`contact`], [`host`], and [`domain`].

use crate::proto;
use chrono::prelude::*;
use futures::future::FutureExt;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use native_tls::TlsConnector;
use tokio::net::TcpStream;
use tokio::prelude::*;

pub mod balance;
pub mod contact;
pub mod domain;
pub mod host;
pub mod nominet;
pub mod poll;
pub mod rgp;
pub mod router;
pub mod verisign;
pub mod fee;
pub mod launch;

use crate::proto::EPPServiceExtension;
pub use router::{Request, Response};

type Sender<T> = futures::channel::oneshot::Sender<Response<T>>;
pub type RequestSender = futures::channel::mpsc::Sender<router::Request>;

async fn write_msg_log(msg: &str, msg_type: &str, root: &std::path::Path) -> tokio::io::Result<()> {
    let now = Utc::now();
    let date = now.format("%F").to_string();
    let time = now.format("%H-%M-%S-%f").to_string();
    let dir = root.join(date);
    let file_path = dir.join(format!("{}_{}.xml", time, msg_type));
    tokio::fs::create_dir_all(&dir).await?;
    let mut file = tokio::fs::File::create(file_path).await?;
    file.write(msg.as_bytes()).await?;
    Ok(())
}

/// Attempts to read and decode an EPP message.
///
/// Reads from a tokio async reader in conformance with RFC 5734 for the binary message data,
/// and decodes in conformance with RFC 5730 and related documents for said message.
/// Will return `Ok(proto::EPPMessage)` on success or `Err(())` on any error.
/// In such cases the client should close the connection and restart.
///
/// # Arguments
/// * `sock` - A tokio async reader
/// * `host` - Host name for error reporting
/// * `root` - Root dir for storing message logs
async fn recv_msg<R: std::marker::Unpin + tokio::io::AsyncRead>(
    sock: &mut R,
    host: &str,
    root: &std::path::Path,
) -> Result<proto::EPPMessage, bool> {
    let data_len = match sock.read_u32().await {
        Ok(l) => l - 4,
        Err(err) => {
            return Err(match err.kind() {
                std::io::ErrorKind::UnexpectedEof => {
                    warn!("{} has closed the connection", host);
                    true
                }
                _ => {
                    error!("Error reading next data unit length from {}: {}", host, err);
                    false
                }
            });
        }
    };
    let mut data_buf = vec![0u8; data_len as usize];
    match sock.read_exact(&mut data_buf).await {
        Ok(n) => {
            if n != data_len as usize {
                error!("Read less data than expected from {}", host);
                return Err(false);
            }
        }
        Err(err) => {
            error!("Error reading next data from {}: {}", host, err);
            return Err(match err.kind() {
                std::io::ErrorKind::UnexpectedEof => true,
                _ => false,
            });
        }
    }
    let data = match String::from_utf8(data_buf) {
        Ok(s) => s,
        Err(err) => {
            error!("Invalid UTF8 from {}: {}", host, err);
            return Err(false);
        }
    };
    debug!("Received EPP message with contents: {}", data);
    match write_msg_log(&data, "recv", root).await {
        Ok(_) => {}
        Err(e) => {
            error!("Failed writing received message to message log: {}", e);
        }
    }
    let message: proto::EPPMessage = match xml_serde::from_str(&data) {
        Ok(m) => m,
        Err(err) => {
            error!("Invalid XML from {}: {}", host, err);
            return Err(false);
        }
    };
    debug!("Decoded EPP message to: {:#?}", message);
    Ok(message)
}

/// Tokio task that attemps to read in EPP messages and push them onto a tokio channel as received.
struct EPPClientReceiver {
    /// Host name for error reporting
    host: String,
    /// Read half of the TLS stream used to connect to the server
    reader: tokio::io::ReadHalf<tokio_tls::TlsStream<TcpStream>>,
    /// Path to store received messages in
    root: std::path::PathBuf,
}

impl EPPClientReceiver {
    /// Starts the tokio task, and returns the receiving end of the channel to read messages from.
    fn run(mut self) -> futures::channel::mpsc::Receiver<Result<proto::EPPMessage, bool>> {
        let (mut sender, receiver) =
            futures::channel::mpsc::channel::<Result<proto::EPPMessage, bool>>(16);
        tokio::spawn(async move {
            loop {
                let msg = recv_msg(&mut self.reader, &self.host, &self.root).await;
                let is_close = if let Err(c) = &msg { *c } else { false };
                match sender.send(msg).await {
                    Ok(_) => {}
                    Err(_) => break,
                }
                if is_close {
                    break;
                }
            }
        });
        receiver
    }
}

/// Main client struct for the EEP client
#[derive(Debug, Default)]
pub struct EPPClient {
    log_dir: std::path::PathBuf,
    host: String,
    tag: String,
    password: String,
    new_password: Option<String>,
    client_cert: Option<String>,
    root_certs: Vec<String>,
    danger_accept_invalid_certs: bool,
    danger_accept_invalid_hostname: bool,
    server_id: String,
    pipelining: bool,
    is_awaiting_response: bool,
    is_closing: bool,
    /// Is the EPP server in a state to receive and process commands
    ready: bool,
    router: router::Router,
    /// What features does the server support
    features: EPPClientServerFeatures,
    nominet_tag_list_subordinate: bool,
    nominet_tag_list_subordinate_client: Option<futures::channel::mpsc::Sender<Request>>,
}

/// Features supported by the EPP server
#[derive(Debug, Default)]
pub struct EPPClientServerFeatures {
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
    /// urn:ietf:params:xml:ns:nsset-1.2 support (NOT AN ACTUAL IETF NAMESPACE)
    nsset_supported: bool,
    /// RFC 8748 support
    fee_supported: bool,
    /// RFC 8334 support
    launch_supported: bool,
    /// urn:ietf:params:xml:ns:fee-0.9 support
    fee_09_supported: bool,
    /// urn:ietf:params:xml:ns:fee-0.8 support
    fee_08_supported: bool,
    /// urn:ietf:params:xml:ns:fee-0.7 support
    fee_07_supported: bool,
    /// urn:ietf:params:xml:ns:fee-0.5 support
    fee_05_supported: bool,
}

impl EPPClientServerFeatures {
    fn has_erratum(&self, name: &str) -> bool {
        match &self.errata {
            Some(s) => s == name,
            None => false,
        }
    }
}

pub struct ClientConf<'a, C> {
    /// The server connection string, in the form `domain:port`
    pub host: &'a str,
    /// The client ID/tag to login with
    pub tag: &'a str,
    /// The password to login with
    pub password: &'a str,
    /// Directory path to log commands to
    pub log_dir: std::path::PathBuf,
    /// PCKS#12 file path for client identity
    pub client_cert: C,
    /// List of PEM file paths
    pub root_certs: &'a[&'a str],
    /// Accept invalid TLS certs
    pub danger_accept_invalid_certs: bool,
    /// Accept TLS certs with a hostname that doesn't match the DNS label
    pub danger_accept_invalid_hostname: bool,
    /// New password to set after login
    pub new_password: C,
    /// Does the server support multiple commands in flight at once
    pub pipelining: bool,
    /// Errata of this server
    pub errata: Option<String>,
}

impl EPPClient {
    /// Creates a new EPP client ready to be started
    ///
    /// # Arguments
    /// * `conf` - Configuration to use for this client
    pub fn new<'a, C: Into<Option<&'a str>>>(conf: ClientConf<'a, C>) -> Self {
        Self {
            log_dir: conf.log_dir,
            host: conf.host.to_string(),
            tag: conf.tag.to_string(),
            password: conf.password.to_string(),
            client_cert: conf.client_cert.into().map(|c| c.to_string()),
            root_certs: conf.root_certs.into_iter().map(|c| c.to_string()).collect(),
            danger_accept_invalid_certs: conf.danger_accept_invalid_certs,
            danger_accept_invalid_hostname: conf.danger_accept_invalid_hostname,
            new_password: conf.new_password.into().map(|c| c.to_string()),
            pipelining: conf.pipelining,
            features: EPPClientServerFeatures {
                errata: conf.errata,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Starts up the EPP server and returns the sending end of a tokio channel to inject
    /// commands into the client to be processed
    pub fn start(mut self) -> futures::channel::mpsc::Sender<Request> {
        info!("EPP Client for {} starting...", &self.host);
        if self.nominet_tag_list_subordinate {
            info!("This is a Nominet Tag list subordinate client");
        }
        let (sender, receiver) = futures::channel::mpsc::channel::<Request>(16);
        tokio::spawn(async move {
            self._main_loop(receiver).await;
        });
        sender
    }

    async fn _main_loop(&mut self, receiver: futures::channel::mpsc::Receiver<Request>) {
        let mut receiver = receiver.fuse();
        loop {
            self.is_closing = false;
            self.is_awaiting_response = false;

            let mut sock = {
                let connect_fut = self._connect().fuse();
                futures::pin_mut!(connect_fut);

                loop {
                    futures::select! {
                        x = receiver.next() => {
                            match x {
                                Some(x) => router::Router::reject_request(x),
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

            {
                let exit_str = format!("All senders for {} dropped, exiting...", self.host);
                let setup_fut = self._setup_connection(&mut sock).fuse();
                futures::pin_mut!(setup_fut);
                match loop {
                    futures::select! {
                        x = receiver.next() => {
                            match x {
                                Some(x) => router::Router::reject_request(x),
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
                            tokio::time::delay_for(tokio::time::Duration::new(5, 0)).await;
                            continue;
                        }
                    }
                }
            }

            let (sock_read, mut sock_write) = tokio::io::split(sock);
            let msg_receiver = EPPClientReceiver {
                host: self.host.clone(),
                reader: sock_read,
                root: self.log_dir.clone(),
            };
            let mut message_channel = msg_receiver.run().fuse();
            let mut keepalive_interval =
                tokio::time::interval(tokio::time::Duration::new(120, 0)).fuse();

            loop {
                if self.pipelining || !self.is_awaiting_response {
                    futures::select! {
                        r = receiver.next() => {
                            match r {
                                Some(r) => match self._handle_request(r, &mut sock_write).await {
                                    Ok(_) => {},
                                    Err(_) => {
                                        tokio::time::delay_for(tokio::time::Duration::new(5, 0)).await;
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
                        _ = keepalive_interval.next() => {
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
                        tokio::time::delay_for(tokio::time::Duration::new(15, 0)).fuse();
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
            tokio::time::delay_for(tokio::time::Duration::new(5, 0)).await;
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
        let mut delay = tokio::time::delay_for(tokio::time::Duration::new(15, 0)).fuse();
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
        req: router::Request,
        sock_write: &mut W,
    ) -> Result<(), ()> {
        match (req, self.nominet_tag_list_subordinate) {
            (router::Request::NominetTagList(t), false) => {
                let client = match &mut self.nominet_tag_list_subordinate_client {
                    Some(c) => c,
                    None => unreachable!(),
                };
                match client.send(router::Request::NominetTagList(t)).await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("Failed to send to subordinate server: {}", e);
                        Err(())
                    }
                }
            }
            (router::Request::Logout(t), _) => {
                match &mut self.nominet_tag_list_subordinate_client {
                    Some(client) => {
                        let (sender, _) = futures::channel::oneshot::channel();
                        match client
                            .send(router::Request::Logout(Box::new(LogoutRequest {
                                return_path: sender,
                            })))
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
                    .handle_request(&self.features, router::Request::Logout(t))
                    .await
                {
                    Some((command, extension, command_id)) => {
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
            (req, _) => match self.router.handle_request(&self.features, req).await {
                Some((command, extension, command_id)) => {
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
                let transaction_id = match uuid::Uuid::parse_str(&transaction_id) {
                    Ok(i) => i,
                    Err(e) => {
                        error!(
                            "Received response with invalid transaction UUID from {}: {}",
                            self.server_id, e
                        );
                        return Err(());
                    }
                };
                self.router
                    .handle_response(&transaction_id, response)
                    .await?;
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
        sock: &mut tokio_tls::TlsStream<TcpStream>,
    ) -> Result<(), bool> {
        let msg = match recv_msg(sock, &self.host, &self.log_dir).await {
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
        self.features.nsset_supported = greeting
            .service_menu
            .supports("urn:ietf:params:xml:ns:nsset-1.2");
        self.features.fee_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:epp:fee-1.0");
        self.features.launch_supported = greeting
            .service_menu
            .supports_ext("urn:ietf:params:xml:ns:launch-1.0");
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

    async fn _login(&mut self, sock: &mut tokio_tls::TlsStream<TcpStream>) -> Result<(), ()> {
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
            if self.features.nsset_supported {
                objects.push("urn:ietf:params:xml:ns:nsset-1.2".to_string())
            }
            if self.features.launch_supported {
                ext_objects.push("urn:ietf:params:xml:ns:launch-1.0".to_string())
            }
            if self.features.fee_supported {
                ext_objects.push("urn:ietf:params:xml:ns:epp:fee-1.0".to_string())
            } else if self.features.fee_09_supported {
                ext_objects.push("urn:ietf:params:xml:ns:fee-0.9".to_string())
            } else if self.features.fee_08_supported {
                ext_objects.push("urn:ietf:params:xml:ns:fee-0.8".to_string())
            } else if self.features.fee_07_supported {
                ext_objects.push("urn:ietf:params:xml:ns:fee-0.7".to_string())
            } else if self.features.fee_05_supported {
                ext_objects.push("urn:ietf:params:xml:ns:fee-0.5".to_string())
            }
            if self.features.nominet_tag_list {
                let new_client = Self {
                    host: self.host.clone(),
                    tag: self.tag.clone(),
                    password: self.password.clone(),
                    nominet_tag_list_subordinate: true,
                    log_dir: self.log_dir.clone(),
                    client_cert: self.client_cert.clone(),
                    ..Default::default()
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
        sock: &mut tokio_tls::TlsStream<TcpStream>,
    ) -> Result<(), bool> {
        let command = proto::EPPLogin {
            client_id: self.tag.clone(),
            password,
            new_password,
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
        match self
            ._send_command(proto::EPPCommandType::Login(command), None, sock, None)
            .await
        {
            Ok(i) => i,
            Err(_) => {
                error!("Failed to send login command");
                return Err(true);
            }
        };
        let msg = match recv_msg(sock, &self.host, &self.log_dir).await {
            Ok(msg) => msg,
            Err(_) => {
                error!("Failed to receive login response");
                return Err(true);
            }
        };
        if let proto::EPPMessageType::Response(response) = msg.message {
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
        debug!("Sending EPP message with contents: {}", encoded_msg);
        let msg_bytes = encoded_msg.as_bytes();
        let msg_len = msg_bytes.len() + 4;
        match sock.write_u32(msg_len as u32).await {
            Ok(_) => {}
            Err(err) => {
                error!("Error writing data unit length to {}: {}", &self.host, err);
                return Err(());
            }
        };
        match sock.write(&msg_bytes).await {
            Ok(_) => {}
            Err(err) => {
                error!("Error writing data unit to {}: {}", &self.host, err);
                return Err(());
            }
        }
        match write_msg_log(&encoded_msg, "send", &self.log_dir).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed writing sent message to message log: {}", e);
            }
        }
        Ok(())
    }

    async fn _close(&mut self, sock: &mut tokio_tls::TlsStream<TcpStream>) {
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

    async fn _connect(&self) -> tokio_tls::TlsStream<TcpStream> {
        loop {
            match self._try_connect().await {
                Ok(s) => {
                    info!("Successfully connected to {}", &self.host);
                    return s;
                }
                Err(_) => {
                    tokio::time::delay_for(tokio::time::Duration::new(5, 0)).await;
                }
            }
        }
    }

    async fn _try_connect(&self) -> Result<tokio_tls::TlsStream<TcpStream>, ()> {
        let hostname = self.host.rsplitn(2, ':').collect::<Vec<_>>().pop().unwrap();
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
        let socket = match TcpStream::connect(&addr).await {
            Ok(s) => s,
            Err(err) => {
                error!("Unable to connect to {}: {}", self.host, err);
                return Err(());
            }
        };
        let mut cx = TlsConnector::builder();
        cx.danger_accept_invalid_certs(self.danger_accept_invalid_certs);
        cx.danger_accept_invalid_hostnames(self.danger_accept_invalid_hostname);
        for root_cert_path in &self.root_certs {
            let root_cert_bytes = match std::fs::read(root_cert_path) {
                Ok(p) => p,
                Err(err) => {
                    error!("Unable read root cert {}: {}", root_cert_path, err);
                    return Err(());
                }
            };
            let root_cert = match native_tls::Certificate::from_pem(&root_cert_bytes) {
                Ok(i) => i,
                Err(err) => {
                    error!("Unable read root cert {}: {}", root_cert_path, err);
                    return Err(());
                }
            };
            cx.disable_built_in_roots(true);
            cx.add_root_certificate(root_cert);
        }
        if let Some(client_cert) = &self.client_cert {
            let pkcs = match std::fs::read(client_cert) {
                Ok(p) => p,
                Err(err) => {
                    error!("Unable read client cert {}: {}", client_cert, err);
                    return Err(());
                }
            };
            let identity = match native_tls::Identity::from_pkcs12(&pkcs, "") {
                Ok(i) => i,
                Err(err) => {
                    error!("Unable read client cert {}: {}", client_cert, err);
                    return Err(());
                }
            };
            cx.identity(identity);
        }
        let cx = tokio_tls::TlsConnector::from(cx.build().unwrap());
        let socket = match cx.connect(&hostname, socket).await {
            Ok(s) => s,
            Err(err) => {
                error!("Unable to start TLS session to {}: {}", self.host, err);
                return Err(());
            }
        };
        Ok(socket)
    }
}

async fn send_epp_client_request<R>(
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
    req: Request,
    receiver: futures::channel::oneshot::Receiver<Response<R>>,
) -> Result<R, Error> {
    match client_sender.try_send(req) {
        Ok(_) => {}
        Err(_) => return Err(Error::InternalServerError),
    }
    let mut receiver = receiver.fuse();
    let mut delay = tokio::time::delay_for(tokio::time::Duration::new(15, 0)).fuse();
    let resp = futures::select! {
        r = receiver => r,
        _ = delay => {
            return Err(Error::Timeout);
        }
    };
    match resp {
        Ok(r) => r,
        Err(_) => Err(Error::InternalServerError),
    }
}

/// Possible errors returned by the EPP client
#[derive(Debug)]
pub enum Error {
    /// The EPP server is not currently able to accept requests
    NotReady,
    /// The EPP server doesn't support the requested action
    Unsupported,
    /// The EPP client or server encountered an internal unexpected error processing the request
    InternalServerError,
    /// The EPP server didn't respond in time to the request
    Timeout,
    /// The EPP server returned an error message (probably invalid parameters)
    Err(String),
}

#[derive(PartialEq, Debug)]
pub enum TransferStatus {
    ClientApproved,
    ClientCancelled,
    ClientRejected,
    Pending,
    ServerApproved,
    ServerCancelled,
}

impl From<&proto::EPPTransferStatus> for TransferStatus {
    fn from(from: &proto::EPPTransferStatus) -> Self {
        use proto::EPPTransferStatus;
        match from {
            EPPTransferStatus::ClientApproved => TransferStatus::ClientApproved,
            EPPTransferStatus::ClientCancelled => TransferStatus::ClientCancelled,
            EPPTransferStatus::ClientRejected => TransferStatus::ClientRejected,
            EPPTransferStatus::Pending => TransferStatus::Pending,
            EPPTransferStatus::ServerApproved => TransferStatus::ServerApproved,
            EPPTransferStatus::ServerCancelled => TransferStatus::ServerCancelled,
        }
    }
}

#[derive(Debug)]
pub struct LogoutRequest {
    pub return_path: Sender<()>,
}

pub fn handle_logout(
    _client: &EPPClientServerFeatures,
    _req: &LogoutRequest,
) -> router::HandleReqReturn<()> {
    Ok((proto::EPPCommandType::Logout {}, None))
}

pub fn handle_logout_response(_response: proto::EPPResponse) -> Response<()> {
    Response::Ok(())
}

/// Ends an EPP session
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn logout(
    mut client_sender: futures::channel::mpsc::Sender<Request>,
) -> Result<(), Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    send_epp_client_request(
        &mut client_sender,
        Request::Logout(Box::new(LogoutRequest {
            return_path: sender,
        })),
        receiver,
    )
    .await
}
