use chrono::prelude::*;

use super::{CommandResponse, RequestMessage, Sender};

#[derive(Debug)]
pub struct CheckRequest {
    pub(super) id: String,
    pub return_path: Sender<CheckResponse>,
}

#[derive(Debug)]
pub struct CheckResponse {
    pub avail: bool,
    pub reason: Option<String>,
}

#[derive(Debug)]
pub struct CreateRequest {
    pub(super) mark: super::mark::Mark,
    pub(super) period: Option<super::Period>,
    pub(super) documents: Vec<Document>,
    pub(super) labels: Vec<CreateLabel>,
    pub(super) variations: Vec<String>,
    pub return_path: Sender<CreateResponse>,
}

#[derive(Debug)]
pub struct Document {
    pub class: DocumentClass,
    pub file_name: String,
    pub file_type: FileType,
    pub contents: Vec<u8>,
}

#[derive(Debug)]
pub enum DocumentClass {
    LicenseeDeclaration,
    AssigneeDeclaration,
    Other,
    DeclarationProofOfUseOneSample,
    OtherProofOfUse,
    CopyOfCourtOrder,
}

#[derive(Debug)]
pub enum FileType {
    Jpg,
    Pdf,
}

#[derive(Debug)]
pub struct CreateLabel {
    pub label: String,
    pub smd_inclusion: bool,
    pub claims_notify: bool,
}

#[derive(Debug)]
pub struct CreateResponse {
    pub id: String,
    pub created_date: DateTime<Utc>,
    pub balance: BalanceData,
}

#[derive(Debug)]
pub struct MarkInfoRequest {
    pub(super) id: String,
    pub return_path: Sender<MarkInfoResponse>,
}

#[derive(Debug)]
pub struct MarkInfoResponse {
    pub id: String,
    pub status: Status<MarkStatus>,
    pub pou_status: Status<MarkPOUStatus>,
    pub labels: Vec<MarkLabel>,
    pub variations: Vec<MarkVariation>,
    pub creation_date: Option<DateTime<Utc>>,
    pub update_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub pou_expiry_date: Option<DateTime<Utc>>,
    pub correct_before: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct Status<T> {
    pub status_type: T,
    pub message: Option<String>,
}

#[derive(Debug)]
pub enum MarkStatus {
    New,
    Verified,
    Incorrect,
    Corrected,
    Invalid,
    Expired,
    Deactivated,
}

#[derive(Debug)]
pub enum MarkPOUStatus {
    NotSet,
    Valid,
    Invalid,
    Expired,
    NA,
    New,
    Incorrect,
    Corrected,
}

#[derive(Debug)]
pub struct MarkLabel {
    pub a_label: String,
    pub u_label: String,
    pub smd_inclusion: bool,
    pub claim_notify: bool,
    pub trex: Option<TrexInfo>,
}

#[derive(Debug)]
pub struct TrexInfo {
    pub enabled: bool,
    pub until: Option<DateTime<Utc>>,
    pub tlds: Vec<TrexTLD>,
}

#[derive(Debug)]
pub struct TrexTLD {
    pub tld: String,
    pub status: TrexStatus,
    pub comment: Option<String>,
}

#[derive(Debug)]
pub enum TrexStatus {
    NotProtectedOverride,
    NotProtectedRegistered,
    NotProtectedExempt,
    NotProtectedOther,
    Protected,
    Unavailable,
    Eligible,
    NoInfo,
}

#[derive(Debug)]
pub struct MarkVariation {
    pub a_label: String,
    pub u_label: String,
    pub variation_type: String,
    pub active: bool,
}

#[derive(Debug)]
pub struct MarkSMDInfoRequest {
    pub(super) id: String,
    pub return_path: Sender<MarkSMDInfoResponse>,
}

#[derive(Debug)]
pub struct MarkSMDInfoResponse {
    pub id: String,
    pub status: Status<MarkStatus>,
    pub smd_id: String,
    pub smd: String,
}

#[derive(Debug)]
pub struct UpdateRequest {
    pub(super) id: String,
    pub(super) add: Vec<UpdateAdd>,
    pub(super) remove: Vec<UpdateRemove>,
    pub(super) new_mark: Option<super::mark::Mark>,
    pub(super) update_labels: Vec<CreateLabel>,
    pub(super) update_cases: Vec<CaseUpdate>,
    pub return_path: Sender<UpdateResponse>,
}

#[derive(Debug)]
pub enum UpdateAdd {
    Document(Document),
    Label(CreateLabel),
    Variation(String),
    Case(AddCase),
}

#[derive(Debug)]
pub enum UpdateRemove {
    Label(String),
    Variation(String),
}

#[derive(Debug)]
pub struct AddCase {
    pub id: String,
    pub case: CaseType,
    pub documents: Vec<CaseDocument>,
    pub labels: Vec<String>,
}

#[derive(Debug)]
pub enum CaseType {
    Udrp {
        case_id: String,
        provider: String,
        case_language: String,
    },
    Court {
        decision_id: String,
        court_name: String,
        country_code: String,
        case_language: String,
        regions: Vec<String>,
    },
}

#[derive(Debug)]
pub struct CaseDocument {
    pub class: CaseDocumentClass,
    pub file_name: String,
    pub file_type: FileType,
    pub contents: Vec<u8>,
}

#[derive(Debug)]
pub enum CaseDocumentClass {
    CourtDecision,
    Other,
}

#[derive(Debug)]
pub struct CaseUpdate {
    pub id: String,
    pub add: Vec<CaseAdd>,
    pub remove: Vec<CaseRemove>,
    pub new_case: Option<CaseType>,
}

#[derive(Debug)]
pub enum CaseAdd {
    Label(String),
    Document(CaseDocument),
}

#[derive(Debug)]
pub enum CaseRemove {
    Label(String),
}

#[derive(Debug)]
pub struct UpdateResponse {}

#[derive(Debug)]
pub struct RenewRequest {
    pub(super) id: String,
    pub(super) add_period: Option<super::Period>,
    pub(super) cur_expiry_date: DateTime<Utc>,
    pub return_path: Sender<RenewResponse>,
}

#[derive(Debug)]
pub struct RenewResponse {
    pub id: String,
    pub new_expiry_date: Option<DateTime<Utc>>,
    pub balance: BalanceData,
}

#[derive(Debug)]
pub struct TransferInitiateRequest {
    pub(super) id: String,
    pub return_path: Sender<TransferInitiateResponse>,
}

#[derive(Debug)]
pub struct TransferInitiateResponse {
    pub id: String,
    pub auth_info: String,
}

#[derive(Debug)]
pub struct TransferRequest {
    pub(super) id: String,
    pub(super) auth_info: String,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferResponse {
    pub id: String,
    pub transfer_date: Option<DateTime<Utc>>,
    pub balance: BalanceData,
}

#[derive(Debug)]
pub struct TrexActivateRequest {
    pub(super) id: String,
    pub(super) labels: Vec<TrexActivateLabel>,
    pub return_path: Sender<TrexActivateResponse>,
}

#[derive(Debug)]
pub struct TrexActivateLabel {
    pub label: String,
    pub period: Option<super::Period>,
}

#[derive(Debug)]
pub struct TrexActivateResponse {}

#[derive(Debug)]
pub struct TrexRenewRequest {
    pub(super) id: String,
    pub(super) labels: Vec<TrexRenewLabel>,
    pub return_path: Sender<TrexRenewResponse>,
}

#[derive(Debug)]
pub struct TrexRenewLabel {
    pub label: String,
    pub current_expiry_date: NaiveDate,
    pub period: Option<super::Period>,
}

#[derive(Debug)]
pub struct TrexRenewResponse {}

#[derive(Debug)]
pub struct BalanceData {
    pub value: String,
    pub currency: String,
    pub status_points: u32,
}

/// Checks if a mark ID is available
///
/// # Arguments
/// * `id` - The ID to check
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn check(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<CheckResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::TMCHCheck(Box::new(CheckRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Creates a new mark
///
/// # Arguments
/// * `mark` - Mark info
/// * `period` - Initial period to create the mark for
/// * `documents` - Initial documents to attach to the mark
/// * `labels` - Initial labels to include in the mark
/// * `variations` - Initial label variations to include in the mark
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn create(
    mark: super::mark::Mark,
    period: Option<super::Period>,
    documents: Vec<Document>,
    labels: Vec<CreateLabel>,
    variations: Vec<String>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<CreateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::TMCHCreate(Box::new(CreateRequest {
            mark,
            period,
            documents,
            labels,
            variations,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Fetches information about a mark
///
/// # Arguments
/// * `id` - The ID of the mark in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn mark_info(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<MarkInfoResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::TMCHMarkInfo(Box::new(MarkInfoRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Fetches a mark SMD
///
/// # Arguments
/// * `id` - The ID of the mark in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn mark_smd_info(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<MarkSMDInfoResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::TMCHMarkSMDInfo(Box::new(MarkSMDInfoRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Fetches a mark encoded SMD
///
/// # Arguments
/// * `id` - The ID of the mark in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn mark_encoded_smd_info(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<MarkSMDInfoResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::TMCHMarkEncodedSMDInfo(Box::new(MarkSMDInfoRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Fetches a mark file
///
/// # Arguments
/// * `id` - The ID of the mark in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn mark_file_info(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<MarkSMDInfoResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::TMCHMarkEncodedSMDInfo(Box::new(MarkSMDInfoRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Update a mark
///
/// # Arguments
/// * `id` - The ID to update
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn update(
    id: &str,
    add: Vec<UpdateAdd>,
    remove: Vec<UpdateRemove>,
    new_mark: Option<super::mark::Mark>,
    update_labels: Vec<CreateLabel>,
    update_cases: Vec<CaseUpdate>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<UpdateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::TMCHUpdate(Box::new(UpdateRequest {
            id: id.to_string(),
            add,
            remove,
            new_mark,
            update_labels,
            update_cases,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Extends a mark's validity
///
/// # Arguments
/// * `id` - The ID to renew
/// * `cur_expriry_date` - The current expiry date of the object to be renewed
/// * `add_period` - How long to extend the validity by
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn renew(
    id: &str,
    cur_expiry_date: DateTime<Utc>,
    add_period: Option<super::Period>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<RenewResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::TMCHRenew(Box::new(RenewRequest {
            id: id.to_string(),
            add_period,
            cur_expiry_date,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Requests a transfer code
///
/// # Arguments
/// * `id` - The ID to transfer
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_initiate(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<TransferInitiateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::TMCHTransferInitiate(Box::new(TransferInitiateRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Requests a transfer
///
/// # Arguments
/// * `id` - The ID to transfer
/// * `auth_code` - Code to authorize the transfer
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer(
    id: &str,
    auth_code: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::TMCHTransfer(Box::new(TransferRequest {
            id: id.to_string(),
            auth_info: auth_code.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}
