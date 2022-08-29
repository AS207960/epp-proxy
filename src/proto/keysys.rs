use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub enum Check {
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:domain")]
    Domain(DomainCheck),
}

#[derive(Debug, Serialize)]
pub enum Create {
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:contact")]
    Contact(ContactCreate),
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:domain")]
    Domain(DomainCreate),
}

#[derive(Debug, Serialize)]
pub enum Update {
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:contact")]
    Contact(ContactUpdate),
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:domain")]
    Domain(DomainUpdate),
}

#[derive(Debug, Serialize)]
pub enum Delete {
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:domain")]
    Domain(DomainDelete),
}

#[derive(Debug, Serialize)]
pub enum Renew {
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:domain")]
    Domain(DomainRenew),
}

#[derive(Debug, Serialize)]
pub enum Transfer {
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:transfer")]
    Domain(DomainTransfer),
}

#[derive(Debug, Deserialize)]
pub enum ResultData {
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}contactInfData",
        alias = "{http://www.key-systems.net/epp/keysys-1.0}creData"
    )]
    Contact(ContactInfoData),
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}infData")]
    Domain(DomainInfoData),
}

#[derive(Debug, Deserialize)]
pub struct Poll {
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}data", default)]
    data: Option<PollData>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}info", default)]
    info: Option<String>
}

#[derive(Debug, Serialize)]
pub struct ContactCreate {
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:checkonly",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub checkonly: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:forceDuplication",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub force_duplication: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:preverify",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub pre_verify: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ContactUpdate {
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:checkonly",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub checkonly: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:preverify",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub pre_verify: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:triggerfoa",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub trigger_foa: Option<bool>
}

#[derive(Debug, Deserialize)]
pub struct ContactInfoData {
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}validated", default)]
    pub validated: bool,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}verification-requested", default)]
    pub verification_requested: bool,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}verified", default)]
    pub verified: bool,
}

#[derive(Debug, Serialize)]
pub struct DomainCheck {
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:allocation-token",
        skip_serializing_if = "Option::is_none",
    )]
    pub allocation_token: Option<String>
}

#[derive(Debug, Serialize)]
pub struct DomainCreate {
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:accept-premiumprice",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub accept_premium_price: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:accept-ssl-requirement",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub accept_ssl_requirement: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:allocation-token",
        skip_serializing_if = "Option::is_none"
    )]
    pub allocation_token: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:ca-legal-type",
        skip_serializing_if = "Option::is_none"
    )]
    pub ca_legal_type: Option<CALegalType>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:ca-trademark",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub ca_trademark: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:eu-accept-trustee-tac",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub eu_accept_trustee_tac: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:eu-registrant-lang",
        skip_serializing_if = "Option::is_none"
    )]
    pub eu_registrant_lang: Option<EULanguage>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:eu-registrant-citizenship",
        skip_serializing_if = "Option::is_none"
    )]
    pub eu_registrant_citizenship: Option<EUCountry>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:de-abuse-contact",
        skip_serializing_if = "Option::is_none"
    )]
    pub de_abuse_contact: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:de-accept-trustee-tac",
        skip_serializing_if = "Option::is_none"
    )]
    pub de_accept_trustee_tac: Option<DETrustee>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:de-general-request",
        skip_serializing_if = "Option::is_none"
    )]
    pub de_general_request: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:de-holder-person",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub de_holder_person: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:fr-accept-trustee-tac",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub fr_accept_trustee_tac: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:gay-accept-requirements",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub gay_accept_requirements: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:intended-use",
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_use: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:name-emailforward",
        skip_serializing_if = "Option::is_none"
    )]
    pub name_emailforward: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-owner-idcard",
        skip_serializing_if = "Option::is_none"
    )]
    pub rs_owner_idcard: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-owner-company-number",
        skip_serializing_if = "Option::is_none"
    )]
    pub rs_owner_company_number: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-admin-idcard",
        skip_serializing_if = "Option::is_none"
    )]
    pub rs_admin_idcard: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-admin-company-number",
        skip_serializing_if = "Option::is_none"
    )]
    pub rs_admin_company_number: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-tech-idcard",
        skip_serializing_if = "Option::is_none"
    )]
    pub rs_tech_idcard: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-tech-company-number",
        skip_serializing_if = "Option::is_none"
    )]
    pub rs_tech_company_number: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:us-nexus-apppurpose",
        skip_serializing_if = "Option::is_none"
    )]
    pub us_purpose: Option<USPurpose>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:us-nexus-category",
        skip_serializing_if = "Option::is_none"
    )]
    pub us_category: Option<USCategory>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:us-nexus-validator",
        skip_serializing_if = "Option::is_none"
    )]
    pub us_validator: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}renewalmode",
        skip_serializing_if = "Option::is_none"
    )]
    pub renewal_mode: Option<RenewalMode>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}transfermode",
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_mode: Option<TransferMode>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}whois-banner0",
        skip_serializing_if = "Option::is_none"
    )]
    pub whois_banner_0: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}whois-banner1",
        skip_serializing_if = "Option::is_none"
    )]
    pub whois_banner_1: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}whois-rsp",
        skip_serializing_if = "Option::is_none"
    )]
    pub whois_rsp: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}whois-url",
        skip_serializing_if = "Option::is_none"
    )]
    pub whois_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DomainUpdate {
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:ca-legal-type",
        skip_serializing_if = "Option::is_none"
    )]
    pub ca_legal_type: Option<CALegalType>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:ca-trademark",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub ca_trademark: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:eu-accept-trustee-tac",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub eu_accept_trustee_tac: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:eu-registrant-lang",
        skip_serializing_if = "Option::is_none"
    )]
    pub eu_registrant_lang: Option<EULanguage>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:eu-registrant-citizenship",
        skip_serializing_if = "Option::is_none"
    )]
    pub eu_registrant_citizenship: Option<EUCountry>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:de-abuse-contact",
        skip_serializing_if = "Option::is_none"
    )]
    pub de_abuse_contact: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:de-accept-trustee-tac",
        skip_serializing_if = "Option::is_none"
    )]
    pub de_accept_trustee_tac: Option<DETrustee>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:de-general-request",
        skip_serializing_if = "Option::is_none"
    )]
    pub de_general_request: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:de-holder-person",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub de_holder_person: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:fr-accept-trustee-tac",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub fr_accept_trustee_tac: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:name-emailforward",
        skip_serializing_if = "Option::is_none"
    )]
    pub name_emailforward: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-owner-idcard",
        skip_serializing_if = "Option::is_none"
    )]
    pub rs_owner_idcard: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-owner-company-number",
        skip_serializing_if = "Option::is_none"
    )]
    pub rs_owner_company_number: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-admin-idcard",
        skip_serializing_if = "Option::is_none"
    )]
    pub rs_admin_idcard: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-admin-company-number",
        skip_serializing_if = "Option::is_none"
    )]
    pub rs_admin_company_number: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-tech-idcard",
        skip_serializing_if = "Option::is_none"
    )]
    pub rs_tech_idcard: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-tech-company-number",
        skip_serializing_if = "Option::is_none"
    )]
    pub rs_tech_company_number: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:us-nexus-apppurpose",
        skip_serializing_if = "Option::is_none"
    )]
    pub us_purpose: Option<USPurpose>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:us-nexus-category",
        skip_serializing_if = "Option::is_none"
    )]
    pub us_category: Option<USCategory>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:us-nexus-validator",
        skip_serializing_if = "Option::is_none"
    )]
    pub us_validator: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:renewalmode",
        skip_serializing_if = "Option::is_none"
    )]
    pub renewal_mode: Option<RenewalMode>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:transfermode",
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_mode: Option<TransferMode>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:whois-banner0",
        skip_serializing_if = "Option::is_none"
    )]
    pub whois_banner_0: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}kesys:whois-banner1",
        skip_serializing_if = "Option::is_none"
    )]
    pub whois_banner_1: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:whois-rsp",
        skip_serializing_if = "Option::is_none"
    )]
    pub whois_rsp: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:whois-url",
        skip_serializing_if = "Option::is_none"
    )]
    pub whois_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DomainInfoData {
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}renDate")]
    pub renewal_date: DateTime<Utc>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}punDate")]
    pub paid_until_date: DateTime<Utc>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}domain-roid", default)]
    pub domain_roid: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:ca-legal-type", default)]
    pub ca_legal_type: Option<CALegalType>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:ca-trademark", default)]
    pub ca_trademark: bool,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:eu-accept-trustee-tac", default)]
    pub eu_accept_trustee_tac: bool,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:eu-registrant-lang", default)]
    pub eu_registrant_lang: Option<EULanguage>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:eu-registrant-citizenship", default)]
    pub eu_registrant_citizenship: Option<EUCountry>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:de-abuse-contact", default)]
    pub de_abuse_contact: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:de-accept-trustee-tac", default)]
    pub de_accept_trustee_tac: Option<DETrustee>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:de-general-request", default)]
    pub de_general_request: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:de-holder-person", default)]
    pub de_holder_person: bool,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:fr-accept-trustee-tac", default)]
    pub fr_accept_trustee_tac: bool,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}name-emailforward", default)]
    pub name_emailforward: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-owner-idcard", default)]
    pub rs_owner_idcard: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-owner-company-number", default)]
    pub rs_owner_company_number: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-admin-idcard", default)]
    pub rs_admin_idcard: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-admin-company-number", default)]
    pub rs_admin_company_number: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-tech-idcard", default)]
    pub rs_tech_idcard: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:rs-tech-company-number", default)]
    pub rs_tech_company_number: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:us-nexus-apppurpose", default)]
    pub us_purpose: Option<USPurpose>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:us-nexus-category", default)]
    pub us_category: Option<USCategory>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:us-nexus-validator", default)]
    pub us_validator: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}renewalmode", default)]
    pub renewal_mode: RenewalMode,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}transferlock", default)]
    pub transfer_lock: bool,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}transfermode", default)]
    pub transfer_mode: TransferMode,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}whois-banner0", default)]
    pub whois_banner_0: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}whois-banner1", default)]
    pub whois_banner_1: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}whois-rsp", default)]
    pub whois_rsp: Option<String>,
    #[serde(rename = "{http://www.key-systems.net/epp/keysys-1.0}whois-url", default)]
    pub whois_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PollData {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}domain", default)]
    pub domain: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}contact", default)]
    pub contact: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}period", default)]
    pub period: Option<u32>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}autorenew", default)]
    pub auto_renew: bool,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}autodelete", default)]
    pub auto_delete: bool,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}svtrid", default)]
    pub server_transaction_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DomainRenew {
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:accept-premiumprice",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub accept_premium_price: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:promotion-code",
        skip_serializing_if = "Option::is_none",
    )]
    pub promotion_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DomainTransfer {
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:accept-premiumprice",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub accept_premium_price: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:accept-quarantine",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub accept_quarantine: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:accept-trade",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub accept_trade: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:allocation-token",
        skip_serializing_if = "Option::is_none"
    )]
    pub allocation_token: Option<String>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:at-requestauthcode",
        skip_serializing_if = "Option::is_none",
    serialize_with = "super::serialize_opt_bool"
    )]
    pub at_request_authcode: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:be-requestauthcode",
        skip_serializing_if = "Option::is_none",
    serialize_with = "super::serialize_opt_bool"
    )]
    pub be_request_authcode: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:eu-accept-trustee-tac",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_opt_bool"
    )]
    pub eu_accept_trustee_tac: Option<bool>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:eu-registrant-lang",
        skip_serializing_if = "Option::is_none"
    )]
    pub eu_registrant_lang: Option<EULanguage>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:eu-registrant-citizenship",
        skip_serializing_if = "Option::is_none"
    )]
    pub eu_registrant_citizenship: Option<EUCountry>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:promotion-code",
        skip_serializing_if = "Option::is_none",
    )]
    pub promotion_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DomainDelete {
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:action",
        skip_serializing_if = "Option::is_none"
    )]
    pub action: Option<DomainDeleteAction>,
    #[serde(
        rename = "{http://www.key-systems.net/epp/keysys-1.0}keysys:target",
        skip_serializing_if = "Option::is_none"
    )]
    pub target: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum DomainDeleteAction {
    #[serde(rename="INSTANT")]
    Instant,
    #[serde(rename="AUTODELETE")]
    AutoDelete,
    #[serde(rename="AUTOEXPIRE")]
    AutoExpire,
    #[serde(rename="PUSH")]
    Push
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RenewalMode {
    #[serde(rename="DEFAULT")]
    Default,
    #[serde(rename="AUTORENEW")]
    AutoRenew,
    #[serde(rename="RENEWONCE")]
    RenewOnce,
    #[serde(rename="AUTODELETE")]
    AutoDelete,
    #[serde(rename="AUTOEXPIRE")]
    AutoExpire,
    #[serde(rename="AUTORENEWMONTHLY")]
    AutoRenewMonthly,
    #[serde(rename="AUTORENEWQUARTERLY")]
    AutoRenewQuarterly,
    #[serde(rename="EXPIREAUCTION")]
    ExpireAuction
}

impl Default for RenewalMode {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransferMode {
    #[serde(rename="DEFAULT")]
    Default,
    #[serde(rename="AUTOAPPROVE")]
    AutoApprove,
    #[serde(rename="AUTODENY")]
    AutoDeny,
}

impl Default for TransferMode {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DETrustee {
    #[serde(rename="0")]
    None,
    #[serde(rename="1")]
    Monthly,
    #[serde(rename="2")]
    Annual
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EULanguage {
    #[serde(rename="bg")]
    Bulgarian,
    #[serde(rename="cs")]
    Czech,
    #[serde(rename="da")]
    Danish,
    #[serde(rename="de")]
    German,
    #[serde(rename="el")]
    ModernGreek,
    #[serde(rename="en")]
    English,
    #[serde(rename="es")]
    Spanish,
    #[serde(rename="et")]
    Estonian,
    #[serde(rename="fi")]
    Finnish,
    #[serde(rename="fr")]
    French,
    #[serde(rename="ga")]
    Gaelic,
    #[serde(rename="hr")]
    Croatian,
    #[serde(rename="hu")]
    Hungarian,
    #[serde(rename="it")]
    Italian,
    #[serde(rename="lt")]
    Lithuanian,
    #[serde(rename="lv")]
    Latvian,
    #[serde(rename="mt")]
    Maltese,
    #[serde(rename="nl")]
    DutchFlemish,
    #[serde(rename="pl")]
    Polish,
    #[serde(rename="pt")]
    Portuguese,
    #[serde(rename="ro")]
    Romanian,
    #[serde(rename="sk")]
    Slovak,
    #[serde(rename="sl")]
    Slovene,
    #[serde(rename="sv")]
    Swedish,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EUCountry {
    #[serde(rename="at")]
    Austria,
    #[serde(rename="be")]
    Belgium,
    #[serde(rename="bg")]
    Bulgaria,
    #[serde(rename="cz")]
    Czech,
    #[serde(rename="cy")]
    Cyprus,
    #[serde(rename="de")]
    Germany,
    #[serde(rename="dk")]
    Denmark,
    #[serde(rename="es")]
    Spain,
    #[serde(rename="ee")]
    Estonia,
    #[serde(rename="fi")]
    Finland,
    #[serde(rename="fr")]
    France,
    #[serde(rename="gr")]
    Greece,
    #[serde(rename="hu")]
    Hungary,
    #[serde(rename="ie")]
    Ireland,
    #[serde(rename="it")]
    Italy,
    #[serde(rename="li")]
    Liechtenstein,
    #[serde(rename="lt")]
    Lithuania,
    #[serde(rename="lu")]
    Luxembourg,
    #[serde(rename="lv")]
    Latvia,
    #[serde(rename="mt")]
    Malta,
    #[serde(rename="nl")]
    Netherlands,
    #[serde(rename="pl")]
    Poland,
    #[serde(rename="pt")]
    Portugal,
    #[serde(rename="ro")]
    Romania,
    #[serde(rename="se")]
    Sweden,
    #[serde(rename="sk")]
    Slovakia,
    #[serde(rename="si")]
    Slovenia,
    #[serde(rename="hr")]
    Croatia
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CALegalType {
    #[serde(rename="ABO")]
    AboriginalPeoples,
    #[serde(rename="ASS")]
    CanadianUnincorporatedAssociation,
    #[serde(rename="CCO")]
    Corporation,
    #[serde(rename="CCT")]
    Citizen,
    #[serde(rename="EDU")]
    CanadianEducationalInstitution,
    #[serde(rename="GOV")]
    Government,
    #[serde(rename="HOP")]
    CanadianHospital,
    #[serde(rename="INB")]
    IndianBand,
    #[serde(rename="LAM")]
    CanadianLibraryArchiveMuseum,
    #[serde(rename="LGR")]
    LegalRepOfCanadianCitizenOrPermanentResident,
    #[serde(rename="HMQ")]
    TheQueen,
    #[serde(rename="OMK")]
    OfficialMark,
    #[serde(rename="PLT")]
    CanadianPoliticalParty,
    #[serde(rename="PRT")]
    Partnership,
    #[serde(rename="RES")]
    PermanentResident,
    #[serde(rename="TDM")]
    TradeMark,
    #[serde(rename="TRD")]
    TradeUnion,
    #[serde(rename="TRS")]
    Trust
}

#[derive(Debug, Serialize, Deserialize)]
pub enum USPurpose {
    #[serde(rename = "P1")]
    Business,
    #[serde(rename = "P2")]
    NonProfit,
    #[serde(rename = "P3")]
    Personal,
    #[serde(rename = "P4")]
    Educational,
    #[serde(rename = "P5")]
    Government,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum USCategory {
    #[serde(rename = "C11")]
    Citizen,
    #[serde(rename = "C12")]
    PermanentResident,
    #[serde(rename = "C21")]
    USOrganisation,
    #[serde(rename = "C31")]
    RegularActivity,
    #[serde(rename = "C32")]
    OfficeOrFacility,
}