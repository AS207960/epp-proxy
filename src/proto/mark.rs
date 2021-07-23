use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::enum_variant_names)]
pub enum Mark {
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}trademark")]
    TradeMark(TradeMark),
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}treatyOrStatute")]
    TreatyOrStatute(TreatyOrStatute),
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}court")]
    Court(Court),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeMark {
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}id")]
    pub id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}markName")]
    pub mark_name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}holder")]
    pub holders: Vec<Holder>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}contact",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub contacts: Vec<Contact>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}jurisdiction")]
    pub jurisdiction: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}class",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub classes: Vec<u32>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}labels",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub labels: Vec<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}goodsAndServices")]
    pub goods_and_services: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}apId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub application_id: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}apDate",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub application_date: Option<DateTime<Utc>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}regNum")]
    pub registration_id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}regDate")]
    pub registration_date: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}exDate",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreatyOrStatute {
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}id")]
    pub id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}markName")]
    pub mark_name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}holder")]
    pub holders: Vec<Holder>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}contact",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub contacts: Vec<Contact>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}protection")]
    pub protections: Vec<Protection>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}labels",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub labels: Vec<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}goodsAndServices")]
    pub goods_and_services: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}refNum")]
    pub reference_number: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}proDate")]
    pub protection_date: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}title")]
    pub title: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}execDate")]
    pub execution_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Protection {
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}cc")]
    pub country_code: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}region",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub region: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}ruling",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub ruling: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Court {
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}id")]
    pub id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}markName")]
    pub mark_name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}holder")]
    pub holders: Vec<Holder>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}contact",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub contacts: Vec<Contact>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}labels",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub labels: Vec<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}goodsAndServices")]
    pub goods_and_services: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}refNum")]
    pub reference_number: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}proDate")]
    pub protection_date: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}cc")]
    pub country_code: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}region",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub region: Vec<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}courtName")]
    pub court_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Holder {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}name",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub name: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}org",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub organisation: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}addr")]
    pub address: Address,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}voice",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub voice: Option<Phone>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}fax",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub fax: Option<Phone>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}email",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email: Option<String>,
    #[serde(rename = "$attr:entitlement")]
    pub entitlement: Entitlement,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Entitlement {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "assignee")]
    Assignee,
    #[serde(rename = "licensee")]
    Licensee,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}name")]
    pub name: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}org",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub organisation: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}addr")]
    pub address: Address,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}voice")]
    pub voice: Phone,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}fax",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub fax: Option<Phone>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}email")]
    pub email: String,
    #[serde(rename = "$attr:type")]
    pub contact_type: ContactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContactType {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "thirdparty")]
    ThirdParty,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}street")]
    pub street: Vec<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}city")]
    pub city: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}sp",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub province: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:mark-1.0}pc",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub postal_code: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:mark-1.0}cc")]
    pub country_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phone {
    #[serde(rename = "$value")]
    pub number: String,
    #[serde(rename = "$attr:x", skip_serializing_if = "Option::is_none", default)]
    pub extension: Option<String>,
}
