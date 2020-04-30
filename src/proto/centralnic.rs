#[derive(Debug, Deserialize)]
pub struct EPPRegType {
    #[serde(rename = "{urn:ietf:params:xml:ns:regtype-0.1}type")]
    pub reg_type: String
}
