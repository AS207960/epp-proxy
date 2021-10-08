use super::{router as outer_router, RequestMessage};
use futures::future::FutureExt;
use futures::stream::StreamExt;
use tokio::io::AsyncWriteExt;

mod recv;
pub(self) mod proto;
pub(self) mod router;

#[derive(Debug)]
pub struct DACClient {
    router: outer_router::Router<router::Router, ()>,
    is_closing: bool,
    rt_host: String,
    td_host: String,
}

impl super::Client for DACClient {
    // Starts up the DAC client and returns the sending end of a tokio channel to inject
    // commands into the client to be processed
    fn start(mut self: Box<Self>) -> futures::channel::mpsc::Sender<RequestMessage> {
        info!("DAC Client for {} and {} starting...", &self.rt_host, &self.td_host);
        let (sender, receiver) = futures::channel::mpsc::channel::<RequestMessage>(16);
        tokio::spawn(async move {
            self._main_loop(receiver).await;
        });
        sender
    }
}

impl DACClient {
    /// Creates a new DAC client ready to be started
    ///
    /// # Arguments
    /// * `rt_host` - Hostname and port of the real time server
    /// * `td_host` - Hostname and port of the time delay server
    pub async fn new(rt_host: &str, td_host: &str) -> std::io::Result<Self> {
        Ok(Self {
            router: outer_router::Router::default(),
            is_closing: false,
            rt_host: rt_host.to_string(),
            td_host: td_host.to_string(),
        })
    }

    async fn _main_loop(&mut self, receiver: futures::channel::mpsc::Receiver<RequestMessage>) {
        let mut receiver = receiver.fuse();
        loop {
            self.is_closing = false;

            let (rt_sock, td_sock) = {
                trace!("Getting connection for {} and {}", self.rt_host, self.td_host);
                let connect_fut = self._connect().fuse();
                futures::pin_mut!(connect_fut);

                loop {
                    futures::select! {
                        x = receiver.next() => {
                            match x {
                                Some(x) => outer_router::Router::<router::Router, ()>::reject_request(x),
                                None => {
                                    info!("All senders for {}/{} dropped, exiting...", self.rt_host, self.td_host);
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
            trace!("Got connection for {} and {}", self.rt_host, self.td_host);

            let (rt_sock_read, mut rt_sock_write) = tokio::io::split(td_sock);
            let (td_sock_read, mut td_sock_write) = tokio::io::split(rt_sock);
            let rt_sock_read = tokio::io::BufReader::new(rt_sock_read);
            let td_sock_read = tokio::io::BufReader::new(td_sock_read);
            let rt_msg_receiver = recv::ClientReceiver {
                host: self.rt_host.clone(),
                reader: rt_sock_read,
            };
            let td_msg_receiver = recv::ClientReceiver {
                host: self.td_host.clone(),
                reader: td_sock_read,
            };
            let mut rt_message_channel = rt_msg_receiver.run().fuse();
            let mut td_message_channel = td_msg_receiver.run().fuse();

            trace!("Entering event loop for {}/{}", self.rt_host, self.td_host);
            loop {
                futures::select! {
                    r = receiver.next() => {
                        match r {
                            Some(r) => match self._handle_request(r, &mut rt_sock_write, &mut td_sock_write).await {
                                Ok(_) => {},
                                Err(_) => {
                                    tokio::time::sleep(tokio::time::Duration::new(5, 0)).await;
                                    break;
                                }
                            },
                            None => {
                                info!("All senders for {}/{} dropped, exiting...", self.rt_host, self.td_host);
                                return
                            }
                        };
                    }
                    m = rt_message_channel.next() => {
                        match m {
                            Some(m) => match m {
                                Ok(m) => match self._handle_rt_response(m).await {
                                    Ok(c) => if c && self.is_closing {
                                        info!("Closing connection to {}...", self.rt_host);
                                        return
                                    },
                                    Err(_) => break
                                },
                                Err(c) => if c && self.is_closing {
                                    info!("Closing connection to {}...", self.rt_host);
                                    return
                                } else {
                                    break
                                }
                            },
                            None => break
                        }
                    }
                    m = td_message_channel.next() => {
                        match m {
                            Some(m) => match m {
                                Ok(m) => match self._handle_td_response(m).await {
                                    Ok(c) => if c && self.is_closing {
                                        info!("Closing connection to {}...", self.td_host);
                                        return
                                    },
                                    Err(_) => break
                                },
                                Err(c) => if c && self.is_closing {
                                    info!("Closing connection to {}...", self.td_host);
                                    return
                                } else {
                                    break
                                }
                            },
                            None => break
                        }
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::new(5, 0)).await;
        }
    }

    async fn _handle_request<W: std::marker::Unpin + tokio::io::AsyncWrite>(
        &mut self,
        req: outer_router::RequestMessage,
        rt_sock_write: &mut W,
        td_sock_write: &mut W,
    ) -> Result<(), ()> {
        match self.router.handle_request(&(), req) {
            Some(((command, env), _)) => {
                if env == router::DACEnv::RealTime || env == router::DACEnv::Both {
                    match self
                        ._send_command(command.clone(), rt_sock_write)
                        .await
                    {
                        Ok(_) => {}
                        Err(_) => return Err(()),
                    }
                }
                if env == router::DACEnv::TimeDelay || env == router::DACEnv::Both {
                    match self
                        ._send_command(command, td_sock_write)
                        .await
                    {
                        Ok(_) => {}
                        Err(_) => return Err(()),
                    }
                }
                Ok(())
            }
            None => Ok(()),
        }
    }

    async fn _send_command<
        W: std::marker::Unpin + tokio::io::AsyncWrite,
    >(
        &self,
        command: proto::DACRequest,
        sock: &mut W,
    ) -> Result<(), ()> {
        let data: Vec<u8> = command.into();
        match sock.write_all(&data).await {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }

    fn _get_cmd_line_from_response(res: &proto::DACResponse) -> String {
         match res {
            proto::DACResponse::DomainRT(rt) => rt.domain.clone(),
            proto::DACResponse::DomainTD(td) => td.domain.clone(),
            proto::DACResponse::AUB(aub) => aub.domain.clone(),
            proto::DACResponse::Usage(_) => "#usage".to_string(),
            proto::DACResponse::Limits(_) => "#limits".to_string(),
        }
    }

    async fn _handle_rt_response(&mut self, res: proto::DACResponse) -> Result<bool, ()> {
        let command_id = match self.router.inner.command_map.remove(&router::DACKey {
            env: router::DACEnv::RealTime,
            cmd: Self::_get_cmd_line_from_response(&res)
        }) {
            Some(c) => c,
            None => {
                warn!(
                    "Received unexpected response from {}: {:?}",
                    self.rt_host, res
                );
                return Ok(false);
            }
        };

        self.router.handle_response(&command_id, res);
        Ok(false)
    }

    async fn _handle_td_response(&mut self, res: proto::DACResponse) -> Result<bool, ()> {
        let command_id = match self.router.inner.command_map.remove(&router::DACKey {
            env: router::DACEnv::TimeDelay,
            cmd: Self::_get_cmd_line_from_response(&res)
        }) {
            Some(c) => c,
            None => {
                warn!(
                    "Received unexpected response from {}: {:?}",
                    self.td_host, res
                );
                return Ok(false);
            }
        };

        self.router.handle_response(&command_id, res);
        Ok(false)
    }

    async fn _lookup_host(host: &str) -> Result<std::net::SocketAddr, ()> {
        Ok(match tokio::net::lookup_host(host).await {
            Ok(mut s) => match s.next() {
                Some(s) => s,
                None => {
                    error!("Resolving {} returned no records", host);
                    return Err(());
                }
            },
            Err(err) => {
                error!("Failed to resolve {}: {}", host, err);
                return Err(());
            }
        })
    }

    async fn _connect(&self) -> (tokio::net::TcpStream, tokio::net::TcpStream) {
        loop {
            match self._try_connect().await {
                Ok(s) => {
                    info!("Successfully connected to {} and {}", &self.rt_host, self.td_host);
                    return s;
                }
                Err(_) => {
                    tokio::time::sleep(tokio::time::Duration::new(5, 0)).await;
                }
            }
        }
    }

    async fn _try_connect(&self) -> Result<(tokio::net::TcpStream, tokio::net::TcpStream), ()> {
        let rt_addr = Self::_lookup_host(&self.rt_host).await?;
        let td_addr = Self::_lookup_host(&self.td_host).await?;

        trace!("Opening TCP connection to {}", self.rt_host);
        let rt_socket = match tokio::net::TcpStream::connect(&rt_addr).await {
            Ok(s) => s,
            Err(err) => {
                error!("Unable to connect to {}: {}", self.rt_host, err);
                return Err(());
            }
        };

        trace!("Opening TCP connection to {}", self.td_host);
        let td_socket = match tokio::net::TcpStream::connect(&td_addr).await {
            Ok(s) => s,
            Err(err) => {
                error!("Unable to connect to {}: {}", self.td_host, err);
                return Err(());
            }
        };

        Ok((rt_socket, td_socket))
    }
}