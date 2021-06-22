use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum TMCHMessageType {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}hello", skip_deserializing)]
    Hello {},
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}greeting", skip_serializing)]
    Greeting(TMCHGreeting),
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}command", skip_deserializing)]
    Command(Box<TMCHCommand>),
    // #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}response", skip_serializing)]
    // Response(Box<TMCHResponse>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TMCHMessage {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}tmch")]
    pub message: TMCHMessageType,
}


#[derive(Debug, Deserialize)]
pub struct TMCHGreeting {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}svID")]
    pub server_id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}svDate")]
    pub server_date: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TMCHCommand {
    #[serde(rename = "$value")]
    pub command: TMCHCommandType,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}clTRID",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_transaction_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum TMCHCommandType {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}check")]
    Check(TMCHCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}create")]
    Create(TMCHCreate),
    // #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}info")]
    // Info(EPPInfo),
    // #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}login")]
    // Login(TMCHLogin),
    // #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}logout")]
    // Logout {},
    // #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}poll")]
    // Poll(EPPPoll),
    // #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}renew")]
    // Renew(EPPRenew),
    // #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}transfer")]
    // Transfer(EPPTransfer),
    // #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}update")]
    // Update(Box<EPPUpdate>),
}

#[derive(Debug, Serialize)]
pub struct TMCHCheck {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}id")]
    id: Vec<String>
}

#[derive(Debug, Serialize)]
pub struct TMCHPeriod {
    #[serde(rename = "$value")]
    value: u8,
    #[serde(rename = "$attr:unit")]
    unit: TMCHPeriodUnit,
}

#[derive(Debug, Serialize)]
pub enum TMCHPeriodUnit {
    Years
}

#[derive(Debug, Serialize)]
pub struct TMCHCreate {
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}period",
        skip_serializing_if = "Option::is_none"
    )]
    period: Option<TMCHPeriod>
}
