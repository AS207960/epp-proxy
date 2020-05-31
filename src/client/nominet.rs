//! EPP commands relating to nominet specific features

use std::convert::TryFrom;
use chrono::prelude::*;
use super::router::HandleReqReturn;
use super::{proto, EPPClientServerFeatures, Request, Response, Error, Sender};

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
    pub originator: String
}

impl From<proto::nominet::EPPCancelData> for CancelData {
    fn from(from: proto::nominet::EPPCancelData) -> Self {
        CancelData {
            domain_name: from.domain_name,
            originator: from.originator
        }
    }
}

#[derive(Debug)]
pub struct ReleaseData {
    pub account_id: String,
    pub account_moved: bool,
    pub from: String,
    pub registrar_tag: String,
    pub domains: Vec<String>
}

impl From<proto::nominet::EPPReleaseData> for ReleaseData {
    fn from(from: proto::nominet::EPPReleaseData) -> Self {
        ReleaseData {
            account_id: from.account.id,
            account_moved: from.account.moved,
            from: from.from,
            registrar_tag: from.registrar_tag,
            domains: from.domain_list.domain_names
        }
    }
}

#[derive(Debug)]
pub struct RegistrarChangeData {
    pub originator: String,
    pub registrar_tag: String,
    pub case_id: String,
    pub domains: Vec<super::domain::InfoResponse>,
    pub contact: super::contact::InfoResponse
}

impl TryFrom<proto::nominet::EPPRegistrarChangeData> for RegistrarChangeData {
    type Error = Error;

    fn try_from(from: proto::nominet::EPPRegistrarChangeData) -> Result<Self, Self::Error> {
        Ok(RegistrarChangeData {
            originator: from.originator,
            registrar_tag: from.registrar_tag,
            case_id: from.case_id,
            domains: from.domain_list.domains.into_iter().map(|d| super::domain::InfoResponse::try_from((d, &None))).collect::<Result<Vec<_>, _>>()?,
            contact: super::contact::InfoResponse::try_from((from.contact, &None))?
        })
    }
}

#[derive(Debug)]
pub struct HostCancelData {
    pub host_objects: Vec<String>,
    pub domain_names: Vec<String>,
}

impl From<proto::nominet::EPPHostCancelData> for HostCancelData {
    fn from(from: proto::nominet::EPPHostCancelData) -> Self {
        HostCancelData {
            host_objects: from.host_list.host_objects,
            domain_names: from.domain_list.domain_names,
        }
    }
}

#[derive(Debug)]
pub struct ProcessData {
    pub stage: ProcessStage,
    pub contact: super::contact::InfoResponse,
    pub process_type: String,
    pub suspend_date: Option<DateTime<Utc>>,
    pub cancel_date: Option<DateTime<Utc>>,
    pub domain_names: Vec<String>
}

#[derive(Debug)]
pub enum ProcessStage {
    Initial,
    Updated,
}

impl TryFrom<proto::nominet::EPPProcessData> for ProcessData {
    type Error = Error;

    fn try_from(from: proto::nominet::EPPProcessData) -> Result<Self, Self::Error> {
        Ok(ProcessData {
            stage: match from.stage {
                proto::nominet::EPPProcessStage::Initial => ProcessStage::Initial,
                proto::nominet::EPPProcessStage::Updated => ProcessStage::Updated,
            },
            contact: super::contact::InfoResponse::try_from((from.contact, &None))?,
            process_type: from.process_type,
            suspend_date: from.suspend_date,
            cancel_date: from.cancel_date,
            domain_names: from.domain_list.domain_names,
        })
    }
}

#[derive(Debug)]
pub struct SuspendData {
    pub reason: String,
    pub cancel_date: Option<DateTime<Utc>>,
    pub domain_names: Vec<String>
}

impl From<proto::nominet::EPPSuspendData> for SuspendData {
    fn from(from: proto::nominet::EPPSuspendData) -> Self {
        SuspendData {
            reason: from.reason,
            cancel_date: from.cancel_date,
            domain_names: from.domain_list.domain_names,
        }
    }
}

#[derive(Debug)]
pub struct DomainFailData {
    pub reason: String,
    pub domain_name: String
}

impl From<proto::nominet::EPPDomainFailData> for DomainFailData {
    fn from(from: proto::nominet::EPPDomainFailData) -> Self {
        DomainFailData {
            reason: from.reason,
            domain_name: from.domain_name,
        }
    }
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

impl TryFrom<proto::nominet::EPPTransferData> for RegistrantTransferData {
    type Error = Error;

    fn try_from(from: proto::nominet::EPPTransferData) -> Result<Self, Self::Error> {
        Ok(RegistrantTransferData {
            originator: from.originator,
            account_id: from.account_id,
            old_account_id: from.old_account_id,
            case_id: from.case_id,
            domain_names: from.domain_list.domain_names,
            contact: super::contact::InfoResponse::try_from((from.contact, &None))?,
        })
    }
}

pub fn handle_tag_list(
    client: &EPPClientServerFeatures,
    _req: &TagListRequest,
) -> HandleReqReturn<TagListResponse> {
    if !client.nominet_tag_list {
        return Err(Err(Error::Unsupported));
    }
    let command = proto::EPPInfo::TagList {};
    Ok((proto::EPPCommandType::Info(command), None))
}

pub fn handle_tag_list_response(response: proto::EPPResponse) -> Response<TagListResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPNominetTagInfoResult(tag_list) => {
                Response::Ok(TagListResponse {
                    tags: tag_list
                        .tags
                        .into_iter()
                        .map(|t| {
                            Ok(Tag {
                                tag: t.tag,
                                name: t.name,
                                trading_name: t.trading_name,
                                handshake: match t.handshake.as_str() {
                                    "Y" => true,
                                    "N" => false,
                                    _ => return Err(Error::InternalServerError),
                                },
                            })
                        })
                        .collect::<Response<Vec<Tag>>>()?,
                })
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

/// Fetches a list of registered tags
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn tag_list(
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<TagListResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::NominetTagList(Box::new(TagListRequest {
            return_path: sender,
        })),
        receiver,
    )
    .await
}
