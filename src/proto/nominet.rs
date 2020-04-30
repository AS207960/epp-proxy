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
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0}contact-nom-ext:trad-name", skip_serializing_if = "Option::is_none")]
    pub trading_name: Option<String>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0}contact-nom-ext:type", default, skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<EPPContactType>,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0}contact-nom-ext:co-no", skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}domainListData", default)]
    pub domains: Option<EPPDataQualityDomainListInfo>,
}

#[derive(Debug, Deserialize)]
pub struct EPPDataQualityDomainListInfo {
    #[serde(rename = "$attr:noDomains")]
    pub num_domains: u32,
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}domainName")]
    pub domains: Vec<String>,
}
