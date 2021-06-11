//! EPP commands relating to EURid extensions

use super::{CommandResponse, RequestMessage, Sender};
use chrono::prelude::*;

#[derive(Debug)]
pub struct HitPointsRequest {
    pub return_path: Sender<HitPointsResponse>,
}

#[derive(Debug)]
pub struct HitPointsResponse {
    pub hit_points: u64,
    pub max_hit_points: u64,
    pub blocked_until: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct RegistrationLimitRequest {
    pub return_path: Sender<RegistrationLimitResponse>,
}

#[derive(Debug)]
pub struct RegistrationLimitResponse {
    pub monthly_registrations: u64,
    pub max_monthly_registrations: Option<u64>,
    pub limited_until: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct DNSSECEligibilityRequest {
    pub(super) name: String,
    pub return_path: Sender<DNSSECEligibilityResponse>,
}

#[derive(Debug)]
pub struct DNSSECEligibilityResponse {
    pub eligible: bool,
    pub message: String,
    pub code: u32,
    pub idn: Option<Idn>,
}

#[derive(Debug)]
pub struct DNSQualityRequest {
    pub(super) name: String,
    pub return_path: Sender<DNSQualityResponse>,
}

#[derive(Debug)]
pub struct DNSQualityResponse {
    pub check_time: Option<DateTime<Utc>>,
    pub score: String,
    pub idn: Option<Idn>,
}

#[derive(Debug)]
pub struct PollResponse {
    pub context: String,
    pub object_type: String,
    pub object: String,
    pub object_unicode: Option<String>,
    pub action: String,
    pub code: u32,
    pub detail: Option<String>,
    pub registrar: Option<String>,
}

#[derive(Debug)]
pub struct ContactExtension {
    pub contact_type: ContactType,
    pub whois_email: Option<String>,
    pub vat: Option<String>,
    pub citizenship_country: Option<String>,
    pub language: String,
}

#[derive(Debug)]
pub struct ContactExtensionUpdate {
    pub whois_email: Option<String>,
    pub vat: Option<String>,
    pub citizenship_country: Option<String>,
    pub language: Option<String>,
}

pub fn is_entity_natural_person(entity: Option<&super::contact::EntityType>) -> bool {
    match entity {
        Some(e) => matches!(
            e,
            super::contact::EntityType::UkIndividual
                | super::contact::EntityType::FinnishIndividual
                | super::contact::EntityType::OtherIndividual
        ),
        None => true,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContactType {
    Billing,
    Tech,
    Registrant,
    OnSite,
    Reseller,
}

#[derive(Debug)]
pub struct Idn {
    pub ace: String,
    pub unicode: String,
}

#[derive(Debug)]
pub struct DomainCheck {
    pub available_date: Option<DateTime<Utc>>,
    pub status: Vec<super::domain::Status>,
}

#[derive(Debug)]
pub struct DomainCreate {
    pub on_site: Option<String>,
    pub reseller: Option<String>,
}

#[derive(Debug)]
pub struct DomainUpdate {
    pub add_on_site: Option<String>,
    pub add_reseller: Option<String>,
    pub remove_on_site: Option<String>,
    pub remove_reseller: Option<String>,
}

#[derive(Debug)]
pub enum DomainDelete {
    Schedule(DateTime<Utc>),
    Cancel,
}

#[derive(Debug)]
pub struct DomainTransfer {
    pub on_site: Option<String>,
    pub reseller: Option<String>,
    pub technical: Option<String>,
    pub billing: String,
    pub registrant: String,
}

#[derive(Debug)]
pub struct DomainInfoRequest {
    pub auth_info: Option<DomainAuthInfo>,
}

#[derive(Debug)]
pub enum DomainAuthInfo {
    Request,
    Cancel,
}

#[derive(Debug)]
pub struct DomainInfo {
    pub on_hold: bool,
    pub quarantined: bool,
    pub suspended: bool,
    pub delayed: bool,
    pub seized: bool,
    pub deletion_date: Option<DateTime<Utc>>,
    pub on_site: Option<String>,
    pub reseller: Option<String>,
    pub max_extension_period: u32,
    pub registrant_country: String,
    pub registrant_country_of_citizenship: Option<String>,
    pub auth_info_valid_until: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct DomainTransferInfo {
    pub on_hold: bool,
    pub quarantined: bool,
    pub delayed: bool,
    pub reason: String,
    pub registrant: String,
    pub billing: String,
    pub on_site: Option<String>,
    pub technical: Option<String>,
    pub reseller: Option<String>,
}

#[derive(Debug)]
pub struct DomainRenewInfo {
    pub removed_deletion: bool,
}

/// Makes a hit points enquiry to the registry
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn hit_points_info(
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<HitPointsResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EURIDHitPoints(Box::new(HitPointsRequest {
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Makes a registration limits enquiry to the registry
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn registration_limit_info(
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<RegistrationLimitResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EURIDRegistrationLimit(Box::new(RegistrationLimitRequest {
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Makes a DNSSEC discount eligibility enquiry to the registry
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn dnssec_eligibility_info(
    name: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<DNSSECEligibilityResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EURIDDNSSECEligibility(Box::new(DNSSECEligibilityRequest {
            name: name.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Makes a DNS quality enquiry to the registry
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn dns_quality_info(
    name: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<DNSQualityResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::EURIDDNSQuality(Box::new(DNSQualityRequest {
            name: name.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}
