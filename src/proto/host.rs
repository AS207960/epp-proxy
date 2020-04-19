use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EPPHostCheck {
    #[serde(rename = "host:name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostCheckData {
    #[serde(rename = "cd", default)]
    pub data: Vec<EPPHostCheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostCheckDatum {
    #[serde(rename = "name")]
    pub name: EPPHostCheckName,
    #[serde(rename = "reason")]
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostCheckName {
    #[serde(rename = "$value")]
    pub name: String,
    #[serde(rename = "avail")]
    pub available: bool,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostInfoData {
    pub name: String,
    #[serde(rename = "roid")]
    pub registry_id: String,
    #[serde(rename = "status", default)]
    pub statuses: Vec<EPPHostStatus>,
    #[serde(rename = "addr", default)]
    pub addresses: Vec<EPPHostAddress>,
    #[serde(rename = "clID")]
    pub client_id: String,
    #[serde(rename = "crID")]
    pub client_created_id: Option<String>,
    #[serde(
        rename = "crDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub creation_date: Option<DateTime<Utc>>,
    #[serde(rename = "upID")]
    pub last_updated_client: Option<String>,
    #[serde(
        rename = "upDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub last_updated_date: Option<DateTime<Utc>>,
    #[serde(
        rename = "trDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub last_transfer_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostStatus {
    #[serde(rename = "s")]
    pub status: EPPHostStatusType,
}

#[derive(Debug, Serialize)]
pub struct EPPHostStatusSer {
    #[serde(rename = "$attr:s")]
    pub status: EPPHostStatusType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EPPHostStatusType {
    #[serde(rename = "clientDeleteProhibited")]
    ClientDeleteProhibited,
    #[serde(rename = "clientUpdateProhibited")]
    ClientUpdateProhibited,
    #[serde(rename = "linked")]
    Linked,
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "pendingCreate")]
    PendingCreate,
    #[serde(rename = "pendingDelete")]
    PendingDelete,
    #[serde(rename = "pendingTransfer")]
    PendingTransfer,
    #[serde(rename = "pendingUpdate")]
    PendingUpdate,
    #[serde(rename = "serverDeleteProhibited")]
    ServerDeleteProhibited,
    #[serde(rename = "serverUpdateProhibited")]
    ServerUpdateProhibited,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostAddress {
    #[serde(rename = "$value")]
    pub address: String,
    #[serde(rename = "ip", default)]
    pub ip_version: EPPHostAddressVersion,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EPPHostAddressVersion {
    #[serde(rename = "v4")]
    IPv4,
    #[serde(rename = "v6")]
    IPv6,
}

impl std::default::Default for EPPHostAddressVersion {
    fn default() -> Self {
        Self::IPv4
    }
}

#[derive(Debug, Serialize)]
pub struct EPPHostCreate {
    #[serde(rename = "host:name")]
    pub name: String,
    #[serde(rename = "host:addr")]
    pub addresses: Vec<EPPHostAddressSer>,
}

#[derive(Debug, Serialize)]
pub struct EPPHostAddressSer {
    #[serde(rename = "$value")]
    pub address: String,
    #[serde(rename = "$attr:ip", default)]
    pub ip_version: EPPHostAddressVersion,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostCreateData {
    pub name: String,
    #[serde(
        rename = "crDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub creation_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPHostUpdate {
    #[serde(rename = "host:name")]
    pub name: String,
    #[serde(rename = "host:add", skip_serializing_if = "Option::is_none")]
    pub add: Option<EPPHostUpdateAdd>,
    #[serde(rename = "host:rem", skip_serializing_if = "Option::is_none")]
    pub remove: Option<EPPHostUpdateRemove>,
    #[serde(rename = "host:chg", skip_serializing_if = "Option::is_none")]
    pub change: Option<EPPHostUpdateChange>,
}

#[derive(Debug, Serialize)]
pub struct EPPHostUpdateAdd {
    #[serde(rename = "$value")]
    pub params: Vec<EPPHostUpdateParam>,
}

#[derive(Debug, Serialize)]
pub struct EPPHostUpdateRemove {
    #[serde(rename = "$value")]
    pub params: Vec<EPPHostUpdateParam>,
}

#[derive(Debug, Serialize)]
pub struct EPPHostUpdateChange {
    #[serde(rename = "host:name")]
    pub name: String,
}

#[derive(Debug, Serialize)]
pub enum EPPHostUpdateParam {
    #[serde(rename = "host:addr")]
    Address(EPPHostAddressSer),
    #[serde(rename = "host:status")]
    Status(EPPHostStatusSer),
}
