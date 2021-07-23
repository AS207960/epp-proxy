use super::super::client;
use super::epp_proto;
use std::convert::TryFrom;

impl From<epp_proto::launch::Phase> for client::launch::LaunchPhase {
    fn from(from: epp_proto::launch::Phase) -> Self {
        client::launch::LaunchPhase {
            phase_type: match epp_proto::launch::phase::PhaseType::from_i32(from.phase_type) {
                Some(p) => match p {
                    epp_proto::launch::phase::PhaseType::Open => client::launch::PhaseType::Open,
                    epp_proto::launch::phase::PhaseType::Sunrise => {
                        client::launch::PhaseType::Sunrise
                    }
                    epp_proto::launch::phase::PhaseType::Landrush => {
                        client::launch::PhaseType::Landrush
                    }
                    epp_proto::launch::phase::PhaseType::Claims => {
                        client::launch::PhaseType::Claims
                    }
                    epp_proto::launch::phase::PhaseType::Custom => {
                        client::launch::PhaseType::Custom
                    }
                },
                None => client::launch::PhaseType::Custom,
            },
            phase_name: from.phase_name,
        }
    }
}

impl From<client::launch::LaunchPhase> for epp_proto::launch::Phase {
    fn from(from: client::launch::LaunchPhase) -> Self {
        epp_proto::launch::Phase {
            phase_type: match from.phase_type {
                client::launch::PhaseType::Open => epp_proto::launch::phase::PhaseType::Open.into(),
                client::launch::PhaseType::Sunrise => {
                    epp_proto::launch::phase::PhaseType::Sunrise.into()
                }
                client::launch::PhaseType::Landrush => {
                    epp_proto::launch::phase::PhaseType::Landrush.into()
                }
                client::launch::PhaseType::Claims => {
                    epp_proto::launch::phase::PhaseType::Claims.into()
                }
                client::launch::PhaseType::Custom => {
                    epp_proto::launch::phase::PhaseType::Custom.into()
                }
            },
            phase_name: from.phase_name,
        }
    }
}

impl From<epp_proto::launch::Phase> for client::launch::LaunchClaimsCheck {
    fn from(from: epp_proto::launch::Phase) -> Self {
        client::launch::LaunchClaimsCheck { phase: from.into() }
    }
}

impl From<client::launch::LaunchClaimKey> for epp_proto::launch::ClaimsKey {
    fn from(from: client::launch::LaunchClaimKey) -> Self {
        epp_proto::launch::ClaimsKey {
            key: from.key,
            validator_id: from.validator_id,
        }
    }
}

impl TryFrom<epp_proto::launch::LaunchInfo> for client::launch::LaunchInfo {
    type Error = tonic::Status;

    fn try_from(from: epp_proto::launch::LaunchInfo) -> Result<Self, Self::Error> {
        Ok(client::launch::LaunchInfo {
            include_mark: from.include_mark,
            phase: match from.phase {
                Some(p) => p.into(),
                None => {
                    return Err(tonic::Status::invalid_argument(
                        "Launch phase must be specified",
                    ))
                }
            },
            application_id: from.application_id,
        })
    }
}

impl TryFrom<epp_proto::launch::LaunchCreate> for client::launch::LaunchCreate {
    type Error = tonic::Status;

    fn try_from(from: epp_proto::launch::LaunchCreate) -> Result<Self, Self::Error> {
        Ok(client::launch::LaunchCreate {
            create_type: match epp_proto::launch::launch_create::CreateType::from_i32(
                from.create_type,
            ) {
                Some(epp_proto::launch::launch_create::CreateType::Registration) => {
                    client::launch::LaunchCreateType::Registration
                }
                Some(epp_proto::launch::launch_create::CreateType::Application) => {
                    client::launch::LaunchCreateType::Application
                }
                None => client::launch::LaunchCreateType::Registration,
            },
            phase: match from.phase {
                Some(p) => p.into(),
                None => {
                    return Err(tonic::Status::invalid_argument(
                        "Launch phase must be specified",
                    ))
                }
            },
            code_mark: from
                .code_mark
                .into_iter()
                .map(|m| client::launch::CodeMark {
                    code: m.code,
                    validator: m.validator,
                    mark: m.mark,
                })
                .collect(),
            signed_mark: from.signed_mark,
            notices: from
                .notices
                .into_iter()
                .map(|n| {
                    Ok(client::launch::Notice {
                        notice_id: n.notice_id,
                        validator: n.validator,
                        not_after: match super::utils::proto_to_chrono(n.not_after) {
                            Some(d) => d,
                            None => {
                                return Err(tonic::Status::invalid_argument(
                                    "Date must be specified",
                                ))
                            }
                        },
                        accepted_date: match super::utils::proto_to_chrono(n.accepted_after) {
                            Some(d) => d,
                            None => {
                                return Err(tonic::Status::invalid_argument(
                                    "Date must be specified",
                                ))
                            }
                        },
                    })
                })
                .collect::<Result<Vec<_>, _>>()?,
            core_nic: from
                .core_nic_augmented_mark
                .into_iter()
                .map(|m| client::launch::CoreNICApplicationInfo {
                    info_type: m.info_type,
                    info: m.info,
                })
                .collect(),
        })
    }
}

impl TryFrom<epp_proto::launch::LaunchData> for client::launch::LaunchUpdate {
    type Error = tonic::Status;

    fn try_from(from: epp_proto::launch::LaunchData) -> Result<Self, Self::Error> {
        Ok(client::launch::LaunchUpdate {
            phase: match from.phase {
                Some(p) => p.into(),
                None => {
                    return Err(tonic::Status::invalid_argument(
                        "Launch phase must be specified",
                    ))
                }
            },
            application_id: from.application_id,
        })
    }
}

impl From<client::launch::LaunchInfoData> for epp_proto::launch::LaunchInfoData {
    fn from(from: client::launch::LaunchInfoData) -> Self {
        epp_proto::launch::LaunchInfoData {
            phase: Some(from.phase.into()),
            application_id: from.application_id,
            status: from.status.map(|s| epp_proto::launch::Status {
                status_type: match s.status_type {
                    client::launch::LaunchStatusType::PendingValidation => {
                        epp_proto::launch::StatusType::PendingValidation.into()
                    }
                    client::launch::LaunchStatusType::Validated => {
                        epp_proto::launch::StatusType::Validated.into()
                    }
                    client::launch::LaunchStatusType::Invalid => {
                        epp_proto::launch::StatusType::Invalid.into()
                    }
                    client::launch::LaunchStatusType::PendingAllocation => {
                        epp_proto::launch::StatusType::PendingAllocation.into()
                    }
                    client::launch::LaunchStatusType::Allocated => {
                        epp_proto::launch::StatusType::Allocated.into()
                    }
                    client::launch::LaunchStatusType::Rejected => {
                        epp_proto::launch::StatusType::Rejected.into()
                    }
                    client::launch::LaunchStatusType::Custom => {
                        epp_proto::launch::StatusType::Custom.into()
                    }
                },
                status_name: s.status_name,
                message: s.message,
            }),
            mark: from.mark,
        }
    }
}

impl From<client::launch::LaunchCreateData> for epp_proto::launch::LaunchData {
    fn from(from: client::launch::LaunchCreateData) -> Self {
        epp_proto::launch::LaunchData {
            phase: Some(from.phase.into()),
            application_id: from.application_id,
        }
    }
}
