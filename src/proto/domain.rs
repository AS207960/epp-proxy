use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EPPDomainCheck {
    #[serde(rename = "domain:name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainCheckData {
    #[serde(rename = "cd", default)]
    pub data: Vec<EPPDomainCheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainCheckDatum {
    #[serde(rename = "name")]
    pub name: EPPDomainCheckName,
    #[serde(rename = "reason")]
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainCheckName {
    #[serde(rename = "$value")]
    pub name: String,
    #[serde(rename = "avail")]
    pub available: bool,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainInfoData {
    pub name: String,
    #[serde(rename = "roid")]
    pub registry_id: String,
    #[serde(rename = "status", default)]
    pub statuses: Vec<EPPDomainStatus>,
    pub registrant: String,
    #[serde(rename = "contact", default)]
    pub contacts: Vec<EPPDomainInfoContact>,
    #[serde(rename = "ns")]
    pub nameservers: EPPDomainInfoNameservers,
    #[serde(rename = "host", default)]
    pub hosts: Vec<String>,
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
    #[serde(
        rename = "exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
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
    #[serde(rename = "domain:authInfo")]
    pub auth_info: Option<EPPDomainAuthInfo>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainStatus {
    #[serde(rename = "s")]
    pub status: EPPDomainStatusType,
    #[serde(rename = "$value")]
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainStatusSer {
    #[serde(rename = "$attr:s", default)]
    pub status: EPPDomainStatusType,
    #[serde(rename = "$value")]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EPPDomainStatusType {
    #[serde(rename = "clientDeleteProhibited")]
    ClientDeleteProhibited,
    #[serde(rename = "clientHold")]
    ClientHold,
    #[serde(rename = "clientRenewProhibited")]
    ClientRenewProhibited,
    #[serde(rename = "clientTransferProhibited")]
    ClientTransferProhibited,
    #[serde(rename = "clientUpdateProhibited")]
    ClientUpdateProhibited,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "pendingCreate")]
    PendingCreate,
    #[serde(rename = "pendingDelete")]
    PendingDelete,
    #[serde(rename = "pendingRenew")]
    PendingRenew,
    #[serde(rename = "pendingTransfer")]
    PendingTransfer,
    #[serde(rename = "pendingUpdate")]
    PendingUpdate,
    #[serde(rename = "serverDeleteProhibited")]
    ServerDeleteProhibited,
    #[serde(rename = "serverHold")]
    ServerHold,
    #[serde(rename = "serverRenewProhibited")]
    ServerRenewProhibited,
    #[serde(rename = "serverTransferProhibited")]
    ServerTransferProhibited,
    #[serde(rename = "serverUpdateProhibited")]
    ServerUpdateProhibited,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainInfoContact {
    #[serde(rename = "type")]
    pub contact_type: String,
    #[serde(rename = "$value")]
    pub contact_id: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainInfoNameservers {
    #[serde(rename = "$value")]
    pub servers: Vec<EPPDomainInfoNameserver>,
}

#[derive(Debug, Deserialize)]
pub enum EPPDomainInfoNameserver {
    #[serde(rename = "domain:hostObj")]
    HostOnly(String),
    #[serde(rename = "domain:hostAttr")]
    HostAndAddress {
        #[serde(rename = "hostName")]
        host: String,
        #[serde(rename = "hostAddr")]
        address: EPPDomainInfoNameserverAddress,
    },
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainInfoNameserverAddress {
    #[serde(rename = "$value")]
    pub address: String,
    #[serde(rename = "ip", default)]
    pub ip_version: EPPDomainInfoNameserverAddressVersion,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EPPDomainInfoNameserverAddressVersion {
    #[serde(rename = "v4")]
    IPv4,
    #[serde(rename = "v6")]
    IPv6,
}

impl std::default::Default for EPPDomainInfoNameserverAddressVersion {
    fn default() -> Self {
        Self::IPv4
    }
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainTransferData {
    pub name: String,
    #[serde(rename = "trStatus")]
    pub transfer_status: super::EPPTransferStatus,
    #[serde(rename = "reID")]
    pub requested_client_id: String,
    #[serde(rename = "reDate", deserialize_with = "super::deserialize_datetime")]
    pub requested_date: DateTime<Utc>,
    #[serde(rename = "acID")]
    pub act_client_id: String,
    #[serde(rename = "acDate", deserialize_with = "super::deserialize_datetime")]
    pub act_date: DateTime<Utc>,
    #[serde(
        rename = "exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainCreate {
    #[serde(rename = "domain:name")]
    pub name: String,
    #[serde(rename = "domain:period", skip_serializing_if = "Option::is_none")]
    pub period: Option<EPPDomainPeriod>,
    #[serde(rename = "domain:ns")]
    pub nameservers: EPPDomainInfoNameserversSer,
    #[serde(rename = "domain:registrant")]
    pub registrant: String,
    #[serde(rename = "domain:contact")]
    pub contacts: Vec<EPPDomainInfoContactSer>,
    #[serde(rename = "domain:authInfo")]
    pub auth_info: EPPDomainAuthInfo,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainInfoNameserversSer {
    #[serde(rename = "$value")]
    pub servers: Vec<EPPDomainInfoNameserverSer>,
}

#[derive(Debug, Serialize)]
pub enum EPPDomainInfoNameserverSer {
    #[serde(rename = "domain:hostObj")]
    HostOnly(String),
    #[serde(rename = "domain:hostAttr")]
    HostAndAddress {
        #[serde(rename = "domain:hostName")]
        host: String,
        #[serde(rename = "domain:hostAddr")]
        address: EPPDomainInfoNameserverAddressSer,
    },
}

#[derive(Debug, Serialize)]
pub struct EPPDomainInfoNameserverAddressSer {
    #[serde(rename = "$value")]
    pub address: String,
    #[serde(rename = "$attr:ip", default)]
    pub ip_version: EPPDomainInfoNameserverAddressVersion,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainInfoContactSer {
    #[serde(rename = "$attr:type")]
    pub contact_type: String,
    #[serde(rename = "$value")]
    pub contact_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPDomainAuthInfo {
    #[serde(rename = "domain:pw")]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainPeriod {
    #[serde(rename = "$attr:unit")]
    pub unit: EPPDomainPeriodUnit,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Serialize)]
pub enum EPPDomainPeriodUnit {
    #[serde(rename = "y")]
    Years,
    #[serde(rename = "m")]
    Months,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainCreateData {
    pub name: String,
    #[serde(rename = "crDate", deserialize_with = "super::deserialize_datetime")]
    pub creation_date: DateTime<Utc>,
    #[serde(
        rename = "exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainRenew {
    #[serde(rename = "domain:name")]
    pub name: String,
    #[serde(rename = "domain:period")]
    pub period: Option<EPPDomainPeriod>,
    #[serde(rename = "domain:curExpDate", serialize_with = "super::serialize_date")]
    pub current_expiry_date: Date<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainRenewData {
    pub name: String,
    #[serde(
        rename = "exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainTransfer {
    #[serde(rename = "domain:name")]
    pub name: String,
    #[serde(rename = "domain:period", skip_serializing_if = "Option::is_none")]
    pub period: Option<EPPDomainPeriod>,
    #[serde(rename = "domain:authInfo")]
    pub auth_info: EPPDomainAuthInfo,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainUpdate {
    #[serde(rename = "domain:name")]
    pub name: String,
    #[serde(rename = "domain:add", skip_serializing_if = "Option::is_none")]
    pub add: Option<EPPDomainUpdateAdd>,
    #[serde(rename = "domain:rem", skip_serializing_if = "Option::is_none")]
    pub remove: Option<EPPDomainUpdateRemove>,
    #[serde(rename = "domain:chg", skip_serializing_if = "Option::is_none")]
    pub change: Option<EPPDomainUpdateChange>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainUpdateAdd {
    #[serde(rename = "$value")]
    pub params: Vec<EPPDomainUpdateParam>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainUpdateRemove {
    #[serde(rename = "$value")]
    pub params: Vec<EPPDomainUpdateParam>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainUpdateChange {
    #[serde(rename = "domain:registrant", skip_serializing_if = "Option::is_none")]
    pub registrant: Option<String>,
    #[serde(rename = "domain:authInfo", skip_serializing_if = "Option::is_none")]
    pub auth_info: Option<EPPDomainAuthInfo>,
}

#[derive(Debug, Serialize)]
pub enum EPPDomainUpdateParam {
    #[serde(rename = "domain:status")]
    Status(EPPDomainStatusSer),
    #[serde(rename = "domain:contact")]
    Contact(EPPDomainInfoContactSer),
    #[serde(rename = "domain:ns")]
    Nameserver(EPPDomainInfoNameserversSer),
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainPanData {
    pub name: EPPDomainPanDomain,
    #[serde(rename = "paTRID")]
    pub transaction_id: super::EPPTransactionIdentifier,
    #[serde(rename = "paDate", deserialize_with = "super::deserialize_datetime")]
    pub action_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainPanDomain {
    #[serde(rename = "paResult")]
    pub result: bool,
    #[serde(rename = "$value")]
    pub domain: String,
}
