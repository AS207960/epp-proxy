//! EPP commands relating to nominet specific features

use super::super::nominet::{
    CancelData, DataQualityData, DataQualityStatus, DomainFailData, HostCancelData, ProcessData,
    ProcessStage, RegistrantTransferData, RegistrarChangeData, ReleaseData, SuspendData, Tag,
    TagListRequest, TagListResponse,
};
use super::super::{proto, Error, Response, ServerFeatures};
use super::router::HandleReqReturn;
use std::convert::TryFrom;

impl From<proto::nominet::EPPCancelData> for CancelData {
    fn from(from: proto::nominet::EPPCancelData) -> Self {
        CancelData {
            domain_name: from.domain_name,
            originator: from.originator,
        }
    }
}

impl From<proto::nominet::EPPReleaseData> for ReleaseData {
    fn from(from: proto::nominet::EPPReleaseData) -> Self {
        ReleaseData {
            account_id: from.account.id,
            account_moved: from.account.moved,
            from: from.from,
            registrar_tag: from.registrar_tag,
            domains: from.domain_list.domain_names,
        }
    }
}

impl
    TryFrom<(
        proto::nominet::EPPRegistrarChangeData,
        &Option<proto::EPPResponseExtension>,
    )> for RegistrarChangeData
{
    type Error = Error;

    fn try_from(
        from: (
            proto::nominet::EPPRegistrarChangeData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (rc_data, extension) = from;
        Ok(RegistrarChangeData {
            originator: rc_data.originator,
            registrar_tag: rc_data.registrar_tag,
            case_id: rc_data.case_id,
            domains: match rc_data.domain_list {
                Some(d) => d
                    .domains
                    .into_iter()
                    .map(|d| super::super::domain::InfoResponse::try_from((d, extension)))
                    .collect::<Result<Vec<_>, _>>()?,
                None => vec![],
            },
            contact: super::super::contact::InfoResponse::try_from((rc_data.contact, extension))?,
        })
    }
}

impl From<proto::nominet::EPPHostCancelData> for HostCancelData {
    fn from(from: proto::nominet::EPPHostCancelData) -> Self {
        HostCancelData {
            host_objects: from.host_list.host_objects,
            domain_names: from.domain_list.domain_names,
        }
    }
}

impl
    TryFrom<(
        proto::nominet::EPPProcessData,
        &Option<proto::EPPResponseExtension>,
    )> for ProcessData
{
    type Error = Error;

    fn try_from(
        from: (
            proto::nominet::EPPProcessData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (process_data, extensions) = from;
        Ok(ProcessData {
            stage: match process_data.stage {
                proto::nominet::EPPProcessStage::Initial => ProcessStage::Initial,
                proto::nominet::EPPProcessStage::Updated => ProcessStage::Updated,
            },
            contact: super::super::contact::InfoResponse::try_from((
                process_data.contact,
                extensions,
            ))?,
            process_type: process_data.process_type,
            suspend_date: process_data.suspend_date,
            cancel_date: process_data.cancel_date,
            domain_names: process_data.domain_list.domain_names,
        })
    }
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

impl From<proto::nominet::EPPDomainFailData> for DomainFailData {
    fn from(from: proto::nominet::EPPDomainFailData) -> Self {
        DomainFailData {
            reason: from.reason,
            domain_name: from.domain_name,
        }
    }
}

impl
    TryFrom<(
        proto::nominet::EPPTransferData,
        &Option<proto::EPPResponseExtension>,
    )> for RegistrantTransferData
{
    type Error = Error;

    fn try_from(
        from: (
            proto::nominet::EPPTransferData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (transfer_data, extensions) = from;
        Ok(RegistrantTransferData {
            originator: transfer_data.originator,
            account_id: transfer_data.account_id,
            old_account_id: transfer_data.old_account_id,
            case_id: transfer_data.case_id,
            domain_names: transfer_data.domain_list.domain_names,
            contact: super::super::contact::InfoResponse::try_from((
                transfer_data.contact,
                extensions,
            ))?,
        })
    }
}

impl From<&proto::nominet::EPPDataQualityInfo> for DataQualityData {
    fn from(from: &proto::nominet::EPPDataQualityInfo) -> Self {
        DataQualityData {
            status: match from.status {
                proto::nominet::EPPDataQualityStatus::Valid => DataQualityStatus::Valid,
                proto::nominet::EPPDataQualityStatus::Invalid => DataQualityStatus::Invalid,
            },
            reason: from.reason.as_ref().map(Into::into),
            date_commenced: from.date_commenced.map(Into::into),
            date_to_suspend: from.date_to_suspend.map(Into::into),
            lock_applied: from.lock_applied,
            domains: from
                .domains
                .as_ref()
                .map(|d| d.domains.iter().map(|s| s.into()).collect()),
        }
    }
}

pub fn handle_tag_list(
    client: &ServerFeatures,
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
            proto::EPPResultDataValue::NominetTagInfoResult(tag_list) => {
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
