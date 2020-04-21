#[derive(Debug, Deserialize)]
pub struct EPPSecDNSData {
    #[serde(rename = "maxSigLife")]
    pub max_signature_life: Option<i64>,
    #[serde(rename = "dsData")]
    pub ds_data: Vec<EPPSecDNSDSData>,
    #[serde(rename = "keyData")]
    pub key_data: Vec<EPPSecDNSKeyData>,
}

#[derive(Debug, Deserialize)]
pub struct EPPSecDNSDSData {
    #[serde(rename = "keyTag")]
    pub key_tag: u16,
    #[serde(rename = "alg")]
    pub algorithm: u8,
    #[serde(rename = "digestType")]
    pub digest_type: u8,
    #[serde(rename = "digest")]
    pub digest: String,
    #[serde(rename = "keyData")]
    pub key_data: Option<EPPSecDNSKeyData>
}

#[derive(Debug, Deserialize)]
pub struct EPPSecDNSKeyData {
    pub flags: u16,
    pub protocol: u8,
    #[serde(rename = "alg")]
    pub algorithm: u8,
    #[serde(rename = "pubKey")]
    pub public_key: String
}

#[derive(Debug, Serialize)]
pub struct EPPSecDNSCreate {
    #[serde(rename = "secDNS:maxSigLife", skip_serializing_if = "Option::is_none")]
    pub max_signature_life: Option<i64>,
    #[serde(rename = "secDNS:dsData")]
    pub ds_data: Vec<EPPSecDNSDSDataSer>,
    #[serde(rename = "secDNS:keyData")]
    pub key_data: Vec<EPPSecDNSKeyDataSer>,
}

#[derive(Debug, Serialize)]
pub struct EPPSecDNSDSDataSer {
    #[serde(rename = "secDNS:keyTag")]
    pub key_tag: u16,
    #[serde(rename = "secDNS:alg")]
    pub algorithm: u8,
    #[serde(rename = "secDNS:digestType")]
    pub digest_type: u8,
    #[serde(rename = "secDNS:digest")]
    pub digest: String,
    #[serde(rename = "secDNS:keyData")]
    pub key_data: Option<EPPSecDNSKeyDataSer>
}

#[derive(Debug, Serialize)]
pub struct EPPSecDNSKeyDataSer {
    #[serde(rename = "secDNS:flags")]
    pub flags: u16,
    #[serde(rename = "secDNS:protocol")]
    pub protocol: u8,
    #[serde(rename = "secDNS:alg")]
    pub algorithm: u8,
    #[serde(rename = "secDNS:pubKey")]
    pub public_key: String
}

#[derive(Debug, Serialize)]
pub struct EPPSecDNSUpdate {
    #[serde(rename = "$attr:urgent")]
    pub urgent: bool,
    #[serde(rename = "secDNS:add", skip_serializing_if = "Option::is_none")]
    pub add: Option<EPPSecDNSUpdateAdd>,
    #[serde(rename = "secDNS:rem", skip_serializing_if = "Option::is_none")]
    pub remove: Option<EPPSecDNSUpdateRemove>,
    #[serde(rename = "secDNS:chg", skip_serializing_if = "Option::is_none")]
    pub change: Option<EPPSecDNSUpdateChange>,
}

#[derive(Debug, Serialize)]
pub struct EPPSecDNSUpdateAdd {
    #[serde(rename = "secDNS:all", skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(rename = "secDNS:dsData")]
    pub ds_data: Vec<EPPSecDNSDSDataSer>,
    #[serde(rename = "secDNS:keyData")]
    pub key_data: Vec<EPPSecDNSKeyDataSer>,
}

#[derive(Debug, Serialize)]
pub struct EPPSecDNSUpdateRemove {
    #[serde(rename = "secDNS:dsData")]
    pub ds_data: Vec<EPPSecDNSDSDataSer>,
    #[serde(rename = "secDNS:keyData")]
    pub key_data: Vec<EPPSecDNSKeyDataSer>,
}

#[derive(Debug, Serialize)]
pub struct EPPSecDNSUpdateChange {
    #[serde(rename = "secDNS:maxSigLife", skip_serializing_if = "Option::is_none")]
    pub max_signature_life: Option<i64>,
}