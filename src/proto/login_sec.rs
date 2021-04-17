use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct EPPLoginSecurity {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:loginSec-1.0}loginSec:userAgent",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_agent: Option<EPPLoginSecurityUserAgent>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:loginSec-1.0}loginSec:pw",
        skip_serializing_if = "Option::is_none"
    )]
    pub password: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:loginSec-1.0}loginSec:pw",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_password: Option<String>
}

#[derive(Debug, Serialize)]
pub struct EPPLoginSecurityUserAgent {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:loginSec-1.0}loginSec:app",
        skip_serializing_if = "Option::is_none"
    )]
    pub app: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:loginSec-1.0}loginSec:tech",
        skip_serializing_if = "Option::is_none"
    )]
    pub tech: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp:loginSec-1.0}loginSec:os",
        skip_serializing_if = "Option::is_none"
    )]
    pub os: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct EPPLoginSecurityData {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:loginSec-1.0}event")]
    pub events: Vec<EPPLoginSecurityEvent>
}

#[derive(Debug, Deserialize)]
pub struct EPPLoginSecurityEvent {
    #[serde(rename = "$attr:type")]
    pub event_type: EPPLoginSecurityEventType,
    #[serde(rename = "$attr:name", default)]
    pub event_name: Option<String>,
    #[serde(rename = "$attr:level")]
    pub level: EPPLoginSecurityEventLevel,
    #[serde(
        rename = "$attr:exDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub expiration_date: Option<DateTime<Utc>>,
    #[serde(rename = "$attr:value", default)]
    pub value: Option<String>,
    #[serde(rename = "$attr:duration", default)]
    pub duration: Option<String>,
    #[serde(rename = "$attr:lang", default)]
    pub lang: Option<String>,
    #[serde(rename = "$value", default)]
    pub msg: Option<String>
}

#[derive(Debug, Deserialize)]
pub enum EPPLoginSecurityEventType {
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "certificate")]
    Certificate,
    #[serde(rename = "cipher")]
    Cipher,
    #[serde(rename = "tlsProtocol")]
    TLSProtocol,
    #[serde(rename = "newPW")]
    NewPassword,
    #[serde(rename = "stat")]
    Statistical,
    #[serde(rename = "custom")]
    Custom
}

#[derive(Debug, Deserialize)]
pub enum EPPLoginSecurityEventLevel {
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error
}