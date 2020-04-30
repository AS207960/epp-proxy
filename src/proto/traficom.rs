use chrono::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub enum EPPContactTraficomRole {
    #[serde(rename = "3")]
    Reseller,
    #[serde(rename = "4")]
    Technical,
    #[serde(rename = "5")]
    Registrant,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EPPContactTraficomType {
    #[serde(rename = "0")]
    PrivatePerson,
    #[serde(rename = "1")]
    Company,
    #[serde(rename = "2")]
    Association,
    #[serde(rename = "3")]
    Institution,
    #[serde(rename = "4")]
    PoliticalParty,
    #[serde(rename = "5")]
    Municipality,
    #[serde(rename = "6")]
    Government,
    #[serde(rename = "7")]
    PublicCommunity,
}

#[derive(Debug, Serialize)]
pub enum EPPDomainDelete {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-ext-1.0}domain-ext:cancel")]
    Cancel {},
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-ext-1.0}domain-ext:schedule")]
    #[allow(dead_code)]
    Schedule {
        #[serde(rename = "{urn:ietf:params:xml:ns:domain-ext-1.0}domain-ext:delDate")]
        delete_date: DateTime<Utc>
    }
}