use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TMMessage {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmNotice-1.0}notice")]
    pub notice: TMNotice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TMNotice {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmNotice-1.0}id")]
    pub id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmNotice-1.0}notBefore")]
    pub not_before: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmNotice-1.0}notAfter")]
    pub not_after: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmNotice-1.0}label")]
    pub label: String
}