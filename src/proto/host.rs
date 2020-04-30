use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EPPHostCheck {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostCheckData {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}cd", default)]
    pub data: Vec<EPPHostCheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostCheckDatum {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}name")]
    pub name: EPPHostCheckName,
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}reason")]
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostCheckName {
    #[serde(rename = "$value")]
    pub name: String,
    #[serde(rename = "$attr:avail")]
    pub available: bool,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostInfoData {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}name")]
    pub name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}roid", default)]
    pub registry_id: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}status", default)]
    pub statuses: Vec<EPPHostStatus>,
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}addr", default)]
    pub addresses: Vec<EPPHostAddress>,
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}clID")]
    pub client_id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}crID")]
    pub client_created_id: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:host-1.0}crDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub creation_date: Option<DateTime<Utc>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}upID")]
    pub last_updated_client: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:host-1.0}upDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub last_updated_date: Option<DateTime<Utc>>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:host-1.0}trDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub last_transfer_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPHostStatus {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPHostAddress {
    #[serde(rename = "$value")]
    pub address: String,
    #[serde(rename = "$attr:ip", default)]
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
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:name")]
    pub name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:addr")]
    pub addresses: Vec<EPPHostAddress>,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostCreateData {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}name")]
    pub name: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:host-1.0}crDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub creation_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPHostUpdate {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:name")]
    pub name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:add", skip_serializing_if = "Option::is_none")]
    pub add: Option<EPPHostUpdateAdd>,
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:rem", skip_serializing_if = "Option::is_none")]
    pub remove: Option<EPPHostUpdateRemove>,
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:chg", skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:name")]
    pub name: String,
}

#[derive(Debug, Serialize)]
pub enum EPPHostUpdateParam {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:addr")]
    Address(EPPHostAddress),
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:status")]
    Status(EPPHostStatus),
}
