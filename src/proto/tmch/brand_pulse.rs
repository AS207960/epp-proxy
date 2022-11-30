use chrono::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}status")]
    pub status: super::TMCHStatus<StatusType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StatusType {
    #[serde(rename = "brandPulseEnabled")]
    Enabled,
    #[serde(rename = "brandPulseDisabled")]
    Disabled,
}

#[derive(Debug, Serialize)]
pub struct Renew {
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}curExpYear")]
    pub current_expiry_year: i32,
}

#[derive(Debug, Serialize)]
pub struct Update {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}status",
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<super::TMCHStatus<StatusType>>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}whitelist",
        skip_serializing_if = "Option::is_none"
    )]
    pub whitelist: Option<UpdateWhitelist>,
}

#[derive(Debug, Serialize)]
pub struct UpdateWhitelist {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}add",
        skip_serializing_if = "Option::is_none"
    )]
    pub add: Option<WhitelistData>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}rem",
        skip_serializing_if = "Option::is_none"
    )]
    pub remove: Option<WhitelistData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhitelistData {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}dn",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub domains: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}match", default)]
    pub matches: Vec<Match>,
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}whitelist", default)]
    pub whitelist: Option<WhitelistData>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}expiry",
        deserialize_with = "super::super::deserialize_date_opt",
        default
    )]
    pub expiry: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct Match {
    #[serde(rename = "$attr:label")]
    pub label: String,
    #[serde(rename = "$attr:tld")]
    pub tld: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}set")]
    pub set: Set,
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}rank")]
    pub rank: Rank,
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}service")]
    pub service: Service,
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}whois")]
    pub whois: Whois,
}

#[derive(Debug, Deserialize)]
pub struct Set {
    #[serde(rename = "$attr:matchset")]
    pub match_set: String,
    #[serde(rename = "$attr:tldset")]
    pub tld_set: String,
}

#[derive(Debug, Deserialize)]
pub struct Rank {
    #[serde(rename = "$value")]
    pub rank: u32,
    #[serde(rename = "$attr:bpMap")]
    pub bp_map: u32,
    #[serde(rename = "$attr:bpUse")]
    pub bp_use: u32,
    #[serde(rename = "$attr:score")]
    pub score: u32,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}aRecord")]
    pub a_record: bool,
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}webSvc")]
    pub web_service: bool,
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}mailSvc")]
    pub mail_service: bool,
}

#[derive(Debug, Deserialize)]
pub struct Whois {
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}registrar")]
    pub registrar: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}registrant")]
    pub registrant: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}contact")]
    pub contact: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}email")]
    pub email: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}creationTime",
        default
    )]
    pub creation: Option<DateTime<Utc>>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:brandPulse-1.0}modificationTime",
        default
    )]
    pub modification: Option<DateTime<Utc>>,
}
