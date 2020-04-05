//! # Async/await EPP client.
//!
//! Messages should be injected into the server using the helper functions in subordinate modules such as
//! [`contact`], [`host`], and [`domain`].

use crate::{proto, xml_ser};
use chrono::prelude::*;
use futures::future::FutureExt;
use futures::stream::StreamExt;
use native_tls::TlsConnector;
use tokio::net::TcpStream;
use tokio::prelude::*;

pub mod contact;
pub mod domain;
pub mod host;
pub mod router;

pub use router::{Request, Response};

type Sender<T> = futures::channel::oneshot::Sender<Response<T>>;

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
async fn recv_msg<R: std::marker::Unpin + tokio::io::AsyncRead>(
    sock: &mut R,
    host: &str,
) -> Result<proto::EPPMessage, ()> {
    let data_len = match sock.read_u32().await {
        Ok(l) => l - 4,
        Err(err) => {
            error!("Error reading next data unit length from {}: {}", host, err);
            return Err(());
        }
    };
    let mut data_buf = vec![0u8; data_len as usize];
    match sock.read_exact(&mut data_buf).await {
        Ok(n) => {
            if n != data_len as usize {
                error!("Read less data than expected from {}", host);
                return Err(());
            }
        }
        Err(err) => {
            error!("Error reading next data from {}: {}", host, err);
            return Err(());
        }
    }
    let data = match String::from_utf8(data_buf) {
        Ok(s) => s,
        Err(err) => {
            error!("Invalid UTF8 from {}: {}", host, err);
            return Err(());
        }
    };
    debug!("Received EPP message with contents: {}", data);
    let message: proto::EPPMessage = match quick_xml::de::from_str(&data) {
        Ok(m) => m,
        Err(err) => {
            error!("Invalid XML from {}: {}", host, err);
            return Err(());
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
}

impl EPPClientReceiver {
    /// Starts the tokio task, and returns the receiving end of the channel to read messages from.
    fn run(mut self) -> futures::channel::mpsc::Receiver<Result<proto::EPPMessage, ()>> {
        let (mut sender, receiver) =
            futures::channel::mpsc::channel::<Result<proto::EPPMessage, ()>>(16);
        tokio::spawn(async move {
            loop {
                let msg = recv_msg(&mut self.reader, &self.host).await;
                match futures::future::poll_fn(|cx| sender.poll_ready(cx)).await {
                    Ok(_) => {}
                    Err(_) => break,
                }
                sender.start_send(msg).unwrap();
            }
        });
        receiver
    }
}

/// Main client struct for the EEP client
#[derive(Debug, Default)]
pub struct EPPClient {
    host: String,
    tag: String,
    password: String,
    server_id: String,
    /// Is the EPP server in a state to receive and process commands
    ready: bool,
    router: router::Router,
    /// What features does the server support
    features: EPPClientServerFeatures,
}

/// Features supported by the EPP server
#[derive(Debug, Default)]
pub struct EPPClientServerFeatures {
    /// RFC 5731 support
    domain_supported: bool,
    /// RFC 5732 support
    host_supported: bool,
    /// RFC 5733 support
    contact_supported: bool,
}

impl EPPClient {
    /// Creates a new EPP client ready to be started
    ///
    /// # Arguments
    /// * `host` - The server connection string, in the form `domain:port`
    /// * `tag` - The client ID/tag to login with
    /// * `password` - The password to login with
    pub fn new(host: &str, tag: &str, password: &str) -> Self {
        Self {
            host: host.to_string(),
            tag: tag.to_string(),
            password: password.to_string(),
            ..Default::default()
        }
    }

    /// Starts up the EPP server and returns the sending end of a tokio channel to inject
    /// commands into the client to be processed
    pub fn start(mut self) -> futures::channel::mpsc::Sender<Request> {
        info!("EPP Client for {} starting...", &self.host);
        let (sender, receiver) = futures::channel::mpsc::channel::<Request>(16);
        tokio::spawn(async move {
            self._main_loop(receiver).await;
        });
        sender
    }

    async fn _main_loop(&mut self, receiver: futures::channel::mpsc::Receiver<Request>) {
        let mut receiver = receiver.fuse();
        loop {
            let mut sock = {
                let connect_fut = self._connect().fuse();
                futures::pin_mut!(connect_fut);

                loop {
                    futures::select! {
                        x = receiver.next() => {
                            match x {
                                Some(x) => router::Router::reject_request(x),
                                None => return
                            };
                        }
                        s = connect_fut => {
                            break s;
                        }
                    }
                }
            };

            {
                let setup_fut = self._setup_connection(&mut sock).fuse();
                futures::pin_mut!(setup_fut);
                match loop {
                    futures::select! {
                        x = receiver.next() => {
                            match x {
                                Some(x) => router::Router::reject_request(x),
                                None => return
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
                            tokio::time::delay_for(tokio::time::Duration::new(5, 0)).await;
                            break;
                        } else {
                            continue;
                        }
                    }
                }
            }

            let (sock_read, mut sock_write) = tokio::io::split(sock);
            let msg_receiver = EPPClientReceiver {
                host: self.host.clone(),
                reader: sock_read,
            };
            let mut message_channel = msg_receiver.run().fuse();

            loop {
                futures::select! {
                    r = receiver.next() => {
                        match r {
                            Some(r) => {
                                match self.router.handle_request(&self.features, r).await {
                                    Some((command, command_id)) => {
                                        match self._send_command(command, &mut sock_write, command_id).await {
                                            Ok(_) => {},
                                            Err(_) => {
                                                tokio::time::delay_for(tokio::time::Duration::new(5, 0)).await;
                                                break;
                                            }
                                        }
                                    }
                                    None => {}
                                }
                            },
                            None => return
                        };
                    }
                    m = message_channel.next() => {
                        match m {
                            Some(m) => match m {
                                Ok(m) => match self._handle_response(m).await {
                                    Ok(_) => {},
                                    Err(_) => break
                                },
                                Err(_) => break
                            },
                            None => break
                        }
                    }
                }
            }
            tokio::time::delay_for(tokio::time::Duration::new(5, 0)).await;
        }
    }

    async fn _handle_response(&mut self, res: proto::EPPMessage) -> Result<(), ()> {
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
                self.router.handle_response(&transaction_id, response).await?;
                if is_closing {
                    Err(())
                } else {
                    Ok(())
                }
            }
            o => {
                warn!(
                    "Received unexpected response from {}: {:?}",
                    self.server_id, o
                );
                Ok(())
            }
        }
    }

    async fn _setup_connection(
        &mut self,
        sock: &mut tokio_tls::TlsStream<TcpStream>,
    ) -> Result<(), bool> {
        let msg = match recv_msg(sock, &self.host).await {
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
        if !greeting.service_menu.languages.contains(&"en".to_string()) {
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
            .objects
            .iter()
            .any(|e| e == "urn:ietf:params:xml:ns:contact-1.0");
        self.features.domain_supported = greeting
            .service_menu
            .objects
            .iter()
            .any(|e| e == "urn:ietf:params:xml:ns:domain-1.0");
        self.features.host_supported = greeting
            .service_menu
            .objects
            .iter()
            .any(|e| e == "urn:ietf:params:xml:ns:host-1.0");

        if !(self.features.contact_supported
            | self.features.domain_supported
            | self.features.host_supported)
        {
            error!("No common supported objects with {}", greeting.server_id);
            return Err(());
        }
        Ok(())
    }

    async fn _login(&self, sock: &mut tokio_tls::TlsStream<TcpStream>) -> Result<(), ()> {
        let mut objects = vec![];

        if self.features.contact_supported {
            objects.push("urn:ietf:params:xml:ns:contact-1.0".to_string())
        }
        if self.features.domain_supported {
            objects.push("urn:ietf:params:xml:ns:domain-1.0".to_string())
        }
        if self.features.host_supported {
            objects.push("urn:ietf:params:xml:ns:host-1.0".to_string())
        }

        let login_command = proto::EPPLogin {
            client_id: self.tag.clone(),
            password: self.password.clone(),
            new_password: None,
            options: proto::EPPLoginOptions {
                version: "1.0".to_string(),
                language: "en".to_string(),
            },
            services: proto::EPPLoginServices {
                objects,
                extension: None,
            },
        };

        match self
            ._send_command(proto::EPPCommandType::Login(login_command), sock, None)
            .await
        {
            Ok(i) => i,
            Err(_) => {
                error!("Failed to send login command");
                return Err(());
            }
        };
        let msg = match recv_msg(sock, &self.host).await {
            Ok(msg) => msg,
            Err(_) => {
                error!("Failed to receive login response");
                return Err(());
            }
        };
        if let proto::EPPMessageType::Response(response) = msg.message {
            if !response.is_success() {
                error!(
                    "Login to {} failed with error: {}",
                    self.server_id,
                    response.response_msg()
                );
                Err(())
            } else {
                info!("Successfully logged into {}", self.server_id);
                Ok(())
            }
        } else {
            error!(
                "Didn't receive response to login command from {}",
                self.server_id
            );
            Err(())
        }
    }

    async fn _send_command<
        W: std::marker::Unpin + tokio::io::AsyncWrite,
        M: Into<Option<uuid::Uuid>>,
    >(
        &self,
        command: proto::EPPCommandType,
        sock: &mut W,
        message_id: M,
    ) -> Result<uuid::Uuid, ()> {
        let message_id = match message_id.into() {
            Some(m) => m,
            None => uuid::Uuid::new_v4(),
        };
        let command = proto::EPPCommand {
            command,
            client_transaction_id: Some(message_id.to_hyphenated().to_string()),
        };
        let message = proto::EPPMessage {
            message: proto::EPPMessageType::Command(command),
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
        let encoded_msg =
            xml_ser::to_string(message, "epp", "urn:ietf:params:xml:ns:epp-1.0").unwrap();
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
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Error writing data unit to {}: {}", &self.host, err);
                Err(())
            }
        }
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
        let cx = TlsConnector::builder().build().unwrap();
        let cx = tokio_tls::TlsConnector::from(cx);
        let socket = match cx.connect(&self.host, socket).await {
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
    client_sender.try_send(req).unwrap();
    let mut receiver = receiver.fuse();
    let mut delay = tokio::time::delay_for(tokio::time::Duration::new(5, 0)).fuse();
    let resp = futures::select! {
        r = receiver => r,
        _ = delay => {
            return Err(Error::Timeout);
        }
    };
    let resp = match resp {
        Ok(r) => r,
        Err(_) => return Err(Error::InternalServerError),
    };
    match resp {
        Response::Ok(r) => Ok(r),
        Response::InternalServerError => Err(Error::InternalServerError),
        Response::Unsupported => Err(Error::Unsupported),
        Response::NotReady => Err(Error::NotReady),
        Response::Err(s) => Err(Error::Err(s)),
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
