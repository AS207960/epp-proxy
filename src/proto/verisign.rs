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
    Percentage(u8)
}
