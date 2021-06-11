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
    pub phase: LaunchPhase,
}

#[derive(Debug)]
pub struct LaunchAvailabilityCheck {
    pub phase: LaunchPhase,
}

#[derive(Debug)]
pub struct LaunchTrademarkCheck {}

#[derive(Debug)]
pub struct LaunchClaimKey {
    pub validator_id: Option<String>,
    pub key: String,
}

#[derive(Debug)]
pub struct LaunchInfo {
    pub include_mark: bool,
    pub phase: LaunchPhase,
    pub application_id: Option<String>,
}

#[derive(Debug)]
pub struct LaunchInfoData {
    pub phase: LaunchPhase,
    pub application_id: Option<String>,
    pub status: Option<LaunchStatus>,
    pub mark: Option<String>,
}

#[derive(Debug)]
pub struct CoreNICApplicationInfo {
    pub info_type: Option<String>,
    pub info: String,
}

#[derive(Debug)]
pub struct LaunchCreate {
    pub phase: LaunchPhase,
    pub code_mark: Vec<CodeMark>,
    pub signed_mark: Option<String>,
    pub create_type: LaunchCreateType,
    pub notices: Vec<Notice>,
    pub core_nic: Vec<CoreNICApplicationInfo>,
}

#[derive(Debug)]
pub enum LaunchCreateType {
    Application,
    Registration,
}

#[derive(Debug)]
pub struct LaunchUpdate {
    pub phase: LaunchPhase,
    pub application_id: Option<String>,
}

#[derive(Debug)]
pub struct LaunchCreateData {
    pub phase: LaunchPhase,
    pub application_id: Option<String>,
}

#[derive(Debug)]
pub struct CodeMark {
    pub code: Option<String>,
    pub validator: Option<String>,
    pub mark: Option<String>,
}

#[derive(Debug)]
pub struct Notice {
    pub notice_id: String,
    pub validator: Option<String>,
    pub not_after: DateTime<Utc>,
    pub accepted_date: DateTime<Utc>,
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

#[derive(Debug)]
pub struct LaunchStatus {
    pub status_type: LaunchStatusType,
    pub status_name: Option<String>,
    pub message: Option<String>,
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
