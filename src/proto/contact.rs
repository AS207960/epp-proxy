use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EPPContactCheck {
    #[serde(rename = "contact:id")]
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactCheckData {
    #[serde(rename = "cd", default)]
    pub data: Vec<EPPContactCheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactCheckDatum {
    #[serde(rename = "id")]
    pub id: EPPContactCheckID,
    #[serde(rename = "reason")]
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactCheckID {
    #[serde(rename = "$value")]
    pub id: String,
    #[serde(rename = "avail")]
    pub available: bool,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactInfoData {
    pub id: String,
    #[serde(rename = "roid")]
    pub registry_id: String,
    #[serde(rename = "status", default)]
    pub statuses: Vec<EPPContactStatus>,
    #[serde(rename = "postalInfo", default)]
    pub postal_info: Vec<EPPContactPostalInfo>,
    #[serde(rename = "voice")]
    pub phone: Option<EPPContactPhone>,
    pub fax: Option<EPPContactPhone>,
    pub email: String,
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
    pub disclose: Option<EPPContactDisclosure>,
    #[serde(rename = "contact:authInfo")]
    pub auth_info: Option<EPPContactAuthInfo>,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactStatus {
    #[serde(rename = "s")]
    pub status: EPPContactStatusType,
}

#[derive(Debug, Serialize)]
pub struct EPPContactStatusSer {
    #[serde(rename = "$attr:s")]
    pub status: EPPContactStatusType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EPPContactStatusType {
    #[serde(rename = "clientDeleteProhibited")]
    ClientDeleteProhibited,
    #[serde(rename = "clientTransferProhibited")]
    ClientTransferProhibited,
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
    #[serde(rename = "serverTransferProhibited")]
    ServerTransferProhibited,
    #[serde(rename = "serverUpdateProhibited")]
    ServerUpdateProhibited,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactPhone {
    #[serde(rename = "x")]
    pub extension: Option<String>,
    #[serde(rename = "$value")]
    pub number: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactPostalInfo {
    #[serde(rename = "type")]
    pub addr_type: EPPContactPostalInfoType,
    pub name: String,
    #[serde(rename = "org")]
    pub organisation: Option<String>,
    #[serde(rename = "addr")]
    pub address: EPPContactAddress,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactAddress {
    #[serde(rename = "street")]
    pub streets: Vec<String>,
    pub city: String,
    #[serde(rename = "sp")]
    pub province: Option<String>,
    #[serde(rename = "pc")]
    pub postal_code: Option<String>,
    #[serde(rename = "cc")]
    pub country_code: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EPPContactPostalInfoType {
    #[serde(rename = "int")]
    Internationalised,
    #[serde(rename = "loc")]
    Local,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactDisclosure {
    #[serde(rename = "flag")]
    pub flag: bool,
    #[serde(rename = "$value")]
    pub elements: Vec<EPPContactDisclosureItem>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactDisclosureSer {
    #[serde(rename = "$attr:flag")]
    pub flag: String,
    #[serde(rename = "$value")]
    pub elements: Vec<EPPContactDisclosureItemSer>,
}

#[derive(Debug, Deserialize)]
pub enum EPPContactDisclosureItem {
    #[serde(rename = "contact:name")]
    Name {
        #[serde(rename = "type")]
        addr_type: EPPContactPostalInfoType,
    },
    #[serde(rename = "contact:org")]
    Organisation {
        #[serde(rename = "type")]
        addr_type: EPPContactPostalInfoType,
    },
    #[serde(rename = "contact:addr")]
    Address {
        #[serde(rename = "type")]
        addr_type: EPPContactPostalInfoType,
    },
    #[serde(rename = "contact:voice")]
    Voice,
    #[serde(rename = "contact:fax")]
    Fax,
    #[serde(rename = "contact:email")]
    Email,
}

#[derive(Debug, Serialize)]
pub enum EPPContactDisclosureItemSer {
    #[serde(rename = "contact:name")]
    Name {
        #[serde(rename = "$attr:type")]
        addr_type: EPPContactPostalInfoType,
    },
    #[serde(rename = "contact:org")]
    Organisation {
        #[serde(rename = "$attr:type")]
        addr_type: EPPContactPostalInfoType,
    },
    #[serde(rename = "contact:addr")]
    Address {
        #[serde(rename = "$attr:type")]
        addr_type: EPPContactPostalInfoType,
    },
    #[serde(rename = "contact:voice")]
    Voice {},
    #[serde(rename = "contact:fax")]
    Fax {},
    #[serde(rename = "contact:email")]
    Email {},
}

#[derive(Debug, Deserialize)]
pub struct EPPContactTransferData {
    pub id: String,
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
}

#[derive(Debug, Serialize)]
pub struct EPPContactCreate {
    #[serde(rename = "contact:id")]
    pub id: String,
    #[serde(rename = "contact:postalInfo")]
    pub postal_info: Vec<EPPContactPostalInfoSer>,
    #[serde(rename = "contact:voice", skip_serializing_if = "Option::is_none")]
    pub phone: Option<EPPContactPhoneSer>,
    #[serde(rename = "contact:fax", skip_serializing_if = "Option::is_none")]
    pub fax: Option<EPPContactPhoneSer>,
    #[serde(rename = "contact:email")]
    pub email: String,
    #[serde(rename = "contact:authInfo")]
    pub auth_info: EPPContactAuthInfo,
    #[serde(rename = "contact:disclose", skip_serializing_if = "Option::is_none")]
    pub disclose: Option<EPPContactDisclosureSer>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactPhoneSer {
    #[serde(rename = "$attr:x", skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(rename = "$value")]
    pub number: String,
}

#[derive(Debug, Serialize)]
pub struct EPPContactPostalInfoSer {
    #[serde(rename = "$attr:type")]
    pub addr_type: EPPContactPostalInfoType,
    #[serde(rename = "contact:name")]
    pub name: String,
    #[serde(rename = "contact:org", skip_serializing_if = "Option::is_none")]
    pub organisation: Option<String>,
    #[serde(rename = "contact:addr")]
    pub address: EPPContactAddressSer,
}

#[derive(Debug, Serialize)]
pub struct EPPContactAddressSer {
    #[serde(rename = "contact:street")]
    pub streets: Vec<String>,
    #[serde(rename = "contact:city")]
    pub city: String,
    #[serde(rename = "contact:sp", skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(rename = "contact:pc", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "contact:cc")]
    pub country_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPContactAuthInfo {
    #[serde(rename = "contact:pw")]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactCreateData {
    pub id: String,
    #[serde(
        rename = "crDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub creation_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactUpdate {
    #[serde(rename = "contact:id")]
    pub id: String,
    #[serde(rename = "contact:add", skip_serializing_if = "Option::is_none")]
    pub add: Option<EPPContactUpdateAdd>,
    #[serde(rename = "contact:rem", skip_serializing_if = "Option::is_none")]
    pub remove: Option<EPPContactUpdateRemove>,
    #[serde(rename = "contact:chg", skip_serializing_if = "Option::is_none")]
    pub change: Option<EPPContactUpdateChange>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactUpdateAdd {
    #[serde(rename = "$value")]
    pub statuses: Vec<EPPContactStatusSer>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactUpdateRemove {
    #[serde(rename = "$value")]
    pub statuses: Vec<EPPContactStatusSer>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactUpdateChange {
    #[serde(rename = "contact:postalInfo")]
    pub postal_info: Vec<EPPContactUpdatePostalInfo>,
    #[serde(rename = "contact:voice", skip_serializing_if = "Option::is_none")]
    pub phone: Option<EPPContactPhoneSer>,
    #[serde(rename = "contact:fax", skip_serializing_if = "Option::is_none")]
    pub fax: Option<EPPContactPhoneSer>,
    #[serde(rename = "contact:email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "contact:disclose", skip_serializing_if = "Option::is_none")]
    pub disclose: Option<EPPContactDisclosureSer>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactUpdatePostalInfo {
    #[serde(rename = "$attr:type")]
    pub addr_type: EPPContactPostalInfoType,
    #[serde(rename = "contact:name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "contact:org", skip_serializing_if = "Option::is_none")]
    pub organisation: Option<String>,
    #[serde(rename = "contact:addr", skip_serializing_if = "Option::is_none")]
    pub address: Option<EPPContactAddressSer>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactTransfer {
    #[serde(rename = "contact:id")]
    pub id: String,
    #[serde(rename = "contact:authInfo")]
    pub auth_info: EPPContactAuthInfo,
}