//! EPP commands relating to nominet specific features

use super::{CommandResponse, RequestMessage, Sender};
use chrono::prelude::*;

#[derive(Debug)]
pub struct HandshakeAcceptRequest {
    pub(super) case_id: String,
    pub(super) registrant: Option<String>,
    pub return_path: Sender<HandshakeResponse>,
}

#[derive(Debug)]
pub struct HandshakeRejectRequest {
    pub(super) case_id: String,
    pub return_path: Sender<HandshakeResponse>,
}

#[derive(Debug)]
pub struct HandshakeResponse {
    pub case_id: String,
    pub domains: Vec<String>,
}

#[derive(Debug)]
pub struct ReleaseRequest {
    pub(super) registrar_tag: String,
    pub(super) object: Object,
    pub return_path: Sender<ReleaseResponse>,
}

#[derive(Debug)]
pub enum Object {
    Domain(String),
    Registrant(String),
}

#[derive(Debug)]
pub struct ReleaseResponse {
    pub pending: bool,
    pub message: Option<String>,
}

#[derive(Debug)]
pub struct ContactValidateRequest {
    pub(super) contact_id: String,
    pub return_path: Sender<ContactValidateResponse>,
}

#[derive(Debug)]
pub struct ContactValidateResponse {}

#[derive(Debug)]
pub struct LockRequest {
    pub(super) object: Object,
    pub(super) lock_type: String,
    pub return_path: Sender<LockResponse>,
}

#[derive(Debug)]
pub struct LockResponse {}

#[derive(Debug)]
pub struct TagListRequest {
    pub return_path: Sender<TagListResponse>,
}

/// Response to a tag list query
#[derive(Debug)]
pub struct TagListResponse {
    /// Tags returned
    pub tags: Vec<Tag>,
}

#[derive(Debug)]
pub struct Tag {
    /// Tag ID
    pub tag: String,
    /// Legal name of the tag
    pub name: String,
    /// Trading name of the tag
    pub trading_name: Option<String>,
    /// Does this tag require handshaking
    pub handshake: bool,
}

#[derive(Debug)]
pub struct CancelData {
    pub domain_name: String,
    pub originator: String,
}

#[derive(Debug)]
pub struct ReleaseData {
    pub account_id: String,
    pub account_moved: bool,
    pub from: String,
    pub registrar_tag: String,
    pub domains: Vec<String>,
}

#[derive(Debug)]
pub struct RegistrarChangeData {
    pub originator: String,
    pub registrar_tag: String,
    pub case_id: Option<String>,
    pub domains: Vec<super::domain::InfoResponse>,
    pub contact: super::contact::InfoResponse,
}

#[derive(Debug)]
pub struct HostCancelData {
    pub host_objects: Vec<String>,
    pub domain_names: Vec<String>,
}

#[derive(Debug)]
pub struct ProcessData {
    pub stage: ProcessStage,
    pub contact: super::contact::InfoResponse,
    pub process_type: String,
    pub suspend_date: Option<DateTime<Utc>>,
    pub cancel_date: Option<DateTime<Utc>>,
    pub domain_names: Vec<String>,
}

#[derive(Debug)]
pub enum ProcessStage {
    Initial,
    Updated,
}

#[derive(Debug)]
pub struct SuspendData {
    pub reason: String,
    pub cancel_date: Option<DateTime<Utc>>,
    pub domain_names: Vec<String>,
}

#[derive(Debug)]
pub struct DomainFailData {
    pub reason: String,
    pub domain_name: String,
}

#[derive(Debug)]
pub struct RegistrantTransferData {
    pub originator: String,
    pub account_id: String,
    pub old_account_id: String,
    pub case_id: Option<String>,
    pub domain_names: Vec<String>,
    pub contact: super::contact::InfoResponse,
}

#[derive(Debug)]
pub enum DataQualityStatus {
    Valid,
    Invalid,
}

#[derive(Debug)]
pub struct DataQualityData {
    pub status: DataQualityStatus,
    pub reason: Option<String>,
    pub date_commenced: Option<DateTime<Utc>>,
    pub date_to_suspend: Option<DateTime<Utc>>,
    pub lock_applied: Option<bool>,
    pub domains: Option<Vec<String>>,
}

/// Accepts a pending transfer in
///
/// # Arguments
/// * `case_id` - The Nominet assigned case ID
/// * `registrant` - Optional account to assign the domain to
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn handshake_accept(
    case_id: &str,
    registrant: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<HandshakeResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::NominetAccept(Box::new(HandshakeAcceptRequest {
            case_id: case_id.to_owned(),
            registrant: registrant.map(Into::into),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Rejects a pending transfer in
///
/// # Arguments
/// * `case_id` - The Nominet assigned case ID
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn handshake_reject(
    case_id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<HandshakeResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::NominetReject(Box::new(HandshakeRejectRequest {
            case_id: case_id.to_owned(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Requests the transfer out of a domain
///
/// # Arguments
/// * `registrar_tag` - Target registrar TAG
/// * `object` - Object to release
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn release(
    registrar_tag: &str,
    object: Object,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<ReleaseResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::NominetRelease(Box::new(ReleaseRequest {
            registrar_tag: registrar_tag.to_owned(),
            object,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Fetches a list of registered tags
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn tag_list(
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<TagListResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::NominetTagList(Box::new(TagListRequest {
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Sets the valid flag on contact data quality
///
/// # Arguments
/// * `id` - ID of the contact object
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn contact_validate(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<ContactValidateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::NominetContactValidate(Box::new(ContactValidateRequest {
            contact_id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Locks contact's domains / specific domain
///
/// # Arguments
/// * `object` - Contact / domain
/// * `lock_type` - Reason for locking
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn lock(
    object: Object,
    lock_type: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<LockResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::NominetLock(Box::new(LockRequest {
            object,
            lock_type: lock_type.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Unlocks contact's domains / specific domain
///
/// # Arguments
/// * `object` - Contact / domain
/// * `lock_type` - Reason for locking
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn unlock(
    object: Object,
    lock_type: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<LockResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::NominetUnlock(Box::new(LockRequest {
            object,
            lock_type: lock_type.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}
