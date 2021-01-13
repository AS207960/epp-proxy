use super::proto;
use chrono::prelude::*;

// pub enum LaunchCheck {
//     Claims(LaunchTrademarkCheck),
//     Availability(LaunchAvailabilityCheck),
//     Trademark(LaunchTrademarkCheck)
// }
//
// impl From<&LaunchCheck> for proto::launch::EPPLaunchCheck {
//     fn from(from: &LaunchCheck) -> Self {
//         from.into()
//     }
// }

#[derive(Debug)]
pub struct LaunchClaimsCheck {
    pub phase: LaunchPhase
}

#[derive(Debug)]
pub struct LaunchAvailabilityCheck {
    pub phase: LaunchPhase
}

#[derive(Debug)]
pub struct LaunchTrademarkCheck {}

impl From<&LaunchClaimsCheck> for proto::launch::EPPLaunchCheck {
    fn from(from: &LaunchClaimsCheck) -> Self {
        proto::launch::EPPLaunchCheck {
            check_type: Some(proto::launch::EPPLaunchCheckType::Claims),
            phase: Some((&from.phase).into()),
        }
    }
}

impl From<&LaunchAvailabilityCheck> for proto::launch::EPPLaunchCheck {
    fn from(from: &LaunchAvailabilityCheck) -> Self {
        proto::launch::EPPLaunchCheck {
            check_type: Some(proto::launch::EPPLaunchCheckType::Availability),
            phase: Some((&from.phase).into()),
        }
    }
}

impl From<&LaunchTrademarkCheck> for proto::launch::EPPLaunchCheck {
    fn from(_from: &LaunchTrademarkCheck) -> Self {
        proto::launch::EPPLaunchCheck {
            check_type: Some(proto::launch::EPPLaunchCheckType::Trademark),
            phase: None,
        }
    }
}

#[derive(Debug)]
pub struct LaunchClaimKey {
    pub validator_id: Option<String>,
    pub key: String,
}

impl From<&proto::launch::EPPLaunchClaimKey> for LaunchClaimKey {
    fn from(from: &proto::launch::EPPLaunchClaimKey) -> Self {
        LaunchClaimKey {
            validator_id: from.validator_id.as_ref().map(String::from),
            key: from.key.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct LaunchInfo {
    pub include_mark: bool,
    pub phase: LaunchPhase,
    pub application_id: Option<String>,
}

impl From<&LaunchInfo> for proto::launch::EPPLaunchInfo {
    fn from(from: &LaunchInfo) -> Self {
        proto::launch::EPPLaunchInfo {
            include_mark: Some(from.include_mark),
            phase: (&from.phase).into(),
            application_id: from.application_id.as_ref().map(Into::into),
        }
    }
}

#[derive(Debug)]
pub struct LaunchInfoData {
    pub phase: LaunchPhase,
    pub application_id: Option<String>,
    pub status: Option<LaunchStatus>,
    pub mark: Option<String>,
}

impl From<&proto::launch::EPPLaunchInfoData> for LaunchInfoData {
    fn from(from: &proto::launch::EPPLaunchInfoData) -> Self {
        LaunchInfoData {
            phase: (&from.phase).into(),
            application_id: from.application_id.as_ref().map(Into::into),
            status: from.status.as_ref().map(Into::into),
            mark: from.mark.as_ref().map(Into::into),
        }
    }
}

#[derive(Debug)]
pub struct LaunchCreate {
    pub phase: LaunchPhase,
    pub code_mark: Vec<CodeMark>,
    pub signed_mark: Option<String>,
    pub create_type: LaunchCreateType,
}

impl From<&LaunchCreate> for proto::launch::EPPLaunchCreate {
    fn from(from: &LaunchCreate) -> Self {
        proto::launch::EPPLaunchCreate {
            create_type: Some(match from.create_type {
                LaunchCreateType::Application => proto::launch::EPPLaunchCreateType::Application,
                LaunchCreateType::Registration => proto::launch::EPPLaunchCreateType::Registration,
            }),
            phase: (&from.phase).into(),
            signed_mark: from.signed_mark.as_ref().map(Into::into),
            code_marks: from.code_mark.iter().map(Into::into).collect(),
            notices: vec![]
        }
    }
}

#[derive(Debug)]
pub enum LaunchCreateType {
    Application,
    Registration
}


#[derive(Debug)]
pub struct LaunchUpdate {
    pub phase: LaunchPhase,
    pub application_id: Option<String>,
}

impl From<&LaunchUpdate> for proto::launch::EPPLaunchInfo {
    fn from(from: &LaunchUpdate) -> Self {
        proto::launch::EPPLaunchInfo {
            include_mark: None,
            phase: (&from.phase).into(),
            application_id: from.application_id.as_ref().map(Into::into)
        }
    }
}

#[derive(Debug)]
pub struct LaunchCreateData {
    pub phase: LaunchPhase,
    pub application_id: Option<String>
}

impl From<&proto::launch::EPPLaunchCreateData> for LaunchCreateData {
    fn from(from: &proto::launch::EPPLaunchCreateData) -> Self {
        LaunchCreateData {
            phase: (&from.phase).into(),
            application_id: from.application_id.as_ref().map(Into::into)
        }
    }
}

#[derive(Debug)]
pub struct CodeMark {
    pub code: Option<String>,
    pub validator: Option<String>,
    pub mark: Option<String>
}

impl From<&CodeMark> for proto::launch::EPPLaunchCodeMark {
    fn from(from: &CodeMark) -> Self {
        proto::launch::EPPLaunchCodeMark {
            code: from.code.as_ref().map(|c| proto::launch::EPPLaunchCode {
                code: c.into(),
                validator_id: from.validator.as_ref().map(Into::into)
            }),
            mark: from.mark.as_ref().map(Into::into)
        }
    }
}

#[derive(Debug)]
pub struct Notice {
    pub notice_id: String,
    pub validator: Option<String>,
    pub not_after: DateTime<Utc>,
    pub accepted_date: DateTime<Utc>,
}

impl From<&Notice> for proto::launch::EPPLaunchNotice {
    fn from(from: &Notice) -> Self {
        proto::launch::EPPLaunchNotice {
            notice_id: proto::launch::EPPLaunchNoticeID {
                code: (&from.notice_id).into(),
                validator_id: from.validator.as_ref().map(Into::into)
            },
            not_after: from.not_after,
            accepted_date: from.accepted_date,
        }
    }
}

#[derive(Debug)]
pub struct LaunchPhase {
    pub phase_type: PhaseType,
    pub phase_name: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum PhaseType {
    Sunrise,
    Landrush,
    Claims,
    Open,
    Custom,
}

impl From<&LaunchPhase> for proto::launch::EPPLaunchPhase {
    fn from(from: &LaunchPhase) -> Self {
        proto::launch::EPPLaunchPhase {
            phase: match from.phase_type {
                PhaseType::Sunrise => proto::launch::EPPLaunchPhaseType::Sunrise,
                PhaseType::Landrush => proto::launch::EPPLaunchPhaseType::Landrush,
                PhaseType::Claims => proto::launch::EPPLaunchPhaseType::Claims,
                PhaseType::Open => proto::launch::EPPLaunchPhaseType::Open,
                PhaseType::Custom => proto::launch::EPPLaunchPhaseType::Custom,
            },
            name: from.phase_name.as_ref().map(Into::into),
        }
    }
}

impl From<&proto::launch::EPPLaunchPhase> for LaunchPhase {
    fn from(from: &proto::launch::EPPLaunchPhase) -> Self {
        Self {
            phase_type: match from.phase {
                proto::launch::EPPLaunchPhaseType::Sunrise => PhaseType::Sunrise,
                proto::launch::EPPLaunchPhaseType::Landrush => PhaseType::Landrush,
                proto::launch::EPPLaunchPhaseType::Claims => PhaseType::Claims,
                proto::launch::EPPLaunchPhaseType::Open => PhaseType::Open,
                proto::launch::EPPLaunchPhaseType::Custom => PhaseType::Custom,
            },
            phase_name: from.name.as_ref().map(Into::into),
        }
    }
}

#[derive(Debug)]
pub struct LaunchStatus {
    pub status_type: LaunchStatusType,
    pub status_name: Option<String>,
    pub message: Option<String>,
}

impl From<&proto::launch::EPPLaunchStatus> for LaunchStatus {
    fn from(from: &proto::launch::EPPLaunchStatus) -> Self {
        LaunchStatus {
            status_type: (&from.status).into(),
            status_name: from.name.as_ref().map(Into::into),
            message: from.message.as_ref().map(Into::into),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LaunchStatusType {
    PendingValidation,
    Validated,
    Invalid,
    PendingAllocation,
    Allocated,
    Rejected,
    Custom,
}

impl From<&proto::launch::EPPLaunchStatusType> for LaunchStatusType {
    fn from(from: &proto::launch::EPPLaunchStatusType) -> Self {
        match from {
            proto::launch::EPPLaunchStatusType::PendingValidation => Self::PendingValidation,
            proto::launch::EPPLaunchStatusType::Validated => Self::Validated,
            proto::launch::EPPLaunchStatusType::Invalid => Self::Invalid,
            proto::launch::EPPLaunchStatusType::PendingAllocation => Self::PendingAllocation,
            proto::launch::EPPLaunchStatusType::Allocated => Self::Allocated,
            proto::launch::EPPLaunchStatusType::Rejected => Self::Rejected,
            proto::launch::EPPLaunchStatusType::Custom => Self::Custom,
        }
    }
}