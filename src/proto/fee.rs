#[derive(Debug, Serialize)]
pub struct EPPFee05Check {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}fee:domain")]
    pub domains: Vec<EPPFee05CheckDomain>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee07Check {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}fee:domain")]
    pub domains: Vec<EPPFee07CheckDomain>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee08Check {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}fee:domain")]
    pub domains: Vec<EPPFee08CheckDomain>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee09Check {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}fee:object")]
    pub objects: Vec<EPPFee09CheckObject>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee011Check {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}fee:currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}fee:command")]
    pub command: EPPFeeCommand,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}fee:period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<super::domain::EPPDomainPeriod>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee10Check {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}fee:currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}fee:command")]
    pub commands: Vec<EPPFee10CheckCommand>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee10CheckCommand {
    #[serde(rename = "$attr:name")]
    pub name: EPPFeeCommand,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}fee:period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<super::domain::EPPDomainPeriod>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee05CheckDomain {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}fee:name")]
    pub name: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.5}fee:currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}fee:command")]
    pub command: EPPFeeCommand,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.5}fee:period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<super::domain::EPPDomainPeriod>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee07CheckDomain {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}fee:name")]
    pub name: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.7}fee:currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}fee:command")]
    pub command: EPPFeeCommand,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.7}fee:period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<super::domain::EPPDomainPeriod>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee08CheckDomain {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}fee:name")]
    pub name: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.8}fee:currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}fee:command")]
    pub command: EPPFeeCommand,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.8}fee:period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<super::domain::EPPDomainPeriod>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee09CheckObject {
    #[serde(rename = "$attr:objURI")]
    pub object_uri: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}fee:objID")]
    pub object_id: EPPFee10ObjectID,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.9}fee:currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}fee:command")]
    pub command: EPPFeeCommand,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.9}fee:period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<super::domain::EPPDomainPeriod>,
}

fn default_as_name() -> String {
    "name".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPFee10ObjectID {
    #[serde(rename = "$attr:element", default = "default_as_name")]
    pub element: String,
    #[serde(rename = "$value")]
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct EPPFee05Info {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.5}fee:currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}fee:command")]
    pub command: EPPFeeCommand,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.5}fee:period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<super::domain::EPPDomainPeriod>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee07Info {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.7}fee:currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}fee:command")]
    pub command: EPPFeeCommand,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.7}fee:period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<super::domain::EPPDomainPeriod>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee011Agreement {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-0.11}currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.11}fee")]
    pub fee: Vec<EPPFee011Fee>,
}

#[derive(Debug, Serialize)]
pub struct EPPFee10Agreement {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:fee-1.0}currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-1.0}fee")]
    pub fee: Vec<EPPFee10Fee>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EPPFeeCommand {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "renew")]
    Renew,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "restore")]
    Restore,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee05CheckData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}cd")]
    pub domains: Vec<EPPFee05CheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee07CheckData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}cd")]
    pub domains: Vec<EPPFee07CheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee08CheckData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}cd")]
    pub domains: Vec<EPPFee08CheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee09CheckData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}cd")]
    pub objects: Vec<EPPFee09CheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee011CheckData {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}cd")]
    pub objects: Vec<EPPFee011CheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee10CheckData {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}cd")]
    pub objects: Vec<EPPFee10CheckDatum>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee05CheckDatum {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}name")]
    pub name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}command")]
    pub command: EPPFeeCommand,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}period")]
    pub period: super::domain::EPPDomainPeriod,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}fee", default)]
    pub fee: Vec<EPPFee05Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}class", default)]
    pub class: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee07CheckDatum {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}name")]
    pub name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}currency", default)]
    pub currency: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}command")]
    pub command: EPPFeeCommand,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}period", default)]
    pub period: Option<super::domain::EPPDomainPeriod>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}fee", default)]
    pub fee: Vec<EPPFee08Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}credit", default)]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}class", default)]
    pub class: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee08CheckDatum {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}name")]
    pub name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}command")]
    pub command: EPPFeeCommand,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}period", default)]
    pub period: Option<super::domain::EPPDomainPeriod>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}fee", default)]
    pub fee: Vec<EPPFee08Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}credit", default)]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}class", default)]
    pub class: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee09CheckDatum {
    #[serde(rename = "$attr:objURI")]
    pub object_uri: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}objID")]
    pub object_id: EPPFee10ObjectID,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}command")]
    pub command: EPPFeeCommand,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}period", default)]
    pub period: Option<super::domain::EPPDomainPeriod>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}fee", default)]
    pub fee: Vec<EPPFee10Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}credit", default)]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}class", default)]
    pub class: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee011CheckDatum {
    #[serde(rename = "$attr:avail", default = "default_as_true")]
    pub available: bool,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}object")]
    pub object: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}command")]
    pub command: EPPFee011Command,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}period", default)]
    pub period: Option<super::domain::EPPDomainPeriod>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0p11}reason", default)]
    pub reason: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}fee", default)]
    pub fee: Vec<EPPFee011Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}credit", default)]
    pub credit: Vec<EPPFee011Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}class", default)]
    pub class: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee10CheckDatum {
    #[serde(rename = "$attr:avail", default = "default_as_true")]
    pub available: bool,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}objID")]
    pub object_id: EPPFee10ObjectID,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}command", default)]
    pub commands: Vec<EPPFee10Command>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}reason", default)]
    pub reason: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}class", default)]
    pub class: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee011Command {
    #[serde(rename = "$value")]
    pub name: EPPFeeCommand,
    #[serde(rename = "$attr:standard", default = "default_as_false")]
    pub standard: bool,
}


#[derive(Debug, Deserialize)]
pub struct EPPFee10Command {
    #[serde(rename = "$attr:name")]
    pub name: EPPFeeCommand,
    #[serde(rename = "$attr:standard", default = "default_as_false")]
    pub standard: bool,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}period", default)]
    pub period: Option<super::domain::EPPDomainPeriod>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}fee", default)]
    pub fee: Vec<EPPFee10Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}credit", default)]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}reason", default)]
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee05InfoData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}command")]
    pub command: EPPFeeCommand,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}period")]
    pub period: super::domain::EPPDomainPeriod,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}fee", default)]
    pub fee: Vec<EPPFee05Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}class", default)]
    pub class: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee07InfoData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}command")]
    pub command: EPPFeeCommand,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}period", default)]
    pub period: Option<super::domain::EPPDomainPeriod>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}fee", default)]
    pub fee: Vec<EPPFee08Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}credit", default)]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}class", default)]
    pub class: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee05TransformData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}fee", default)]
    pub fee: Vec<EPPFee05Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}balance", default)]
    pub balance: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}creditLimit", default)]
    pub credit_limit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee07TransformData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}fee", default)]
    pub fee: Vec<EPPFee08Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}credit", default)]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}balance", default)]
    pub balance: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}creditLimit", default)]
    pub credit_limit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee08TransformData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}fee", default)]
    pub fee: Vec<EPPFee08Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}credit", default)]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}balance", default)]
    pub balance: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}creditLimit", default)]
    pub credit_limit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee09TransformData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}fee", default)]
    pub fee: Vec<EPPFee10Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}credit", default)]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}balance", default)]
    pub balance: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}creditLimit", default)]
    pub credit_limit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee011TransformData {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}fee", default)]
    pub fee: Vec<EPPFee011Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}credit", default)]
    pub credit: Vec<EPPFee011Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}balance", default)]
    pub balance: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-0.11}creditLimit", default)]
    pub credit_limit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee10TransformData {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}fee", default)]
    pub fee: Vec<EPPFee10Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}credit", default)]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}balance", default)]
    pub balance: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}creditLimit", default)]
    pub credit_limit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee07TransferData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}period", default)]
    pub period: Option<super::domain::EPPDomainPeriod>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}fee", default)]
    pub fee: Vec<EPPFee08Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}credit", default)]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}balance", default)]
    pub balance: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}creditLimit", default)]
    pub credit_limit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee08TransferData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}period", default)]
    pub period: Option<super::domain::EPPDomainPeriod>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}fee", default)]
    pub fee: Vec<EPPFee08Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}credit", default)]
    pub credit: Vec<EPPFee10Credit>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee09TransferData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}period", default)]
    pub period: Option<super::domain::EPPDomainPeriod>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}fee", default)]
    pub fee: Vec<EPPFee10Fee>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}credit", default)]
    pub credit: Vec<EPPFee10Credit>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee05DeleteData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}credit")]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}balance", default)]
    pub balance: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}creditLimit", default)]
    pub credit_limit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee07DeleteData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}credit")]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}balance", default)]
    pub balance: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}creditLimit", default)]
    pub credit_limit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee08DeleteData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}credit")]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}balance", default)]
    pub balance: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}creditLimit", default)]
    pub credit_limit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee09DeleteData {
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}currency")]
    pub currency: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}credit")]
    pub credit: Vec<EPPFee10Credit>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}balance", default)]
    pub balance: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}creditLimit", default)]
    pub credit_limit: Option<String>,
}

fn default_as_true() -> bool {
    true
}

fn default_as_false() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct EPPFee05Fee {
    #[serde(rename = "$attr:description", default)]
    pub description: Option<String>,
    #[serde(rename = "$attr:refundable", default = "default_as_true")]
    pub refundable: bool,
    #[serde(rename = "$attr:grace-period", default)]
    pub grace_period: Option<String>,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPFee08Fee {
    #[serde(rename = "$attr:description", default)]
    pub description: Option<String>,
    #[serde(rename = "$attr:refundable", default = "default_as_true")]
    pub refundable: bool,
    #[serde(rename = "$attr:grace-period", default)]
    pub grace_period: Option<String>,
    #[serde(rename = "$attr:applied", default)]
    pub applied: EPPFee10Applied,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPFee10Fee {
    #[serde(
    rename = "$attr:description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    #[serde(
    rename = "$attr:refundable",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub refundable: Option<bool>,
    #[serde(
    rename = "$attr:grace-period",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub grace_period: Option<String>,
    #[serde(
    rename = "$attr:applied",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub applied: Option<EPPFee10Applied>,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EPPFee011Fee {
    #[serde(
        rename = "$attr:description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub description: Option<String>,
    #[serde(
        rename = "$attr:refundable",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub refundable: Option<bool>,
    #[serde(
        rename = "$attr:grace-period",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub grace_period: Option<String>,
    #[serde(
        rename = "$attr:applied",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub applied: Option<EPPFee10Applied>,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EPPFee10Applied {
    #[serde(rename = "immediate")]
    Immediate,
    #[serde(rename = "delayed")]
    Delayed,
}

impl Default for EPPFee10Applied {
    fn default() -> Self {
        EPPFee10Applied::Immediate
    }
}

#[derive(Debug, Deserialize)]
pub struct EPPFee10Credit {
    #[serde(rename = "$attr:description", default)]
    pub description: Option<String>,
    #[serde(rename = "$value")]
    pub value: String,
}


#[derive(Debug, Deserialize)]
pub struct EPPFee011Credit {
    #[serde(rename = "$attr:description", default)]
    pub description: Option<String>,
    #[serde(rename = "$value")]
    pub value: String,
}
