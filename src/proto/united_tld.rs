#[derive(Debug, Deserialize)]
pub struct EPPBalance {
    #[serde(rename = "{http://www.unitedtld.com/epp/finance-1.0}balance")]
    pub balance: String,
    #[serde(
        rename = "{http://www.unitedtld.com/epp/finance-1.0}threshold",
        default
    )]
    pub thresholds: Vec<EPPBalanceThreshold>,
}

#[derive(Debug, Deserialize)]
pub struct EPPBalanceThreshold {
    #[serde(rename = "$attr:op", default)]
    pub threshold_type: String,
    #[serde(rename = "$value")]
    pub threshold: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPChargeCheckData {
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}cd")]
    pub domains: Vec<EPPChargeCheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPChargeCheckDatum {
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}name")]
    pub name: String,
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}set")]
    pub sets: Vec<EPPChargeSet>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPChargeData {
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}charge:set")]
    pub sets: Vec<EPPChargeSet>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPChargeSet {
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}charge:category")]
    pub category: EPPChargeCategory,
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}charge:type")]
    pub set_type: EPPChargeType,
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}charge:amount")]
    pub amount: Vec<EPPChargeAmount>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPChargeCategory {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(
        rename = "$attr:name",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPChargeType {
    #[serde(rename = "$value")]
    pub value: EPPChargeTypes,
    #[serde(
        rename = "$attr:name",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EPPChargeTypes {
    #[serde(rename = "fee")]
    Fee,
    #[serde(rename = "price")]
    Price,
    #[serde(rename = "custom")]
    Custom,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPChargeAmount {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(
        rename = "$attr:name",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub name: Option<String>,
    #[serde(rename = "$attr:command")]
    pub command: EPPChargeCommand,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EPPChargeCommand {
    #[serde(rename = "check")]
    Check,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "renew")]
    Renew,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "custom")]
    Custom,
}
