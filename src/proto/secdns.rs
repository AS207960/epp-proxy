#[derive(Debug, Deserialize, Serialize)]
pub struct EPPSecDNSData {
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:maxSigLife", skip_serializing_if = "Option::is_none")]
    pub max_signature_life: Option<i64>,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:dsData", default)]
    pub ds_data: Vec<EPPSecDNSDSData>,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:keyData", default)]
    pub key_data: Vec<EPPSecDNSKeyData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPSecDNSDSData {
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:keyTag")]
    pub key_tag: u16,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:alg")]
    pub algorithm: u8,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:digestType")]
    pub digest_type: u8,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:digest")]
    pub digest: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:keyData", skip_serializing_if = "Option::is_none", default)]
    pub key_data: Option<EPPSecDNSKeyData>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPSecDNSKeyData {
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:flags")]
    pub flags: u16,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:protocol")]
    pub protocol: u8,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:alg")]
    pub algorithm: u8,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:pubKey")]
    pub public_key: String
}

#[derive(Debug, Serialize)]
pub struct EPPSecDNSUpdate {
    #[serde(rename = "$attr:urgent", skip_serializing_if = "Option::is_none", serialize_with = "super::serialize_opt_bool")]
    pub urgent: Option<bool>,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:rem", skip_serializing_if = "Option::is_none")]
    pub remove: Option<EPPSecDNSUpdateRemove>,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:add", skip_serializing_if = "Option::is_none")]
    pub add: Option<EPPSecDNSUpdateAdd>,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:chg", skip_serializing_if = "Option::is_none")]
    pub change: Option<EPPSecDNSUpdateChange>,
}

#[derive(Debug, Serialize)]
pub struct EPPSecDNSUpdateAdd {
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:dsData")]
    pub ds_data: Vec<EPPSecDNSDSData>,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:keyData")]
    pub key_data: Vec<EPPSecDNSKeyData>,
}

#[derive(Debug, Serialize)]
pub struct EPPSecDNSUpdateRemove {
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:all", skip_serializing_if = "Option::is_none", serialize_with = "super::serialize_opt_bool")]
    pub all: Option<bool>,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:dsData")]
    pub ds_data: Vec<EPPSecDNSDSData>,
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:keyData")]
    pub key_data: Vec<EPPSecDNSKeyData>,
}

#[derive(Debug, Serialize)]
pub struct EPPSecDNSUpdateChange {
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:maxSigLife", skip_serializing_if = "Option::is_none")]
    pub max_signature_life: Option<i64>,
}