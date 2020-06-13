use chrono::prelude::*;

#[derive(Debug, Deserialize)]
pub struct EPPRGPData {
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgpStatus", default)]
    pub state: Vec<EPPRGPStatus>,
}

#[derive(Debug, Deserialize)]
pub struct EPPRGPStatus {
    #[serde(rename = "$attr:s")]
    pub state: EPPRGPState,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EPPRGPState {
    #[serde(rename = "addPeriod")]
    AddPeriod,
    #[serde(rename = "autoRenewPeriod")]
    AutoRenewPeriod,
    #[serde(rename = "renewPeriod")]
    RenewPeriod,
    #[serde(rename = "transferPeriod")]
    TransferPeriod,
    #[serde(rename = "redemptionPeriod")]
    RedemptionPeriod,
    #[serde(rename = "pendingRestore")]
    PendingRestore,
    #[serde(rename = "pendingDelete")]
    PendingDelete,
}

#[derive(Debug, Serialize)]
pub struct EPPRGPUpdate {
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgp:restore")]
    pub restore: EPPRGPRestore,
}

#[derive(Debug, Serialize)]
pub struct EPPRGPRestore {
    #[serde(rename = "$attr:op")]
    pub operation: EPPRGPRestoreOperation,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgp:report",
        skip_serializing_if = "Option::is_none"
    )]
    pub report: Option<EPPRGPReport>,
}

#[derive(Debug, Serialize)]
pub enum EPPRGPRestoreOperation {
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "report")]
    #[allow(dead_code)]
    Report,
}

#[derive(Debug, Serialize)]
pub struct EPPRGPReport {
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgp:preData")]
    pub pre_data: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgp:postData")]
    pub post_data: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgp:delTime")]
    pub delete_time: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgp:resTime")]
    pub restore_time: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgp:resReason")]
    pub restore_reason: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgp:statement")]
    pub statement: Vec<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgp:other",
        skip_serializing_if = "Option::is_none"
    )]
    pub other: Option<String>,
}
