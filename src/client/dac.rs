use super::{CommandResponse, RequestMessage, Sender};
use chrono::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum DACEnv {
    RealTime,
    TimeDelay,
}

#[derive(Debug)]
pub struct DACDomainRequest {
    pub(super) domain: String,
    pub(super) env: DACEnv,
    pub return_path: Sender<DACDomainResponse>,
}

#[derive(Debug)]
pub enum DomainState {
    Registered,
    Available,
    NotWithinRegistry,
    RulesPrevent,
}

#[derive(Debug)]
pub enum DomainStatus {
    Unknown,
    RegisteredUntilExpiry,
    RenewalRequired,
    NoLongerRequired,
}

#[derive(Debug)]
pub struct DACDomainResponse {
    pub registration_state: DomainState,
    pub detagged: bool,
    pub suspended: Option<bool>,
    pub created: Date<Utc>,
    pub expiry: Date<Utc>,
    pub status: DomainStatus,
    pub tag: String,
}

#[derive(Debug)]
pub struct DACUsageRequest {
    pub(super) env: DACEnv,
    pub return_path: Sender<DACUsageResponse>,
}

#[derive(Debug)]
pub struct DACUsageResponse {
    pub usage_60: u64,
    pub usage_24: u64,
}

/// Get the DAC data for a domain
///
/// # Arguments
/// * `name` - The domain to query
/// * `env` - The DAC environment to query
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn domain(
    name: &str,
    env: DACEnv,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<DACDomainResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::DACDomain(Box::new(DACDomainRequest {
            domain: name.to_string(),
            env,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Get the used amount of the limits
///
/// # Arguments
/// * `env` - The DAC environment to query
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn usage(
    env: DACEnv,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<DACUsageResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::DACUsage(Box::new(DACUsageRequest {
            env,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Get the usage limits
///
/// # Arguments
/// * `env` - The DAC environment to query
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn limits(
    env: DACEnv,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<DACUsageResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::DACLimits(Box::new(DACUsageRequest {
            env,
            return_path: sender,
        })),
        receiver,
    )
    .await
}
