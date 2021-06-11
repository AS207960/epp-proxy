//! # Async/await EPP client.
//!
//! Messages should be injected into the server using the helper functions in subordinate modules such as
//! [`contact`], [`host`], and [`domain`].

use crate::proto;
use futures::future::FutureExt;

pub mod epp;
pub mod epp_like;
pub mod tmch;

pub mod balance;
pub mod contact;
pub mod domain;
pub mod eurid;
pub mod fee;
pub mod host;
pub mod launch;
pub mod maintenance;
pub mod nominet;
pub mod poll;
pub mod rgp;
pub mod router;
pub mod traficom;
pub mod verisign;

pub use router::{CommandResponse, Request, RequestSender, Response, Sender};

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
    /// http://www.verisign.com/epp/sync-1.0support
    verisign_sync_supported: bool,
}

impl ServerFeatures {
    fn has_erratum(&self, name: &str) -> bool {
        match &self.errata {
            Some(s) => s == name,
            None => false,
        }
    }
}

pub enum ClientCertConf<'a> {
    /// PCKS#12 file path for client identity
    PKCS12(&'a str),
    /// PCKS#11 HSM details
    PKCS11 {
        key_id: &'a str,
        cert_chain: &'a str,
    },
}

pub struct ClientConf<'a, C: Into<Option<&'a str>>, S: Into<Option<ClientCertConf<'a>>>> {
    /// The server connection string, in the form `domain:port`
    pub host: &'a str,
    /// The client ID/tag to login with
    pub tag: &'a str,
    /// The password to login with
    pub password: &'a str,
    /// Directory path to log commands to
    pub log_dir: std::path::PathBuf,
    pub client_cert: S,
    /// PCKS#11 key ID client identity (requires HSM)
    pub client_key_id: C,
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
}

async fn send_epp_client_request<R>(
    client_sender: &mut futures::channel::mpsc::Sender<router::Request>,
    req: router::Request,
    receiver: futures::channel::oneshot::Receiver<Response<R>>,
) -> Result<R, Error> {
    match client_sender.try_send(req) {
        Ok(_) => {}
        Err(_) => return Err(Error::InternalServerError),
    }
    let mut receiver = receiver.fuse();
    let mut delay = Box::pin(tokio::time::sleep(tokio::time::Duration::new(15, 0)).fuse());
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

/// Ends an EPP session
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn logout(
    mut client_sender: futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<()>, Error> {
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
