//! EPP commands relating to email forwarding objects

use chrono::prelude::*;

use super::{fee, CommandResponse, RequestMessage, Sender};

#[derive(Debug)]
pub struct CheckRequest {
    pub(super) name: String,
    pub(super) fee_check: Option<fee::FeeCheck>,
    pub return_path: Sender<CheckResponse>,
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
}

#[derive(Debug)]
pub struct InfoRequest {
    pub(super) name: String,
    pub(super) auth_info: Option<String>,
    pub return_path: Sender<InfoResponse>,
}

/// Response to a info query
#[derive(Debug)]
pub struct InfoResponse {
    /// Domain name in question
    pub name: String,
    /// Internal registry ID
    pub registry_id: String,
    /// Statuses attached to the domain
    pub statuses: Vec<super::domain::Status>,
    /// Contact ID of the registrant
    pub registrant: String,
    /// Additional contacts on the domain
    pub contacts: Vec<super::domain::InfoContact>,
    /// Email to forward to
    pub forward_to: String,
    /// Sponsoring client ID
    pub client_id: String,
    /// ID of the client that originally registered the email forwarding
    pub client_created_id: Option<String>,
    /// Date of initial registration
    pub creation_date: Option<DateTime<Utc>>,
    /// Date of registration expiration
    pub expiry_date: Option<DateTime<Utc>>,
    /// ID of the client that last updated the email forwarding
    pub last_updated_client: Option<String>,
    /// Date of last update
    pub last_updated_date: Option<DateTime<Utc>>,
    /// Date of last transfer
    pub last_transfer_date: Option<DateTime<Utc>>,
    /// Redemption grace period state of the email forwarding
    pub rgp_state: Vec<super::rgp::RGPState>,
    pub auth_info: Option<String>,
    pub whois_info: Option<super::verisign::InfoWhois>,
    pub personal_registration: Option<super::personal_registration::PersonalRegistrationInfo>
}

#[derive(Debug)]
pub struct CreateRequest {
    pub(super) name: String,
    pub(super) period: Option<super::Period>,
    pub(super) registrant: String,
    pub(super) contacts: Vec<super::domain::InfoContact>,
    pub(super) auth_info: String,
    pub(super) forward_to: String,
    pub(super) fee_agreement: Option<fee::FeeAgreement>,
    pub(super) personal_registration: Option<super::personal_registration::PersonalRegistrationInfo>,
    pub return_path: Sender<CreateResponse>,
}

#[derive(Debug)]
pub struct CreateResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    pub data: CreateData,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
}

#[derive(Debug)]
pub struct CreateData {
    /// The actual domain name created
    pub name: String,
    /// What date did the server log as the date of creation
    pub creation_date: Option<DateTime<Utc>>,
    /// When will the domain expire
    pub expiration_date: Option<DateTime<Utc>>,
    pub personal_registration: Option<super::personal_registration::PersonalRegistrationCreate>
}

#[derive(Debug)]
pub struct DeleteRequest {
    pub(super) name: String,
    pub return_path: Sender<DeleteResponse>,
}

#[derive(Debug)]
pub struct DeleteResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
}

#[derive(Debug)]
pub struct UpdateRequest {
    pub(super) name: String,
    pub(super) add: Vec<UpdateObject>,
    pub(super) remove: Vec<UpdateObject>,
    pub(super) new_registrant: Option<String>,
    pub(super) new_auth_info: Option<String>,
    pub(super) new_forward_to: Option<String>,
    pub(super) fee_agreement: Option<fee::FeeAgreement>,
    pub return_path: Sender<UpdateResponse>,
}

#[derive(Debug)]
pub enum UpdateObject {
    Status(super::domain::Status),
    Contact(super::domain::InfoContact),
}

#[derive(Debug)]
pub struct UpdateResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
}

#[derive(Debug)]
pub struct RenewRequest {
    pub(super) name: String,
    pub(super) add_period: Option<super::Period>,
    pub(super) cur_expiry_date: DateTime<Utc>,
    pub(super) fee_agreement: Option<fee::FeeAgreement>,
    pub return_path: Sender<RenewResponse>,
}

#[derive(Debug)]
pub struct RenewResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    pub data: RenewData,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
}

#[derive(Debug)]
pub struct RenewData {
    pub name: String,
    pub new_expiry_date: Option<DateTime<Utc>>,
    pub personal_registration: Option<super::personal_registration::PersonalRegistrationCreate>
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
    pub(super) add_period: Option<super::Period>,
    pub(super) fee_agreement: Option<fee::FeeAgreement>,
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
    pub personal_registration: Option<super::personal_registration::PersonalRegistrationCreate>
}

#[derive(Debug)]
pub struct PanData {
    pub name: String,
    pub result: bool,
    pub server_transaction_id: Option<String>,
    pub client_transaction_id: Option<String>,
    pub date: DateTime<Utc>,
}

/// Checks if an email forwarding is available
///
/// # Arguments
/// * `email` - The email in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn check(
    email: &str,
    fee_check: Option<fee::FeeCheck>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<CheckResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EmailForwardCheck(Box::new(CheckRequest {
            name: email.to_string(),
            fee_check,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Fetches information about an email forwarding
///
/// # Arguments
/// * `email` - The email in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn info(
    email: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<InfoResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EmailForwardInfo(Box::new(InfoRequest {
            name: email.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

pub struct CreateInfo<'a> {
    pub email: &'a str,
    pub period: Option<super::Period>,
    pub registrant: &'a str,
    pub contacts: Vec<super::domain::InfoContact>,
    pub auth_info: &'a str,
    pub forward_to: &'a str,
    pub fee_agreement: Option<fee::FeeAgreement>,
    pub personal_registration: Option<super::personal_registration::PersonalRegistrationInfo>
}

/// Registers a new email forwarding
///
/// # Arguments
/// * `email` - The email to be registered
/// * `period` - How long to register for
/// * `registrant` - Registrant contact ID,
/// * `contacts` - Other contact types for the domain
/// * `auth_info` - Auth info password for future transfers
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn create(
    info: CreateInfo<'_>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<CreateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EmailForwardCreate(Box::new(CreateRequest {
            name: info.email.to_string(),
            period: info.period,
            registrant: info.registrant.to_string(),
            contacts: info.contacts,
            forward_to: info.forward_to.to_string(),
            auth_info: info.auth_info.to_string(),
            fee_agreement: info.fee_agreement,
            personal_registration: info.personal_registration,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Deletes an email forwarding
///
/// # Arguments
/// * `email` - The email in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn delete(
    email: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<DeleteResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EmailForwardDelete(Box::new(DeleteRequest {
            name: email.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

#[derive(Default)]
pub struct UpdateInfo<'a> {
    pub email: &'a str,
    pub add: Vec<UpdateObject>,
    pub remove: Vec<UpdateObject>,
    pub new_registrant: Option<&'a str>,
    pub new_forward_to: Option<&'a str>,
    pub new_auth_info: Option<&'a str>,
    pub fee_agreement: Option<fee::FeeAgreement>,
}

/// Updates properties of an email forwarding
///
/// # Arguments
/// * `email` - The email to be updated
/// * `add` - Attributes to be added
/// * `remove` - Attributes to be removed
/// * `new_registrant` - New registrant ID
/// * `new_auth_info` - New auth info password for future transfers
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn update(
    info: UpdateInfo<'_>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<UpdateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EmailForwardUpdate(Box::new(UpdateRequest {
            name: info.email.to_string(),
            add: info.add,
            remove: info.remove,
            new_registrant: info.new_registrant.map(|s| s.into()),
            new_forward_to: info.new_forward_to.map(|s| s.into()),
            new_auth_info: info.new_auth_info.map(|s| s.into()),
            fee_agreement: info.fee_agreement,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Renews an email forwarding
///
/// # Arguments
/// * `email` - The email in question
/// * `add_period` - How much time to add to the domain
/// * `cur_expiry_date` - The current expiry date
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn renew(
    email: &str,
    add_period: Option<super::Period>,
    cur_expiry_date: DateTime<Utc>,
    fee_agreement: Option<fee::FeeAgreement>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<RenewResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EmailForwardRenew(Box::new(RenewRequest {
            name: email.to_string(),
            add_period,
            cur_expiry_date,
            fee_agreement,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Queries the current transfer status of an email forwarding
///
/// # Arguments
/// * `email` - The email to be queried
/// * `auth_info` - Auth info for the domain (not always required)
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_query(
    email: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EmailForwardTransferQuery(Box::new(TransferQueryRequest {
            name: email.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Requests the transfer of an email forwarding
///
/// # Arguments
/// * `email` - The email to be transferred
/// * `add_period` - How much time to add to the domain's expiry on transfer
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_request(
    email: &str,
    add_period: Option<super::Period>,
    auth_info: &str,
    fee_agreement: Option<fee::FeeAgreement>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EmailForwardTransferRequest(Box::new(TransferRequestRequest {
            name: email.to_string(),
            add_period,
            auth_info: auth_info.to_string(),
            fee_agreement,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Cancels the pending transfer of an email forwarding
///
/// # Arguments
/// * `email` - The email to be cancelled
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_cancel(
    email: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EmailForwardTransferCancel(Box::new(TransferAcceptRejectRequest {
            name: email.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Accepts the transfer of an email forwarding
///
/// # Arguments
/// * `email` - The email to be approved
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_accept(
    email: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EmailForwardTransferAccept(Box::new(TransferAcceptRejectRequest {
            name: email.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Rejects the transfer of an email forwarding
///
/// # Arguments
/// * `email` - The email to be rejected
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_reject(
    email: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EmailForwardTransferReject(Box::new(TransferAcceptRejectRequest {
            name: email.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
    .await
}
