use super::super::client;
use super::epp_proto;
use std::convert::TryFrom;

// fn mark_status_from_i32(from: i32) -> Option<client::tmch::MarkStatus> {
//     epp_proto::tmch::MarkStatusType::from_i32(from).and_then(|e| match e {
//         epp_proto::tmch::MarkStatusType::Unknown => None,
//         epp_proto::tmch::MarkStatusType::New => Some(client::tmch::MarkStatus::New),
//         epp_proto::tmch::MarkStatusType::Verified => Some(client::tmch::MarkStatus::Verified),
//         epp_proto::tmch::MarkStatusType::Incorrect => Some(client::tmch::MarkStatus::Incorrect),
//         epp_proto::tmch::MarkStatusType::Corrected => Some(client::tmch::MarkStatus::Corrected),
//         epp_proto::tmch::MarkStatusType::Invalid => Some(client::tmch::MarkStatus::Invalid),
//         epp_proto::tmch::MarkStatusType::Expired => Some(client::tmch::MarkStatus::Expired),
//         epp_proto::tmch::MarkStatusType::Deactivated => Some(client::tmch::MarkStatus::Deactivated),
//     })
// }

fn i32_from_mark_status(from: client::tmch::MarkStatus) -> i32 {
    match from {
        client::tmch::MarkStatus::New => epp_proto::tmch::MarkStatusType::New.into(),
        client::tmch::MarkStatus::Verified => epp_proto::tmch::MarkStatusType::Verified.into(),
        client::tmch::MarkStatus::Incorrect => epp_proto::tmch::MarkStatusType::Incorrect.into(),
        client::tmch::MarkStatus::Corrected => epp_proto::tmch::MarkStatusType::Corrected.into(),
        client::tmch::MarkStatus::Invalid => epp_proto::tmch::MarkStatusType::Invalid.into(),
        client::tmch::MarkStatus::Expired => epp_proto::tmch::MarkStatusType::Expired.into(),
        client::tmch::MarkStatus::Deactivated => {
            epp_proto::tmch::MarkStatusType::Deactivated.into()
        }
    }
}

fn i32_from_mark_pou_status(from: client::tmch::MarkPOUStatus) -> i32 {
    match from {
        client::tmch::MarkPOUStatus::NotSet => epp_proto::tmch::MarkPouStatusType::PouNotSet.into(),
        client::tmch::MarkPOUStatus::Valid => epp_proto::tmch::MarkPouStatusType::PouValid.into(),
        client::tmch::MarkPOUStatus::Invalid => {
            epp_proto::tmch::MarkPouStatusType::PouInvalid.into()
        }
        client::tmch::MarkPOUStatus::Expired => {
            epp_proto::tmch::MarkPouStatusType::PouExpired.into()
        }
        client::tmch::MarkPOUStatus::NA => epp_proto::tmch::MarkPouStatusType::Pouna.into(),
        client::tmch::MarkPOUStatus::New => epp_proto::tmch::MarkPouStatusType::PouNew.into(),
        client::tmch::MarkPOUStatus::Incorrect => {
            epp_proto::tmch::MarkPouStatusType::PouIncorrect.into()
        }
        client::tmch::MarkPOUStatus::Corrected => {
            epp_proto::tmch::MarkPouStatusType::PouCorrected.into()
        }
    }
}

impl From<client::tmch::MarkLabel> for epp_proto::tmch::MarkLabel {
    fn from(res: client::tmch::MarkLabel) -> Self {
        epp_proto::tmch::MarkLabel {
            a_label: res.a_label,
            u_label: res.u_label,
            smd_inclusion: res.smd_inclusion,
            claim_notify: res.claim_notify,
        }
    }
}

impl From<client::tmch::MarkVariation> for epp_proto::tmch::MarkVariation {
    fn from(res: client::tmch::MarkVariation) -> Self {
        epp_proto::tmch::MarkVariation {
            a_label: res.a_label,
            u_label: res.u_label,
            variation_type: res.variation_type,
            active: res.active,
        }
    }
}

impl From<client::tmch::Status<client::tmch::MarkStatus>> for epp_proto::tmch::MarkStatus {
    fn from(res: client::tmch::Status<client::tmch::MarkStatus>) -> Self {
        epp_proto::tmch::MarkStatus {
            status_type: i32_from_mark_status(res.status_type),
            message: res.message,
        }
    }
}

impl From<client::tmch::Status<client::tmch::MarkPOUStatus>> for epp_proto::tmch::MarkPouStatus {
    fn from(res: client::tmch::Status<client::tmch::MarkPOUStatus>) -> Self {
        epp_proto::tmch::MarkPouStatus {
            status_type: i32_from_mark_pou_status(res.status_type),
            message: res.message,
        }
    }
}

impl From<client::tmch::MarkInfoResponse> for epp_proto::tmch::MarkInfoResponse {
    fn from(res: client::tmch::MarkInfoResponse) -> Self {
        epp_proto::tmch::MarkInfoResponse {
            id: res.id,
            status: Some(res.status.into()),
            pou_status: Some(res.pou_status.into()),
            labels: res.labels.into_iter().map(Into::into).collect(),
            variations: res.variations.into_iter().map(Into::into).collect(),
            creation_date: super::utils::chrono_to_proto(res.creation_date),
            update_date: super::utils::chrono_to_proto(res.update_date),
            expiry_date: super::utils::chrono_to_proto(res.expiry_date),
            pou_expiry_date: super::utils::chrono_to_proto(res.pou_expiry_date),
            correct_before: super::utils::chrono_to_proto(res.correct_before),
            cmd_resp: None,
        }
    }
}

impl From<client::tmch::MarkSMDInfoResponse> for epp_proto::tmch::MarkSmdInfoResponse {
    fn from(res: client::tmch::MarkSMDInfoResponse) -> Self {
        epp_proto::tmch::MarkSmdInfoResponse {
            id: res.id,
            status: Some(res.status.into()),
            smd_id: res.smd_id,
            smd: res.smd,
            cmd_resp: None,
        }
    }
}

impl From<client::tmch::BalanceData> for epp_proto::tmch::BalanceData {
    fn from(res: client::tmch::BalanceData) -> Self {
        epp_proto::tmch::BalanceData {
            value: res.value,
            currency: res.currency,
            status_points: res.status_points,
        }
    }
}

impl From<epp_proto::tmch::Document> for client::tmch::Document {
    fn from(res: epp_proto::tmch::Document) -> Self {
        client::tmch::Document {
            class: match epp_proto::tmch::DocumentClass::try_from(res.document_class) {
                Ok(epp_proto::tmch::DocumentClass::Other) => client::tmch::DocumentClass::Other,
                Err(_) => client::tmch::DocumentClass::Other,
                Ok(epp_proto::tmch::DocumentClass::LicenseeDeclaration) => {
                    client::tmch::DocumentClass::LicenseeDeclaration
                }
                Ok(epp_proto::tmch::DocumentClass::AssigneeDeclaration) => {
                    client::tmch::DocumentClass::AssigneeDeclaration
                }
                Ok(epp_proto::tmch::DocumentClass::DeclarationProofOfUseOneSample) => {
                    client::tmch::DocumentClass::DeclarationProofOfUseOneSample
                }
                Ok(epp_proto::tmch::DocumentClass::OtherProofOfUse) => {
                    client::tmch::DocumentClass::OtherProofOfUse
                }
                Ok(epp_proto::tmch::DocumentClass::CopyOfCourtOrder) => {
                    client::tmch::DocumentClass::CopyOfCourtOrder
                }
            },
            file_name: res.file_name,
            file_type: match epp_proto::tmch::FileType::try_from(res.file_type) {
                Ok(epp_proto::tmch::FileType::Pdf) => client::tmch::FileType::Pdf,
                Err(_) => client::tmch::FileType::Pdf,
                Ok(epp_proto::tmch::FileType::Jpg) => client::tmch::FileType::Jpg,
            },
            contents: res.contents,
        }
    }
}

impl From<epp_proto::tmch::CaseDocument> for client::tmch::CaseDocument {
    fn from(res: epp_proto::tmch::CaseDocument) -> Self {
        client::tmch::CaseDocument {
            class: match epp_proto::tmch::CourtDocumentClass::try_from(res.document_class) {
                Ok(epp_proto::tmch::CourtDocumentClass::CourtOther) => {
                    client::tmch::CaseDocumentClass::Other
                }
                Err(_) => client::tmch::CaseDocumentClass::Other,
                Ok(epp_proto::tmch::CourtDocumentClass::CourtDecision) => {
                    client::tmch::CaseDocumentClass::CourtDecision
                }
            },
            file_name: res.file_name,
            file_type: match epp_proto::tmch::FileType::try_from(res.file_type) {
                Ok(epp_proto::tmch::FileType::Pdf) => client::tmch::FileType::Pdf,
                Err(_) => client::tmch::FileType::Pdf,
                Ok(epp_proto::tmch::FileType::Jpg) => client::tmch::FileType::Jpg,
            },
            contents: res.contents,
        }
    }
}

impl From<epp_proto::tmch::CreateLabel> for client::tmch::CreateLabel {
    fn from(res: epp_proto::tmch::CreateLabel) -> Self {
        client::tmch::CreateLabel {
            label: res.label,
            smd_inclusion: res.smd_inclusion,
            claims_notify: res.claims_notify,
        }
    }
}

impl From<client::tmch::CheckResponse> for epp_proto::tmch::MarkCheckResponse {
    fn from(res: client::tmch::CheckResponse) -> Self {
        epp_proto::tmch::MarkCheckResponse {
            available: res.avail,
            reason: res.reason,
            cmd_resp: None,
        }
    }
}

impl From<client::tmch::CreateResponse> for epp_proto::tmch::MarkCreateResponse {
    fn from(res: client::tmch::CreateResponse) -> Self {
        epp_proto::tmch::MarkCreateResponse {
            id: res.id,
            created_date: super::utils::chrono_to_proto(Some(res.created_date)),
            balance: Some(res.balance.into()),
            cmd_resp: None,
        }
    }
}

impl From<client::tmch::TransferInitiateResponse>
    for epp_proto::tmch::MarkTransferInitiateResponse
{
    fn from(res: client::tmch::TransferInitiateResponse) -> Self {
        epp_proto::tmch::MarkTransferInitiateResponse {
            id: res.id,
            auth_info: res.auth_info,
            cmd_resp: None,
        }
    }
}

impl From<client::tmch::TransferResponse> for epp_proto::tmch::MarkTransferResponse {
    fn from(res: client::tmch::TransferResponse) -> Self {
        epp_proto::tmch::MarkTransferResponse {
            id: res.id,
            transfer_date: super::utils::chrono_to_proto(res.transfer_date),
            balance: Some(res.balance.into()),
            cmd_resp: None,
        }
    }
}

impl From<client::tmch::RenewResponse> for epp_proto::tmch::MarkRenewResponse {
    fn from(res: client::tmch::RenewResponse) -> Self {
        epp_proto::tmch::MarkRenewResponse {
            id: res.id,
            new_expiry_date: super::utils::chrono_to_proto(res.new_expiry_date),
            balance: Some(res.balance.into()),
            cmd_resp: None,
        }
    }
}

impl From<client::tmch::UpdateResponse> for epp_proto::tmch::MarkUpdateResponse {
    fn from(_res: client::tmch::UpdateResponse) -> Self {
        epp_proto::tmch::MarkUpdateResponse { cmd_resp: None }
    }
}

impl TryFrom<epp_proto::tmch::AddCase> for client::tmch::AddCase {
    type Error = tonic::Status;

    fn try_from(from: epp_proto::tmch::AddCase) -> Result<Self, Self::Error> {
        Ok(client::tmch::AddCase {
            id: from.id,
            case: match from.case {
                None => return Err(tonic::Status::invalid_argument("case must be specified")),
                Some(epp_proto::tmch::add_case::Case::Udrp(u)) => client::tmch::CaseType::Udrp {
                    case_id: u.case_id,
                    provider: u.provider,
                    case_language: u.case_language,
                },
                Some(epp_proto::tmch::add_case::Case::Court(c)) => client::tmch::CaseType::Court {
                    decision_id: c.decision_id,
                    court_name: c.court_name,
                    country_code: c.country_code,
                    regions: c.regions,
                    case_language: c.case_language,
                },
            },
            documents: from.documents.into_iter().map(Into::into).collect(),
            labels: from.labels.into_iter().map(Into::into).collect(),
        })
    }
}

impl From<epp_proto::tmch::CaseUpdate> for client::tmch::CaseUpdate {
    fn from(from: epp_proto::tmch::CaseUpdate) -> Self {
        client::tmch::CaseUpdate {
            id: from.id,
            add: from
                .add
                .into_iter()
                .filter_map(|a| a.update)
                .map(|u| match u {
                    epp_proto::tmch::case_add::Update::Label(l) => client::tmch::CaseAdd::Label(l),
                    epp_proto::tmch::case_add::Update::Document(d) => {
                        client::tmch::CaseAdd::Document(d.into())
                    }
                })
                .collect(),
            remove: from
                .remove
                .into_iter()
                .filter_map(|a| a.update)
                .map(|u| match u {
                    epp_proto::tmch::case_remove::Update::Label(l) => {
                        client::tmch::CaseRemove::Label(l)
                    }
                })
                .collect(),
            new_case: from.new_case.map(|c| match c {
                epp_proto::tmch::case_update::NewCase::NewUdrp(u) => client::tmch::CaseType::Udrp {
                    case_id: u.case_id,
                    provider: u.provider,
                    case_language: u.case_language,
                },
                epp_proto::tmch::case_update::NewCase::NewCourt(c) => {
                    client::tmch::CaseType::Court {
                        decision_id: c.decision_id,
                        court_name: c.court_name,
                        country_code: c.country_code,
                        regions: c.regions,
                        case_language: c.case_language,
                    }
                }
            }),
        }
    }
}
