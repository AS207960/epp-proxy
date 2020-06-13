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
pub struct EPPDataQualityInfo {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}status")]
    pub status: String,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}reason")]
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
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}lockApplied")]
    pub lock_applied: Option<String>,
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
    #[serde(rename = "$attr:moved")]
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
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}caseId")]
    pub case_id: String,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}domainListData")]
    pub domain_list: EPPExpandedDomainListData,
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
