use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EPPLaunchCheck {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:phase",
        skip_serializing_if = "Option::is_none"
    )]
    pub phase: Option<EPPLaunchPhase>,
    #[serde(
        rename = "$attr:type",
        skip_serializing_if = "Option::is_none"
    )]
    pub check_type: Option<EPPLaunchCheckType>
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EPPLaunchCheckType {
    #[serde(rename = "claims")]
    Claims,
    #[serde(rename = "avail")]
    Availability,
    #[serde(rename = "trademark")]
    Trademark
}

#[derive(Debug, Deserialize)]
pub struct EPPLaunchCheckData {
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:phase", default)]
    pub phase: Option<EPPLaunchPhase>,
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}cd", default)]
    pub data: Vec<EPPLaunchCheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPLaunchCheckDatum {
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}name")]
    pub name: EPPLaunchCheckName,
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}claimKey")]
    pub claim_key: Vec<EPPLaunchClaimKey>,
}

#[derive(Debug, Deserialize)]
pub struct EPPLaunchCheckName {
    #[serde(rename = "$value")]
    pub name: String,
    #[serde(rename = "$attr:exists")]
    pub exists: bool,
}

#[derive(Debug, Deserialize)]
pub struct EPPLaunchClaimKey {
    #[serde(rename = "$value")]
    pub key: String,
    #[serde(rename = "$attr:validatorID", default)]
    pub validator_id: Option<String>
}

#[derive(Debug, Serialize)]
pub struct EPPLaunchInfo {
    #[serde(
        rename = "$attr:includeMark",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_mark: Option<bool>,
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:phase")]
    pub phase: EPPLaunchPhase,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:applicationID",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_id: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct EPPLaunchInfoData {
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}phase")]
    pub phase: EPPLaunchPhase,
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}applicationID", default)]
    pub application_id: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}status", default)]
    pub status: Option<EPPLaunchStatus>,
    #[serde(rename = "$value", default)]
    pub mark: Option<String>,
    // #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}mark", default)]
    // pub mark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EPPLaunchCreate {
    #[serde(
        rename = "$attr:type",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_type: Option<EPPLaunchCreateType>,
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:phase")]
    pub phase: EPPLaunchPhase,
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:codeMark")]
    pub code_marks: Vec<EPPLaunchCodeMark>,
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:notice")]
    pub notices: Vec<EPPLaunchNotice>,
    #[serde(
        rename = "$valueRaw",
        skip_serializing_if = "Option::is_none"
    )]
    pub signed_mark: Option<String>,
    #[serde(
        rename = "{http://xmlns.corenic.net/epp/mark-ext-1.0}ext:augmentedMark",
        skip_serializing_if = "Option::is_none"
    )]
    pub augmented_mark: Option<super::corenic::EPPAugmentedMark>,
    // #[serde(rename = "{urn:ietf:params:xml:ns:signedMark-1.0}signedMark")]
    // pub signed_mark: Vec<String>,
    // #[serde(rename = "{urn:ietf:params:xml:ns:signedMark-1.0}encodedSignedMark")]
    // pub encoded_signed_mark: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPLaunchCreateData {
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}phase")]
    pub phase: EPPLaunchPhase,
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}applicationID", default)]
    pub application_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum EPPLaunchCreateType {
    #[serde(rename = "application")]
    Application,
    #[serde(rename = "registration")]
    Registration,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPLaunchPhase {
    #[serde(rename = "$value")]
    pub phase: EPPLaunchPhaseType,
    #[serde(
        rename = "$attr:name",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EPPLaunchPhaseType {
    #[serde(rename = "sunrise")]
    Sunrise,
    #[serde(rename = "landrush")]
    Landrush,
    #[serde(rename = "claims")]
    Claims,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "custom")]
    Custom,
}

#[derive(Debug, Deserialize)]
pub struct EPPLaunchStatus {
    #[serde(rename = "$attr:s")]
    pub status: EPPLaunchStatusType,
    #[serde(rename = "$attr:name", default)]
    pub name: Option<String>,
    #[serde(rename = "$value")]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum EPPLaunchStatusType {
    #[serde(rename = "pendingValidation")]
    PendingValidation,
    #[serde(rename = "validated")]
    Validated,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "pendingAllocation")]
    PendingAllocation,
    #[serde(rename = "allocated")]
    Allocated,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "custom")]
    Custom
}

#[derive(Debug, Serialize)]
pub struct EPPLaunchCodeMark {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:code",
        skip_serializing_if = "Option::is_none"
    )]
    pub code: Option<EPPLaunchCode>,
    #[serde(
        rename = "$valueRaw",
        skip_serializing_if = "Option::is_none"
    )]
    pub mark: Option<String>,
    // #[serde(
    //     rename = "{urn:ietf:params:xml:ns:mark-1.0}mark",
    //     skip_serializing_if = "Option::is_none"
    // )]
    // pub mark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EPPLaunchCode {
    #[serde(rename = "$value")]
    pub code: String,
    #[serde(
        rename = "$attr:validatorID",
        skip_serializing_if = "Option::is_none"
    )]
    pub validator_id: Option<String>
}

#[derive(Debug, Serialize)]
pub struct EPPLaunchNotice {
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:noticeID")]
    pub notice_id: EPPLaunchNoticeID,
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:notAfter")]
    pub not_after: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:acceptedDate")]
    pub accepted_date: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct EPPLaunchNoticeID {
    #[serde(rename = "$value")]
    pub code: String,
    #[serde(
        rename = "$attr:validatorID",
        skip_serializing_if = "Option::is_none"
    )]
    pub validator_id: Option<String>
}
