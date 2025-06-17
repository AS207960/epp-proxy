#[derive(Debug, Serialize)]
pub struct EPPTTLInfoRequest {
    #[serde(rename = "$attr:policy")]
    pub policy: bool
}

#[derive(Debug, Serialize)]
pub struct EPPTTLSet {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:ttl-1.0}ttl:ttl")]
    pub ttl: Vec<EPPTTLCommand>
}

#[derive(Debug, Deserialize)]
pub struct EPPTTLInfoData {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:ttl-1.0}ttl:ttl")]
    pub ttl: Vec<EPPTTLResponse>
}

#[derive(Debug, Serialize)]
pub struct EPPTTLCommand {
    #[serde(rename = "$value", default, skip_serializing_if="Option::is_none")]
    pub ttl: Option<u32>,
    #[serde(rename = "$attr:for")]
    pub for_rr: EPPTTLRR,
    #[serde(rename = "$attr:custom", default, skip_serializing_if="Option::is_none")]
    pub custom_rr: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct EPPTTLResponse {
    #[serde(rename = "$value", default)]
    pub ttl: Option<u32>,
    #[serde(rename = "$attr:for")]
    pub for_rr: EPPTTLRR,
    #[serde(rename = "$attr:custom", default)]
    pub custom_rr: String,
    #[serde(rename = "$attr:min", default)]
    pub min: Option<u32>,
    #[serde(rename = "$attr:default", default)]
    pub default: Option<u32>,
    #[serde(rename = "$attr:max", default)]
    pub max: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EPPTTLRR {
    NS,
    DS,
    DNAME,
    A,
    AAAA,
    #[serde(rename = "custom")]
    Custom
}