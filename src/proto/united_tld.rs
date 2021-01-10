#[derive(Debug, Deserialize)]
pub struct EPPBalance {
    #[serde(rename = "{http://www.unitedtld.com/epp/finance-1.0}balance")]
    pub balance: String,
    #[serde(rename = "{http://www.unitedtld.com/epp/finance-1.0}threshold", default)]
    pub thresholds: Vec<EPPBalanceThreshold>,
}

#[derive(Debug, Deserialize)]
pub struct EPPBalanceThreshold {
    #[serde(rename = "$attr:op", default)]
    pub threshold_type: String,
    #[serde(rename = "$value")]
    pub threshold: String,
}