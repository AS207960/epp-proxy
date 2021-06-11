use chrono::prelude::*;
use futures::sink::SinkExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub(super) async fn write_msg_log(
    msg: &str,
    msg_type: &str,
    root: &std::path::Path,
) -> tokio::io::Result<()> {
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
    let data = super::epp_like::recv_length_msg(sock, host, root).await?;
    let msg = decode_fn(data, host).map_err(|_| false)?;
    Ok(msg)
}

/// Tokio task that attemps to read in messages and push them onto a tokio channel as received.
pub(super) struct ClientReceiver<T, R: std::marker::Unpin + tokio::io::AsyncRead> {
    /// Host name for error reporting
    pub host: String,
    /// Read half of the TLS stream used to connect to the server
    pub reader: tokio::io::ReadHalf<R>,
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
