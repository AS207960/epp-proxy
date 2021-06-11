use super::super::launch::{
    CodeMark, LaunchAvailabilityCheck, LaunchClaimKey, LaunchClaimsCheck, LaunchCreate,
    LaunchCreateData, LaunchCreateType, LaunchInfo, LaunchInfoData, LaunchPhase, LaunchStatus,
    LaunchStatusType, LaunchTrademarkCheck, LaunchUpdate, Notice, PhaseType,
};
use super::proto;

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

impl From<&proto::launch::EPPLaunchClaimKey> for LaunchClaimKey {
    fn from(from: &proto::launch::EPPLaunchClaimKey) -> Self {
        LaunchClaimKey {
            validator_id: from.validator_id.as_ref().map(String::from),
            key: from.key.to_string(),
        }
    }
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

impl From<&LaunchCreate> for proto::launch::EPPLaunchCreate {
    fn from(from: &LaunchCreate) -> Self {
        if from.core_nic.is_empty() {
            proto::launch::EPPLaunchCreate {
                create_type: Some(match from.create_type {
                    LaunchCreateType::Application => {
                        proto::launch::EPPLaunchCreateType::Application
                    }
                    LaunchCreateType::Registration => {
                        proto::launch::EPPLaunchCreateType::Registration
                    }
                }),
                phase: (&from.phase).into(),
                signed_mark: from.signed_mark.as_ref().map(Into::into),
                code_marks: from.code_mark.iter().map(Into::into).collect(),
                notices: from.notices.iter().map(Into::into).collect(),
                augmented_mark: None,
            }
        } else {
            proto::launch::EPPLaunchCreate {
                create_type: Some(match from.create_type {
                    LaunchCreateType::Application => {
                        proto::launch::EPPLaunchCreateType::Application
                    }
                    LaunchCreateType::Registration => {
                        proto::launch::EPPLaunchCreateType::Registration
                    }
                }),
                phase: (&from.phase).into(),
                signed_mark: None,
                code_marks: from.code_mark.iter().map(Into::into).collect(),
                notices: from.notices.iter().map(Into::into).collect(),
                augmented_mark: Some(proto::corenic::EPPAugmentedMark {
                    signed_mark: from.signed_mark.as_ref().map(Into::into),
                    application_info: from
                        .core_nic
                        .iter()
                        .map(|i| proto::corenic::EPPApplicationInfo {
                            info_type: i.info_type.as_ref().map(Into::into),
                            info: i.info.to_string(),
                        })
                        .collect(),
                }),
            }
        }
    }
}

impl From<&LaunchUpdate> for proto::launch::EPPLaunchInfo {
    fn from(from: &LaunchUpdate) -> Self {
        proto::launch::EPPLaunchInfo {
            include_mark: None,
            phase: (&from.phase).into(),
            application_id: from.application_id.as_ref().map(Into::into),
        }
    }
}

impl From<&proto::launch::EPPLaunchCreateData> for LaunchCreateData {
    fn from(from: &proto::launch::EPPLaunchCreateData) -> Self {
        LaunchCreateData {
            phase: (&from.phase).into(),
            application_id: from.application_id.as_ref().map(Into::into),
        }
    }
}

impl From<&CodeMark> for proto::launch::EPPLaunchCodeMark {
    fn from(from: &CodeMark) -> Self {
        proto::launch::EPPLaunchCodeMark {
            code: from.code.as_ref().map(|c| proto::launch::EPPLaunchCode {
                code: c.into(),
                validator_id: from.validator.as_ref().map(Into::into),
            }),
            mark: from.mark.as_ref().map(Into::into),
        }
    }
}

impl From<&Notice> for proto::launch::EPPLaunchNotice {
    fn from(from: &Notice) -> Self {
        proto::launch::EPPLaunchNotice {
            notice_id: proto::launch::EPPLaunchNoticeID {
                code: (&from.notice_id).into(),
                validator_id: from.validator.as_ref().map(Into::into),
            },
            not_after: from.not_after,
            accepted_date: from.accepted_date,
        }
    }
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

impl From<&proto::launch::EPPLaunchStatus> for LaunchStatus {
    fn from(from: &proto::launch::EPPLaunchStatus) -> Self {
        LaunchStatus {
            status_type: (&from.status).into(),
            status_name: from.name.as_ref().map(Into::into),
            message: from.message.as_ref().map(Into::into),
        }
    }
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
