//! # Async/await EPP client.
//!
//! Messages should be injected into the server using the helper functions in subordinate modules such as
//! [`contact`], [`host`], and [`domain`].

use crate::proto;
use futures::future::FutureExt;

pub mod epp;
pub mod epp_like;
pub mod nominet_dac;
pub mod tmch_client;

pub mod balance;
pub mod contact;
pub mod dac;
pub mod domain;
pub mod email_forward;
pub mod eurid;
pub mod fee;
pub mod host;
pub mod isnic;
pub mod keysys;
pub mod launch;
pub mod maintenance;
pub mod mark;
pub mod nominet;
pub mod personal_registration;
pub mod poll;
pub mod rgp;
pub mod router;
pub mod tmch;
pub mod traficom;
pub mod verisign;

pub use router::{CommandResponse, RequestMessage, RequestSender, Response, Sender};

pub enum ClientCertConf<'a> {
    /// PCKS#12 file path for client identity
    PKCS12(&'a str),
    /// PCKS#11 HSM details
    PKCS11 {
        key_id: &'a str,
        cert_chain: &'a str,
    },
}

pub struct NominetDACConf<'a> {
    pub real_time: &'a str,
    pub time_delay: &'a str,
}

pub struct ClientConf<'a, C: Into<Option<&'a str>>, M: crate::metrics::Metrics> {
    /// The server connection string, in the form `domain:port`
    pub host: &'a str,
    /// The client ID/tag to login with
    pub tag: &'a str,
    /// The password to login with
    pub password: &'a str,
    pub log_storage: crate::StorageScoped,
    pub metrics_registry: M,
    pub client_cert: Option<ClientCertConf<'a>>,
    /// Source address to bind the TLS connection to, for IP based ACLs etc.
    pub source_address: Option<&'a std::net::IpAddr>,
    /// List of PEM file paths
    pub root_certs: &'a [&'a str],
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
    pub nominet_dac: Option<NominetDACConf<'a>>,
    /// Should the client send keepalive commands automatically
    pub keepalive: bool,
}

async fn send_epp_client_request<R>(
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
    req: RequestMessage,
    receiver: futures::channel::oneshot::Receiver<Response<R>>,
) -> Result<R, Error> {
    match client_sender.try_send(req) {
        Ok(_) => {}
        Err(_) => return Err(Error::ServerInternal),
    }
    let mut receiver = receiver.fuse();
    let mut delay = Box::pin(tokio::time::sleep(tokio::time::Duration::new(60, 0)).fuse());
    let resp = futures::select! {
        r = receiver => r,
        _ = delay => {
            return Err(Error::Timeout);
        }
    };
    match resp {
        Ok(r) => r,
        Err(_) => Err(Error::ServerInternal),
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
    ServerInternal,
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

#[derive(Debug)]
pub struct Period {
    /// Unit of time
    pub unit: PeriodUnit,
    /// Number of units of time
    pub value: u32,
}

#[derive(Debug)]
pub enum PeriodUnit {
    Years,
    Months,
}

#[derive(Debug)]
pub struct Phone {
    /// Initial dialable part of the number
    pub number: String,
    /// Optional internal extension
    pub extension: Option<String>,
}

#[derive(Debug)]
pub struct BlankRequest {
    pub return_path: Sender<()>,
}

/// Ends an EPP session
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn logout(
    mut client_sender: futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<()>, Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    send_epp_client_request(
        &mut client_sender,
        RequestMessage::Logout(Box::new(BlankRequest {
            return_path: sender,
        })),
        receiver,
    )
    .await
}

pub trait Client {
    fn start(
        self: Box<Self>,
    ) -> (
        futures::channel::mpsc::Sender<RequestMessage>,
        futures::channel::mpsc::UnboundedReceiver<router::CommandTransactionID>,
    );
}
