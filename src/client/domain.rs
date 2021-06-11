//! EPP commands relating to domain objects

use chrono::prelude::*;

use super::{fee, launch, CommandResponse, Request, Sender};

#[derive(Debug)]
pub struct CheckRequest {
    pub(super) name: String,
    pub(super) fee_check: Option<fee::FeeCheck>,
    pub(super) launch_check: Option<launch::LaunchAvailabilityCheck>,
    pub return_path: Sender<CheckResponse>,
}

#[derive(Debug)]
pub struct ClaimsCheckRequest {
    pub(super) name: String,
    pub(super) launch_check: launch::LaunchClaimsCheck,
    pub return_path: Sender<ClaimsCheckResponse>,
}

#[derive(Debug)]
pub struct TrademarkCheckRequest {
    pub(super) name: String,
    pub return_path: Sender<ClaimsCheckResponse>,
}

/// Response to a domain check query
#[derive(Debug)]
pub struct CheckResponse {
    /// Is the domain available for registration
    pub avail: bool,
    /// An optional reason for the domain's status
    pub reason: Option<String>,
    /// Fee information (if supplied by the registry)
    pub fee_check: Option<fee::FeeCheckData>,
    pub donuts_fee_check: Option<fee::DonutsFeeData>,
    pub eurid_check: Option<super::eurid::DomainCheck>,
    pub eurid_idn: Option<super::eurid::IDN>,
}

/// Response to a domain claims check query
#[derive(Debug)]
pub struct ClaimsCheckResponse {
    /// Does a trademark claim exist
    pub exists: bool,
    /// Claims key for this domain
    pub claims_key: Vec<launch::LaunchClaimKey>,
}

#[derive(Debug)]
pub enum InfoHost {
    All,
    Delegated,
    Subordinate,
    None,
}

#[derive(Debug)]
pub struct InfoRequest {
    pub(super) name: String,
    pub(super) auth_info: Option<String>,
    pub(super) launch_info: Option<launch::LaunchInfo>,
    pub(super) hosts: Option<InfoHost>,
    pub(super) eurid_data: Option<super::eurid::DomainInfoRequest>,
    pub return_path: Sender<InfoResponse>,
}

/// Response to a domain info query
#[derive(Debug)]
pub struct InfoResponse {
    /// Domain name in question
    pub name: String,
    /// Internal registry ID
    pub registry_id: String,
    /// Statuses attached to the domain
    pub statuses: Vec<Status>,
    /// Contact ID of the registrant
    pub registrant: String,
    /// Additional contacts on the domain
    pub contacts: Vec<InfoContact>,
    /// Nameservers for the domain
    pub nameservers: Vec<InfoNameserver>,
    /// Host names attached to the domain
    pub hosts: Vec<String>,
    /// Sponsoring client ID
    pub client_id: String,
    /// ID of the client that originally registered the domain
    pub client_created_id: Option<String>,
    /// Date of initial registration
    pub creation_date: Option<DateTime<Utc>>,
    /// Date of registration expiration
    pub expiry_date: Option<DateTime<Utc>>,
    /// ID of the client that last updated the domain
    pub last_updated_client: Option<String>,
    /// Date of last update
    pub last_updated_date: Option<DateTime<Utc>>,
    /// Date of last transfer
    pub last_transfer_date: Option<DateTime<Utc>>,
    /// Redemption grace period state of the domain
    pub rgp_state: Vec<super::rgp::RGPState>,
    pub auth_info: Option<String>,
    /// DNSSEC data
    pub sec_dns: Option<SecDNSData>,
    pub launch_info: Option<launch::LaunchInfoData>,
    pub donuts_fee_data: Option<fee::DonutsFeeData>,
    pub whois_info: Option<super::verisign::InfoWhois>,
    pub eurid_data: Option<super::eurid::DomainInfo>,
    pub eurid_idn: Option<super::eurid::IDN>,
}

/// Additional contact associated with a domain
#[derive(Debug)]
pub struct InfoContact {
    /// Type of contact
    pub contact_type: String,
    /// Contact ID of the contact
    pub contact_id: String,
}

/// Nameserver associated with a domain
#[derive(Debug)]
pub enum InfoNameserver {
    /// Host only type
    HostOnly(String),
    /// Host name with glue records
    HostAndAddress {
        host: String,
        addresses: Vec<super::host::Address>,
        eurid_idn: Option<super::eurid::IDN>,
    },
}

/// DNSSEC key data
#[derive(Debug)]
pub struct SecDNSData {
    pub max_sig_life: Option<i64>,
    pub data: SecDNSDataType,
}

#[derive(Debug)]
pub enum SecDNSDataType {
    DSData(Vec<SecDNSDSData>),
    KeyData(Vec<SecDNSKeyData>),
}

#[derive(Debug)]
pub struct SecDNSDSData {
    pub key_tag: u16,
    pub algorithm: u8,
    pub digest_type: u8,
    pub digest: String,
    pub key_data: Option<SecDNSKeyData>,
}

#[derive(Debug)]
pub struct SecDNSKeyData {
    pub flags: u16,
    pub protocol: u8,
    pub algorithm: u8,
    pub public_key: String,
}

#[derive(Debug)]
pub struct CreateRequest {
    pub(super) name: String,
    pub(super) period: Option<Period>,
    pub(super) registrant: String,
    pub(super) contacts: Vec<InfoContact>,
    pub(super) nameservers: Vec<InfoNameserver>,
    pub(super) auth_info: String,
    pub(super) sec_dns: Option<SecDNSData>,
    pub(super) launch_create: Option<launch::LaunchCreate>,
    pub(super) fee_agreement: Option<fee::FeeAgreement>,
    pub(super) donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub(super) eurid_data: Option<super::eurid::DomainCreate>,
    pub return_path: Sender<CreateResponse>,
}

/// Domain registration period
#[derive(Debug)]
pub struct Period {
    /// Unit of time
    pub unit: PeriodUnit,
    /// Number of units of time
    pub value: u32,
}

/// Domain registration period time unit
#[derive(Debug)]
pub enum PeriodUnit {
    Years,
    Months,
}

#[derive(Debug)]
pub struct CreateResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    pub data: CreateData,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
    pub donuts_fee_data: Option<fee::DonutsFeeData>,
    pub launch_create: Option<launch::LaunchCreateData>,
}

#[derive(Debug)]
pub struct CreateData {
    /// The actual domain name created
    pub name: String,
    /// What date did the server log as the date of creation
    pub creation_date: Option<DateTime<Utc>>,
    /// When will the domain expire
    pub expiration_date: Option<DateTime<Utc>>,
    pub eurid_idn: Option<super::eurid::IDN>,
}

#[derive(Debug)]
pub struct DeleteRequest {
    pub(super) name: String,
    pub(super) launch_info: Option<launch::LaunchUpdate>,
    pub(super) donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub(super) eurid_data: Option<super::eurid::DomainDelete>,
    pub return_path: Sender<DeleteResponse>,
}

#[derive(Debug)]
pub struct DeleteResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
    pub eurid_idn: Option<super::eurid::IDN>,
}

#[derive(Debug)]
pub struct UpdateRequest {
    pub(super) name: String,
    pub(super) add: Vec<UpdateObject>,
    pub(super) remove: Vec<UpdateObject>,
    pub(super) new_registrant: Option<String>,
    pub(super) new_auth_info: Option<String>,
    pub(super) sec_dns: Option<UpdateSecDNS>,
    pub(super) launch_info: Option<launch::LaunchUpdate>,
    pub(super) fee_agreement: Option<fee::FeeAgreement>,
    pub(super) donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub(super) eurid_data: Option<super::eurid::DomainUpdate>,
    pub return_path: Sender<UpdateResponse>,
}

#[derive(Debug)]
pub enum UpdateObject {
    Status(Status),
    Contact(InfoContact),
    Nameserver(InfoNameserver),
}

#[derive(Debug)]
pub struct UpdateSecDNS {
    pub urgent: Option<bool>,
    pub remove: Option<UpdateSecDNSRemove>,
    pub add: Option<SecDNSDataType>,
    pub new_max_sig_life: Option<i64>,
}

#[derive(Debug)]
pub enum UpdateSecDNSRemove {
    All(bool),
    Data(SecDNSDataType),
}

#[derive(Debug)]
pub struct UpdateResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
    pub donuts_fee_data: Option<fee::DonutsFeeData>,
}

#[derive(Debug)]
pub struct VerisignSyncRequest {
    pub(super) name: String,
    pub(super) month: u32,
    pub(super) day: u32,
    pub return_path: Sender<UpdateResponse>,
}

#[derive(Debug)]
pub struct RenewRequest {
    pub(super) name: String,
    pub(super) add_period: Option<Period>,
    pub(super) cur_expiry_date: DateTime<Utc>,
    pub(super) fee_agreement: Option<fee::FeeAgreement>,
    pub(super) donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub return_path: Sender<RenewResponse>,
}

#[derive(Debug)]
pub struct RenewResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    pub data: RenewData,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
    pub donuts_fee_data: Option<fee::DonutsFeeData>,
}

#[derive(Debug)]
pub struct RenewData {
    pub name: String,
    pub new_expiry_date: Option<DateTime<Utc>>,
    pub eurid_idn: Option<super::eurid::IDN>,
    pub eurid_data: Option<super::eurid::DomainRenewInfo>,
}

#[derive(Debug)]
pub struct TransferQueryRequest {
    pub(super) name: String,
    pub(super) auth_info: Option<String>,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferRequestRequest {
    pub(super) name: String,
    pub(super) auth_info: String,
    pub(super) add_period: Option<Period>,
    pub(super) fee_agreement: Option<fee::FeeAgreement>,
    pub(super) donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub(super) eurid_data: Option<super::eurid::DomainTransfer>,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferAcceptRejectRequest {
    pub(super) name: String,
    pub(super) auth_info: Option<String>,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    pub data: TransferData,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
    pub donuts_fee_data: Option<fee::DonutsFeeData>,
}

#[derive(Debug)]
pub struct TransferData {
    pub name: String,
    pub status: super::TransferStatus,
    /// Which client requested the transfer
    pub requested_client_id: String,
    /// The date of the transfer request
    pub requested_date: DateTime<Utc>,
    /// Whcich client last acted / needs to act
    pub act_client_id: String,
    /// Date on which a client acted / must act by
    pub act_date: DateTime<Utc>,
    /// New domain expiry date if amended by the transfer
    pub expiry_date: Option<DateTime<Utc>>,
    pub eurid_idn: Option<super::eurid::IDN>,
    pub eurid_data: Option<super::eurid::DomainTransferInfo>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    ClientDeleteProhibited,
    ClientHold,
    ClientRenewProhibited,
    ClientTransferProhibited,
    ClientUpdateProhibited,
    Inactive,
    Ok,
    PendingCreate,
    PendingDelete,
    PendingRenew,
    PendingTransfer,
    PendingUpdate,
    ServerDeleteProhibited,
    ServerHold,
    ServerRenewProhibited,
    ServerTransferProhibited,
    ServerUpdateProhibited,
}

#[derive(Debug)]
pub struct PanData {
    pub name: String,
    pub result: bool,
    pub server_transaction_id: Option<String>,
    pub client_transaction_id: Option<String>,
    pub date: DateTime<Utc>,
}

/// Checks if a domain name is available
///
/// # Arguments
/// * `domain` - The domain in question
/// * `launch_check` - Launch availability info
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn check(
    domain: &str,
    fee_check: Option<fee::FeeCheck>,
    launch_check: Option<launch::LaunchAvailabilityCheck>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<CheckResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainCheck(Box::new(CheckRequest {
            name: domain.to_string(),
            fee_check,
            launch_check,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Checks if a domain name has claims registered for a launch phase
///
/// # Arguments
/// * `domain` - The domain in question
/// * `launch_check` - Launch claims info
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn launch_claims_check(
    domain: &str,
    launch_check: launch::LaunchClaimsCheck,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<ClaimsCheckResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainClaimsCheck(Box::new(ClaimsCheckRequest {
            name: domain.to_string(),
            launch_check,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Checks if a domain name has trademarks registered for it
///
/// # Arguments
/// * `domain` - The domain in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn launch_trademark_check(
    domain: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<ClaimsCheckResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTrademarkCheck(Box::new(TrademarkCheckRequest {
            name: domain.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Fetches information about a domain name
///
/// # Arguments
/// * `domain` - The domain in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn info(
    domain: &str,
    auth_info: Option<&str>,
    hosts: Option<InfoHost>,
    launch_info: Option<launch::LaunchInfo>,
    eurid_data: Option<super::eurid::DomainInfoRequest>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<InfoResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainInfo(Box::new(InfoRequest {
            name: domain.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            hosts,
            launch_info,
            eurid_data,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

pub struct CreateInfo<'a> {
    pub domain: &'a str,
    pub period: Option<Period>,
    pub registrant: &'a str,
    pub contacts: Vec<InfoContact>,
    pub nameservers: Vec<InfoNameserver>,
    pub auth_info: &'a str,
    pub sec_dns: Option<SecDNSData>,
    pub launch_create: Option<launch::LaunchCreate>,
    pub fee_agreement: Option<fee::FeeAgreement>,
    pub donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub eurid_data: Option<super::eurid::DomainCreate>,
}

/// Registers a new domain
///
/// # Arguments
/// * `domain` - The domain to be registered
/// * `period` - How long to register for
/// * `registrant` - Registrant contact ID,
/// * `contacts` - Other contact types for the domain
/// * `nameservers` - Domain nameservers
/// * `auth_info` - Auth info password for future transfers
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn create(
    info: CreateInfo<'_>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<CreateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainCreate(Box::new(CreateRequest {
            name: info.domain.to_string(),
            period: info.period,
            registrant: info.registrant.to_string(),
            contacts: info.contacts,
            nameservers: info.nameservers,
            auth_info: info.auth_info.to_string(),
            sec_dns: info.sec_dns,
            launch_create: info.launch_create,
            fee_agreement: info.fee_agreement,
            donuts_fee_agreement: info.donuts_fee_agreement,
            eurid_data: info.eurid_data,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Deletes a domain name
///
/// # Arguments
/// * `domain` - The domain in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn delete(
    domain: &str,
    launch_info: Option<launch::LaunchUpdate>,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    eurid_data: Option<super::eurid::DomainDelete>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<DeleteResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainDelete(Box::new(DeleteRequest {
            name: domain.to_string(),
            launch_info,
            donuts_fee_agreement,
            eurid_data,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

pub struct UpdateInfo<'a> {
    pub domain: &'a str,
    pub add: Vec<UpdateObject>,
    pub remove: Vec<UpdateObject>,
    pub new_registrant: Option<&'a str>,
    pub new_auth_info: Option<&'a str>,
    pub sec_dns: Option<UpdateSecDNS>,
    pub launch_info: Option<launch::LaunchUpdate>,
    pub fee_agreement: Option<fee::FeeAgreement>,
    pub donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub eurid_data: Option<super::eurid::DomainUpdate>,
}

/// Updates properties of a domain name
///
/// # Arguments
/// * `domain` - The domain to be updated
/// * `add` - Attributes to be added
/// * `remove` - Attributes to be removed
/// * `new_registrant` - New registrant ID
/// * `new_auth_info` - New auth info password for future transfers
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn update(
    info: UpdateInfo<'_>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<UpdateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainUpdate(Box::new(UpdateRequest {
            name: info.domain.to_string(),
            add: info.add,
            remove: info.remove,
            new_registrant: info.new_registrant.map(|s| s.into()),
            new_auth_info: info.new_auth_info.map(|s| s.into()),
            sec_dns: info.sec_dns,
            launch_info: info.launch_info,
            fee_agreement: info.fee_agreement,
            donuts_fee_agreement: info.donuts_fee_agreement,
            eurid_data: info.eurid_data,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Performs a Verisign ConsoliDate
///
/// # Arguments
/// * `domain` - The domain to be updated
/// * `month` - Month to move renewal to
/// * `day` - Day of month to move renewal to
pub async fn verisign_sync(
    domain: &str,
    month: u32,
    day: u32,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<UpdateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::VerisignSync(Box::new(VerisignSyncRequest {
            name: domain.to_string(),
            month,
            day,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Renews a domain name
///
/// # Arguments
/// * `domain` - The domain in question
/// * `add_period` - How much time to add to the domain
/// * `cur_expiry_date` - The current expiry date
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn renew(
    domain: &str,
    add_period: Option<Period>,
    cur_expiry_date: DateTime<Utc>,
    fee_agreement: Option<fee::FeeAgreement>,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<RenewResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainRenew(Box::new(RenewRequest {
            name: domain.to_string(),
            add_period,
            cur_expiry_date,
            fee_agreement,
            donuts_fee_agreement,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Queries the current transfer status of a domain name
///
/// # Arguments
/// * `domain` - The domain to be queried
/// * `auth_info` - Auth info for the domain (not always required)
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_query(
    domain: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTransferQuery(Box::new(TransferQueryRequest {
            name: domain.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Requests the transfer of a domain name
///
/// # Arguments
/// * `domain` - The domain to be transferred
/// * `add_period` - How much time to add to the domain's expiry on transfer
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_request(
    domain: &str,
    add_period: Option<Period>,
    auth_info: &str,
    fee_agreement: Option<fee::FeeAgreement>,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    eurid_data: Option<super::eurid::DomainTransfer>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTransferRequest(Box::new(TransferRequestRequest {
            name: domain.to_string(),
            add_period,
            auth_info: auth_info.to_string(),
            fee_agreement,
            donuts_fee_agreement,
            eurid_data,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Cancels the pending transfer of a domain name
///
/// # Arguments
/// * `domain` - The domain to be cancelled
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_cancel(
    domain: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTransferCancel(Box::new(TransferAcceptRejectRequest {
            name: domain.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Accepts the transfer of a domain name
///
/// # Arguments
/// * `domain` - The domain to be approved
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_accept(
    domain: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTransferAccept(Box::new(TransferAcceptRejectRequest {
            name: domain.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Rejects the transfer of a domain name
///
/// # Arguments
/// * `domain` - The domain to be rejected
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_reject(
    domain: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTransferReject(Box::new(TransferAcceptRejectRequest {
            name: domain.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
    .await
}
