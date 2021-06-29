use chrono::prelude::*;
use std::collections::HashMap;

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
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}info")]
    Info(TMCHInfo),
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}login")]
    Login(TMCHLogin),
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}logout")]
    Logout {},
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}poll")]
    Poll(TMCHPoll),
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}renew")]
    Renew(TMCHRenew),
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}transfer")]
    Transfer(TMCHTransfer),
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}update")]
    Update(TMCHUpdate),
}

#[derive(Debug, Serialize)]
pub struct TMCHCheck {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}id")]
    pub id: Vec<String>
}

#[derive(Debug, Serialize)]
pub struct TMCHPeriod {
    #[serde(rename = "$value")]
    pub value: u8,
    #[serde(rename = "$attr:unit")]
    pub unit: TMCHPeriodUnit,
}

#[derive(Debug, Serialize)]
pub enum TMCHPeriodUnit {
    Years
}

#[derive(Debug, Serialize)]
pub struct TMCHCreate {
    // TODO: Add abstract mark field
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<TMCHPeriod>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}document",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub documents: Vec<TMCHDocument>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}label",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<TMCHLabel>,
}

#[derive(Debug, Serialize)]
pub struct TMCHDocument {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}docType")]
    pub document_class: TMCHDocumentClass,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}fileName",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_name: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}fileType")]
    pub file_type: TMCHFileType,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}fileContent")]
    pub file_content: String,
}

#[derive(Debug, Serialize)]
pub enum TMCHDocumentClass {
    #[serde(rename="tmLicenseeDecl")]
    LicenseeDeclaration,
    #[serde(rename="tmAssigneeDecl")]
    AssigneeDeclaration,
    #[serde(rename="tmOther")]
    Other,
    #[serde(rename="declProofOfUseOneSample")]
    DeclarationProofOfUseOneSample,
    #[serde(rename="proofOfUseOther")]
    OtherProofOfUse,
    #[serde(rename="copyOfCourtOrder")]
    CopyOfCourtOrder,
}

#[derive(Debug, Serialize)]
pub enum TMCHFileType {
    #[serde(rename="jpg")]
    JPG,
    #[serde(rename="pdf")]
    PDF
}

#[derive(Debug, Serialize)]
pub struct TMCHLabel {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}aLabel")]
    pub label: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}smdInclusion",
        skip_serializing_if = "Option::is_none"
    )]
    pub smd_inclusion: Option<TMCHNotify>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}claimsNotify",
        skip_serializing_if = "Option::is_none"
    )]
    pub claims_notify: Option<TMCHNotify>
}

#[derive(Debug, Serialize)]
pub struct TMCHNotify {
    #[serde(rename = "$value", default)]
    pub value: String,
    #[serde(rename = "$attr:enable")]
    pub enable: bool,
}


#[derive(Debug, Serialize)]
pub struct TMCHInfo {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}id")]
    pub id: String,
    #[serde(rename = "$attr:type")]
    pub id_type: TMCHInfoType,
}

#[derive(Debug, Serialize)]
pub enum TMCHInfoType {
    #[serde(rename = "enc")]
    EncodedSignedMark,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "smd")]
    SingedMark
}

#[derive(Debug, Serialize)]
pub struct TMCHLogin {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}clID")]
    pub client_id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}pw")]
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct TMCHPoll {
    #[serde(rename = "$attr:op")]
    operation: TMCHPollOperation,
    #[serde(rename = "$attr:msgID")]
    message_id: String,
}

#[derive(Debug, Serialize)]
pub enum TMCHPollOperation {
    #[serde(rename = "ack")]
    Acknowledge,
    #[serde(rename = "req")]
    Request
}

#[derive(Debug, Serialize)]
pub struct TMCHRenew {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}id")]
    pub id: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}curExpDate",
        serialize_with = "super::serialize_date"
    )]
    pub current_expiry_date: Date<Utc>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}period",
        skip_serializing_if = "Option::is_none"
    )]
    pub period: Option<TMCHPeriod>,
}

#[derive(Debug, Serialize)]
pub struct TMCHTransfer {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}id")]
    pub id: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}authCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_code: Option<String>,
    #[serde(rename="$attr:op")]
    pub operation: TMCHTransferOperation
}

#[derive(Debug, Serialize)]
pub enum TMCHTransferOperation {
    #[serde(rename = "initiate")]
    Initiate,
    #[serde(rename = "execute")]
    Execute
}

#[derive(Debug, Serialize)]
pub struct TMCHUpdate {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}id")]
    pub id: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}case",
        skip_serializing_if = "Option::is_none"
    )]
    pub case: Option<TMCHCase>,
    // #[serde(
    //     rename = "{urn:ietf:params:xml:ns:tmch-1.1}add",
    //     skip_serializing_if = "Option::is_none"
    // )]
    // pub add: Option<TMCHAdd>,
    // #[serde(
    //     rename = "{urn:ietf:params:xml:ns:tmch-1.1}rem",
    //     skip_serializing_if = "Option::is_none"
    // )]
    // pub remove: Option<TMCHRemove>,
    // #[serde(
    //     rename = "{urn:ietf:params:xml:ns:tmch-1.1}chg",
    //     skip_serializing_if = "Option::is_none"
    // )]
    // pub change: Option<TMCHChange>,
}

#[derive(Debug, Serialize)]
pub struct TMCHCase {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}id")]
    pub id: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}udrp",
        skip_serializing_if = "Option::is_none"
    )]
    pub udrp: Option<TMCHUDRP>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}court",
        skip_serializing_if = "Option::is_none"
    )]
    pub court: Option<TMCHCourt>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}document",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub documents: Vec<TMCHCaseDocument>,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}label",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<TMCHCaseLabel>,
}

#[derive(Debug, Serialize)]
pub struct TMCHUDRP {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}caseNo")]
    pub case_number: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}udrpProvider")]
    pub udrp_provider: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}caseLang")]
    pub case_language: String,
}

#[derive(Debug, Serialize)]
pub struct TMCHCourt {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}refNum")]
    pub reference: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}cc")]
    pub country_code: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}region",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub regions: Vec<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}courtName")]
    pub court_name: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}caseLang")]
    pub case_language: String,
}


#[derive(Debug, Serialize)]
pub struct TMCHCaseDocument {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}docType")]
    pub document_class: TMCHCaseDocumentClass,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}fileName",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_name: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}fileType")]
    pub file_type: TMCHFileType,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}fileContent")]
    pub file_content: String,
}

#[derive(Debug, Serialize)]
pub enum TMCHCaseDocumentClass {
    #[serde(rename="courtCaseDocument")]
    CourtCaseDocument,
    #[serde(rename="tmOther")]
    Other,
}

#[derive(Debug, Serialize)]
pub struct TMCHCaseLabel {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}aLabel")]
    pub label: String
}

#[derive(Debug, Deserialize)]
pub struct TMCHResponse {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}result")]
    pub results: Vec<TMCHResult>,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}msgQ", default)]
    pub message_queue: Option<TMCHMessageQueue>,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}resData", default)]
    pub data: Option<TMCHResultData>,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}trID")]
    pub transaction_id: TMCHTransactionIdentifier,
}

#[derive(Debug, Deserialize)]
pub struct TMCHTransactionIdentifier {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}clTRID")]
    pub client_transaction_id: Option<String>,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}svTRID")]
    pub server_transaction_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TMCHResult {
    #[serde(rename = "$attr:code")]
    pub code: TMCHResultCode,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}msg")]
    pub message: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}value")]
    pub values: Option<Vec<HashMap<String, String>>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}extValue")]
    pub extra_values: Option<Vec<TMCHResultExtraValue>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TMCHResultCode {
    Success,
    SuccessNoMessages,
    SuccessAckToDequeue,
    SuccessEndingSession,
    CommandSyntaxError,
    AuthorizationError,
    InvalidAuthorization,
    ObjectDoesNotExist,
    ParameterValuePolicyError,
    CommandFailed,
    Other(u16),
}

impl TMCHResultCode {
    fn is_success(&self) -> bool {
        matches!(
            self,
            TMCHResultCode::Success
                | TMCHResultCode::SuccessNoMessages
                | TMCHResultCode::SuccessAckToDequeue
                | TMCHResultCode::SuccessEndingSession
        )
    }

    fn is_closing(&self) -> bool {
        matches!(self, TMCHResultCode::SuccessEndingSession)
    }

    fn is_server_error(&self) -> bool {
        matches!(self, TMCHResultCode::CommandFailed)
    }
}

impl From<u16> for TMCHResultCode {
    fn from(value: u16) -> TMCHResultCode {
        match value {
            1000 => TMCHResultCode::Success,
            1300 => TMCHResultCode::SuccessNoMessages,
            1301 => TMCHResultCode::SuccessAckToDequeue,
            1500 => TMCHResultCode::SuccessEndingSession,
            2001 => TMCHResultCode::CommandSyntaxError,
            2201 => TMCHResultCode::AuthorizationError,
            2202 => TMCHResultCode::InvalidAuthorization,
            2303 => TMCHResultCode::ObjectDoesNotExist,
            2306 => TMCHResultCode::ParameterValuePolicyError,
            2400 => TMCHResultCode::CommandFailed,
            o => TMCHResultCode::Other(o),
        }
    }
}

impl<'de> serde::Deserialize<'de> for TMCHResultCode {
    fn deserialize<D>(deserializer: D) -> Result<TMCHResultCode, D::Error>
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
            Ok(i) => Ok(TMCHResultCode::from(i)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TMCHResultExtraValue {
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}value")]
    pub value: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:tmch-1.1}reason")]
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct TMCHMessageQueue {
    #[serde(rename = "$attr:count")]
    pub count: u64,
    #[serde(rename = "$attr:id")]
    pub id: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:tmch-1.1}qDate",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub enqueue_date: Option<DateTime<Utc>>,
    #[serde(rename = "{urn:ietf:params:xml:ns:epp-1.0}msg")]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TMCHResultData {
    #[serde(rename = "$value")]
    pub value: TMCHResultDataValue,
}

#[derive(Debug, Deserialize)]
pub enum TMCHResultDataValue {

}