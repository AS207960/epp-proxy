#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

pub mod client;
pub mod grpc;
pub mod metrics;
pub mod proto;

#[allow(missing_docs)]
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ClientCertConfig {
    PKCS12(String),
    PKCS11 { key_id: String, cert_chain: String },
}

#[derive(Debug, Deserialize)]
struct NominetDACConfig {
    real_time: String,
    time_delay: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    /// Unique registry ID
    pub id: String,
    /// Type/protocol of server being connected to
    #[serde(default)]
    server_type: ConfigServerType,
    /// Server host in the form `domain:port`
    server: String,
    /// Source address to bind to when connecting to the server
    #[serde(default)]
    source_address: Option<std::net::IpAddr>,
    /// Client ID to login to the server
    pub tag: String,
    /// Password to login to the server
    pub password: String,
    /// New password if the password is to be changed
    pub new_password: Option<String>,
    /// The zones the server is responsible for such as `co.uk` or `ch`
    zones: Vec<String>,
    /// PKCS12 file for TLS client auth
    client_cert: Option<ClientCertConfig>,
    /// Root certificates to trust on this connection
    root_certs: Option<Vec<String>>,
    /// Accept invalid TLS certs
    danger_accept_invalid_certs: Option<bool>,
    /// Accept TLS certs with a hostname that doesn't match the DNS label
    danger_accept_invalid_hostnames: Option<bool>,
    /// Does the server support pipelining?
    pipelining: bool,
    /// For naughty servers
    pub errata: Option<String>,
    nominet_dac: Option<NominetDACConfig>,
}

#[derive(Debug, Deserialize, Default)]
enum ConfigServerType {
    #[serde(rename = "EPP")]
    #[default]
    Epp,
    #[serde(rename = "TMCH")]
    Tmch,
}

#[derive(Debug, Deserialize)]
struct HSMConfigFile {
    pin: String,
}

/// Route requests to the correct EPP client for the authoritative registry
#[derive(Debug, Default)]
pub struct Router {
    pub id_to_client: HashMap<String, client::RequestSender>,
    zone_to_client: HashMap<String, (client::RequestSender, String)>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_client(&mut self, epp_client: Box<dyn client::Client>, config: ConfigFile) {
        let epp_client_sender = epp_client.start().0;

        for zone in &config.zones {
            self.zone_to_client
                .insert(zone.clone(), (epp_client_sender.clone(), config.id.clone()));
        }
        self.id_to_client.insert(config.id, epp_client_sender);
    }

    /// Fetches client sender by registry ID
    pub fn client_by_id(&self, id: &str) -> Option<client::RequestSender> {
        self.id_to_client.get(id).cloned()
    }

    /// Searches for client sender by a domain the client is authoritative for
    pub fn client_by_domain(&self, domain: &str) -> Option<(client::RequestSender, String)> {
        let mut domain_parts = domain.split('.').collect::<Vec<_>>();
        domain_parts.reverse();
        loop {
            if let Some(c) = self.zone_to_client.get(
                &domain_parts
                    .clone()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join(".")
                    .to_lowercase(),
            ) {
                return Some(c.clone());
            }
            domain_parts.pop()?;
        }
    }
}

pub fn oauth_client() -> rust_keycloak::oauth::OAuthClient {
    dotenv::dotenv().ok();

    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let well_known_url = std::env::var("OAUTH_WELL_KNOWN").unwrap_or_else(|_| {
        "https://sso.as207960.net/auth/realms/dev/.well-known/openid-configuration".to_string()
    });

    let config =
        rust_keycloak::oauth::OAuthClientConfig::new(&client_id, &client_secret, &well_known_url)
            .unwrap();

    rust_keycloak::oauth::OAuthClient::new(config)
}

pub async fn server_identity() -> tonic::transport::Identity {
    dotenv::dotenv().ok();

    let cert_file = std::env::var("TLS_CERT_FILE").expect("TLS_CERT_FILE must be set");
    let key_file = std::env::var("TLS_KEY_FILE").expect("TLS_KEY_FILE must be set");

    let cert = tokio::fs::read(cert_file)
        .await
        .expect("Can't read TLS cert");
    let key = tokio::fs::read(key_file).await.expect("Can't read TLS key");
    tonic::transport::Identity::from_pem(cert, key)
}

fn cvt(r: libc::c_int) -> Result<libc::c_int, openssl::error::ErrorStack> {
    if r <= 0 {
        Err(openssl::error::ErrorStack::get())
    } else {
        Ok(r)
    }
}

fn cvt_p<T>(r: *mut T) -> Result<*mut T, openssl::error::ErrorStack> {
    if r.is_null() {
        Err(openssl::error::ErrorStack::get())
    } else {
        Ok(r)
    }
}

struct P11EngineInner(*mut openssl_sys::ENGINE);

/// Holding type with drop for OpenSSL engine references
#[derive(Clone)]
pub struct P11Engine(std::sync::Arc<std::sync::Mutex<P11EngineInner>>);

impl Drop for P11EngineInner {
    fn drop(&mut self) {
        trace!("Dropping PKCS#11 engine");
        unsafe {
            cvt(openssl_sys::ENGINE_free(self.0)).unwrap();
        }
    }
}

unsafe impl Send for P11EngineInner {}

impl P11Engine {
    fn new(engine: *mut openssl_sys::ENGINE) -> Self {
        Self(std::sync::Arc::new(std::sync::Mutex::new(P11EngineInner(
            engine,
        ))))
    }

    fn claim(&self) -> std::sync::MutexGuard<'_, P11EngineInner> {
        self.0.lock().unwrap()
    }
}

impl std::ops::Deref for P11EngineInner {
    type Target = *mut openssl_sys::ENGINE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for P11EngineInner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub async fn setup_pkcs11_engine(hsm_conf_file: Option<&str>) -> Option<P11Engine> {
    if let Some(hsm_conf_file) = hsm_conf_file {
        info!("Loading PKCS#11 module");
        let file = match std::fs::File::open(hsm_conf_file) {
            Ok(f) => f,
            Err(e) => {
                error!("Can't open config file {}: {}", hsm_conf_file, e);
                std::process::exit(-1);
            }
        };
        let conf: HSMConfigFile = match serde_json::from_reader(file) {
            Ok(c) => c,
            Err(e) => {
                error!("Can't parse config file {}: {}", hsm_conf_file, e);
                std::process::exit(-1);
            }
        };

        let engine_id = std::ffi::CString::new("pkcs11").unwrap();
        let engine_pin_ctrl = std::ffi::CString::new("PIN").unwrap();
        let engine_pin = std::ffi::CString::new(conf.pin).unwrap();

        let engine = match match tokio::task::spawn_blocking(
            move || -> Result<P11Engine, openssl::error::ErrorStack> {
                unsafe {
                    // Something here seems to be blocking, even though we shouldn't be talking to the HSM yet.
                    openssl_sys::ENGINE_load_builtin_engines();
                    let engine =
                        P11Engine::new(cvt_p(openssl_sys::ENGINE_by_id(engine_id.as_ptr()))?);
                    cvt(openssl_sys::ENGINE_init(**engine.claim()))?;
                    cvt(openssl_sys::ENGINE_ctrl_cmd_string(
                        **engine.claim(),
                        engine_pin_ctrl.as_ptr(),
                        engine_pin.as_ptr(),
                        1,
                    ))?;
                    info!("Loaded PKCS#11 engine");
                    Ok(engine)
                }
            },
        )
        .await
        {
            Ok(e) => e,
            Err(e) => {
                error!("Can't setup OpenSSL: {}", e);
                std::process::exit(-1);
            }
        } {
            Ok(e) => e,
            Err(e) => {
                error!("Can't setup OpenSSL: {}", e);
                std::process::exit(-1);
            }
        };
        Some(engine)
    } else {
        None
    }
}

pub async fn create_client(
    log_storage: StorageScoped,
    config: &ConfigFile,
    pkcs11_engine: &Option<P11Engine>,
    metrics_registry: metrics::ScopedMetrics,
    keepalive: bool,
) -> Box<dyn client::Client> {
    let client_conf = client::ClientConf {
        host: &config.server,
        tag: &config.tag,
        password: &config.password,
        source_address: config.source_address.as_ref(),
        log_storage,
        metrics_registry,
        keepalive,
        client_cert: match &config.client_cert {
            Some(ClientCertConfig::PKCS12(s)) => Some(client::ClientCertConf::PKCS12(s)),
            Some(ClientCertConfig::PKCS11 { key_id, cert_chain }) => {
                Some(client::ClientCertConf::PKCS11 { key_id, cert_chain })
            }
            _ => None,
        },
        root_certs: &match config.root_certs.as_ref() {
            Some(r) => r.iter().map(|c| c.as_str()).collect::<Vec<_>>(),
            None => vec![],
        },
        danger_accept_invalid_certs: config.danger_accept_invalid_certs.unwrap_or(false),
        danger_accept_invalid_hostname: config.danger_accept_invalid_hostnames.unwrap_or(false),
        new_password: config.new_password.as_deref(),
        pipelining: config.pipelining,
        errata: config.errata.clone(),
        nominet_dac: config.nominet_dac.as_ref().map(|d| client::NominetDACConf {
            real_time: &d.real_time,
            time_delay: &d.time_delay,
        }),
    };
    match match config.server_type {
        ConfigServerType::Epp => client::epp::EPPClient::new(client_conf, pkcs11_engine.clone())
            .await
            .map(|c| Box::new(c) as Box<dyn client::Client>),
        ConfigServerType::Tmch => {
            client::tmch_client::TMCHClient::new(client_conf, pkcs11_engine.clone())
                .await
                .map(|c| Box::new(c) as Box<dyn client::Client>)
        }
    } {
        Ok(c) => c,
        Err(e) => {
            error!("Can't create client: {}", e);
            std::process::exit(-1);
        }
    }
}

#[tonic::async_trait]
pub trait Storage: std::fmt::Debug + Send + Sync {
    async fn write_msg_log(
        &self,
        tag: &str,
        msg: &str,
        msg_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone)]
pub struct FSStorage {
    root: std::path::PathBuf,
}

impl FSStorage {
    pub fn new(root: std::path::PathBuf) -> Self {
        Self { root }
    }
}

#[tonic::async_trait]
impl Storage for FSStorage {
    async fn write_msg_log(
        &self,
        tag: &str,
        msg: &str,
        msg_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use chrono::prelude::*;
        use tokio::io::AsyncWriteExt;

        let now = Utc::now();
        let time = now.format("%FT%H-%M-%S-%f").to_string();
        let dir = self
            .root
            .join(tag)
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
}

#[derive(Debug)]
pub struct TokioSleep;

impl aws_sdk_s3::config::AsyncSleep for TokioSleep {
    fn sleep(&self, duration: std::time::Duration) -> aws_sdk_s3::config::Sleep {
        aws_sdk_s3::config::Sleep::new(tokio::time::sleep(duration))
    }
}

#[derive(Debug, Clone)]
pub struct S3Storage {
    client: aws_sdk_s3::Client,
    bucket: String,
}

impl S3Storage {
    pub fn new(
        endpoint_url: impl Into<String>,
        credentials_provider: impl aws_credential_types::provider::ProvideCredentials + 'static,
        region: impl Into<aws_sdk_s3::config::Region>,
        bucket: impl Into<String>,
    ) -> Self {
        let app_name = aws_sdk_s3::config::AppName::new("epp-proxy").unwrap();
        let sleep_impl = std::sync::Arc::new(TokioSleep);

        let config = aws_sdk_s3::config::Builder::new()
            .app_name(app_name)
            .endpoint_url(endpoint_url)
            .credentials_provider(credentials_provider)
            .region(region.into())
            .retry_config(aws_sdk_s3::config::retry::RetryConfig::standard())
            .sleep_impl(aws_sdk_s3::config::SharedAsyncSleep::new(sleep_impl))
            .build();
        let client = aws_sdk_s3::Client::from_conf(config);

        Self {
            client,
            bucket: bucket.into(),
        }
    }
}

#[tonic::async_trait]
impl Storage for S3Storage {
    async fn write_msg_log(
        &self,
        tag: &str,
        msg: &str,
        msg_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use chrono::prelude::*;

        let byte_stream = aws_sdk_s3::primitives::ByteStream::from(msg.as_bytes().to_vec());

        let now = Utc::now();
        let time = now.format("%Y/%m/%d/%H/%FT%H-%M-%S-%f").to_string();
        let key = format!("{}/{}_{}.xml", tag, time, msg_type);

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(byte_stream)
            .send()
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct StorageScoped {
    storage: std::sync::Arc<Box<dyn Storage>>,
    tag: String,
}

impl StorageScoped {
    pub fn new(storage: Box<dyn Storage>, tag: &str) -> Self {
        Self {
            storage: std::sync::Arc::new(storage),
            tag: tag.to_string(),
        }
    }

    pub fn new_arc(storage: impl Into<std::sync::Arc<Box<dyn Storage>>>, tag: &str) -> Self {
        Self {
            storage: storage.into(),
            tag: tag.to_string(),
        }
    }

    async fn write_msg_log(
        &self,
        msg: &str,
        msg_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.storage.write_msg_log(&self.tag, msg, msg_type).await
    }
}
