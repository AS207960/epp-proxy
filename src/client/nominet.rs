//! EPP commands relating to nominet specific features

use super::{CommandResponse, RequestMessage, Sender};
use chrono::prelude::*;

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
