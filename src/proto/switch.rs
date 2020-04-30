#[derive(Debug, Deserialize)]
pub struct EPPBalance {
    #[serde(rename = "{https://www.nic.ch/epp/balance-1.0}currency")]
    pub balance: String,
    #[serde(rename = "{https://www.nic.ch/epp/balance-1.0}currency")]
    pub currency: String,
}
