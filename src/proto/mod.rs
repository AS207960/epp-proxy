//! Serde structs for serialisation and deserialisation of EPP XML messages
//! (these are insane, stay away if you value your sanity)

use chrono::prelude::*;
use std::collections::HashMap;

pub mod contact;
pub mod domain;
pub mod host;
pub mod nominet;
pub mod switch;
pub mod change_poll;
pub mod rgp;

#[derive(Debug, Serialize, Deserialize)]
pub enum EPPMessageType {
    #[serde(rename = "hello", skip_deserializing)]
    Hello{},
    #[serde(rename = "greeting", skip_serializing)]
    Greeting(EPPGreeting),
    #[serde(rename = "command", skip_deserializing)]
    Command(Box<EPPCommand>),
    #[serde(rename = "response", skip_serializing)]
    Response(Box<EPPResponse>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EPPMessage {
    #[serde(rename = "$value")]
    pub message: EPPMessageType,
}

#[derive(Debug, Deserialize)]
pub struct EPPGreeting {
    #[serde(rename = "svID")]
    pub server_id: String,
    #[serde(rename = "svDate")]
    pub server_date: DateTime<Utc>,
    #[serde(rename = "svcMenu")]
    pub service_menu: EPPServiceMenu,
}

#[derive(Debug, Deserialize)]
pub struct EPPServiceMenu {
    #[serde(rename = "version")]
    pub versions: Vec<String>,
    #[serde(rename = "lang")]
    pub languages: Vec<String>,
    #[serde(rename = "objURI")]
    pub objects: Vec<String>,
    #[serde(rename = "svcExtension")]
    pub extension: Option<EPPServiceExtension>,
}

impl EPPServiceMenu {
    pub fn supports(&self, obj: &str) -> bool {
        self.objects.iter().any(|e| e == obj)
    }
    pub fn supports_ext(&self, obj: &str) -> bool {
        self.extension.as_ref().map_or(false,|e| e.extensions.iter().any(|e| e == obj))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EPPServiceExtension {
    #[serde(rename = "extURI")]
    pub extensions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub enum EPPCommandType {
    #[serde(rename = "login")]
    Login(EPPLogin),
    #[serde(rename = "logout")]
    Logout{},
    #[serde(rename = "check")]
    Check(EPPCheck),
    #[serde(rename = "info")]
    Info(EPPInfo),
    #[serde(rename = "create")]
    Create(EPPCreate),
    #[serde(rename = "delete")]
    Delete(EPPDelete),
    #[serde(rename = "update")]
    Update(Box<EPPUpdate>),
    #[serde(rename = "renew")]
    Renew(EPPRenew),
    #[serde(rename = "transfer")]
    Transfer(EPPTransfer),
    #[serde(rename = "poll")]
    Poll(EPPPoll),
}

#[derive(Debug, Serialize)]
pub enum EPPCommandExtensionType {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0}contact-nom-ext:create")]
    NominetContactExtCreate(nominet::EPPContactInfoSet),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0}contact-nom-ext:update")]
    NominetContactExtUpdate(nominet::EPPContactInfoSet),
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgp:update")]
    EPPRGPUpdate(rgp::EPPRGPUpdate)
}

#[derive(Debug, Serialize)]
pub struct EPPCommand {
    #[serde(rename = "$value")]
    pub command: EPPCommandType,
    #[serde(rename = "extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<EPPCommandExtensionType>,
    #[serde(rename = "clTRID", skip_serializing_if = "Option::is_none")]
    pub client_transaction_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum EPPResponseExtensionType {
    #[serde(rename = "contact-nom-ext:infData")]
    NominetContactExtInfo(nominet::EPPContactInfo),
    #[serde(rename = "warning:ignored-field")]
    NominetIgnoredField(nominet::EPPIgnoredField),
    #[serde(rename = "nom-data-quality:infData")]
    NominetDataQuality(nominet::EPPDataQualityInfo),
    #[serde(rename = "changePoll:changeData")]
    EPPChangePoll(change_poll::EPPChangeData),
    #[serde(rename = "rgp:infData")]
    EPPRGPInfo(rgp::EPPRGPData),
    #[serde(rename = "rgp:upData")]
    EPPRGPUpdate(rgp::EPPRGPData),
}

#[derive(Debug, Deserialize)]
pub struct EPPResponseExtension {
    #[serde(rename = "$value")]
    pub value: Vec<EPPResponseExtensionType>
}

#[derive(Debug, Deserialize)]
pub struct EPPResponse {
    #[serde(rename = "result")]
    pub results: Vec<EPPResult>,
    #[serde(rename = "extension", default)]
    pub extension: Option<EPPResponseExtension>,
    #[serde(rename = "msgQ")]
    pub message_queue: Option<EPPMessageQueue>,
    #[serde(rename = "resData")]
    pub data: Option<EPPResultData>,
    #[serde(rename = "trID")]
    pub transaction_id: EPPTransactionIdentifier,
}

impl EPPResponse {
    pub fn is_success(&self) -> bool {
        match self.results.first() {
            Some(r) => r.code.is_success(),
            None => false,
        }
    }
    pub fn is_closing(&self) -> bool {
        match self.results.first() {
            Some(r) => r.code.is_closing(),
            None => false,
        }
    }
    pub fn is_pending(&self) -> bool {
        match self.results.first() {
            Some(r) => r.code == EPPResultCode::SuccessActionPending,
            None => false,
        }
    }
    pub fn is_server_error(&self) -> bool {
        match self.results.first() {
            Some(r) => r.code.is_server_error(),
            None => false,
        }
    }

    pub fn response_msg(&self) -> String {
        let mut output = vec![];
        for r in &self.results {
            match r.extra_values.as_ref().map(|v| {
                v.iter()
                    .map(|e| {
                        let val = e
                            .value
                            .iter()
                            .next()
                            .map(|(k, v)| format!("{}: {}", k, v))
                            .unwrap_or_default();
                        format!("({}) {}", val, e.reason)
                    })
                    .collect::<Vec<_>>()
            }) {
                Some(extra) => {
                    output.push(format!(
                        "({:?}) {}: {}",
                        r.code,
                        r.message,
                        extra.join(", ")
                    ));
                }
                None => {
                    output.push(format!("({:?}) {}", r.code, r.message));
                }
            }
        }
        output.join(", ")
    }
}

#[derive(Debug, Deserialize)]
pub struct EPPResult {
    pub code: EPPResultCode,
    #[serde(rename = "msg")]
    pub message: String,
    #[serde(rename = "value")]
    pub values: Option<Vec<HashMap<String, String>>>,
    #[serde(rename = "extValue")]
    pub extra_values: Option<Vec<EPPResultExtraValue>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum EPPResultCode {
    Success,
    SuccessActionPending,
    SuccessNoMessages,
    SuccessAckToDequeue,
    SuccessEndingSession,
    UnknownCommand,
    CommandSyntaxError,
    CommandUseError,
    RequiredParameterMissing,
    ParameterValueRangeError,
    ParameterValueSyntaxError,
    UnimplementedProtocolVersion,
    UnimplementedCommand,
    UnimplementedOption,
    UnimplementedExtension,
    BillingFailure,
    ObjectNotEligibleForRenewal,
    ObjectNotEligibleForTransfer,
    AuthenticationError,
    AuthorizationError,
    InvalidAuthorization,
    ObjectPendingTransfer,
    ObjectNotPendingTransfer,
    ObjectExists,
    ObjectDoesNotExist,
    ObjectStatusProhibitsOperation,
    ObjectAssociationProhibitsOperation,
    ParameterValuePolicyError,
    UnimplementedObjectService,
    DataManagementPolicyViolation,
    CommandFailed,
    CommandFailedServerClosingConnection,
    AuthenticationServerClosingConnection,
    SessionLimitExceededServerClosingConnection,
    Other(u16),
}

impl EPPResultCode {
    fn is_success(&self) -> bool {
        match self {
            EPPResultCode::Success
            | EPPResultCode::SuccessActionPending
            | EPPResultCode::SuccessNoMessages
            | EPPResultCode::SuccessAckToDequeue
            | EPPResultCode::SuccessEndingSession => true,
            _ => false,
        }
    }

    fn is_closing(&self) -> bool {
        match self {
            EPPResultCode::SuccessEndingSession
            | EPPResultCode::CommandFailedServerClosingConnection
            | EPPResultCode::AuthenticationServerClosingConnection
            | EPPResultCode::SessionLimitExceededServerClosingConnection => true,
            _ => false,
        }
    }

    fn is_server_error(&self) -> bool {
        match self {
            EPPResultCode::CommandFailed
            | EPPResultCode::CommandFailedServerClosingConnection
            | EPPResultCode::AuthenticationServerClosingConnection
            | EPPResultCode::SessionLimitExceededServerClosingConnection => true,
            _ => false,
        }
    }
}

impl From<u16> for EPPResultCode {
    fn from(value: u16) -> EPPResultCode {
        match value {
            1000 => EPPResultCode::Success,
            1001 => EPPResultCode::SuccessActionPending,
            1300 => EPPResultCode::SuccessNoMessages,
            1301 => EPPResultCode::SuccessAckToDequeue,
            1500 => EPPResultCode::SuccessEndingSession,
            2000 => EPPResultCode::UnknownCommand,
            2001 => EPPResultCode::CommandSyntaxError,
            2002 => EPPResultCode::CommandUseError,
            2003 => EPPResultCode::RequiredParameterMissing,
            2004 => EPPResultCode::ParameterValueRangeError,
            2005 => EPPResultCode::ParameterValueSyntaxError,
            2100 => EPPResultCode::UnimplementedProtocolVersion,
            2101 => EPPResultCode::UnimplementedCommand,
            2102 => EPPResultCode::UnimplementedOption,
            2103 => EPPResultCode::UnimplementedExtension,
            2104 => EPPResultCode::BillingFailure,
            2105 => EPPResultCode::ObjectNotEligibleForRenewal,
            2106 => EPPResultCode::ObjectNotEligibleForTransfer,
            2200 => EPPResultCode::AuthenticationError,
            2201 => EPPResultCode::AuthorizationError,
            2202 => EPPResultCode::InvalidAuthorization,
            2300 => EPPResultCode::ObjectPendingTransfer,
            2301 => EPPResultCode::ObjectNotPendingTransfer,
            2302 => EPPResultCode::ObjectExists,
            2303 => EPPResultCode::ObjectDoesNotExist,
            2304 => EPPResultCode::ObjectStatusProhibitsOperation,
            2305 => EPPResultCode::ObjectAssociationProhibitsOperation,
            2306 => EPPResultCode::ParameterValuePolicyError,
            2307 => EPPResultCode::UnimplementedObjectService,
            2308 => EPPResultCode::DataManagementPolicyViolation,
            2400 => EPPResultCode::CommandFailed,
            2500 => EPPResultCode::CommandFailedServerClosingConnection,
            2501 => EPPResultCode::AuthenticationServerClosingConnection,
            2502 => EPPResultCode::SessionLimitExceededServerClosingConnection,
            o => EPPResultCode::Other(o),
        }
    }
}

impl<'de> serde::Deserialize<'de> for EPPResultCode {
    fn deserialize<D>(deserializer: D) -> Result<EPPResultCode, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct U16Visitor;

        impl<'de> serde::de::Visitor<'de> for U16Visitor {
            type Value = u16;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer between 0 and 2^16")
            }

            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(u16::from(value))
            }

            fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(value)
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::u16;
                if value >= u32::from(u16::MIN) && value <= u32::from(u16::MAX) {
                    Ok(value as u16)
                } else {
                    Err(E::custom(format!("u16 out of range: {}", value)))
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::u16;
                if value >= u64::from(u16::MIN) && value <= u64::from(u16::MAX) {
                    Ok(value as u16)
                } else {
                    Err(E::custom(format!("u16 out of range: {}", value)))
                }
            }
        }

        match deserializer.deserialize_u16(U16Visitor) {
            Ok(i) => Ok(EPPResultCode::from(i)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EPPResultExtraValue {
    pub value: HashMap<String, String>,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPMessageQueue {
    pub count: u64,
    pub id: String,
    #[serde(rename = "qDate")]
    pub enqueue_date: Option<DateTime<Utc>>,
    #[serde(rename = "msg")]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPResultData {
    #[serde(rename = "$value")]
    pub value: EPPResultDataValue,
}

#[derive(Debug, Deserialize)]
pub enum EPPResultDataValue {
    #[serde(rename = "domain:chkData")]
    EPPDomainCheckResult(domain::EPPDomainCheckData),
    #[serde(rename = "domain:infData")]
    EPPDomainInfoResult(Box<domain::EPPDomainInfoData>),
    #[serde(rename = "domain:trnData")]
    EPPDomainTransferResult(domain::EPPDomainTransferData),
    #[serde(rename = "domain:creData")]
    EPPDomainCreateResult(domain::EPPDomainCreateData),
    #[serde(rename = "domain:renData")]
    EPPDomainRenewResult(domain::EPPDomainRenewData),
    #[serde(rename = "domain:panData")]
    EPPDomainPendingActionNotification(domain::EPPDomainPanData),
    #[serde(rename = "host:chkData")]
    EPPHostCheckResult(host::EPPHostCheckData),
    #[serde(rename = "host:infData")]
    EPPHostInfoResult(Box<host::EPPHostInfoData>),
    #[serde(rename = "host:creData")]
    EPPHostCreateResult(host::EPPHostCreateData),
    #[serde(rename = "contact:chkData")]
    EPPContactCheckResult(contact::EPPContactCheckData),
    #[serde(rename = "contact:infData")]
    EPPContactInfoResult(Box<contact::EPPContactInfoData>),
    #[serde(rename = "contact:creData")]
    EPPContactCreateResult(contact::EPPContactCreateData),
    #[serde(rename = "tag:listData")]
    EPPNominetTagInfoResult(nominet::EPPTagListData),
    #[serde(rename = "balance:infData")]
    EPPSwitchBalanceInfoResult(switch::EPPBalance),
}

#[derive(Debug, Deserialize)]
pub struct EPPTransactionIdentifier {
    #[serde(rename = "clTRID")]
    pub client_transaction_id: Option<String>,
    #[serde(rename = "svTRID")]
    pub server_transaction_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EPPLogin {
    #[serde(rename = "clID")]
    pub client_id: String,
    #[serde(rename = "pw")]
    pub password: String,
    #[serde(rename = "newPW", skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    pub options: EPPLoginOptions,
    #[serde(rename = "svcs")]
    pub services: EPPLoginServices,
}

#[derive(Debug, Serialize)]
pub struct EPPLoginOptions {
    pub version: String,
    #[serde(rename = "lang")]
    pub language: String,
}

#[derive(Debug, Serialize)]
pub struct EPPLoginServices {
    #[serde(rename = "objURI")]
    pub objects: Vec<String>,
    #[serde(rename = "svcExtension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<EPPServiceExtension>,
}

#[derive(Debug, Serialize)]
pub enum EPPCheck {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:check")]
    Domain(domain::EPPDomainCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:check")]
    Host(host::EPPHostCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:check")]
    Contact(contact::EPPContactCheck),
}

#[derive(Debug, Serialize)]
pub enum EPPInfo {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:info")]
    Domain(domain::EPPDomainCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:info")]
    Host(host::EPPHostCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:info")]
    Contact(contact::EPPContactCheck),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-tag-1.0}tag:list")]
    TagList{},
    #[serde(rename = "{https://www.nic.ch/epp/balance-1.0}balance:info")]
    #[allow(dead_code)]
    SwitchBalace{},
}

#[derive(Debug, Serialize)]
pub enum EPPCreate {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:create")]
    Host(host::EPPHostCreate),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:create")]
    Contact(contact::EPPContactCreate),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:create")]
    Domain(domain::EPPDomainCreate),
}

#[derive(Debug, Serialize)]
pub enum EPPDelete {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:delete")]
    Host(host::EPPHostCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:delete")]
    Contact(contact::EPPContactCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:delete")]
    Domain(domain::EPPDomainCheck),
}

#[derive(Debug, Serialize)]
pub enum EPPUpdate {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:update")]
    Host(host::EPPHostUpdate),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:update")]
    Contact(contact::EPPContactUpdate),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:update")]
    Domain(domain::EPPDomainUpdate),
}

#[derive(Debug, Serialize)]
pub enum EPPRenew {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:renew")]
    Domain(domain::EPPDomainRenew),
}

#[derive(Debug, Serialize)]
pub struct EPPTransfer {
    #[serde(rename = "$attr:op")]
    pub operation: EPPTransferOperation,
    #[serde(rename = "$value")]
    pub command: EPPTransferCommand,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EPPTransferOperation {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "request")]
    Request,
}

#[derive(Debug, Serialize)]
pub enum EPPTransferCommand {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:transfer")]
    DomainQuery(domain::EPPDomainCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:transfer")]
    DomainRequest(domain::EPPDomainTransfer),
}

#[derive(Debug, Serialize)]
pub struct EPPPoll {
    #[serde(rename = "$attr:op")]
    pub operation: EPPPollOperation,
    #[serde(rename = "$attr:msgID", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum EPPPollOperation {
    #[serde(rename = "req")]
    Request,
    #[serde(rename = "ack")]
    Acknowledge,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EPPTransferStatus {
    #[serde(rename = "clientApproved")]
    ClientApproved,
    #[serde(rename = "clientCancelled")]
    ClientCancelled,
    #[serde(rename = "clientRejected")]
    ClientRejected,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "serverApproved")]
    ServerApproved,
    #[serde(rename = "serverCancelled")]
    ServerCancelled,
}

struct DateTimeVisitor;

impl<'de> serde::de::Visitor<'de> for DateTimeVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a formatted date and time string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.parse::<DateTime<Utc>>() {
            Ok(v) => Ok(v.with_timezone(&Utc)),
            Err(_) => Utc
                .datetime_from_str("2019-04-04T20:00:09", "%FT%T")
                .map_err(E::custom),
        }
    }
}

struct OptDateTimeVisitor;

impl<'de> serde::de::Visitor<'de> for OptDateTimeVisitor {
    type Value = Option<DateTime<Utc>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a formatted date and time string")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        d.deserialize_str(DateTimeVisitor).map(Some)
    }
}

fn deserialize_datetime<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    d.deserialize_str(DateTimeVisitor)
}


fn deserialize_datetime_opt<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    d.deserialize_option(OptDateTimeVisitor)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_date<S>(d: &Date<Utc>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
{
    s.serialize_str(&d.format("%Y-%m-%d").to_string())
}
