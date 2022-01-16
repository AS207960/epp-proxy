use super::super::tmch::{
    AddCase, BalanceData, CaseAdd, CaseDocument, CaseDocumentClass, CaseRemove, CheckRequest,
    CheckResponse, CreateLabel, CreateRequest, CreateResponse, Document, DocumentClass, FileType,
    MarkInfoRequest, MarkInfoResponse, MarkLabel, MarkPOUStatus, MarkSMDInfoRequest,
    MarkSMDInfoResponse, MarkStatus, MarkVariation, RenewRequest, RenewResponse, Status,
    TransferInitiateRequest, TransferInitiateResponse, TransferRequest, TransferResponse,
    TrexStatus, UpdateAdd, UpdateRemove, UpdateRequest, UpdateResponse,
};
use super::super::{Error, Period, PeriodUnit, Response};
use super::router::HandleReqReturn;
use super::tmch_proto;
use crate::client::tmch::CaseType;
use std::convert::{TryFrom, TryInto};

pub(crate) fn check_mark_id<T>(id: &str) -> Result<(), Response<T>> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new("\\d+-\\d+").unwrap();
    }
    if RE.is_match(id) {
        Ok(())
    } else {
        Err(Err(Error::Err("invalid mark_id".to_string())))
    }
}

pub(crate) fn check_case_id<T>(id: &str) -> Result<(), Response<T>> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new("case-\\d+").unwrap();
    }
    if RE.is_match(id) {
        Ok(())
    } else {
        Err(Err(Error::Err("invalid case_id".to_string())))
    }
}

impl From<&CreateLabel> for tmch_proto::TMCHLabel {
    fn from(from: &CreateLabel) -> Self {
        tmch_proto::TMCHLabel {
            label: from.label.to_string(),
            smd_inclusion: Some(tmch_proto::TMCHNotify {
                value: "".to_string(),
                enable: from.smd_inclusion,
            }),
            claims_notify: Some(tmch_proto::TMCHNotify {
                value: "".to_string(),
                enable: from.smd_inclusion,
            }),
            trex_activate: None,
            trex_renew: None,
        }
    }
}

impl From<&Document> for tmch_proto::TMCHDocument {
    fn from(from: &Document) -> Self {
        tmch_proto::TMCHDocument {
            document_class: match from.class {
                DocumentClass::AssigneeDeclaration => {
                    tmch_proto::TMCHDocumentClass::AssigneeDeclaration
                }
                DocumentClass::LicenseeDeclaration => {
                    tmch_proto::TMCHDocumentClass::LicenseeDeclaration
                }
                DocumentClass::DeclarationProofOfUseOneSample => {
                    tmch_proto::TMCHDocumentClass::DeclarationProofOfUseOneSample
                }
                DocumentClass::OtherProofOfUse => tmch_proto::TMCHDocumentClass::OtherProofOfUse,
                DocumentClass::CopyOfCourtOrder => tmch_proto::TMCHDocumentClass::CopyOfCourtOrder,
                DocumentClass::Other => tmch_proto::TMCHDocumentClass::Other,
            },
            file_name: Some(from.file_name.to_string()),
            file_type: match from.file_type {
                FileType::Jpg => tmch_proto::TMCHFileType::Jpg,
                FileType::Pdf => tmch_proto::TMCHFileType::Pdf,
            },
            file_content: base64::encode(&from.contents),
        }
    }
}

impl From<&CaseDocument> for tmch_proto::TMCHCaseDocument {
    fn from(from: &CaseDocument) -> Self {
        tmch_proto::TMCHCaseDocument {
            document_class: match from.class {
                CaseDocumentClass::CourtDecision => {
                    tmch_proto::TMCHCaseDocumentClass::CourtCaseDocument
                }
                CaseDocumentClass::Other => tmch_proto::TMCHCaseDocumentClass::Other,
            },
            file_name: Some(from.file_name.to_string()),
            file_type: match from.file_type {
                FileType::Jpg => tmch_proto::TMCHFileType::Jpg,
                FileType::Pdf => tmch_proto::TMCHFileType::Pdf,
            },
            file_content: base64::encode(&from.contents),
        }
    }
}

impl From<&AddCase> for tmch_proto::TMCHCase {
    fn from(from: &AddCase) -> Self {
        tmch_proto::TMCHCase {
            id: from.id.to_string(),
            case_type: None,
            documents: from.documents.iter().map(Into::into).collect(),
            labels: from
                .labels
                .iter()
                .map(|l| tmch_proto::TMCHCaseLabel {
                    label: l.to_string(),
                })
                .collect(),
        }
    }
}

impl From<&CaseType> for tmch_proto::TMCHCaseType {
    fn from(from: &CaseType) -> Self {
        match from {
            CaseType::Court {
                decision_id,
                country_code,
                regions,
                court_name,
                case_language,
            } => tmch_proto::TMCHCaseType::Court(tmch_proto::TMCHCourt {
                reference: decision_id.to_string(),
                country_code: country_code.to_string(),
                regions: regions.iter().map(Into::into).collect(),
                court_name: court_name.to_string(),
                case_language: case_language.to_string(),
            }),
            CaseType::Udrp {
                case_language,
                case_id,
                provider,
            } => tmch_proto::TMCHCaseType::Udrp(tmch_proto::TMCHUdrp {
                case_number: case_id.to_string(),
                udrp_provider: provider.to_string(),
                case_language: case_language.to_string(),
            }),
        }
    }
}

pub fn handle_check(_client: &(), req: &CheckRequest) -> HandleReqReturn<CheckResponse> {
    check_mark_id(&req.id)?;
    let command = tmch_proto::TMCHCheck {
        id: vec![req.id.to_string()],
    };
    Ok(tmch_proto::TMCHCommandType::Check(command))
}

pub fn handle_check_response(response: tmch_proto::TMCHResponse) -> Response<CheckResponse> {
    match response.data {
        Some(value) => match value.value {
            tmch_proto::TMCHResultDataValue::TMCHCheck(msg) => {
                if let Some(mark_check) = msg.data.first() {
                    Response::Ok(CheckResponse {
                        avail: mark_check.id.available,
                        reason: mark_check.reason.as_ref().map(Into::into),
                    })
                } else {
                    Err(Error::ServerInternal)
                }
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_create(_client: &(), req: &CreateRequest) -> HandleReqReturn<CreateResponse> {
    let command = tmch_proto::TMCHCreate {
        mark: Some((&req.mark).into()),
        period: match req.period.as_ref() {
            Some(r) => Some(r.try_into().map_err(Result::Err)?),
            None => None,
        },
        documents: req.documents.iter().map(Into::into).collect(),
        labels: req.labels.iter().map(Into::into).collect(),
        variations: if req.variations.is_empty() {
            vec![]
        } else {
            vec![tmch_proto::variation::Variation {
                labels: req
                    .variations
                    .iter()
                    .map(|l| tmch_proto::variation::Label {
                        id: None,
                        a_label: l.to_string(),
                        u_label: None,
                        variation_type: None,
                        active: None,
                    })
                    .collect(),
            }]
        },
    };
    Ok(tmch_proto::TMCHCommandType::Create(Box::new(command)))
}

pub fn handle_create_response(response: tmch_proto::TMCHResponse) -> Response<CreateResponse> {
    match response.data {
        Some(value) => match value.value {
            tmch_proto::TMCHResultDataValue::TMCHCreate(msg) => Response::Ok(CreateResponse {
                id: msg.id,
                created_date: msg.creation_date,
                balance: msg.balance.into(),
            }),
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_mark_info(_client: &(), req: &MarkInfoRequest) -> HandleReqReturn<MarkInfoResponse> {
    check_mark_id(&req.id)?;
    let command = tmch_proto::TMCHInfo {
        id: req.id.to_string(),
        id_type: tmch_proto::TMCHInfoType::Info,
    };
    Ok(tmch_proto::TMCHCommandType::Info(command))
}

pub fn handle_mark_smd_info(
    _client: &(),
    req: &MarkSMDInfoRequest,
) -> HandleReqReturn<MarkSMDInfoResponse> {
    check_mark_id(&req.id)?;
    let command = tmch_proto::TMCHInfo {
        id: req.id.to_string(),
        id_type: tmch_proto::TMCHInfoType::SingedMark,
    };
    Ok(tmch_proto::TMCHCommandType::Info(command))
}

pub fn handle_mark_encoded_smd_info(
    _client: &(),
    req: &MarkSMDInfoRequest,
) -> HandleReqReturn<MarkSMDInfoResponse> {
    check_mark_id(&req.id)?;
    let command = tmch_proto::TMCHInfo {
        id: req.id.to_string(),
        id_type: tmch_proto::TMCHInfoType::EncodedSignedMark,
    };
    Ok(tmch_proto::TMCHCommandType::Info(command))
}

pub fn handle_mark_file_info(
    _client: &(),
    req: &MarkSMDInfoRequest,
) -> HandleReqReturn<MarkSMDInfoResponse> {
    check_mark_id(&req.id)?;
    let command = tmch_proto::TMCHInfo {
        id: req.id.to_string(),
        id_type: tmch_proto::TMCHInfoType::File,
    };
    Ok(tmch_proto::TMCHCommandType::Info(command))
}

impl From<tmch_proto::TMCHStatus<tmch_proto::TMCHStatusType>> for Status<MarkStatus> {
    fn from(from: tmch_proto::TMCHStatus<tmch_proto::TMCHStatusType>) -> Self {
        Status {
            status_type: match from.status {
                tmch_proto::TMCHStatusType::New => MarkStatus::New,
                tmch_proto::TMCHStatusType::Verified => MarkStatus::Verified,
                tmch_proto::TMCHStatusType::Incorrect => MarkStatus::Incorrect,
                tmch_proto::TMCHStatusType::Corrected => MarkStatus::Corrected,
                tmch_proto::TMCHStatusType::Invalid => MarkStatus::Invalid,
                tmch_proto::TMCHStatusType::Expired => MarkStatus::Expired,
                tmch_proto::TMCHStatusType::Deactivated => MarkStatus::Deactivated,
            },
            message: from.message,
        }
    }
}

impl From<tmch_proto::TMCHStatus<tmch_proto::TMCHPOUStatusType>> for Status<MarkPOUStatus> {
    fn from(from: tmch_proto::TMCHStatus<tmch_proto::TMCHPOUStatusType>) -> Self {
        Status {
            status_type: match from.status {
                tmch_proto::TMCHPOUStatusType::NotSet => MarkPOUStatus::NotSet,
                tmch_proto::TMCHPOUStatusType::NA => MarkPOUStatus::NA,
                tmch_proto::TMCHPOUStatusType::New => MarkPOUStatus::New,
                tmch_proto::TMCHPOUStatusType::Incorrect => MarkPOUStatus::Incorrect,
                tmch_proto::TMCHPOUStatusType::Corrected => MarkPOUStatus::Corrected,
                tmch_proto::TMCHPOUStatusType::Valid => MarkPOUStatus::Valid,
                tmch_proto::TMCHPOUStatusType::Invalid => MarkPOUStatus::Invalid,
                tmch_proto::TMCHPOUStatusType::Expired => MarkPOUStatus::Expired,
            },
            message: from.message,
        }
    }
}

impl From<tmch_proto::trex::TLDStatus> for TrexStatus {
    fn from(from: tmch_proto::trex::TLDStatus) -> Self {
        match from {
            tmch_proto::trex::TLDStatus::NotProtectedRegistered => {
                TrexStatus::NotProtectedRegistered
            }
            tmch_proto::trex::TLDStatus::NotProtectedExempt => TrexStatus::NotProtectedExempt,
            tmch_proto::trex::TLDStatus::NotProtectedOther => TrexStatus::NotProtectedOther,
            tmch_proto::trex::TLDStatus::NotProtectedOverride => TrexStatus::NotProtectedOverride,
            tmch_proto::trex::TLDStatus::Eligible => TrexStatus::Eligible,
            tmch_proto::trex::TLDStatus::Protected => TrexStatus::Protected,
            tmch_proto::trex::TLDStatus::Unavailable => TrexStatus::Unavailable,
            tmch_proto::trex::TLDStatus::NoInfo => TrexStatus::NoInfo,
        }
    }
}

impl From<tmch_proto::TMCHInfoLabel> for MarkLabel {
    fn from(from: tmch_proto::TMCHInfoLabel) -> Self {
        MarkLabel {
            a_label: from.label,
            u_label: from.ulabel,
            smd_inclusion: from.smd_inclusion.map(|n| n.enable).unwrap_or(false),
            claim_notify: from.claims_notify.map(|n| n.enable).unwrap_or(false),
            trex: from.trex.map(Into::into),
        }
    }
}

impl From<tmch_proto::variation::Label> for MarkVariation {
    fn from(from: tmch_proto::variation::Label) -> Self {
        MarkVariation {
            a_label: from.a_label.clone(),
            u_label: from.u_label.unwrap_or(from.a_label),
            variation_type: from.variation_type.unwrap_or_default(),
            active: from.active.map(|a| a.enable).unwrap_or(true),
        }
    }
}

impl From<tmch_proto::TMCHBalance> for BalanceData {
    fn from(from: tmch_proto::TMCHBalance) -> Self {
        BalanceData {
            value: from.amount.value,
            currency: from.amount.currency,
            status_points: from.status_points,
        }
    }
}

impl TryFrom<&Period> for tmch_proto::TMCHPeriod {
    type Error = Error;

    fn try_from(from: &Period) -> Result<Self, Self::Error> {
        Ok(tmch_proto::TMCHPeriod {
            value: std::cmp::min(99, std::cmp::max(from.value, 1)),
            unit: match from.unit {
                PeriodUnit::Years => tmch_proto::TMCHPeriodUnit::Years,
                PeriodUnit::Months => {
                    return Err(Error::Err(
                        "month based periods are not valid for TMCH".to_string(),
                    ))
                }
            },
        })
    }
}

pub fn handle_mark_info_response(response: tmch_proto::TMCHResponse) -> Response<MarkInfoResponse> {
    match response.data {
        Some(value) => match value.value {
            tmch_proto::TMCHResultDataValue::TMCHInfo(msg) => match (msg.pou_status, msg.mark) {
                (Some(pou_status), Some(_mark)) => Response::Ok(MarkInfoResponse {
                    id: msg.id,
                    status: msg.status.into(),
                    pou_status: pou_status.into(),
                    labels: msg.labels.into_iter().map(Into::into).collect(),
                    variations: msg
                        .variations
                        .into_iter()
                        .flat_map(|v| v.labels)
                        .map(Into::into)
                        .collect(),
                    creation_date: msg.creation_date,
                    update_date: msg.update_date,
                    expiry_date: msg.expiry_date,
                    pou_expiry_date: msg.pou_expiry_date,
                    correct_before: msg.correct_before,
                }),
                _ => Err(Error::ServerInternal),
            },
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

#[derive(Debug, Serialize)]
struct SMDInfo {
    #[serde(rename = "{urn:ietf:params:xml:ns:signedMark-1.0}signedMark")]
    pub signed_mark: String,
}

pub fn handle_mark_smd_info_response(
    response: tmch_proto::TMCHResponse,
) -> Response<MarkSMDInfoResponse> {
    match response.data {
        Some(value) => match value.value {
            tmch_proto::TMCHResultDataValue::TMCHInfo(msg) => match (msg.signed_mark, msg.smd_id) {
                (Some(smd), Some(smd_id)) => {
                    let smd = SMDInfo { signed_mark: smd };

                    Response::Ok(MarkSMDInfoResponse {
                        id: msg.id,
                        status: msg.status.into(),
                        smd_id,
                        smd: xml_serde::to_string(&smd).unwrap(),
                    })
                }
                _ => Err(Error::ServerInternal),
            },
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_mark_encoded_smd_info_response(
    response: tmch_proto::TMCHResponse,
) -> Response<MarkSMDInfoResponse> {
    match response.data {
        Some(value) => match value.value {
            tmch_proto::TMCHResultDataValue::TMCHInfo(msg) => {
                match (msg.encoded_signed_mark, msg.smd_id) {
                    (Some(smd), Some(smd_id)) => Response::Ok(MarkSMDInfoResponse {
                        id: msg.id,
                        status: msg.status.into(),
                        smd_id,
                        smd,
                    }),
                    _ => Err(Error::ServerInternal),
                }
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_mark_file_info_response(
    response: tmch_proto::TMCHResponse,
) -> Response<MarkSMDInfoResponse> {
    match response.data {
        Some(value) => match value.value {
            tmch_proto::TMCHResultDataValue::TMCHInfo(msg) => match (msg.enc_file, msg.smd_id) {
                (Some(smd), Some(smd_id)) => Response::Ok(MarkSMDInfoResponse {
                    id: msg.id,
                    status: msg.status.into(),
                    smd_id,
                    smd,
                }),
                _ => Err(Error::ServerInternal),
            },
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_update(_client: &(), req: &UpdateRequest) -> HandleReqReturn<UpdateResponse> {
    let mut add_documents: Vec<tmch_proto::TMCHDocument> = vec![];
    let mut add_labels: Vec<tmch_proto::TMCHLabel> = vec![];
    let mut add_variations: Vec<String> = vec![];
    let mut add_cases: Vec<tmch_proto::TMCHCase> = vec![];
    let mut rem_labels: Vec<tmch_proto::TMCHLabel> = vec![];
    let mut rem_variations: Vec<String> = vec![];
    let mut rem_cases: Vec<tmch_proto::TMCHCase> = vec![];
    let mut chg_cases: Vec<tmch_proto::TMCHCaseType> = vec![];

    for add in &req.add {
        match add {
            UpdateAdd::Document(d) => {
                add_documents.push(d.into());
            }
            UpdateAdd::Label(d) => {
                add_labels.push(d.into());
            }
            UpdateAdd::Variation(d) => {
                add_variations.push(d.to_string());
            }
            UpdateAdd::Case(d) => {
                check_case_id(&d.id)?;
                add_cases.push(d.into());
            }
        }
    }

    for remove in &req.remove {
        match remove {
            UpdateRemove::Label(d) => {
                rem_labels.push(tmch_proto::TMCHLabel {
                    label: d.to_string(),
                    smd_inclusion: None,
                    claims_notify: None,
                    trex_activate: None,
                    trex_renew: None,
                });
            }
            UpdateRemove::Variation(d) => {
                rem_variations.push(d.to_string());
            }
        }
    }

    for case in &req.update_cases {
        check_case_id(&case.id)?;
        let mut add_case_documents: Vec<tmch_proto::TMCHCaseDocument> = vec![];
        let mut add_case_labels: Vec<tmch_proto::TMCHCaseLabel> = vec![];
        let mut rem_case_labels: Vec<tmch_proto::TMCHCaseLabel> = vec![];
        for case_add in &case.add {
            match case_add {
                CaseAdd::Document(d) => {
                    add_case_documents.push(d.into());
                }
                CaseAdd::Label(l) => {
                    add_case_labels.push(tmch_proto::TMCHCaseLabel {
                        label: l.to_string(),
                    });
                }
            }
        }
        for case_rem in &case.remove {
            match case_rem {
                CaseRemove::Label(l) => {
                    rem_case_labels.push(tmch_proto::TMCHCaseLabel {
                        label: l.to_string(),
                    });
                }
            }
        }

        if !(add_case_labels.is_empty() && add_case_documents.is_empty()) {
            add_cases.push(tmch_proto::TMCHCase {
                id: case.id.clone(),
                case_type: None,
                documents: add_case_documents,
                labels: add_case_labels,
            })
        }

        if !rem_case_labels.is_empty() {
            rem_cases.push(tmch_proto::TMCHCase {
                id: case.id.clone(),
                case_type: None,
                documents: vec![],
                labels: rem_case_labels,
            })
        }

        if let Some(case) = &case.new_case {
            chg_cases.push(case.into())
        }
    }

    let command = tmch_proto::TMCHUpdate {
        id: req.id.to_string(),
        case: None,
        add: if add_documents.is_empty()
            && add_labels.is_empty()
            && add_variations.is_empty()
            && add_cases.is_empty()
        {
            None
        } else {
            Some(tmch_proto::TMCHAdd {
                documents: add_documents,
                labels: add_labels,
                variations: if add_variations.is_empty() {
                    vec![]
                } else {
                    vec![tmch_proto::variation::Variation {
                        labels: add_variations
                            .into_iter()
                            .map(|l| tmch_proto::variation::Label {
                                id: None,
                                a_label: l,
                                u_label: None,
                                active: None,
                                variation_type: None,
                            })
                            .collect(),
                    }]
                },
                cases: add_cases,
            })
        },
        remove: if rem_labels.is_empty() && rem_variations.is_empty() && rem_cases.is_empty() {
            None
        } else {
            Some(tmch_proto::TMCHRemove {
                labels: rem_labels,
                variations: if rem_variations.is_empty() {
                    vec![]
                } else {
                    vec![tmch_proto::variation::Variation {
                        labels: rem_variations
                            .into_iter()
                            .map(|l| tmch_proto::variation::Label {
                                id: None,
                                a_label: l,
                                u_label: None,
                                active: None,
                                variation_type: None,
                            })
                            .collect(),
                    }]
                },
                cases: rem_cases,
            })
        },
        change: if req.new_mark.is_none() && req.update_labels.is_empty() {
            None
        } else {
            Some(tmch_proto::TMCHChange {
                mark: req.new_mark.as_ref().map(Into::into),
                labels: req.update_labels.iter().map(Into::into).collect(),
                case: None,
            })
        },
    };
    Ok(tmch_proto::TMCHCommandType::Update(Box::new(command)))
}

pub fn handle_update_response(response: tmch_proto::TMCHResponse) -> Response<UpdateResponse> {
    match response.data {
        Some(_) => Err(Error::ServerInternal),
        None => Response::Ok(UpdateResponse {}),
    }
}

pub fn handle_renew(_client: &(), req: &RenewRequest) -> HandleReqReturn<RenewResponse> {
    check_mark_id(&req.id)?;
    let command = tmch_proto::TMCHRenew {
        id: req.id.to_string(),
        current_expiry_date: req.cur_expiry_date.date(),
        period: match req.add_period.as_ref() {
            Some(r) => Some(r.try_into().map_err(Result::Err)?),
            None => None,
        },
    };
    Ok(tmch_proto::TMCHCommandType::Renew(command))
}

pub fn handle_renew_response(response: tmch_proto::TMCHResponse) -> Response<RenewResponse> {
    match response.data {
        Some(value) => match value.value {
            tmch_proto::TMCHResultDataValue::TMCHRenew(msg) => Response::Ok(RenewResponse {
                id: msg.id,
                new_expiry_date: msg.expiry_date,
                balance: msg.balance.into(),
            }),
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_transfer_initiate(
    _client: &(),
    req: &TransferInitiateRequest,
) -> HandleReqReturn<TransferInitiateResponse> {
    check_mark_id(&req.id)?;
    let command = tmch_proto::TMCHTransfer {
        id: req.id.to_string(),
        auth_code: None,
        operation: tmch_proto::TMCHTransferOperation::Initiate,
    };
    Ok(tmch_proto::TMCHCommandType::Transfer(command))
}

pub fn handle_transfer_initiate_response(
    response: tmch_proto::TMCHResponse,
) -> Response<TransferInitiateResponse> {
    match response.data {
        Some(value) => match value.value {
            tmch_proto::TMCHResultDataValue::TMCHInfo(msg) => match msg.auth_info {
                Some(auth_info) => Response::Ok(TransferInitiateResponse {
                    id: msg.id,
                    auth_info,
                }),
                None => Err(Error::ServerInternal),
            },
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_transfer(_client: &(), req: &TransferRequest) -> HandleReqReturn<TransferResponse> {
    check_mark_id(&req.id)?;
    let command = tmch_proto::TMCHTransfer {
        id: req.id.to_string(),
        auth_code: Some(req.auth_info.to_string()),
        operation: tmch_proto::TMCHTransferOperation::Execute,
    };
    Ok(tmch_proto::TMCHCommandType::Transfer(command))
}

pub fn handle_transfer_response(response: tmch_proto::TMCHResponse) -> Response<TransferResponse> {
    match response.data {
        Some(value) => match value.value {
            tmch_proto::TMCHResultDataValue::TMCHTransfer(msg) => Response::Ok(TransferResponse {
                id: msg.id,
                transfer_date: msg.transfer_date,
                balance: msg.balance.into(),
            }),
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}
