use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EPPEmailForwardCheck {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:name")]
    pub name: String,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:authInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_info: Option<EPPEmailForwardAuthInfo>,
}

#[derive(Debug, Deserialize)]
pub struct EPPEmailForwardCheckData {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}cd", default)]
    pub data: Vec<EPPEmailForwardCheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPEmailForwardCheckDatum {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}name")]
    pub name: EPPEmailForwardCheckName,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}reason", default)]
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPEmailForwardCheckName {
    #[serde(rename = "$value")]
    pub name: String,
    #[serde(rename = "$attr:avail")]
    pub available: bool,
}

#[derive(Debug, Serialize)]
pub struct EPPEmailForwardInfo {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:name")]
    pub name: String,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:authInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_info: Option<EPPEmailForwardAuthInfo>,
}

#[derive(Debug, Deserialize)]
pub struct EPPEmailForwardInfoData {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}name")]
    pub name: String,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}roid")]
    pub registry_id: String,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}status", default)]
    pub statuses: Vec<super::domain::EPPDomainStatus>,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}registrant", default)]
    pub registrant: Option<String>,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}contact", default)]
    pub contacts: Vec<super::domain::EPPDomainInfoContact>,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}fwdTo", default)]
    pub forward_to: Option<String>,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}clID")]
    pub client_id: String,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}crID")]
    pub client_created_id: Option<String>,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}crDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub creation_date: Option<DateTime<Utc>>,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}upID")]
    pub last_updated_client: Option<String>,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}upDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub last_updated_date: Option<DateTime<Utc>>,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}trDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub last_transfer_date: Option<DateTime<Utc>>,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}authInfo")]
    pub auth_info: Option<EPPEmailForwardAuthInfo>,
}

#[derive(Debug, Deserialize)]
pub struct EPPEmailForwardTransferData {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}name")]
    pub name: String,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}trStatus")]
    pub transfer_status: super::EPPTransferStatus,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}reID")]
    pub requested_client_id: String,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}reDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub requested_date: DateTime<Utc>,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}acID")]
    pub act_client_id: String,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}acDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub act_date: DateTime<Utc>,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPEmailForwardCreate {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:name")]
    pub name: String,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:fwdTo")]
    pub forward_to: String,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<super::domain::EPPDomainPeriod>,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:registrant",
        skip_serializing_if = "Option::is_none"
    )]
    pub registrant: Option<String>,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:contact")]
    pub contacts: Vec<super::domain::EPPDomainInfoContact>,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:authInfo")]
    pub auth_info: EPPEmailForwardAuthInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPEmailForwardAuthInfo {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:pw", default)]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPEmailForwardCreateData {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}name")]
    pub name: String,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}crDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub creation_date: DateTime<Utc>,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPEmailForwardRenew {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:name")]
    pub name: String,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:curExpDate",
        serialize_with = "super::serialize_date"
    )]
    pub current_expiry_date: NaiveDate,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:period")]
    pub period: Option<super::domain::EPPDomainPeriod>,
}

#[derive(Debug, Deserialize)]
pub struct EPPEmailForwardRenewData {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}name")]
    pub name: String,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPEmailForwardTransfer {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:name")]
    pub name: String,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<super::domain::EPPDomainPeriod>,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:authInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_info: Option<EPPEmailForwardAuthInfo>,
}

#[derive(Debug, Serialize)]
pub struct EPPEmailForwardUpdate {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:name")]
    pub name: String,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:add",
        skip_serializing_if = "Option::is_none"
    )]
    pub add: Option<EPPEmailForwardUpdateAddRemove>,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:rem",
        skip_serializing_if = "Option::is_none"
    )]
    pub remove: Option<EPPEmailForwardUpdateAddRemove>,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:chg",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<EPPEmailForwardUpdateChange>,
}

#[derive(Debug, Serialize)]
pub struct EPPEmailForwardUpdateAddRemove {
    #[serde(rename = "$value")]
    pub params: Vec<EPPEmailForwardUpdateParam>,
}

#[derive(Debug, Serialize)]
pub struct EPPEmailForwardUpdateChange {
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:fwdTo",
        skip_serializing_if = "Option::is_none"
    )]
    pub forward_to: Option<String>,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:registrant",
        skip_serializing_if = "Option::is_none"
    )]
    pub registrant: Option<String>,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:authInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_info: Option<EPPEmailForwardAuthInfo>,
}

#[derive(Debug, Serialize)]
pub enum EPPEmailForwardUpdateParam {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:status")]
    Status(super::domain::EPPDomainStatus),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:contact")]
    Contact(super::domain::EPPDomainInfoContact),
}

#[derive(Debug, Deserialize)]
pub struct EPPEmailForwardPanData {
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}name")]
    pub name: EPPEmailForwardPanDomain,
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}paTRID")]
    pub transaction_id: super::EPPTransactionIdentifier,
    #[serde(
        rename = "{http://www.nic.name/epp/emailFwd-1.0}paDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub action_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct EPPEmailForwardPanDomain {
    #[serde(rename = "$attr:paResult")]
    pub result: bool,
    #[serde(rename = "$value")]
    pub domain: String,
}
