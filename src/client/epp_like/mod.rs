use chrono::prelude::*;
use futures::sink::SinkExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub(super) mod tls_client;

pub(super) async fn write_msg_log(
    msg: &str,
    msg_type: &str,
    root: &std::path::Path,
) -> tokio::io::Result<()> {
    let now = Utc::now();
    let time = now.format("%FT%H-%M-%S-%f").to_string();
    let dir = root
        .join(format!("{:04}", now.year()))
        .join(format!("{:02}", now.month()))
        .join(format!("{:02}", now.day()))
        .join(format!("{:02}", now.hour()));
    let file_path = dir.join(format!("{}_{}.xml", time, msg_type));
    tokio::fs::create_dir_all(&dir).await?;
    let mut file = tokio::fs::File::create(file_path).await?;
    file.write_all(msg.as_bytes()).await?;
    Ok(())
}

/// Attempts to read a message where the first two bytes give length.
///
/// Reads from a tokio async reader in conformance with RFC 5734 for the binary message data.
/// Will return `Ok(String)` on success or `Err(bool)` on any error.
/// The error types states if the connection has been closed by the remote end already.
/// In error cases the client should close the connection and restart.
///
/// # Arguments
/// * `sock` - A tokio async reader
/// * `host` - Host name for error reporting
/// * `root` - Root dir for storing message logs
pub(super) async fn recv_length_msg<R: std::marker::Unpin + tokio::io::AsyncRead>(
    sock: &mut R,
    host: &str,
    root: &std::path::Path,
) -> Result<String, bool> {
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
            return Err(matches!(err.kind(), std::io::ErrorKind::UnexpectedEof));
        }
    }
    let data = match String::from_utf8(data_buf) {
        Ok(s) => s,
        Err(err) => {
            error!("Invalid UTF8 from {}: {}", host, err);
            return Err(false);
        }
    };
    debug!("Received message from {} with contents: {}", host, data);
    match write_msg_log(&data, "recv", root).await {
        Ok(_) => {}
        Err(e) => {
            error!("Failed writing received message to message log: {}", e);
        }
    }
    Ok(data)
}

pub(super) async fn recv_msg<T, R: std::marker::Unpin + tokio::io::AsyncRead>(
    sock: &mut R,
    host: &str,
    root: &std::path::Path,
    decode_fn: fn(data: String, host: &str) -> Result<T, ()>,
) -> Result<T, bool> {
    let data = recv_length_msg(sock, host, root).await?;
    let msg = decode_fn(data, host).map_err(|_| false)?;
    Ok(msg)
}

pub(super) async fn send_msg<T, W: std::marker::Unpin + tokio::io::AsyncWrite>(
    host: &str,
    sock: &mut W,
    root: &std::path::Path,
    encode_fn: fn(message: &T, host: &str) -> Result<String, ()>,
    message: &T,
) -> Result<(), ()> {
    let encoded_msg = encode_fn(message, host)?;
    debug!("Sending message to {} with contents: {}", host, encoded_msg);
    let msg_bytes = encoded_msg.as_bytes();
    let msg_len = msg_bytes.len() + 4;
    match sock.write_u32(msg_len as u32).await {
        Ok(_) => {}
        Err(err) => {
            error!("Error writing data unit length to {}: {}", &host, err);
            return Err(());
        }
    };
    match sock.write(msg_bytes).await {
        Ok(_) => {}
        Err(err) => {
            error!("Error writing data unit to {}: {}", &host, err);
            return Err(());
        }
    }
    match write_msg_log(&encoded_msg, "send", root).await {
        Ok(_) => {}
        Err(e) => {
            error!("Failed writing sent message to message log: {}", e);
        }
    }
    Ok(())
}

/// Tokio task that attemps to read in messages and push them onto a tokio channel as received.
pub(super) struct ClientReceiver<T, R: std::marker::Unpin + tokio::io::AsyncRead> {
    /// Host name for error reporting
    pub host: String,
    /// Read half of the TLS stream used to connect to the server
    pub reader: R,
    /// Path to store received messages in
    pub root: std::path::PathBuf,
    pub decode_fn: fn(data: String, host: &str) -> Result<T, ()>,
}

impl<
        T: 'static + std::marker::Send,
        R: 'static + std::marker::Unpin + tokio::io::AsyncRead + std::marker::Send,
    > ClientReceiver<T, R>
{
    /// Starts the tokio task, and returns the receiving end of the channel to read messages from.
    pub fn run(mut self) -> futures::channel::mpsc::Receiver<Result<T, bool>> {
        let (mut sender, receiver) = futures::channel::mpsc::channel::<Result<T, bool>>(16);
        tokio::spawn(async move {
            loop {
                let msg = recv_msg(&mut self.reader, &self.host, &self.root, self.decode_fn).await;
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

pub(super) async fn make_tcp_socket(
    host: &str,
    source_addr: &Option<std::net::IpAddr>,
) -> Result<tokio::net::TcpStream, ()> {
    let addr = match tokio::net::lookup_host(host).await {
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
    };

    trace!("Setting up TCP socket for {}", host);
    let socket = match if addr.is_ipv4() {
        tokio::net::TcpSocket::new_v4()
    } else if addr.is_ipv6() {
        tokio::net::TcpSocket::new_v6()
    } else {
        unreachable!()
    } {
        Ok(s) => s,
        Err(err) => {
            error!("Unable to create TCP socket for {}: {}", host, err);
            return Err(());
        }
    };

    match socket.set_reuseaddr(true) {
        Ok(()) => {}
        Err(err) => {
            error!("Unable to setup TCP socket for {}: {}", host, err);
            return Err(());
        }
    }

    if let Some(bind_addr) = source_addr {
        trace!("Setting source address to {} for {}", bind_addr, host);
        match socket.bind(std::net::SocketAddr::new(bind_addr.to_owned(), 0)) {
            Ok(()) => {}
            Err(err) => {
                error!("Unable to setup TCP socket for {}: {}", host, err);
                return Err(());
            }
        }
    }

    Ok(match socket.connect(addr).await {
        Ok(s) => s,
        Err(err) => {
            error!("Unable to connect to {}: {}", host, err);
            return Err(());
        }
    })
}
