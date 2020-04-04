use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EPPDomainCheck {
    #[serde(rename = "domain:name")]
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainCheckData {
    #[serde(rename = "cd", default)]
    pub data: Vec<EPPDomainCheckDatum>
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainCheckDatum {
    #[serde(rename = "name")]
    pub name: EPPDomainCheckName,
    #[serde(rename = "reason")]
    pub reason: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainCheckName {
    #[serde(rename = "$value")]
    pub name: String,
    #[serde(rename = "avail")]
    pub available: bool
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
    #[serde(rename = "crDate", deserialize_with = "super::deserialize_datetime_opt", default)]
    pub creation_date: Option<DateTime<Utc>>,
    #[serde(rename = "exDate", deserialize_with = "super::deserialize_datetime_opt", default)]
    pub expiry_date: Option<DateTime<Utc>>,
    #[serde(rename = "upID")]
    pub last_updated_client: Option<String>,
    #[serde(rename = "upDate", deserialize_with = "super::deserialize_datetime_opt", default)]
    pub last_updated_date: Option<DateTime<Utc>>,
    #[serde(rename = "trDate", deserialize_with = "super::deserialize_datetime_opt", default)]
    pub last_transfer_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainStatus {
    #[serde(rename = "s", default)]
    pub status: String
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
    pub servers: Vec<EPPDomainInfoNameserver>
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
        address: EPPDomainInfoNameserverAddress
    }
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainInfoNameserverAddress {
    #[serde(rename = "$value")]
    pub address: String,
    #[serde(rename = "ip", default)]
    pub ip_version: EPPDomainInfoNameserverAddressVersion
}

#[derive(Debug, Deserialize)]
pub enum EPPDomainInfoNameserverAddressVersion {
    #[serde(rename = "v4")]
    IPv4,
    #[serde(rename = "v6")]
    IPv6
}

impl std::default::Default for EPPDomainInfoNameserverAddressVersion {
    fn default() -> Self {
        Self::IPv4
    }
}