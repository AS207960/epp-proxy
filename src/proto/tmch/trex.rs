use chrono::prelude::*;

#[derive(Debug, Deserialize)]
pub struct TrexInfo {
    #[serde(rename = "$attr:enable")]
    pub enable: bool,
    #[serde(rename = "$attr:until", deserialize_with = "super::super::deserialize_datetime_opt", default)]
    pub until: Option<DateTime<Utc>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}tld", default)]
    pub tlds: Vec<TLDInfo>,
}

#[derive(Debug, Deserialize)]
pub struct TLDInfo {
    #[serde(rename = "$value")]
    pub tld: String,
    #[serde(rename = "$attr:s")]
    pub status: TLDStatus,
    #[serde(rename = "$attr:comment", default)]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum TLDStatus {
    #[serde(rename = "notprotected:override")]
    NotProtectedOverride,
    #[serde(rename = "notprotected:registered")]
    NotProtectedRegistered,
    #[serde(rename = "notprotected:exempt")]
    NotProtectedExempt,
    #[serde(rename = "notprotected:other")]
    NotProtectedOther,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "eligible")]
    Eligible,
    #[serde(rename = "noinfo")]
    NoInfo,
}

#[derive(Debug, Serialize)]
pub struct Activate {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<super::TMCHPeriod>,
}

#[derive(Debug, Serialize)]
pub struct Renew {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}curExpDate",
        serialize_with = "super::super::serialize_date"
    )]
    pub current_expiry_date: Date<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<super::TMCHPeriod>,
}