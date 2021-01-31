use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EPPContactCheck {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:id")]
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactCheckData {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}cd", default)]
    pub data: Vec<EPPContactCheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactCheckDatum {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}id")]
    pub id: EPPContactCheckID,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}reason")]
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactCheckID {
    #[serde(rename = "$value")]
    pub id: String,
    #[serde(rename = "$attr:avail")]
    pub available: bool,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactInfoData {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}id")]
    pub id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}roid", default)]
    pub registry_id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}status", default)]
    pub statuses: Vec<EPPContactStatus>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:role", default)]
    pub traficom_role: Option<super::traficom::EPPContactTraficomRole>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:type", default)]
    pub traficom_type: Option<super::traficom::EPPContactTraficomType>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}postalInfo", default)]
    pub postal_info: Vec<EPPContactPostalInfo>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}voice")]
    pub phone: Option<EPPContactPhone>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}fax")]
    pub fax: Option<EPPContactPhone>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}email")]
    pub email: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}legalemail")]
    pub traficom_legal_email: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}clID")]
    pub client_id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}crID")]
    pub client_created_id: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}crDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub creation_date: Option<DateTime<Utc>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}upID")]
    pub last_updated_client: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}upDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub last_updated_date: Option<DateTime<Utc>>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}trDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub last_transfer_date: Option<DateTime<Utc>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}disclose")]
    pub disclose: Option<EPPContactDisclosure>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:authInfo")]
    pub auth_info: Option<EPPContactAuthInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPContactStatus {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPContactPhone {
    #[serde(rename = "$attr:x", skip_serializing_if = "Option::is_none", default)]
    pub extension: Option<String>,
    #[serde(rename = "$value")]
    pub number: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPContactPostalInfo {
    #[serde(rename = "$attr:type")]
    pub addr_type: EPPContactPostalInfoType,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:name", default)]
    pub name: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:firstname",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub traficom_first_name: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:lastname",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub traficom_last_name: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:org",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub organisation: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:infinnish",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub traficom_is_finnish: Option<bool>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:registernumber",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub traficom_register_number: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:identity",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub traficom_identity: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:birthDate",
        deserialize_with = "super::deserialize_date_opt",
        serialize_with = "super::serialize_date_opt",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub traficom_birth_date: Option<Date<Utc>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:addr")]
    pub address: EPPContactAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPContactAddress {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:street")]
    pub streets: Vec<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:city")]
    pub city: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:sp",
        skip_serializing_if = "Option::is_none"
    )]
    pub province: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:pc",
        skip_serializing_if = "Option::is_none"
    )]
    pub postal_code: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:cc")]
    pub country_code: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EPPContactPostalInfoType {
    #[serde(rename = "int")]
    Internationalised,
    #[serde(rename = "loc")]
    Local,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPContactDisclosure {
    #[serde(rename = "$attr:flag", default)]
    pub flag: bool,
    #[serde(rename = "$value")]
    pub elements: Vec<EPPContactDisclosureItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EPPContactDisclosureItem {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:infDataDisclose")]
    DisclosureType {
        #[serde(rename = "$attr:flag")]
        flag: bool,
        #[serde(rename = "$value")]
        value: Vec<String>,
    },
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:name")]
    Name {
        #[serde(rename = "$attr:type")]
        addr_type: EPPContactPostalInfoType,
    },
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:org")]
    Organisation {
        #[serde(rename = "$attr:type")]
        addr_type: EPPContactPostalInfoType,
    },
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:addr")]
    Address {
        #[serde(rename = "$attr:type")]
        addr_type: EPPContactPostalInfoType,
    },
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:voice")]
    Voice {},
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:fax")]
    Fax {},
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:email")]
    Email {},
}

#[derive(Debug, Deserialize)]
pub struct EPPContactTransferData {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}id")]
    pub id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}trStatus")]
    pub transfer_status: super::EPPTransferStatus,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}reID")]
    pub requested_client_id: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}reDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub requested_date: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}acID")]
    pub act_client_id: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}acDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub act_date: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactCreate {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:id")]
    pub id: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:role",
        skip_serializing_if = "Option::is_none"
    )]
    pub traficom_role: Option<super::traficom::EPPContactTraficomRole>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:type",
        skip_serializing_if = "Option::is_none"
    )]
    pub traficom_type: Option<super::traficom::EPPContactTraficomType>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:postalInfo")]
    pub postal_info: Vec<EPPContactPostalInfo>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:voice",
        skip_serializing_if = "Option::is_none"
    )]
    pub phone: Option<EPPContactPhone>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:fax",
        skip_serializing_if = "Option::is_none"
    )]
    pub fax: Option<EPPContactPhone>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:email")]
    pub email: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:legalemail",
        skip_serializing_if = "Option::is_none"
    )]
    pub traficom_legal_email: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:authInfo")]
    pub auth_info: EPPContactAuthInfo,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:disclose",
        skip_serializing_if = "Option::is_none"
    )]
    pub disclose: Option<EPPContactDisclosure>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPContactAuthInfo {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:pw", default)]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactCreateData {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}id")]
    pub id: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}crDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub creation_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactUpdate {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:id")]
    pub id: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:role",
        skip_serializing_if = "Option::is_none"
    )]
    pub traficom_role: Option<super::traficom::EPPContactTraficomRole>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:add",
        skip_serializing_if = "Option::is_none"
    )]
    pub add: Option<EPPContactUpdateAdd>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:rem",
        skip_serializing_if = "Option::is_none"
    )]
    pub remove: Option<EPPContactUpdateRemove>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:chg",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<EPPContactUpdateChange>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactUpdateAdd {
    #[serde(rename = "$value")]
    pub statuses: Vec<EPPContactStatus>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactUpdateRemove {
    #[serde(rename = "$value")]
    pub statuses: Vec<EPPContactStatus>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactUpdateChange {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:postalInfo")]
    pub postal_info: Vec<EPPContactUpdatePostalInfo>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:voice",
        skip_serializing_if = "Option::is_none"
    )]
    pub phone: Option<EPPContactPhone>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:fax",
        skip_serializing_if = "Option::is_none"
    )]
    pub fax: Option<EPPContactPhone>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:email",
        skip_serializing_if = "Option::is_none"
    )]
    pub email: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:disclose",
        skip_serializing_if = "Option::is_none"
    )]
    pub disclose: Option<EPPContactDisclosure>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:authInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_info: Option<EPPContactAuthInfo>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactUpdatePostalInfo {
    #[serde(rename = "$attr:type")]
    pub addr_type: EPPContactPostalInfoType,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:name",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:org",
        skip_serializing_if = "Option::is_none"
    )]
    pub organisation: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:addr",
        skip_serializing_if = "Option::is_none"
    )]
    pub address: Option<EPPContactAddress>,
}

#[derive(Debug, Serialize)]
pub struct EPPContactTransfer {
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:id")]
    pub id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:authInfo")]
    pub auth_info: EPPContactAuthInfo,
}
