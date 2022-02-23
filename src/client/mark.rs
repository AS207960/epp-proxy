use chrono::prelude::*;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Mark {
    TradeMark(TradeMark),
    TreatyOrStatute(TreatyOrStatute),
    Court(Court),
}

#[derive(Debug)]
pub struct TradeMark {
    pub id: String,
    pub mark_name: String,
    pub holders: Vec<Holder>,
    pub contacts: Vec<Contact>,
    pub jurisdiction: String,
    pub classes: Vec<u32>,
    pub labels: Vec<String>,
    pub goods_and_services: String,
    pub application_id: Option<String>,
    pub application_date: Option<DateTime<Utc>>,
    pub registration_id: String,
    pub registration_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct TreatyOrStatute {
    pub id: String,
    pub mark_name: String,
    pub holders: Vec<Holder>,
    pub contacts: Vec<Contact>,
    pub protections: Vec<Protection>,
    pub labels: Vec<String>,
    pub goods_and_services: String,
    pub reference_number: String,
    pub protection_date: DateTime<Utc>,
    pub title: String,
    pub execution_date: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Protection {
    pub country_code: String,
    pub region: Option<String>,
    pub ruling: Vec<String>,
}

#[derive(Debug)]
pub struct Court {
    pub id: String,
    pub mark_name: String,
    pub holders: Vec<Holder>,
    pub contacts: Vec<Contact>,
    pub labels: Vec<String>,
    pub goods_and_services: String,
    pub reference_number: String,
    pub protection_date: DateTime<Utc>,
    pub country_code: String,
    pub region: Vec<String>,
    pub court_name: String,
}

#[derive(Debug)]
pub struct Holder {
    pub name: Option<String>,
    pub organisation: Option<String>,
    pub address: Address,
    pub voice: Option<super::Phone>,
    pub fax: Option<super::Phone>,
    pub email: Option<String>,
    pub entitlement: Entitlement,
}

#[derive(Debug)]
pub enum Entitlement {
    Owner,
    Assignee,
    Licensee,
}

#[derive(Debug)]
pub struct Contact {
    pub name: String,
    pub organisation: Option<String>,
    pub address: Address,
    pub voice: super::Phone,
    pub fax: Option<super::Phone>,
    pub email: String,
    pub contact_type: ContactType,
}

#[derive(Debug)]
pub enum ContactType {
    Owner,
    Agent,
    ThirdParty,
}

#[derive(Debug)]
pub struct Address {
    pub street_1: Option<String>,
    pub street_2: Option<String>,
    pub street_3: Option<String>,
    pub city: String,
    pub province: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: String,
}
