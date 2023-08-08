//! EPP commands relating to nominet specific features

use super::super::nominet::{
    CancelData, DataQualityData, DataQualityStatus, DomainFailData, HandshakeAcceptRequest,
    HandshakeRejectRequest, HandshakeResponse, HostCancelData, LockRequest, LockResponse, Object,
    ProcessData, ProcessStage, RegistrantTransferData, RegistrarChangeData, ReleaseData,
    ReleaseRequest, ReleaseResponse, SuspendData, Tag, TagListRequest, TagListResponse,
    DomainInfo, RegistrationStatus, BillType, DomainCreate, DomainUpdate
};
use super::super::{proto, Error, Response};
use super::router::HandleReqReturn;
use super::ServerFeatures;
use crate::client::nominet::{ContactValidateRequest, ContactValidateResponse};
use std::convert::TryFrom;

impl From<&proto::nominet::EPPDomainInfoData> for DomainInfo {
    fn from(value: &proto::nominet::EPPDomainInfoData) -> Self {
        DomainInfo {
            registration_status: match value.reg_status {
                proto::nominet::EPPDomainRegistrationStatus::RegisteredUntilExpiry => RegistrationStatus::RegisteredUntilExpiry,
                proto::nominet::EPPDomainRegistrationStatus::RenewalRequired => RegistrationStatus::RenewalRequired,
                proto::nominet::EPPDomainRegistrationStatus::NoLongerRequired => RegistrationStatus::NoLongerRequired,
            },
            first_bill: value.first_bill.as_ref().map(Into::into),
            recur_bill: value.recur_bill.as_ref().map(Into::into),
            auto_bill: value.auto_bill.map(|v| v as u32),
            next_bill: value.next_bill.map(|v| v as u32),
            auto_period: value.auto_period.map(|v| v as u32),
            next_period: value.next_period.map(|v| v as u32),
            renew_not_required: value.renewal_not_required,
            notes: value.notes.clone(),
            reseller: value.reseller.clone(),
        }
    }
}

impl From<&DomainCreate> for proto::nominet::EPPDomainCreate {
    fn from(value: &DomainCreate) -> Self {
        proto::nominet::EPPDomainCreate {
            first_bill: value.first_bill.map(Into::into),
            recur_bill: value.recur_bill.map(Into::into),
            auto_bill: value.auto_bill.map(|v| v as u8),
            next_bill: value.next_bill.map(|v| v as u8),
            auto_period: value.auto_period.map(|v| v as u8),
            next_period: value.next_period.map(|v| v as u8),
            notes: value.notes.clone(),
            reseller: value.reseller.clone(),
        }
    }
}

impl From<&DomainUpdate> for proto::nominet::EPPDomainUpdate {
    fn from(value: &DomainUpdate) -> Self {
        proto::nominet::EPPDomainUpdate {
            first_bill: value.first_bill.map(Into::into).map(Some),
            recur_bill: value.recur_bill.map(Into::into).map(Some),
            auto_bill: match value.auto_bill {
                Some(0) => Some(None),
                Some(v) => Some(Some(v as u8)),
                None => None,
            },
            next_bill: match value.next_bill {
                Some(0) => Some(None),
                Some(v) => Some(Some(v as u8)),
                None => None,
            },
            auto_period: match value.auto_period {
                Some(0) => Some(None),
                Some(v) => Some(Some(v as u8)),
                None => None,
            },
            next_period: match value.next_period {
                Some(0) => Some(None),
                Some(v) => Some(Some(v as u8)),
                None => None,
            },
            renew_not_required: value.renew_not_required,
            notes: value.notes.clone(),
            reseller: match &value.reseller {
                Some(v) if v.is_empty() => Some(None),
                Some(v) => Some(Some(v.to_owned())),
                None => None,
            },
        }
    }
}

impl From<&proto::nominet::EPPDomainBillCode> for BillType {
    fn from(value: &proto::nominet::EPPDomainBillCode) -> Self {
        match value {
            proto::nominet::EPPDomainBillCode::Customer => BillType::BillCustomer,
            proto::nominet::EPPDomainBillCode::Registrar => BillType::BillRegistrar,
        }
    }
}

impl From<BillType> for proto::nominet::EPPDomainBillCode {
    fn from(value: BillType) -> Self {
        match value {
            BillType::BillCustomer => proto::nominet::EPPDomainBillCode::Customer,
            BillType::BillRegistrar => proto::nominet::EPPDomainBillCode::Registrar,
        }
    }
}

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

pub fn handle_accept(
    client: &ServerFeatures,
    req: &HandshakeAcceptRequest,
) -> HandleReqReturn<HandshakeResponse> {
    if !client.nominet_handshake {
        return Err(Err(Error::Unsupported));
    }
    let command = proto::nominet::EPPHandshakeAccept {
        case_id: req.case_id.to_owned(),
        registrant: req.registrant.as_deref().map(Into::into),
    };
    Ok((
        proto::EPPCommandType::Update(Box::new(proto::EPPUpdate::NominetHandshakeAccept(command))),
        None,
    ))
}

pub fn handle_reject(
    client: &ServerFeatures,
    req: &HandshakeRejectRequest,
) -> HandleReqReturn<HandshakeResponse> {
    if !client.nominet_handshake {
        return Err(Err(Error::Unsupported));
    }
    let command = proto::nominet::EPPHandshakeReject {
        case_id: req.case_id.to_owned(),
    };
    Ok((
        proto::EPPCommandType::Update(Box::new(proto::EPPUpdate::NominetHandshakeReject(command))),
        None,
    ))
}

pub fn handle_handshake_response(response: proto::EPPResponse) -> Response<HandshakeResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::NominetHandshakeData(msg) => {
                Response::Ok(HandshakeResponse {
                    case_id: msg.case_id,
                    domains: msg
                        .domain_list
                        .map(|l| l.domain_names)
                        .unwrap_or_else(Vec::new),
                })
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_release(
    client: &ServerFeatures,
    req: &ReleaseRequest,
) -> HandleReqReturn<ReleaseResponse> {
    if !client.nominet_release {
        return Err(Err(Error::Unsupported));
    }
    let command = proto::nominet::EPPRelease {
        registrar_tag: req.registrar_tag.to_owned(),
        object: match &req.object {
            Object::Domain(d) => proto::nominet::EPPReleaseObject::Domain(d.to_owned()),
            Object::Registrant(d) => proto::nominet::EPPReleaseObject::Registrant(d.to_owned()),
        },
    };
    Ok((
        proto::EPPCommandType::Update(Box::new(proto::EPPUpdate::NominetRelease(command))),
        None,
    ))
}

pub fn handle_release_response(response: proto::EPPResponse) -> Response<ReleaseResponse> {
    let pending = response.is_pending();
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::NominetReleasePending(msg) => {
                Response::Ok(ReleaseResponse {
                    pending,
                    message: Some(msg),
                })
            }
            _ => Err(Error::ServerInternal),
        },
        None => Response::Ok(ReleaseResponse {
            pending,
            message: None,
        }),
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
                                    _ => return Err(Error::ServerInternal),
                                },
                            })
                        })
                        .collect::<Response<Vec<Tag>>>()?,
                })
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_contact_validate(
    client: &ServerFeatures,
    req: &ContactValidateRequest,
) -> HandleReqReturn<ContactValidateResponse> {
    if !client.nominet_data_quality {
        return Err(Err(Error::Unsupported));
    }
    let command = proto::EPPUpdate::Contact(proto::contact::EPPContactUpdate {
        id: req.contact_id.to_string(),
        traficom_role: None,
        add: None,
        remove: None,
        change: None,
    });
    Ok((
        proto::EPPCommandType::Update(Box::new(command)),
        Some(vec![
            proto::EPPCommandExtensionType::NominetDataQualityUpdate(
                proto::nominet::EPPDataQualityUpdate::Validate {},
            ),
        ]),
    ))
}

pub fn handle_contact_validate_response(
    response: proto::EPPResponse,
) -> Response<ContactValidateResponse> {
    match response.data {
        Some(_) => Err(Error::ServerInternal),
        None => Response::Ok(ContactValidateResponse {}),
    }
}

pub fn handle_lock(client: &ServerFeatures, req: &LockRequest) -> HandleReqReturn<LockResponse> {
    if !client.nominet_data_quality {
        return Err(Err(Error::Unsupported));
    }
    let command = proto::EPPUpdate::NominetLock(proto::nominet::EPPLock {
        lock_type: req.lock_type.to_string(),
        object_type: match req.object {
            Object::Domain(_) => proto::nominet::EPPLockObjectType::Domain,
            Object::Registrant(_) => proto::nominet::EPPLockObjectType::Registrant,
        },
        object: match &req.object {
            Object::Domain(d) => proto::nominet::EPPLockObject::Domain(d.to_owned()),
            Object::Registrant(d) => proto::nominet::EPPLockObject::Registrant(d.to_owned()),
        },
    });
    Ok((proto::EPPCommandType::Update(Box::new(command)), None))
}

pub fn handle_unlock(client: &ServerFeatures, req: &LockRequest) -> HandleReqReturn<LockResponse> {
    if !client.nominet_data_quality {
        return Err(Err(Error::Unsupported));
    }
    let command = proto::EPPUpdate::NominetUnlock(proto::nominet::EPPLock {
        lock_type: req.lock_type.to_string(),
        object_type: match req.object {
            Object::Domain(_) => proto::nominet::EPPLockObjectType::Domain,
            Object::Registrant(_) => proto::nominet::EPPLockObjectType::Registrant,
        },
        object: match &req.object {
            Object::Domain(d) => proto::nominet::EPPLockObject::Domain(d.to_owned()),
            Object::Registrant(d) => proto::nominet::EPPLockObject::Registrant(d.to_owned()),
        },
    });
    Ok((proto::EPPCommandType::Update(Box::new(command)), None))
}

pub fn handle_lock_response(response: proto::EPPResponse) -> Response<LockResponse> {
    match response.data {
        Some(_) => Err(Error::ServerInternal),
        None => Response::Ok(LockResponse {}),
    }
}
