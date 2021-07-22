#[derive(Debug, Serialize, Deserialize)]
pub struct Variation {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}label",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub labels: Vec<Label>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub id: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}aLabel")]
    pub a_label: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}uLabel",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub u_label: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}type",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub variation_type: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}active",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub active: Option<super::TMCHNotify>,
}