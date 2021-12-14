use super::proto::tmch as tmch_proto;
use super::{router as outer_router, BlankRequest, RequestMessage};
use chrono::prelude::*;
use futures::future::FutureExt;
use futures::stream::StreamExt;

mod mark;
mod poll;
mod router;
mod trex;

fn recv_msg(data: String, host: &str) -> Result<tmch_proto::TMCHMessage, ()> {
    let message: tmch_proto::TMCHMessage = match xml_serde::from_str(&data) {
        Ok(m) => m,
        Err(err) => {
            error!("Invalid XML from {}: {}", host, err);
            return Err(());
        }
    };
    debug!("Decoded TMCH message from {} to: {:#?}", host, message);
    Ok(message)
}

fn send_msg(data: &tmch_proto::TMCHMessage, host: &str) -> Result<String, ()> {
    let encoded_msg = match xml_serde::to_string(data) {
        Ok(m) => m,
        Err(err) => {
            error!("Error serialising message for {}: {}", host, err);
            return Err(());
        }
    };
    Ok(encoded_msg)
}

/// Main client struct for the TMCH client
#[derive(Debug)]
pub struct TMCHClient {
    log_dir: std::path::PathBuf,
    host: String,
    client_id: String,
    password: String,
    server_id: String,
    pipelining: bool,
    is_awaiting_response: bool,
    is_closing: bool,
    router: outer_router::Router<router::Router, ()>,
    tls_client: super::epp_like::tls_client::TLSClient,
}

impl super::Client for TMCHClient {
    // Starts up the TMCH client and returns the sending end of a tokio channel to inject
    // commands into the client to be processed
    fn start(mut self: Box<Self>) -> futures::channel::mpsc::Sender<RequestMessage> {
        info!("TMCH Client for {} starting...", &self.host);
        let (sender, receiver) = futures::channel::mpsc::channel::<RequestMessage>(16);
        tokio::spawn(async move {
            self._main_loop(receiver).await;
        });
        sender
    }
}

impl TMCHClient {
    /// Creates a new TMCH client ready to be started
    ///
    /// # Arguments
    /// * `conf` - Configuration to use for this client
    pub async fn new<'a, C: Into<Option<&'a str>>>(
        conf: super::ClientConf<'a, C>,
        pkcs11_engine: Option<crate::P11Engine>,
    ) -> std::io::Result<Self> {
        let tls_client =
            super::epp_like::tls_client::TLSClient::new((&conf).into(), pkcs11_engine).await?;

        Ok(Self {
            log_dir: conf.log_dir,
            host: conf.host.to_string(),
            client_id: conf.tag.to_string(),
            password: conf.password.to_string(),
            pipelining: conf.pipelining,
            server_id: String::new(),
            is_awaiting_response: false,
            is_closing: false,
            router: outer_router::Router::default(),
            tls_client,
        })
    }

    async fn _main_loop(&mut self, receiver: futures::channel::mpsc::Receiver<RequestMessage>) {
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
                                Some(x) => outer_router::Router::<router::Router, ()>::reject_request(x),
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

            {
                let exit_str = format!("All senders for {} dropped, exiting...", self.host);
                trace!("Setting up connection to {}", self.host);
                let setup_fut = self._setup_connection(&mut sock).fuse();
                futures::pin_mut!(setup_fut);
                match loop {
                    futures::select! {
                        x = receiver.next() => {
                            match x {
                                Some(x) => outer_router::Router::<router::Router, ()>::reject_request(x),
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
            trace!("Connection setup to {}", self.host);

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
        let message = tmch_proto::TMCHMessage {
            message: tmch_proto::TMCHMessageType::Hello {},
        };
        self.is_awaiting_response = true;
        let receiver =
            super::epp_like::send_msg(&self.host, sock_write, &self.log_dir, send_msg, &message)
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
    }

    async fn _handle_request<W: std::marker::Unpin + tokio::io::AsyncWrite>(
        &mut self,
        req: outer_router::RequestMessage,
        sock_write: &mut W,
    ) -> Result<(), ()> {
        if let outer_router::RequestMessage::Logout(_) = req {
            self.is_closing = true;
        }
        match self.router.handle_request(&(), req) {
            Some((command, command_id)) => {
                self.is_awaiting_response = true;
                match self._send_command(command, sock_write, command_id).await {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                }
            }
            None => Ok(()),
        }
    }

    async fn _handle_response(&mut self, res: tmch_proto::TMCHMessage) -> Result<bool, ()> {
        self.is_awaiting_response = false;
        match res.message {
            tmch_proto::TMCHMessageType::Response(response) => {
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
            tmch_proto::TMCHMessageType::Greeting(greeting) => {
                self._process_greeting(greeting).await.map(|_| false)
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
    ) -> Result<(), bool> {
        let msg = match super::epp_like::recv_msg(sock, &self.host, &self.log_dir, recv_msg).await {
            Ok(m) => m,
            Err(_) => {
                info!("Restarting connection...");
                self._close(sock).await;
                return Err(false);
            }
        };

        if let tmch_proto::TMCHMessageType::Greeting(greeting) = msg.message {
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

    async fn _process_greeting(&mut self, greeting: tmch_proto::TMCHGreeting) -> Result<(), ()> {
        if (greeting.server_date - Utc::now()).num_minutes() >= 5 {
            warn!(
                "Local time out by more than 5 minutes from time reported by {}",
                greeting.server_id
            );
        }

        Ok(())
    }

    async fn _login(
        &mut self,
        sock: &mut super::epp_like::tls_client::TLSConnection,
    ) -> Result<(), ()> {
        let command = tmch_proto::TMCHLogin {
            client_id: self.client_id.clone(),
            password: self.password.clone(),
            services: Some(tmch_proto::TMCHLoginServices {
                extensions: Some(tmch_proto::TMCHLoginServiceExtension {
                    uris: vec![
                        "urn:ietf:params:xml:ns:tmch:variation".to_string(),
                        "urn:ietf:params:xml:ns:tmch:trex".to_string(),
                        "urn:ietf:params:xml:ns:brandPulse-1.0".to_string(),
                    ],
                }),
            }),
        };

        match self
            ._send_command(tmch_proto::TMCHCommandType::Login(command), sock, None)
            .await
        {
            Ok(i) => i,
            Err(_) => {
                error!("Failed to send login command");
                return Err(());
            }
        };
        let msg = match super::epp_like::recv_msg(sock, &self.host, &self.log_dir, recv_msg).await {
            Ok(msg) => msg,
            Err(_) => {
                error!("Failed to receive login response");
                return Err(());
            }
        };
        if let tmch_proto::TMCHMessageType::Response(response) = msg.message {
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
        command: tmch_proto::TMCHCommandType,
        sock: &mut W,
        message_id: M,
    ) -> Result<uuid::Uuid, ()> {
        let message_id = match message_id.into() {
            Some(m) => m,
            None => uuid::Uuid::new_v4(),
        };
        let command = tmch_proto::TMCHCommand {
            command,
            client_transaction_id: Some(message_id.to_hyphenated().to_string()),
            extension: None,
        };
        let message = tmch_proto::TMCHMessage {
            message: tmch_proto::TMCHMessageType::Command(Box::new(command)),
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

pub fn handle_logout(_client: &(), _req: &BlankRequest) -> router::HandleReqReturn<()> {
    Ok(tmch_proto::TMCHCommandType::Logout {})
}

pub fn handle_logout_response(_response: tmch_proto::TMCHResponse) -> super::Response<()> {
    super::Response::Ok(())
}
