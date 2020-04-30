//! Serde structs for serialisation and deserialisation of EPP XML messages
//! (these are insane, stay away if you value your sanity)

use chrono::prelude::*;
use std::collections::HashMap;

pub mod change_poll;
pub mod contact;
pub mod domain;
pub mod host;
pub mod nominet;
pub mod rgp;
pub mod secdns;
pub mod switch;
pub mod verisign;
pub mod centralnic;
pub mod traficom;

#[derive(Debug, Serialize, Deserialize)]
pub enum EPPMessageType {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}hello", skip_deserializing)]
    Hello {},
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}greeting", skip_serializing)]
    Greeting(EPPGreeting),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}command", skip_deserializing)]
    Command(Box<EPPCommand>),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}response", skip_serializing)]
    Response(Box<EPPResponse>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EPPMessage {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}epp")]
    pub message: EPPMessageType,
}

#[derive(Debug, Deserialize)]
pub struct EPPGreeting {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}svID")]
    pub server_id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}svDate")]
    pub server_date: DateTime<Utc>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}svcMenu")]
    pub service_menu: EPPServiceMenu,
}

#[derive(Debug, Deserialize)]
pub struct EPPServiceMenu {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}version")]
    pub versions: Vec<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}lang")]
    pub languages: Vec<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}objURI")]
    pub objects: Vec<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}svcExtension")]
    pub extension: Option<EPPServiceExtension>,
}

impl EPPServiceMenu {
    pub fn supports(&self, obj: &str) -> bool {
        self.objects.iter().any(|e| e == obj)
    }
    pub fn supports_ext(&self, obj: &str) -> bool {
        self.extension
            .as_ref()
            .map_or(false, |e| e.extensions.iter().any(|e| e == obj))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EPPServiceExtension {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}extURI")]
    pub extensions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub enum EPPCommandType {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}login")]
    Login(EPPLogin),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}logout")]
    Logout {},
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}check")]
    Check(EPPCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}info")]
    Info(EPPInfo),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}create")]
    Create(EPPCreate),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}delete")]
    Delete(EPPDelete),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}update")]
    Update(Box<EPPUpdate>),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}renew")]
    Renew(EPPRenew),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}transfer")]
    Transfer(EPPTransfer),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}poll")]
    Poll(EPPPoll),
}

#[derive(Debug, Serialize)]
pub enum EPPCommandExtensionType {
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0}contact-nom-ext:create"
    )]
    NominetContactExtCreate(nominet::EPPContactInfo),
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0}contact-nom-ext:update"
    )]
    NominetContactExtUpdate(nominet::EPPContactInfo),
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgp:update")]
    EPPRGPUpdate(rgp::EPPRGPUpdate),
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:create")]
    EPPSecDNSCreate(secdns::EPPSecDNSData),
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:update")]
    EPPSecDNSUpdate(secdns::EPPSecDNSUpdate),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-ext-1.0}domain-ext:delete")]
    TraficomDelete(traficom::EPPDomainDelete)
}

#[derive(Debug, Serialize)]
pub struct EPPCommand {
    #[serde(rename = "$value")]
    pub command: EPPCommandType,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<EPPCommandExtensionType>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}clTRID", skip_serializing_if = "Option::is_none")]
    pub client_transaction_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPResponse {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}result")]
    pub results: Vec<EPPResult>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}extension", default)]
    pub extension: Option<EPPResponseExtension>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}msgQ", default)]
    pub message_queue: Option<EPPMessageQueue>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}resData", default)]
    pub data: Option<EPPResultData>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}trID")]
    pub transaction_id: EPPTransactionIdentifier,
}

#[derive(Debug, Deserialize)]
pub struct EPPResponseExtension {
    #[serde(rename = "$value", default)]
    pub value: Vec<EPPResponseExtensionType>
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
                            .map(|(k, v)| format!("{}: {}", k, v.as_deref().unwrap_or("")))
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
    #[serde(rename = "$attr:code")]
    pub code: EPPResultCode,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}msg")]
    pub message: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}value")]
    pub values: Option<Vec<HashMap<String, String>>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}extValue")]
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
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}value")]
    pub value: HashMap<String, Option<String>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}reason")]
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPMessageQueue {
    #[serde(rename = "$attr:count")]
    pub count: u64,
    #[serde(rename = "$attr:id")]
    pub id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}qDate")]
    pub enqueue_date: Option<DateTime<Utc>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}msg")]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EPPResultData {
    #[serde(rename = "$value")]
    pub value: EPPResultDataValue,
}

#[derive(Debug, Deserialize)]
pub enum EPPResultDataValue {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}chkData")]
    EPPDomainCheckResult(domain::EPPDomainCheckData),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}infData")]
    EPPDomainInfoResult(Box<domain::EPPDomainInfoData>),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}trnData")]
    EPPDomainTransferResult(domain::EPPDomainTransferData),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}creData")]
    EPPDomainCreateResult(domain::EPPDomainCreateData),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}renData")]
    EPPDomainRenewResult(domain::EPPDomainRenewData),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}panData")]
    EPPDomainPendingActionNotification(domain::EPPDomainPanData),
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}chkData")]
    EPPHostCheckResult(host::EPPHostCheckData),
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}infData")]
    EPPHostInfoResult(Box<host::EPPHostInfoData>),
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}creData")]
    EPPHostCreateResult(host::EPPHostCreateData),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}chkData")]
    EPPContactCheckResult(contact::EPPContactCheckData),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}infData")]
    EPPContactInfoResult(Box<contact::EPPContactInfoData>),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}trnData")]
    EPPContactTransferResult(contact::EPPContactTransferData),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}creData")]
    EPPContactCreateResult(contact::EPPContactCreateData),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-tag-1.0}listData")]
    EPPNominetTagInfoResult(nominet::EPPTagListData),
    #[serde(rename = "{https://www.nic.ch/epp/balance-1.0}infData")]
    EPPSwitchBalanceInfoResult(switch::EPPBalance),
}

#[derive(Debug, Deserialize)]
pub struct EPPTransactionIdentifier {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}clTRID")]
    pub client_transaction_id: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}svTRID")]
    pub server_transaction_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EPPLogin {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}clID")]
    pub client_id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}pw")]
    pub password: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}newPW", skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}options")]
    pub options: EPPLoginOptions,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}svcs")]
    pub services: EPPLoginServices,
}

#[derive(Debug, Serialize)]
pub struct EPPLoginOptions {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}version")]
    pub version: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}lang")]
    pub language: String,
}

#[derive(Debug, Serialize)]
pub struct EPPLoginServices {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}objURI")]
    pub objects: Vec<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}svcExtension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<EPPServiceExtension>,
}

#[derive(Debug, Deserialize)]
pub enum EPPResponseExtensionType {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0}infData")]
    NominetContactExtInfo(nominet::EPPContactInfo),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-warning-1.1}ignored-field")]
    NominetIgnoredField(nominet::EPPIgnoredField),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-warning-1.1}truncated-field")]
    NominetTruncatedField(nominet::EPPTruncatedField),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}infData")]
    NominetDataQuality(nominet::EPPDataQualityInfo),
    #[serde(rename = "{urn:ietf:params:xml:ns:changePoll-1.0}changeData")]
    EPPChangePoll(change_poll::EPPChangeData),
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}infData")]
    EPPRGPInfo(rgp::EPPRGPData),
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}upData")]
    EPPRGPUpdate(rgp::EPPRGPData),
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}infData")]
    EPPSecDNSInfo(secdns::EPPSecDNSData),
    #[serde(rename = "{urn:ietf:params:xml:ns:regtype-0.1}infData")]
    EPPCentralnicRegTypeInfoResult(centralnic::EPPRegType),
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
    TagList {},
    #[serde(rename = "{https://www.nic.ch/epp/balance-1.0}balance:info")]
    #[allow(dead_code)]
    SwitchBalace {},
    #[serde(rename = "{http://www.verisign.com/epp/balance-1.0}balance:info")]
    #[allow(dead_code)]
    VerisignBalace {},
}

#[derive(Debug, Serialize)]
pub enum EPPCreate {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:create")]
    Host(host::EPPHostCreate),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:create")]
    Contact(Box<contact::EPPContactCreate>),
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
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "reject")]
    Reject,
}

#[derive(Debug, Serialize)]
pub enum EPPTransferCommand {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:transfer")]
    DomainQuery(domain::EPPDomainCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:transfer")]
    DomainRequest(domain::EPPDomainTransfer),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:transfer")]
    ContactQuery(contact::EPPContactCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:transfer")]
    ContactRequest(contact::EPPContactTransfer),
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
            Err(_) => match Utc.datetime_from_str(value, "%FT%T%.f") {
                Ok(t) => Ok(t),
                Err(_) => Utc.datetime_from_str(value, "%FT%T").map_err(E::custom),
            }
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
    let date = d.deserialize_option(OptDateTimeVisitor)?;
    Ok(match date {
        Some(d) => if d == Utc.ymd(1, 1, 1).and_hms(0, 0, 0) {
            None
        } else {
            Some(d)
        },
        None => None
    })
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_date<S>(d: &Date<Utc>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    s.serialize_str(&d.format("%Y-%m-%d").to_string())
}


#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_opt_bool<S>(d: &Option<bool>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    match d {
        Some(d) => if *d {
            s.serialize_str("true")
        } else {
            s.serialize_str("false")
        },
        None => s.serialize_none()
    }
}
