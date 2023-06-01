use foreign_types_shared::ForeignType;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

lazy_static! {
    static ref TLS_CONNECT_LOCK: tokio::sync::Semaphore = tokio::sync::Semaphore::new(1);
}

pub struct TLSConfig {
    /// The server connection string, in the form `domain:port`
    pub host: String,
    pub client_cert: Option<ClientCertConf>,
    /// When specificed binds the socket to the given source IP address
    pub source_addr: Option<std::net::IpAddr>,
    /// List of PEM file paths
    pub root_certs: Vec<String>,
    /// Accept invalid TLS certs
    pub danger_accept_invalid_certs: bool,
    /// Accept TLS certs with a hostname that doesn't match the DNS label
    pub danger_accept_invalid_hostname: bool,
}

impl<'a, C: Into<Option<&'a str>>> From<&super::super::ClientConf<'a, C>> for TLSConfig {
    fn from(conf: &super::super::ClientConf<'a, C>) -> Self {
        Self {
            host: conf.host.to_string(),
            client_cert: conf.client_cert.as_ref().map(|c| match c {
                super::super::ClientCertConf::PKCS12(s) => ClientCertConf::PKCS12(s.to_string()),
                super::super::ClientCertConf::PKCS11 { cert_chain, key_id } => {
                    ClientCertConf::PKCS11 {
                        key_id: key_id.to_string(),
                        cert_chain: cert_chain.to_string(),
                    }
                }
            }),
            source_addr: conf.source_address.map(|a| a.to_owned()),
            root_certs: conf.root_certs.iter().map(|c| c.to_string()).collect(),
            danger_accept_invalid_certs: conf.danger_accept_invalid_certs,
            danger_accept_invalid_hostname: conf.danger_accept_invalid_hostname,
        }
    }
}

pub enum ClientCertConf {
    /// PCKS#12 file path for client identity
    PKCS12(String),
    /// PCKS#11 HSM details
    PKCS11 { key_id: String, cert_chain: String },
}

#[derive(Clone, Debug)]
pub struct TLSClient {
    host: String,
    hostname: String,
    source_addr: Option<std::net::IpAddr>,
    tls_context: openssl::ssl::SslContext,
    should_lock: bool,
}

#[derive(Debug)]
pub struct TLSConnection {
    host: String,
    socket: tokio_openssl::SslStream<TcpStream>,
}

impl TLSClient {
    /// Creates a new TLS client ready to connect
    ///
    /// # Arguments
    /// * `conf` - Configuration to use for this client
    pub async fn new(
        conf: TLSConfig,
        pkcs11_engine: Option<crate::P11Engine>,
    ) -> std::io::Result<Self> {
        let mut should_lock = false;

        let mut context_builder =
            openssl::ssl::SslContext::builder(openssl::ssl::SslMethod::tls())?;

        let mut opts = openssl::ssl::SslOptions::ALL
            | openssl::ssl::SslOptions::NO_COMPRESSION
            | openssl::ssl::SslOptions::NO_SSLV2
            | openssl::ssl::SslOptions::NO_SSLV3
            | openssl::ssl::SslOptions::NO_TLSV1_1
            | openssl::ssl::SslOptions::SINGLE_DH_USE
            | openssl::ssl::SslOptions::SINGLE_ECDH_USE;
        opts &= !openssl::ssl::SslOptions::DONT_INSERT_EMPTY_FRAGMENTS;
        context_builder.set_options(opts);

        let mode =
            openssl::ssl::SslMode::AUTO_RETRY | openssl::ssl::SslMode::ACCEPT_MOVING_WRITE_BUFFER
                | openssl::ssl::SslMode::ENABLE_PARTIAL_WRITE | openssl::ssl::SslMode::RELEASE_BUFFERS;
        context_builder.set_mode(mode);

        context_builder.set_cipher_list(
            "DEFAULT:!aNULL:!eNULL:!MD5:!3DES:!DES:!RC4:!IDEA:!SEED:!aDSS:!SRP:!PSK",
        )?;

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

        if conf.root_certs.is_empty() {
            context_builder.set_default_verify_paths()?;
        } else {
            let mut cert_store = openssl::x509::store::X509StoreBuilder::new()?;
            for root_cert_path in conf.root_certs.iter() {
                let root_cert_bytes = tokio::fs::read(root_cert_path).await?;
                let root_cert = openssl::x509::X509::from_pem(&root_cert_bytes)?;
                cert_store.add_cert(root_cert)?;
            }
            let cert_store = cert_store.build();
            context_builder.set_cert_store(cert_store);
        }

        if !conf.danger_accept_invalid_hostname {
            context_builder
                .verify_param_mut()
                .set_hostflags(openssl::x509::verify::X509CheckFlags::NO_PARTIAL_WILDCARDS);
            context_builder.verify_param_mut().set_host(&hostname)?;
        }

        let mut priv_key = None;

        if let Some(client_cert) = conf.client_cert {
            match client_cert {
                ClientCertConf::PKCS12(pkcs12_file) => {
                    let pkcs = tokio::fs::read(pkcs12_file).await?;
                    let identity = openssl::pkcs12::Pkcs12::from_der(&pkcs)?.parse2("")?;
                    if let Some(cert) = &identity.cert {
                        context_builder.set_certificate(cert)?;
                    }
                    if let Some(pkey) = &identity.pkey {
                        context_builder.set_private_key(pkey)?;
                    }
                    for cert in identity.ca.into_iter().flatten() {
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
            should_lock = true;
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
            host: conf.host.to_string(),
            hostname,
            source_addr: conf.source_addr,
            tls_context: context,
            should_lock,
        })
    }

    pub async fn connect(&self) -> TLSConnection {
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

    async fn _try_connect(&self) -> Result<TLSConnection, ()> {
        // Many servers drop the connection if no TLS data is sent within ~10 seconds.
        // We therefore have to make sure only one thread at a time connects TLS otherwise a thread
        // could open a TCP stream and then wait a while for another thread to negotiate TLS
        // (it shouldn't be blocking because async, but sometimes it is), by which time the server
        // has dropped the connection and it all has to start again.
        //
        // Please don't ask how this happens, or how I found out, just don't try and fix this.
        trace!("Getting connect lock for {}", self.hostname);
        let lock = if self.should_lock || TLS_CONNECT_LOCK.available_permits() == 0 {
            Some(TLS_CONNECT_LOCK.acquire().await.unwrap())
        } else {
            None
        };
        trace!("Setting up TLS stream for {}", self.hostname);

        trace!("Opening TCP connection to {}", self.hostname);
        let socket = super::make_tcp_socket(&self.host, &self.source_addr).await?;

        trace!("Creating TLS context for {}", self.hostname);
        let mut cx = match (move || -> std::io::Result<tokio_openssl::SslStream<TcpStream>> {
            let mut ssl = openssl::ssl::Ssl::new(&self.tls_context)?;
            ssl.set_hostname(&self.hostname)?;
            let cx = tokio_openssl::SslStream::new(ssl, socket)?;
            Ok(cx)
        })() {
            Ok(s) => s,
            Err(err) => {
                error!("Unable to create TLS context: {}", err);
                return Err(());
            }
        };
        // I know this is disgusting, but OpenSSL isn't actually async compatible when using a HSM.
        trace!("Negotiating TLS connection to {}", self.hostname);
        let res = match if self.should_lock {
            let mut cx = Box::pin(cx);
            tokio::task::spawn_blocking(
                move || -> Result<tokio_openssl::SslStream<TcpStream>, openssl::ssl::Error> {
                    futures::executor::block_on(std::pin::Pin::as_mut(&mut cx).connect())?;
                    Ok(*std::pin::Pin::into_inner(cx))
                },
            )
            .await
        } else {
            Ok(std::pin::Pin::new(&mut cx).connect().await.map(|_| cx))
        } {
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
        Ok(TLSConnection {
            host: self.host.clone(),
            socket: res?,
        })
    }
}

impl TLSConnection {
    pub async fn close(&mut self) {
        match self.socket.shutdown().await {
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
}

impl<'a> tokio::io::AsyncRead for TLSConnection {
    fn poll_read(
        mut self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context,
        buf: &mut tokio::io::ReadBuf,
    ) -> core::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.socket).poll_read(cx, buf)
    }
}

impl<'a> tokio::io::AsyncWrite for TLSConnection {
    fn poll_write(
        mut self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context,
        buf: &[u8],
    ) -> core::task::Poll<std::io::Result<usize>> {
        std::pin::Pin::new(&mut self.socket).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context,
    ) -> core::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.socket).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context,
    ) -> core::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.socket).poll_shutdown(cx)
    }
}
