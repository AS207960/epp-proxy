use chrono::prelude::*;

#[derive(Debug, Deserialize)]
pub struct EPPBalance {
    #[serde(rename = "{http://www.verisign.com/epp/balance-1.0}balance")]
    pub balance: String,
    #[serde(rename = "{http://www.verisign.com/epp/balance-1.0}creditLimit")]
    pub credit_limit: String,
    #[serde(rename = "{http://www.verisign.com/epp/balance-1.0}availableCredit")]
    pub available_credit: String,
    #[serde(rename = "{http://www.verisign.com/epp/balance-1.0}creditThreshold")]
    pub credit_threshold: EPPCreditThreshold,
}

#[derive(Debug, Deserialize)]
pub enum EPPCreditThreshold {
    #[serde(rename = "{http://www.verisign.com/epp/balance-1.0}fixed")]
    Fixed(String),
    #[serde(rename = "{http://www.verisign.com/epp/balance-1.0}percent")]
    Percentage(u8),
}

#[derive(Debug, Serialize)]
pub struct EPPNameStoreExt {
    #[serde(rename = "{http://www.verisign-grs.com/epp/namestoreExt-1.1}namestoreExt:subProduct")]
    pub sub_product: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPRGPPollData {
    #[serde(rename = "{http://www.verisign.com/epp/rgp-poll-1.0}name")]
    pub name: String,
    #[serde(rename = "{http://www.verisign.com/epp/rgp-poll-1.0}rgpStatus")]
    pub status: super::rgp::EPPRGPStatus,
    #[serde(
        rename = "{http://www.verisign.com/epp/rgp-poll-1.0}reqDate",
        serialize_with = "super::serialize_datetime"
    )]
    pub request_date: DateTime<Utc>,
    #[serde(
        rename = "{http://www.verisign.com/epp/rgp-poll-1.0}reportDueDate",
        serialize_with = "super::serialize_datetime"
    )]
    pub report_due_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct EPPLowBalanceData {
    #[serde(rename = "{http://www.verisign.com/epp/lowbalance-poll-1.0}registrarName")]
    pub registrar_name: String,
    #[serde(rename = "{http://www.verisign.com/epp/lowbalance-poll-1.0}creditLimit")]
    pub credit_limit: String,
    #[serde(rename = "{http://www.verisign.com/epp/lowbalance-poll-1.0}creditThreshold")]
    pub credit_threshold: EPPLowCreditThreshold,
    #[serde(rename = "{http://www.verisign.com/epp/lowbalance-poll-1.0}availableCredit")]
    pub available_credit: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPLowCreditThreshold {
    #[serde(rename = "$attr:type")]
    pub credit_type: EPPLowCreditThresholdType,
    #[serde(rename = "$value")]
    pub threshold: String,
}

#[derive(Debug, Deserialize)]
pub enum EPPLowCreditThresholdType {
    #[serde(rename = "FIXED")]
    Fixed,
    #[serde(rename = "PERCENT")]
    Percentage,
}

#[derive(Debug, Serialize)]
pub struct EPPWhoisInfoExt {
    #[serde(rename = "{http://www.verisign.com/epp/whoisInf-1.0}whoisInf:flag")]
    pub flag: bool,
}

#[derive(Debug, Deserialize)]
pub struct EPPWhoisInfoExtData {
    #[serde(rename = "{http://www.verisign.com/epp/whoisInf-1.0}registrar")]
    pub registrar: String,
    #[serde(
        rename = "{http://www.verisign.com/epp/whoisInf-1.0}whoisServer",
        default
    )]
    pub whois_server: Option<String>,
    #[serde(rename = "{http://www.verisign.com/epp/whoisInf-1.0}url", default)]
    pub url: Option<String>,
    #[serde(
        rename = "{http://www.verisign.com/epp/whoisInf-1.0}irisServer",
        default
    )]
    pub iris_server: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EPPSyncUpdate {
    #[serde(
        rename = "{http://www.verisign.com/epp/sync-1.0}sync:expMonthDay",
        serialize_with = "serialize_month_day"
    )]
    pub month_day: EPPSyncUpdateMonthDay,
}

#[derive(Debug)]
pub struct EPPSyncUpdateMonthDay {
    pub month: u32,
    pub day: u32,
}

fn serialize_month_day<S>(d: &EPPSyncUpdateMonthDay, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    s.serialize_str(&format!(
        "--{:0>2}-{:0>2}",
        std::cmp::min(12, std::cmp::max(1, d.month)),
        std::cmp::min(31, std::cmp::max(1, d.day))
    ))
}
