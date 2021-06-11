#[derive(Debug, Serialize)]
pub struct EPPAugmentedMark {
    #[serde(rename = "$valueRaw", skip_serializing_if = "Option::is_none")]
    pub signed_mark: Option<String>,
    #[serde(rename = "{http://xmlns.corenic.net/epp/mark-ext-1.0}ext:applicationInfo")]
    pub application_info: Vec<EPPApplicationInfo>,
}

#[derive(Debug, Serialize)]
pub struct EPPApplicationInfo {
    #[serde(rename = "$attr:type", skip_serializing_if = "Option::is_none")]
    pub info_type: Option<String>,
    #[serde(rename = "$value")]
    pub info: String,
}
