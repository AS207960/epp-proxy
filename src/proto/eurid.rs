use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EURIDDomainCheckData {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/domain-ext-2.4}domain")]
    pub domains: Vec<EURIDDomainCheckDatum>
}

#[derive(Debug, Serialize)]
pub struct EURIDDomainCheckDatum {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/domain-ext-2.4}name")]
    pub name: String,
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/domain-ext-2.4}availableDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub available_date: Option<DateTime<Utc>>,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/domain-ext-2.4}status", default)]
    pub status: Vec<super::domain::EPPDomainStatus>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EURIDContactInfo {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:type")]
    pub contact_type: EURIDContactType,
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:whoisEmial",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub whois_email: Option<String>,
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:vat",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub vat: Option<String>,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:lang")]
    pub language: String,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:naturalPerson")]
    pub natural_person: bool,
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:countryOfCitizenship",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub country_of_citizenship: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EURIDContactUpdate {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:chg")]
    pub change: EURIDContactUpdateInfo
}

#[derive(Debug, Serialize)]
pub struct EURIDContactUpdateInfo {
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:type",
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_type: Option<EURIDContactType>,
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:whoisEmial",
        skip_serializing_if = "Option::is_none"
    )]
    pub whois_email: Option<String>,
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:vat",
        skip_serializing_if = "Option::is_none"
    )]
    pub vat: Option<String>,
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:lang",
        skip_serializing_if = "Option::is_none"
    )]
    pub language: Option<String>,
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:naturalPerson",
        skip_serializing_if = "Option::is_none"
    )]
    pub natural_person: Option<bool>,
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:countryOfCitizenship",
        skip_serializing_if = "Option::is_none"
    )]
    pub country_of_citizenship: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EURIDContactType {
    Billing,
    Tech,
    Registrant,
    OnSite,
    Reseller,
}

#[derive(Debug, Serialize)]
pub struct EURIDDNSQualityInfo {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/dnsQuality-2.0}dnsQuality:name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct EURIDDNSQualityInfoData {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/dnsQuality-2.0}name")]
    pub name: String,
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/dnsQuality-2.0}checkTime",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub check_time: Option<DateTime<Utc>>,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/dnsQuality-2.0}score")]
    pub score: String,
}

#[derive(Debug, Serialize)]
pub struct EURIDDNSSECEligibilityInfo {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/dnssecEligibility-1.0}dnssecEligibility:name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct EURIDDNSSECEligibilityInfoData {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/dnssecEligibility-1.0}name")]
    pub name: String,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/dnssecEligibility-1.0}eligible")]
    pub eligible: bool,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/dnssecEligibility-1.0}msg")]
    pub msg: String,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/dnssecEligibility-1.0}code")]
    pub code: u32,
}

#[derive(Debug, Serialize)]
pub struct EURIDHomoglyphData {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/homoglyph-1.0}domain")]
    pub domains: Vec<EURIDHomoglyphDomainData>
}

#[derive(Debug, Deserialize)]
pub struct EURIDHomoglyphDomainData {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/homoglyph-1.0}name")]
    pub unicode: String,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/homoglyph-1.0}blockedBy")]
    pub blocked_by: Vec<String>
}

#[derive(Debug, Serialize)]
pub enum EURIDAuthInfo {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/authInfo-1.1}authInfo:request")]
    Request {},
    #[serde(rename = "{http://www.eurid.eu/xml/epp/authInfo-1.1}authInfo:cancel")]
    Cancel {},
}

#[derive(Debug, Deserialize)]
pub struct EURIDAuthInfoData {
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/authInfo-1.1}validUntil",
        deserialize_with = "super::deserialize_datetime",
        default
    )]
    pub valid_until: DateTime<Utc>
}

#[derive(Debug, Deserialize)]
pub struct EURIDIDNMapping {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/idn-1.0}name")]
    pub names: Vec<EURIDIDNNameMapping>
}

#[derive(Debug, Deserialize)]
pub struct EURIDIDNNameMapping {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/idn-1.0}ace")]
    pub ace: String,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/idn-1.0}unicode")]
    pub unicode: String
}

#[derive(Debug, Deserialize)]
pub struct EURIDRegistrarFinanceInfoData {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrarFinance-1.0}paymentMode")]
    payment_mode: EURIDRegistrarFinancePaymentMode,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrarFinance-1.0}accountBalance")]
    account_balance: String,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrarFinance-1.0}availableAmount")]
    available_amount: Option<String>,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrarFinance-1.0}dueAmount")]
    due_amount: Option<String>,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrarFinance-1.0}overdueAmount")]
    overdue_amount: Option<String>,
}

pub enum  EURIDRegistrarFinancePaymentMode {
    #[serde(rename = "PRE_PAYMENT")]
    PrePayment,
    #[serde(rename = "POST_PAYMENT")]
    PostPayment
}

#[derive(Debug, Deserialize)]
pub struct EURIDRegistrarHitPointsInfoData {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrarHitPoints-1.0}nbrHitPoints")]
    hit_points: u64,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrarHitPoints-1.0}maxNbrHitPoints")]
    max_hit_points: u64,
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/registrarHitPoints-1.0}blockedUntil",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    blocked_until: Option<DateTime<Utc>>
}

#[derive(Debug, Deserialize)]
pub struct EURIDRegistrationLimitInfoData {
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrationLimit-1.1}monthlyRegistrations")]
    monthly_registrations: u64,
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrationLimit-1.1}maxMonthlyRegistrations", default)]
    max_monthly_registrations: Option<u64>,
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/registrationLimit-1.1}limitedUntil",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    limited_until: Option<DateTime<Utc>>
}
