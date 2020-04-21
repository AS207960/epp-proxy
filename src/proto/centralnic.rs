#[derive(Debug, Deserialize)]
pub struct EPPRegType {
    #[serde(rename = "type")]
    pub reg_type: String
}
