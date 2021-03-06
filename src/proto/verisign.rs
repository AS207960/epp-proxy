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
