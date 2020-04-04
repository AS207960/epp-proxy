use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EPPContactCheck {
    #[serde(rename = "contact:id")]
    pub id: String
}

#[derive(Debug, Deserialize)]
pub struct EPPContactCheckData {
    #[serde(rename = "cd", default)]
    pub data: Vec<EPPContactCheckDatum>
}

#[derive(Debug, Deserialize)]
pub struct EPPContactCheckDatum {
    #[serde(rename = "id")]
    pub id: EPPContactCheckID,
    #[serde(rename = "reason")]
    pub reason: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct EPPContactCheckID {
    #[serde(rename = "$value")]
    pub id: String,
    #[serde(rename = "avail")]
    pub available: bool
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
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub email: String,
    #[serde(rename = "clID")]
    pub client_id: String,
    #[serde(rename = "crID")]
    pub client_created_id: Option<String>,
    #[serde(rename = "crDate", deserialize_with = "super::deserialize_datetime_opt", default)]
    pub creation_date: Option<DateTime<Utc>>,
    #[serde(rename = "upID")]
    pub last_updated_client: Option<String>,
    #[serde(rename = "upDate", deserialize_with = "super::deserialize_datetime_opt", default)]
    pub last_updated_date: Option<DateTime<Utc>>,
    #[serde(rename = "trDate", deserialize_with = "super::deserialize_datetime_opt", default)]
    pub last_transfer_date: Option<DateTime<Utc>>,
    pub disclose: Option<EPPContactDisclosure>,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactStatus {
    #[serde(rename = "s")]
    pub status: String
}

#[derive(Debug, Serialize)]
pub struct EPPContactStatusSer {
    #[serde(rename = "$attr:s")]
    pub status: String
}

#[derive(Debug, Deserialize)]
pub struct EPPContactPostalInfo {
    #[serde(rename = "type")]
    pub addr_type: EPPContactPostalInfoType,
    pub name: String,
    #[serde(rename = "org")]
    pub organisation: Option<String>,
    #[serde(rename = "addr")]
    pub address: EPPContactAddress
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

#[derive(Debug, Deserialize, PartialEq)]
pub enum EPPContactPostalInfoType {
    #[serde(rename = "int")]
    Internationalised,
    #[serde(rename = "loc")]
    Local
}

#[derive(Debug, Deserialize)]
pub struct EPPContactDisclosure {
    #[serde(rename = "flag")]
    pub flag: bool
}

//#[derive(Debug, Deserialize, Serialize)]
//pub enum EPPHostAddressVersion {
//    #[serde(rename = "v4")]
//    IPv4,
//    #[serde(rename = "v6")]
//    IPv6
//}
//
//impl std::default::Default for EPPHostAddressVersion {
//    fn default() -> Self {
//        Self::IPv4
//    }
//}
//
//#[derive(Debug, Serialize)]
//pub struct EPPHostCreate {
//    #[serde(rename = "host:name")]
//    pub name: String,
//    #[serde(rename = "host:addr")]
//    pub addresses: Vec<EPPHostAddressSer>
//}
//
//#[derive(Debug, Serialize)]
//pub struct EPPHostAddressSer {
//    #[serde(rename = "$value")]
//    pub address: String,
//    #[serde(rename = "$attr:ip", default)]
//    pub ip_version: EPPHostAddressVersion
//}
//
//#[derive(Debug, Deserialize)]
//pub struct EPPHostCreateData {
//    pub name: String,
//    #[serde(rename = "crDate", deserialize_with = "super::deserialize_datetime_opt", default)]
//    pub creation_date: Option<DateTime<Utc>>
//}
//
//#[derive(Debug, Serialize)]
//pub struct EPPHostDelete {
//    #[serde(rename = "host:name")]
//    pub name: String,
//}
//
//#[derive(Debug, Serialize)]
//pub struct EPPHostUpdate {
//    #[serde(rename = "host:name")]
//    pub name: String,
//    #[serde(rename = "host:add", skip_serializing_if = "Option::is_none")]
//    pub add: Option<EPPHostUpdateAdd>,
//    #[serde(rename = "host:rem", skip_serializing_if = "Option::is_none")]
//    pub remove: Option<EPPHostUpdateRemove>,
//    #[serde(rename = "host:chg", skip_serializing_if = "Option::is_none")]
//    pub change: Option<EPPHostUpdateChange>,
//}
//
//#[derive(Debug, Serialize)]
//pub struct EPPHostUpdateAdd {
//    #[serde(rename = "$value")]
//    pub params: Vec<EPPHostUpdateParam>
//}
//
//#[derive(Debug, Serialize)]
//pub struct EPPHostUpdateRemove {
//    #[serde(rename = "$value")]
//    pub params: Vec<EPPHostUpdateParam>
//}
//
//#[derive(Debug, Serialize)]
//pub struct EPPHostUpdateChange {
//    #[serde(rename = "host:name")]
//    pub name: String
//}
//
//#[derive(Debug, Serialize)]
//pub enum EPPHostUpdateParam {
//    #[serde(rename = "host:addr")]
//    Address(EPPHostAddressSer),
//    #[serde(rename = "host:status")]
//    Status(EPPHostStatusSer),
//}