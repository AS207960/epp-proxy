use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub enum EPPMaintenanceInfo {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}maint:id")]
    Id(String),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}maint:list")]
    List(EPPMaintenanceInfoList)
}

#[derive(Debug, Serialize)]
pub enum EPPMaintenanceInfo02 {
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}maint:id")]
    Id(String),
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}maint:list")]
    List(EPPMaintenanceInfoList)
}

#[derive(Debug, Serialize)]
pub struct EPPMaintenanceInfoList {}

#[derive(Debug, Deserialize)]
pub enum EPPMaintenanceInfoData {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}list")]
    List(EPPMaintenanceInfoListData),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}maint")]
    Maintenance(EPPMaintenanceItem),
}

#[derive(Debug, Deserialize)]
pub enum EPPMaintenanceInfoData02 {
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}list")]
    List(EPPMaintenanceInfoListData02),
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}maint")]
    Maintenance(EPPMaintenanceItem02),
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceInfoListData {
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}listItem")]
    pub list: Vec<EPPMaintenanceListItem>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceInfoListData02 {
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}maint")]
    pub list: Vec<EPPMaintenanceListItem02>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceItem {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}id")]
    pub id: EPPMaintenanceID,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}type", default)]
    pub item_type: Vec<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}pollType",
        default
    )]
    pub poll_type: Option<EPPMaintenancePollType>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}systems")]
    pub systems: EPPMaintenanceSystems,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}environment")]
    pub environment: EPPMaintenanceEnvironment,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}start",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub start: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}end",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub end: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}reason")]
    pub reason: EPPMaintenanceReason,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}detail", default)]
    pub detail: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}description",
        default
    )]
    pub description: Vec<EPPMaintenanceDescription>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}tlds", default)]
    pub tlds: Option<EPPMaintenanceTLDs>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}intervention",
        default
    )]
    pub intervention: Option<EPPMaintenanceIntervention>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}crDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub created_date: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}upDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub update_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceItem02 {
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}id")]
    pub id: EPPMaintenanceID,
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}type", default)]
    pub item_type: Vec<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:maintenance-0.2}pollType",
        default
    )]
    pub poll_type: Option<EPPMaintenancePollType>,
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}systems")]
    pub systems: EPPMaintenanceSystems02,
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}environment")]
    pub environment: EPPMaintenanceEnvironment,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:maintenance-0.2}start",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub start: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:maintenance-0.2}end",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub end: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}reason")]
    pub reason: EPPMaintenanceReason,
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}detail", default)]
    pub detail: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:maintenance-0.2}description",
        default
    )]
    pub description: Vec<EPPMaintenanceDescription>,
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}tlds", default)]
    pub tlds: Option<EPPMaintenanceTLDs02>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:maintenance-0.2}intervention",
        default
    )]
    pub intervention: Option<EPPMaintenanceIntervention02>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:maintenance-0.2}crDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub created_date: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:maintenance-0.2}upDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub update_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceListItem {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}id")]
    pub id: EPPMaintenanceID,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}start",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub start: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}start",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub end: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}crDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub created_date: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}upDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub update_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceListItem02 {
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}id")]
    pub id: EPPMaintenanceID,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:maintenance-0.2}start",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub start: Option<DateTime<Utc>>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:maintenance-0.2}start",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub end: Option<DateTime<Utc>>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:maintenance-0.2}crDate",
        deserialize_with = "super::deserialize_datetime"
    )]
    pub created_date: DateTime<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:maintenance-0.2}upDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub update_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceID {
    #[serde(rename = "$attr:name", default)]
    pub name: Option<String>,
    #[serde(rename = "$attr:lang", default)]
    pub lang: Option<String>,
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
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}system")]
    pub systems: Vec<EPPMaintenanceSystem>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceSystems02 {
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}system")]
    pub systems: Vec<EPPMaintenanceSystem02>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceSystem {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}name")]
    pub name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}host", default)]
    pub host: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}impact")]
    pub impact: EPPMaintenanceImpact,
}


#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceSystem02 {
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}name")]
    pub name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}host", default)]
    pub host: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}impact")]
    pub impact: EPPMaintenanceImpact,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceTLDs {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}tld")]
    pub tlds: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceTLDs02 {
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}tld")]
    pub tlds: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceIntervention {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}connection")]
    pub connection: bool,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-1.0}implementation")]
    pub implementation: bool,
}

#[derive(Debug, Deserialize)]
pub struct EPPMaintenanceIntervention02 {
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}connection")]
    pub connection: bool,
    #[serde(rename = "{urn:ietf:params:xml:ns:maintenance-0.2}implementation")]
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
    Ote,
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
    #[serde(rename = "full", alias = "blackout")]
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
    Html,
}

impl Default for EPPMaintenanceDescriptionType {
    fn default() -> Self {
        EPPMaintenanceDescriptionType::Plain
    }
}
