use chrono::prelude::*;

#[derive(Debug, Deserialize)]
pub struct EPPTagListData {
    #[serde(rename = "infData")]
    pub tags: Vec<EPPTagInfoData>,
}

#[derive(Debug, Deserialize)]
pub struct EPPTagInfoData {
    #[serde(rename = "registrar-tag")]
    pub tag: String,
    pub name: String,
    #[serde(rename = "trad-name")]
    pub trading_name: Option<String>,
    pub handshake: String,
}

#[derive(Debug, Serialize)]
pub struct EPPContactInfoSet {
    #[serde(
        rename = "contact-nom-ext:trad-name",
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_name: Option<String>,
    #[serde(
        rename = "contact-nom-ext:type",
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_type: Option<EPPContactTypeVal>,
    #[serde(
        rename = "contact-nom-ext:co-no",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPContactInfo {
    #[serde(rename = "trad-name")]
    pub trading_name: Option<String>,
    #[serde(rename = "type", default)]
    pub contact_type: Option<EPPContactTypeVal>,
    #[serde(rename = "oco-no")]
    pub company_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EPPContactTypeVal {
    #[serde(rename = "$value")]
    pub value: EPPContactType,
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
    #[serde(rename = "field-name")]
    pub field_name: String,
    #[serde(rename = "$value")]
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPDataQualityInfo {
    pub status: String,
    pub reason: Option<String>,
    #[serde(
        rename = "dateCommenced",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub date_commenced: Option<DateTime<Utc>>,
    #[serde(
        rename = "dateToSuspend",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub date_to_suspend: Option<DateTime<Utc>>,
    #[serde(rename = "lockApplied")]
    pub lock_applied: Option<String>,
    #[serde(rename = "domainListData", default)]
    pub domains: Option<EPPDataQualityDomainListInfo>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDataQualityDomainListInfo {
    #[serde(rename = "noDomains")]
    pub num_domains: u32,
    #[serde(rename = "domainName")]
    pub domains: Vec<String>,
}
