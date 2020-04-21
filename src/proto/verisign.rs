#[derive(Debug, Deserialize)]
pub struct EPPBalance {
    pub balance: String,
    #[serde(rename = "creditLimit")]
    pub credit_limit: String,
    #[serde(rename = "availableCredit")]
    pub available_credit: String,
}
