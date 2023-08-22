use chrono::prelude::*;

#[derive(Debug, Deserialize)]
pub struct EPPTagListData {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-tag-1.0}infData")]
    pub tags: Vec<EPPTagInfoData>,
}

#[derive(Debug, Deserialize)]
pub struct EPPTagInfoData {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-tag-1.0}registrar-tag")]
    pub tag: String,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-tag-1.0}name")]
    pub name: String,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-tag-1.0}trad-name")]
    pub trading_name: Option<String>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-tag-1.0}handshake")]
    pub handshake: String,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainCreate {
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:first-bill",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_bill: Option<EPPDomainBillCode>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:recur-bill",
        skip_serializing_if = "Option::is_none"
    )]
    pub recur_bill: Option<EPPDomainBillCode>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:auto-bill",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_bill: Option<u8>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:next-bill",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_bill: Option<u8>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:auto-period",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_period: Option<u8>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:next-period",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_period: Option<u8>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:notes",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub notes: Vec<String>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:reseller",
        skip_serializing_if = "Option::is_none"
    )]
    pub reseller: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EPPDomainUpdate {
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:first-bill",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_bill: Option<Option<EPPDomainBillCode>>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:recur-bill",
        skip_serializing_if = "Option::is_none"
    )]
    pub recur_bill: Option<Option<EPPDomainBillCode>>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:auto-bill",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_bill: Option<Option<u8>>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:next-bill",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_bill: Option<Option<u8>>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:auto-period",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_period: Option<Option<u8>>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:next-period",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_period: Option<Option<u8>>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:renew-not-required",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_flag_bool"
    )]
    pub renew_not_required: Option<bool>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:notes",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub notes: Vec<String>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}domain-nom-ext:reseller",
        skip_serializing_if = "Option::is_none"
    )]
    pub reseller: Option<Option<String>>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainCheckData {
    #[serde(rename = "$attr:abuse-limit")]
    pub abuse_limit: u64,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainInfoData {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}reg-status")]
    pub reg_status: EPPDomainRegistrationStatus,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}first-bill", default)]
    pub first_bill: Option<EPPDomainBillCode>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}recur-bill", default)]
    pub recur_bill: Option<EPPDomainBillCode>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}auto-bill", default)]
    pub auto_bill: Option<u8>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}next-bill", default)]
    pub next_bill: Option<u8>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}auto-period", default)]
    pub auto_period: Option<u8>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}next-period", default)]
    pub next_period: Option<u8>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}renew-not-required", default)]
    pub renewal_not_required: Option<bool>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}notes", default)]
    pub notes: Vec<String>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/domain-nom-ext-1.2}reseller", default)]
    pub reseller: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EPPDomainBillCode {
    #[serde(rename = "th")]
    Registrar,
    #[serde(rename = "bc")]
    Customer,
}

#[derive(Debug, Deserialize)]
pub enum EPPDomainRegistrationStatus {
    #[serde(rename = "Registered until expiry date.")]
    RegisteredUntilExpiry,
    #[serde(rename = "Renewal required.")]
    RenewalRequired,
    #[serde(rename = "No longer required")]
    NoLongerRequired,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPContactInfo {
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0}contact-nom-ext:trad-name",
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_name: Option<String>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0}contact-nom-ext:type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_type: Option<EPPContactType>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0}contact-nom-ext:co-no",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EPPContactType {
    #[serde(rename = "LTD")]
    UkLimitedCompany,
    #[serde(rename = "PLC")]
    UkPublicLimitedCompany,
    #[serde(rename = "PTNR")]
    UkPartnership,
    #[serde(rename = "STRA")]
    UkSoleTrader,
    #[serde(rename = "LLP")]
    UkLimitedLiabilityPartnership,
    #[serde(rename = "IP")]
    UkIndustrialProvidentRegisteredCompany,
    #[serde(rename = "IND")]
    UkIndividual,
    #[serde(rename = "SCH")]
    UkSchool,
    #[serde(rename = "RCHAR")]
    UkRegisteredCharity,
    #[serde(rename = "GOV")]
    UkGovernmentBody,
    #[serde(rename = "CRC")]
    UkCorporationByRoyalCharter,
    #[serde(rename = "STAT")]
    UkStatutoryBody,
    #[serde(rename = "FIND")]
    NonUkIndividual,
    #[serde(rename = "FCORP")]
    NonUkCompany,
    #[serde(rename = "OTHER")]
    OtherUkEntity,
    #[serde(rename = "FOTHER")]
    OtherNonUkEntity,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct EPPIgnoredField {
    #[serde(rename = "$attr:field-name")]
    pub field_name: String,
    #[serde(rename = "$value")]
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPIgnoredAttribute {
    #[serde(rename = "$attr:attribute-name")]
    pub attribute_name: String,
    #[serde(rename = "$value")]
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostIgnored {
    #[serde(rename = "$attr:host-name")]
    pub host_name: String,
    #[serde(rename = "$value")]
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPTruncatedField {
    #[serde(rename = "$attr:field-name")]
    pub field_name: String,
    #[serde(rename = "$value")]
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPPostalInfoIgnored {
    #[serde(rename = "$attr:type")]
    pub addr_type: super::contact::EPPContactPostalInfoType,
    #[serde(rename = "$value")]
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub enum EPPDataQualityStatus {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "invalid")]
    Invalid,
}

#[derive(Debug, Serialize)]
pub enum EPPDataQualityUpdate {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}validate")]
    Validate {},
}

#[derive(Debug, Deserialize)]
pub struct EPPDataQualityInfo {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}status")]
    pub status: EPPDataQualityStatus,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}reason",
        default
    )]
    pub reason: Option<String>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}dateCommenced",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub date_commenced: Option<DateTime<Utc>>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}dateToSuspend",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub date_to_suspend: Option<DateTime<Utc>>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}lockApplied",
        default
    )]
    pub lock_applied: Option<bool>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}domainListData",
        default
    )]
    pub domains: Option<EPPDataQualityDomainListInfo>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDataQualityDomainListInfo {
    #[serde(rename = "$attr:noDomains")]
    pub num_domains: u32,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}domainName")]
    pub domains: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPCancelData {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}domainName")]
    pub domain_name: String,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}orig")]
    pub originator: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPReleaseData {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}accountId")]
    pub account: EPPReleaseAccountData,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}from")]
    pub from: String,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}registrarTag")]
    pub registrar_tag: String,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}domainListData")]
    pub domain_list: EPPDomainListData,
}

#[derive(Debug, Deserialize)]
pub struct EPPReleaseAccountData {
    #[serde(rename = "$attr:moved", default)]
    pub moved: bool,
    #[serde(rename = "$value")]
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainListData {
    #[serde(rename = "$attr:noDomains")]
    pub count: u32,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}domainName")]
    pub domain_names: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPExpandedDomainListData {
    #[serde(rename = "$attr:noDomains")]
    pub count: u32,
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}infData")]
    pub domains: Vec<super::domain::EPPDomainInfoData>,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostListData {
    #[serde(rename = "$attr:noHosts")]
    pub count: u32,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}hostObj")]
    pub host_objects: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPRegistrarChangeData {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}orig")]
    pub originator: String,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}registrarTag")]
    pub registrar_tag: String,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}caseId",
        default
    )]
    pub case_id: Option<String>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}domainListData",
        default
    )]
    pub domain_list: Option<EPPExpandedDomainListData>,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}infData")]
    pub contact: super::contact::EPPContactInfoData,
}

#[derive(Debug, Deserialize)]
pub struct EPPHostCancelData {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}hostListData")]
    pub host_list: EPPHostListData,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}domainListData")]
    pub domain_list: EPPDomainListData,
}

#[derive(Debug, Deserialize)]
pub struct EPPProcessData {
    #[serde(rename = "$attr:stage")]
    pub stage: EPPProcessStage,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}infData")]
    pub contact: super::contact::EPPContactInfoData,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}processType")]
    pub process_type: String,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}suspendDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub suspend_date: Option<DateTime<Utc>>,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}cancelDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub cancel_date: Option<DateTime<Utc>>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}domainListData")]
    pub domain_list: EPPDomainListData,
}

#[derive(Debug, Deserialize)]
pub enum EPPProcessStage {
    #[serde(rename = "initial")]
    Initial,
    #[serde(rename = "updated")]
    Updated,
}

#[derive(Debug, Deserialize)]
pub struct EPPSuspendData {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}reason")]
    pub reason: String,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}cancelDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub cancel_date: Option<DateTime<Utc>>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}domainListData")]
    pub domain_list: EPPDomainListData,
}

#[derive(Debug, Deserialize)]
pub struct EPPDomainFailData {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}reason")]
    pub reason: String,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}domainName")]
    pub domain_name: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPTransferData {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}orig")]
    pub originator: String,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}accountId")]
    pub account_id: String,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}oldAccountId")]
    pub old_account_id: String,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}caseId",
        default
    )]
    pub case_id: Option<String>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}domainListData")]
    pub domain_list: EPPDomainListData,
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}infData")]
    pub contact: super::contact::EPPContactInfoData,
}

#[derive(Debug, Serialize)]
pub struct EPPHandshakeAccept {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-handshake-1.0}handshake:caseId")]
    pub case_id: String,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/std-handshake-1.0}handshake:registrant",
        skip_serializing_if = "Option::is_none"
    )]
    pub registrant: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EPPHandshakeReject {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-handshake-1.0}handshake:caseId")]
    pub case_id: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPHandshakeData {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-handshake-1.0}caseId")]
    pub case_id: String,
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/std-handshake-1.0}domainListData",
        default
    )]
    pub domain_list: Option<EPPHandshakeDomainListData>,
}

#[derive(Debug, Deserialize)]
pub struct EPPHandshakeDomainListData {
    #[serde(rename = "$attr:noDomains")]
    pub count: u32,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-handshake-1.0}domainName")]
    pub domain_names: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct EPPRelease {
    #[serde(rename = "$value")]
    pub object: EPPReleaseObject,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-release-1.0}release:registrarTag")]
    pub registrar_tag: String,
}

#[derive(Debug, Serialize)]
pub enum EPPReleaseObject {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-release-1.0}release:domainName")]
    Domain(String),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-release-1.0}release:registrant")]
    Registrant(String),
}

#[derive(Debug, Serialize)]
pub struct EPPLock {
    #[serde(rename = "$attr:type")]
    pub lock_type: String,
    #[serde(rename = "$attr:object")]
    pub object_type: EPPLockObjectType,
    #[serde(rename = "$value")]
    pub object: EPPLockObject,
}

#[derive(Debug, Serialize)]
pub enum EPPLockObject {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-locks-1.0}lock:domainName")]
    Domain(String),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-locks-1.0}lock:contactId")]
    Registrant(String),
}

#[derive(Debug, Serialize)]
pub enum EPPLockObjectType {
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "contact")]
    Registrant,
}

#[derive(Debug, Serialize)]
pub struct EPPUnrenew {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-unrenew-1.0}unrenew:domainName")]
    pub domains: Vec<String>,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_flag_bool<S>(d: &Option<bool>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
{
    match d {
        Some(d) => {
            if *d {
                s.serialize_str("Y")
            } else {
                s.serialize_str("N")
            }
        }
        None => s.serialize_none(),
    }
}