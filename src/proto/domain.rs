use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EPPDomainCheck {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:name")]
    pub name: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:authInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_info: Option<EPPDomainAuthInfo>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainCheckData {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}cd", default)]
    pub data: Vec<EPPDomainCheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainCheckDatum {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}name")]
    pub name: EPPDomainCheckName,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}reason")]
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainCheckName {
    #[serde(rename = "$value")]
    pub name: String,
    #[serde(rename = "$attr:avail")]
    pub available: bool,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainInfo {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:name")]
    pub name: EPPDomainInfoName,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:authInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_info: Option<EPPDomainAuthInfo>,
}

#[derive(Debug, Serialize)]
pub enum EPPDomainInfoHosts {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "del")]
    Delegated,
    #[serde(rename = "sub")]
    Subordinate,
    #[serde(rename = "none")]
    None
}

#[derive(Debug, Serialize)]
pub struct EPPDomainInfoName {
    #[serde(
        rename = "$attr:hosts",
        skip_serializing_if = "Option::is_none"
    )]
    pub hosts: Option<EPPDomainInfoHosts>,
    #[serde(rename = "$value")]
    pub name: String
}


#[derive(Debug, Deserialize)]
pub struct EPPDomainInfoData {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}name")]
    pub name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}roid", default)]
    pub registry_id: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}status", default)]
    pub statuses: Vec<EPPDomainStatus>,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}registrant", default)]
    pub registrant: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}contact", default)]
    pub contacts: Vec<EPPDomainInfoContact>,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}ns", default)]
    pub nameservers: Option<EPPDomainInfoNameservers>,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}host", default)]
    pub hosts: Vec<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}clID")]
    pub client_id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}crID")]
    pub client_created_id: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}crDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub creation_date: Option<DateTime<Utc>>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}upID")]
    pub last_updated_client: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}upDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub last_updated_date: Option<DateTime<Utc>>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}trDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub last_transfer_date: Option<DateTime<Utc>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}authInfo")]
    pub auth_info: Option<EPPDomainAuthInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPDomainStatus {
    #[serde(rename = "$attr:s")]
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
    #[serde(rename = "Granted")]
    Granted,
    #[serde(rename = "pendingCreate")]
    PendingCreate,
    #[serde(rename = "pendingDelete")]
    PendingDelete,
    #[serde(rename = "Terminated")]
    Terminated,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPDomainInfoContact {
    #[serde(rename = "$attr:type")]
    pub contact_type: String,
    #[serde(rename = "$value")]
    pub contact_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPDomainInfoNameservers {
    #[serde(rename = "$value")]
    pub servers: Vec<EPPDomainInfoNameserver>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EPPDomainInfoNameserver {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:hostObj")]
    HostOnly(String),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:hostAttr")]
    HostAndAddress {
        #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:hostName")]
        host: String,
        #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:hostAddr", default)]
        addresses: Vec<EPPDomainInfoNameserverAddress>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPDomainInfoNameserverAddress {
    #[serde(rename = "$value")]
    pub address: String,
    #[serde(rename = "$attr:ip", default)]
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
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}name")]
    pub name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}trStatus")]
    pub transfer_status: super::EPPTransferStatus,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}reID")]
    pub requested_client_id: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}reDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub requested_date: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}acID")]
    pub act_client_id: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}acDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub act_date: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainCreate {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:name")]
    pub name: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<EPPDomainPeriod>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:ns",
        skip_serializing_if = "Option::is_none"
    )]
    pub nameservers: Option<EPPDomainInfoNameservers>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:registrant",
        skip_serializing_if = "Option::is_none"
    )]
    pub registrant: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:contact")]
    pub contacts: Vec<EPPDomainInfoContact>,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:authInfo")]
    pub auth_info: EPPDomainAuthInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPDomainAuthInfo {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:pw", default)]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPDomainPeriod {
    #[serde(rename = "$attr:unit")]
    pub unit: EPPDomainPeriodUnit,
    #[serde(rename = "$value")]
    pub value: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EPPDomainPeriodUnit {
    #[serde(rename = "y")]
    Years,
    #[serde(rename = "m")]
    Months,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainCreateData {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}name")]
    pub name: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}crDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub creation_date: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainRenew {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:name")]
    pub name: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:curExpDate",
        serialize_with = "super::serialize_date"
    )]
    pub current_expiry_date: Date<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:period")]
    pub period: Option<EPPDomainPeriod>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainRenewData {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:name")]
    pub name: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainTransfer {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:name")]
    pub name: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<EPPDomainPeriod>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:authInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_info: Option<EPPDomainAuthInfo>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainUpdate {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:name")]
    pub name: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:add",
        skip_serializing_if = "Option::is_none"
    )]
    pub add: Option<EPPDomainUpdateAddRemove>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:rem",
        skip_serializing_if = "Option::is_none"
    )]
    pub remove: Option<EPPDomainUpdateAddRemove>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:chg",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<EPPDomainUpdateChange>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainUpdateAddRemove {
    #[serde(rename = "$value")]
    pub params: Vec<EPPDomainUpdateParam>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainUpdateChange {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:registrant",
        skip_serializing_if = "Option::is_none"
    )]
    pub registrant: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:authInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_info: Option<EPPDomainAuthInfo>,
}

#[derive(Debug, Serialize)]
pub enum EPPDomainUpdateParam {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:status")]
    Status(EPPDomainStatus),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:contact")]
    Contact(EPPDomainInfoContact),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:ns")]
    Nameserver(EPPDomainInfoNameservers),
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainPanData {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}name")]
    pub name: EPPDomainPanDomain,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}paTRID")]
    pub transaction_id: super::EPPTransactionIdentifier,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:domain-1.0}paDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub action_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainPanDomain {
    #[serde(rename = "$attr:paResult")]
    pub result: bool,
    #[serde(rename = "$value")]
    pub domain: String,
}
