use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EPPMaintenanceInfo {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}maint:id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}maint:list",
        skip_serializing_if = "Option::is_none"
    )]
    pub list: Option<EPPMaintenanceInfoList>,
}

#[derive(Debug, Serialize)]
pub struct EPPMaintenanceInfoList {}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceInfoData {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}item", default)]
    pub item: Option<EPPMaintenanceItem>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}list", default)]
    pub list: Option<EPPMaintenanceInfoListData>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceInfoListData {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}listItem")]
    pub list: Vec<EPPMaintenanceListItem>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceItem {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}id")]
    pub id: EPPMaintenanceID,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}type", default)]
    pub item_type: Vec<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}pollType",
        default
    )]
    pub poll_type: Option<EPPMaintenancePollType>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}systems")]
    pub systems: EPPMaintenanceSystems,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}environment")]
    pub environment: EPPMaintenanceEnvironment,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}start",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub start: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}start",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub end: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}reason")]
    pub reason: EPPMaintenanceReason,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}detail", default)]
    pub detail: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}description",
        default
    )]
    pub description: Vec<EPPMaintenanceDescription>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}tlds", default)]
    pub tlds: Option<EPPMaintenanceTLDs>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}intervention",
        default
    )]
    pub intervention: Option<EPPMaintenanceIntervention>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}crDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub created_date: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}upDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub update_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceListItem {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}id")]
    pub id: EPPMaintenanceID,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}start",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub start: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}start",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub end: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}crDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub created_date: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}upDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub update_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceID {
    #[serde(rename = "$attr:name", default)]
    pub name: Option<String>,
    #[serde(rename = "$value")]
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceEnvironment {
    #[serde(rename = "$attr:name", default)]
    pub name: Option<String>,
    #[serde(rename = "$attr:type")]
    pub env_type: EPPMaintenanceEnvironmentEnum,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceDescription {
    #[serde(rename = "$attr:type", default)]
    pub description_type: EPPMaintenanceDescriptionType,
    #[serde(rename = "$value")]
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceSystems {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}system")]
    pub systems: Vec<EPPMaintenanceSystem>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceSystem {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}name")]
    pub name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}host", default)]
    pub host: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}impact")]
    pub impact: EPPMaintenanceImpact,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceTLDs {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}tld")]
    pub tlds: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceIntervention {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}connection")]
    pub connection: bool,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}implementation")]
    pub implementation: bool,
}

#[derive(Debug, Deserialize)]
pub enum EPPMaintenancePollType {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "courtesy")]
    Courtesy,
    #[serde(rename = "end")]
    End,
}

#[derive(Debug, Deserialize)]
pub enum EPPMaintenanceEnvironmentEnum {
    #[serde(rename = "production")]
    Production,
    #[serde(rename = "ote")]
    OTE,
    #[serde(rename = "staging")]
    Staging,
    #[serde(rename = "dev")]
    Development,
    #[serde(rename = "custom")]
    Custom,
}

#[derive(Debug, Deserialize)]
pub enum EPPMaintenanceReason {
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "emergency")]
    Emergency,
}

#[derive(Debug, Deserialize)]
pub enum EPPMaintenanceImpact {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, Deserialize)]
pub enum EPPMaintenanceDescriptionType {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "html")]
    HTML,
}

impl Default for EPPMaintenanceDescriptionType {
    fn default() -> Self {
        EPPMaintenanceDescriptionType::Plain
    }
}
