//! Serde structs for serialisation and deserialisation of EPP XML messages
//! (these are insane, stay away if you value your sanity)

use chrono::prelude::*;
use std::collections::HashMap;

pub mod centralnic;
pub mod change_poll;
pub mod contact;
pub mod corenic;
pub mod domain;
pub mod email_forward;
pub mod eurid;
pub mod fee;
pub mod host;
pub mod isnic;
pub mod launch;
pub mod login_sec;
pub mod maintenance;
pub mod mark;
pub mod nominet;
pub mod personal_registration;
pub mod qualified_lawyer;
pub mod rgp;
pub mod secdns;
pub mod switch;
pub mod tm_notice;
pub mod tmch;
pub mod traficom;
pub mod united_tld;
pub mod verisign;

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
    #[serde(
        rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.0}nom-data-quality:update"
    )]
    NominetDataQualityUpdate(nominet::EPPDataQualityUpdate),
    #[serde(rename = "{urn:ietf:params:xml:ns:rgp-1.0}rgp:update")]
    EPPRGPUpdate(rgp::EPPRGPUpdate),
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:create")]
    EPPSecDNSCreate(secdns::EPPSecDNSData),
    #[serde(rename = "{urn:ietf:params:xml:ns:secDNS-1.1}secDNS:update")]
    EPPSecDNSUpdate(secdns::EPPSecDNSUpdate),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-ext-1.0}domain-ext:delete")]
    TraficomDelete(traficom::EPPDomainDelete),
    #[serde(
        rename = "{http://www.verisign-grs.com/epp/namestoreExt-1.1}namestoreExt:namestoreExt"
    )]
    VerisignNameStoreExt(verisign::EPPNameStoreExt),
    #[serde(rename = "{http://www.verisign.com/epp/whoisInf-1.0}whoisInf:whoisInf")]
    VerisignWhoisInfExt(verisign::EPPWhoisInfoExt),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}fee:check")]
    EPPFee05Check(fee::EPPFee05Check),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}fee:check")]
    EPPFee07Check(fee::EPPFee07Check),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}fee:check")]
    EPPFee08Check(fee::EPPFee08Check),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}fee:check")]
    EPPFee09Check(fee::EPPFee09Check),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.11}fee:check")]
    EPPFee011Check(fee::EPPFee011Check),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}fee:check")]
    EPPFee10Check(fee::EPPFee10Check),
    #[allow(dead_code)]
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}fee:info")]
    EPPFee05Info(fee::EPPFee05Info),
    #[allow(dead_code)]
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}fee:info")]
    EPPFee07Info(fee::EPPFee07Info),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.11}fee:create")]
    EPPFee011Create(fee::EPPFee011Agreement),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}fee:create")]
    EPPFee10Create(fee::EPPFee10Agreement),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.11}fee:renew")]
    EPPFee011Renew(fee::EPPFee011Agreement),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}fee:renew")]
    EPPFee10Renew(fee::EPPFee10Agreement),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.11}fee:transfer")]
    EPPFee011Transfer(fee::EPPFee011Agreement),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}fee:transfer")]
    EPPFee10Transfer(fee::EPPFee10Agreement),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.11}fee:update")]
    EPPFee011Update(fee::EPPFee011Agreement),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}fee:update")]
    EPPFee10Update(fee::EPPFee10Agreement),
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:check")]
    EPPLaunchCheck(launch::EPPLaunchCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:info")]
    EPPLaunchInfo(launch::EPPLaunchInfo),
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:create")]
    EPPLaunchCreate(launch::EPPLaunchCreate),
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:update")]
    EPPLaunchUpdate(launch::EPPLaunchInfo),
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}launch:delete")]
    EPPLaunchDelete(launch::EPPLaunchInfo),
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}charge:agreement")]
    EPPDonutsChargeAgreement(united_tld::EPPChargeData),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:loginSec-1.0}loginSec:loginSec")]
    EPPLoginSecurity(login_sec::EPPLoginSecurity),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/authInfo-1.1}authInfo:info")]
    EURIDAuthInfo(eurid::EURIDAuthInfo),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:create")]
    EURIDContactCreate(eurid::EURIDContactInfo),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}contact-ext:update")]
    EURIDContactUpdate(eurid::EURIDContactUpdate),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/domain-ext-2.4}domain-ext:create")]
    EURIDDomainCreate(eurid::EURIDDomainCreate),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/domain-ext-2.4}domain-ext:update")]
    EURIDDomainUpdate(eurid::EURIDDomainUpdate),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/domain-ext-2.4}domain-ext:transfer")]
    EURIDDomainTransfer(eurid::EURIDDomainTransfer),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/domain-ext-2.4}domain-ext:delete")]
    EURIDDomainDelete(eurid::EURIDDomainDelete),
    #[serde(rename = "{urn:ietf:params:xml:ns:qualifiedLawyer-1.0}qualifiedLawyer:create")]
    QualifiedLawyerCreate(qualified_lawyer::QualifiedLawyerInfoData),
    #[serde(rename = "{urn:ietf:params:xml:ns:qualifiedLawyer-1.0}qualifiedLawyer:update")]
    QualifiedLawyerUpdate(qualified_lawyer::QualifiedLawyerInfoData),
    #[serde(rename = "{http://www.verisign.com/epp/sync-1.0}sync:update")]
    VerisignSyncUpdate(verisign::EPPSyncUpdate),
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-domain-1.0}is-domain:create")]
    ISNICDomainCreate(isnic::DomainCreateRenew),
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-domain-1.0}is-domain:renew")]
    ISNICDomainRenew(isnic::DomainCreateRenew),
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-domain-1.0}is-domain:update")]
    ISNICDomainUpdate(isnic::DomainUpdate),
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:create")]
    ISNICContactCreate(isnic::ContactCreate),
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:update")]
    ISNICContactUpdate(isnic::ContactUpdate),
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-host-1.0}is-host:create")]
    ISNICHostCreate(isnic::HostCreateUpdate),
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-host-1.0}is-host:update")]
    ISNICHostUpdate(isnic::HostCreateUpdate),
    #[serde(rename = "{http://www.nic.name/epp/persReg-1.0}persReg:create")]
    PersonalRegistrationCreate(personal_registration::PersonalRegistrationCreate),
}

#[derive(Debug, Serialize)]
pub struct EPPCommand {
    #[serde(rename = "$value")]
    pub command: EPPCommandType,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp-1.0}extension",
        skip_serializing_if = "Option::is_none"
    )]
    pub extension: Option<EPPCommandExtension>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp-1.0}clTRID",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_transaction_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EPPCommandExtension {
    #[serde(rename = "$value")]
    pub value: Vec<EPPCommandExtensionType>,
}

#[derive(Debug, Deserialize)]
pub struct EPPResponse {
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}result")]
    pub results: Vec<EPPResult>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}extension", default)]
    pub extension: Option<EPPResponseExtension>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}msgQ")]
    pub message_queue: Option<EPPMessageQueue>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}resData", default)]
    pub data: Option<EPPResultData>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}trID")]
    pub transaction_id: EPPTransactionIdentifier,
}

#[derive(Debug, Deserialize)]
pub struct EPPResponseExtension {
    #[serde(rename = "$value", default)]
    pub value: Vec<EPPResponseExtensionType>,
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
                    .map(|e| format!("({}) {}", e.value, e.reason))
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
                    match r.values.as_ref().map(|v| {
                        v.iter()
                            .map(|e| {
                                e.iter()
                                    .next()
                                    .map(|(k, v)| format!("{}: {}", k, v))
                                    .unwrap_or_default()
                            })
                            .collect::<Vec<_>>()
                    }) {
                        Some(v) => {
                            output.push(format!("({:?}) {}: {}", r.code, r.message, v.join(", ")));
                        }
                        None => {
                            output.push(format!("({:?}) {}", r.code, r.message));
                        }
                    }
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
        matches!(
            self,
            EPPResultCode::Success
                | EPPResultCode::SuccessActionPending
                | EPPResultCode::SuccessNoMessages
                | EPPResultCode::SuccessAckToDequeue
                | EPPResultCode::SuccessEndingSession
        )
    }

    fn is_closing(&self) -> bool {
        matches!(
            self,
            EPPResultCode::SuccessEndingSession
                | EPPResultCode::CommandFailedServerClosingConnection
                | EPPResultCode::AuthenticationServerClosingConnection
                | EPPResultCode::SessionLimitExceededServerClosingConnection
        )
    }

    fn is_server_error(&self) -> bool {
        matches!(
            self,
            EPPResultCode::CommandFailed
                | EPPResultCode::CommandFailedServerClosingConnection
                | EPPResultCode::AuthenticationServerClosingConnection
                | EPPResultCode::SessionLimitExceededServerClosingConnection
        )
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
    pub value: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}reason")]
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct EPPMessageQueue {
    #[serde(rename = "$attr:count")]
    pub count: u64,
    #[serde(rename = "$attr:id", default)]
    pub id: Option<String>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp-1.0}qDate",
        deserialize_with = "deserialize_datetime_opt",
        default
    )]
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
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}chkData")]
    EPPEmailForwardCheckResult(email_forward::EPPEmailForwardCheckData),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}infData")]
    EPPEmailForwardInfoResult(Box<email_forward::EPPEmailForwardInfoData>),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}trnData")]
    EPPEmailForwardTransferResult(email_forward::EPPEmailForwardTransferData),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}creData")]
    EPPEmailForwardCreateResult(email_forward::EPPEmailForwardCreateData),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}renData")]
    EPPEmailForwardRenewResult(email_forward::EPPEmailForwardRenewData),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}panData")]
    EPPEmailForwardPendingActionNotification(email_forward::EPPEmailForwardPanData),
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
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}panData")]
    EPPContactPendingActionNotification(contact::EPPContactPanData),
    #[serde(rename = "{urn:ietf:params:xml:ns:obj-1.0}trnData")]
    TraficomTrnData(traficom::EPPObjTrnData),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}infData")]
    EPPMaintenanceInfo(maintenance::EPPMaintenanceInfoData),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-tag-1.0}listData")]
    NominetTagInfoResult(nominet::EPPTagListData),
    #[serde(rename = "{https://www.nic.ch/epp/balance-1.0}infData")]
    SwitchBalanceInfoResult(switch::EPPBalance),
    #[serde(rename = "{http://www.verisign.com/epp/balance-1.0}infData")]
    VerisignBalanceInfoResult(verisign::EPPBalance),
    #[serde(rename = "{http://www.unitedtld.com/epp/finance-1.0}infData")]
    UnitedTLDBalaceInfoResult(united_tld::EPPBalance),
    #[serde(rename = "{http://www.verisign.com/epp/rgp-poll-1.0}pollData")]
    VerisignRGPPollData(verisign::EPPRGPPollData),
    #[serde(rename = "{http://www.verisign.com/epp/lowbalance-poll-1.0}pollData")]
    VerisignLowBalanceData(verisign::EPPLowBalanceData),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}cancData")]
    NominetCancelData(nominet::EPPCancelData),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}relData")]
    NominetReleaseData(nominet::EPPReleaseData),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}rcData")]
    NominetRegistrarChangeData(nominet::EPPRegistrarChangeData),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}hostCancData")]
    NominetHostCancelData(nominet::EPPHostCancelData),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}processData")]
    NominetProcessData(nominet::EPPProcessData),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}suspData")]
    NominetSuspendData(nominet::EPPSuspendData),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}domainFailData")]
    NominetDomainFailData(nominet::EPPDomainFailData),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-notifications-1.2}trnData")]
    NominetTransferData(nominet::EPPTransferData),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-handshake-1.0}hanData")]
    NominetHandshakeData(nominet::EPPHandshakeData),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-release-1.0}releasePending")]
    NominetReleasePending(String),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrarFinance-1.0}infData")]
    EURIDRegistrarFinanceData(eurid::EURIDRegistrarFinanceInfoData),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrarHitPoints-1.0}infData")]
    EURIDRegistrarHitPointsData(eurid::EURIDRegistrarHitPointsInfoData),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrationLimit-1.1}infData")]
    EURIDRegistrationLimitData(eurid::EURIDRegistrationLimitInfoData),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/poll-1.2}pollData")]
    EURIDPollData(eurid::EURIDPollData),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/dnsQuality-2.0}dnsQuality:infData")]
    EURIDDNSQualityData(eurid::EURIDDNSQualityInfoData),
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/dnssecEligibility-1.0}dnssecEligibility:infData"
    )]
    EURIDDNSSECEligibilityInfoData(eurid::EURIDDNSSECEligibilityInfoData),
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}infData")]
    ISNICAccountInfo(isnic::AccountInfo),
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
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp-1.0}newPW",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "{urn:ietf:params:xml:ns:epp-1.0}svcExtension",
        skip_serializing_if = "Option::is_none"
    )]
    pub extension: Option<EPPServiceExtension>,
}

#[derive(Debug, Deserialize)]
pub enum EPPResponseExtensionType {
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/contact-nom-ext-1.0}infData")]
    NominetContactExtInfo(nominet::EPPContactInfo),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-warning-1.1}ignored-field")]
    NominetIgnoredField(nominet::EPPIgnoredField),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-warning-1.1}ignored-attribute")]
    NominetIgnoredAttribute(nominet::EPPIgnoredAttribute),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-warning-1.1}postalInfo-ignored")]
    NominetPostalInfoIgnored(nominet::EPPPostalInfoIgnored),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-warning-1.1}truncated-field")]
    NominetTruncatedField(nominet::EPPTruncatedField),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-warning-1.1}host-ignored")]
    NominetHostIgnored(nominet::EPPHostIgnored),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-data-quality-1.1}infData")]
    NominetDataQuality(nominet::EPPDataQualityInfo),
    #[serde(rename = "{http://www.verisign.com/epp/whoisInf-1.0}whoisInfData")]
    VerisignWhoisInfo(verisign::EPPWhoisInfoExtData),
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
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}chkData")]
    EPPFee05CheckData(fee::EPPFee05CheckData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}chkData")]
    EPPFee07CheckData(fee::EPPFee07CheckData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}chkData")]
    EPPFee08CheckData(fee::EPPFee08CheckData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}chkData")]
    EPPFee09CheckData(fee::EPPFee09CheckData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.11}chkData")]
    EPPFee011CheckData(fee::EPPFee011CheckData),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}chkData")]
    EPPFee10CheckData(fee::EPPFee10CheckData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}infData")]
    EPPFee05InfoData(fee::EPPFee05InfoData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}infData")]
    EPPFee07InfoData(fee::EPPFee07InfoData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}creData")]
    EPPFee05CreateData(fee::EPPFee05TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}creData")]
    EPPFee07CreateData(fee::EPPFee07TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}creData")]
    EPPFee08CreateData(fee::EPPFee08TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}creData")]
    EPPFee09CreateData(fee::EPPFee09TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.11}creData")]
    EPPFee011CreateData(fee::EPPFee011TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}creData")]
    EPPFee10CreateData(fee::EPPFee10TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}delData")]
    EPPFee05DeleteData(fee::EPPFee05DeleteData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}delData")]
    EPPFee07DeleteData(fee::EPPFee07DeleteData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}delData")]
    EPPFee08DeleteData(fee::EPPFee08DeleteData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}delData")]
    EPPFee09DeleteData(fee::EPPFee09DeleteData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.11}delData")]
    EPPFee011DeleteData(fee::EPPFee011TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}delData")]
    EPPFee10DeleteData(fee::EPPFee10TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}renData")]
    EPPFee05RenewData(fee::EPPFee05TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}renData")]
    EPPFee07RenewData(fee::EPPFee07TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}renData")]
    EPPFee08RenewData(fee::EPPFee08TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}renData")]
    EPPFee09RenewData(fee::EPPFee09TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.11}renData")]
    EPPFee011RenewData(fee::EPPFee011TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}renData")]
    EPPFee10RenewData(fee::EPPFee10TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}trnData")]
    EPPFee05TransferData(fee::EPPFee05TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}trnData")]
    EPPFee07TransferData(fee::EPPFee07TransferData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}trnData")]
    EPPFee08TransferData(fee::EPPFee08TransferData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}trnData")]
    EPPFee09TransferData(fee::EPPFee09TransferData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.11}trnData")]
    EPPFee011TransferData(fee::EPPFee011TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}trnData")]
    EPPFee10TransferData(fee::EPPFee10TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.5}updData")]
    EPPFee05UpdateData(fee::EPPFee05TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.7}updData")]
    EPPFee07UpdateData(fee::EPPFee07TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.8}updData")]
    EPPFee08UpdateData(fee::EPPFee08TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.9}updData")]
    EPPFee09UpdateData(fee::EPPFee09TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:fee-0.11}updData")]
    EPPFee011UpdateData(fee::EPPFee011TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:fee-1.0}updData")]
    EPPFee10UpdateData(fee::EPPFee10TransformData),
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}chkData")]
    EPPLaunchCheckData(launch::EPPLaunchCheckData),
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}infData")]
    EPPLaunchInfoData(launch::EPPLaunchInfoData),
    #[serde(rename = "{urn:ietf:params:xml:ns:launch-1.0}creData")]
    EPPLaunchCreateData(launch::EPPLaunchCreateData),
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}chkData")]
    EPPDonutsChargeCheckData(united_tld::EPPChargeCheckData),
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}infData")]
    EPPDonutsChargeInfoData(united_tld::EPPChargeData),
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}creData")]
    EPPDonutsChargeCreateData(united_tld::EPPChargeData),
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}trnData")]
    EPPDonutsChargeTransferData(united_tld::EPPChargeData),
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}renData")]
    EPPDonutsChargeRenewData(united_tld::EPPChargeData),
    #[serde(rename = "{http://www.unitedtld.com/epp/charge-1.0}upData")]
    EPPDonutsChargeUpdateData(united_tld::EPPChargeData),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:loginSec-1.0}loginSec:loginSecData")]
    EPPLoginSecurityData(login_sec::EPPLoginSecurityData),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/authInfo-1.1}infData")]
    EURIDAuthInfoData(eurid::EURIDAuthInfoData),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/homoglyph-1.0}chkData")]
    EURIDHomoglyphCheckData(eurid::EURIDHomoglyphData),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/contact-ext-1.3}infData")]
    EURIDContactInfoData(eurid::EURIDContactInfo),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/domain-ext-2.4}chkData")]
    EURIDDomainCheckData(eurid::EURIDDomainCheckData),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/domain-ext-2.4}infData")]
    EURIDDomainInfoData(eurid::EURIDDomainInfo),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/domain-ext-2.4}renData")]
    EURIDDomainRenewData(eurid::EURIDDomainRenewData),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/domain-ext-2.4}trnData")]
    EURIDDomainTransferData(eurid::EURIDDomainTransferData),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/idn-1.0}mapping")]
    EURIDIDNMapping(eurid::EURIDIDNMapping),
    #[serde(rename = "{urn:ietf:params:xml:ns:qualifiedLawyer-1.0}info")]
    QualifiedLawyerInfo(qualified_lawyer::QualifiedLawyerInfoData),
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-domain-1.0}infData")]
    ISNICDomainInfo(isnic::DomainInfo),
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}infData")]
    ISNICContactInfo(isnic::ContactInfo),
    #[serde(rename = "{http://www.nic.name/epp/persReg-1.0}creData")]
    PersonalRegistrationCreateData(personal_registration::PersonalRegistrationCreateData),
    #[serde(rename = "{http://www.nic.name/epp/persReg-1.0}renData")]
    PersonalRegistrationRenewData(personal_registration::PersonalRegistrationCreateData),
    #[serde(rename = "{http://www.nic.name/epp/persReg-1.0}trnData")]
    PersonalRegistrationTransferData(personal_registration::PersonalRegistrationCreateData),
    #[serde(rename = "{http://www.nic.name/epp/persReg-1.0}infData")]
    PersonalRegistrationInfoData(personal_registration::PersonalRegistrationInfoData),
}

#[derive(Debug, Serialize)]
pub enum EPPCheck {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:check")]
    Domain(domain::EPPDomainCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:check")]
    Host(host::EPPHostCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:check")]
    Contact(contact::EPPContactCheck),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:check")]
    EmailForward(email_forward::EPPEmailForwardCheck),
}

#[derive(Debug, Serialize)]
pub enum EPPInfo {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:info")]
    Domain(domain::EPPDomainInfo),
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:info")]
    Host(host::EPPHostCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:info")]
    Contact(contact::EPPContactCheck),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:info")]
    EmailForward(email_forward::EPPEmailForwardCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:epp:maintenance-0.3}maint:info")]
    Maintenance(maintenance::EPPMaintenanceInfo),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/nom-tag-1.0}tag:list")]
    TagList {},
    #[serde(rename = "{https://www.nic.ch/epp/balance-1.0}balance:info")]
    SwitchBalace {},
    #[serde(rename = "{http://www.verisign.com/epp/balance-1.0}balance:info")]
    VerisignBalace {},
    #[serde(rename = "{http://www.unitedtld.com/epp/finance-1.0}finance:info")]
    UnitedTLDBalace {},
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrarFinance-1.0}registrarFinance:info")]
    EURIDRegistrarFinance {},
    #[serde(
        rename = "{http://www.eurid.eu/xml/epp/registrarHitPoints-1.0}registrarHitPoints:info"
    )]
    EURIDRegistrarHitPoints {},
    #[serde(rename = "{http://www.eurid.eu/xml/epp/registrationLimit-1.1}registrationLimit:info")]
    EURIDRegistrationLimit {},
    #[serde(rename = "{http://www.eurid.eu/xml/epp/dnsQuality-2.0}dnsQuality:info")]
    EURIDDNSQuality(eurid::EURIDDNSQualityInfo),
    #[serde(rename = "{http://www.eurid.eu/xml/epp/dnssecEligibility-1.0}dnssecEligibility:info")]
    EURIDDNSSECEligibilityInfo(eurid::EURIDDNSSECEligibilityInfo),
    // #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}is-account:info")]
    // ISNICAccountInfo {},
}

#[derive(Debug, Serialize)]
pub enum EPPCreate {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:create")]
    Host(host::EPPHostCreate),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:create")]
    Contact(Box<contact::EPPContactCreate>),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:create")]
    Domain(domain::EPPDomainCreate),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:create")]
    EmailForward(email_forward::EPPEmailForwardCreate),
}

#[derive(Debug, Serialize)]
pub enum EPPDelete {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:delete")]
    Host(host::EPPHostCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:delete")]
    Contact(contact::EPPContactCheck),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:delete")]
    Domain(domain::EPPDomainCheck),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:delete")]
    EmailForward(email_forward::EPPEmailForwardCheck),
}

#[derive(Debug, Serialize)]
pub enum EPPUpdate {
    #[serde(rename = "{urn:ietf:params:xml:ns:host-1.0}host:update")]
    Host(host::EPPHostUpdate),
    #[serde(rename = "{urn:ietf:params:xml:ns:contact-1.0}contact:update")]
    Contact(contact::EPPContactUpdate),
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:update")]
    Domain(domain::EPPDomainUpdate),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:update")]
    EmailForward(email_forward::EPPEmailForwardUpdate),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-handshake-1.0}handshake:accept")]
    NominetHandshakeAccept(nominet::EPPHandshakeAccept),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-handshake-1.0}handshake:reject")]
    NominetHandshakeReject(nominet::EPPHandshakeReject),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-release-1.0}release:release")]
    NominetRelease(nominet::EPPRelease),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-locks-1.0}lock:lock")]
    NominetLock(nominet::EPPLock),
    #[serde(rename = "{http://www.nominet.org.uk/epp/xml/std-locks-1.0}lock:unlock")]
    NominetUnlock(nominet::EPPLock),
}

#[derive(Debug, Serialize)]
pub enum EPPRenew {
    #[serde(rename = "{urn:ietf:params:xml:ns:domain-1.0}domain:renew")]
    Domain(domain::EPPDomainRenew),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:renew")]
    EmailForward(email_forward::EPPEmailForwardRenew),
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
    #[serde(rename = "approve")]
    Accept,
    #[serde(rename = "reject")]
    Reject,
    #[serde(rename = "cancel")]
    Cancel,
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
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:transfer")]
    EmailForwardQuery(email_forward::EPPEmailForwardCheck),
    #[serde(rename = "{http://www.nic.name/epp/emailFwd-1.0}emailFwd:transfer")]
    EmailForwardRequest(email_forward::EPPEmailForwardTransfer),
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
            },
        }
    }
}

struct DateVisitor;

impl<'de> serde::de::Visitor<'de> for DateVisitor {
    type Value = Date<Utc>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a formatted date string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            Ok(v) => Ok(Date::from_utc(v, Utc)),
            Err(e) => Err(E::custom(e)),
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

struct OptDateVisitor;

impl<'de> serde::de::Visitor<'de> for OptDateVisitor {
    type Value = Option<Date<Utc>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a formatted date string")
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
        d.deserialize_str(DateVisitor).map(Some)
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
        Some(d) => {
            if d == Utc.ymd(1, 1, 1).and_hms(0, 0, 0) {
                None
            } else {
                Some(d)
            }
        }
        None => None,
    })
}

fn deserialize_date_opt<'de, D>(d: D) -> Result<Option<Date<Utc>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let date = d.deserialize_option(OptDateVisitor)?;
    Ok(match date {
        Some(d) => {
            if d == Utc.ymd(1, 1, 1) {
                None
            } else {
                Some(d)
            }
        }
        None => None,
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
fn serialize_date_opt<S>(d: &Option<Date<Utc>>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    match d {
        Some(d) => s.serialize_str(&d.format("%Y-%m-%d").to_string()),
        None => s.serialize_none(),
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_opt_bool<S>(d: &Option<bool>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    match d {
        Some(d) => {
            if *d {
                s.serialize_str("true")
            } else {
                s.serialize_str("false")
            }
        }
        None => s.serialize_none(),
    }
}
