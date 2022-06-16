//! EPP commands relating to nominet specific features

use chrono::prelude::*;
use super::{fee, CommandResponse, RequestMessage, Sender};

#[derive(Debug)]
pub struct RestoreRequest {
    pub(super) name: String,
    pub(super) donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub return_path: Sender<RestoreResponse>,
}

/// Response to a RGP query
#[derive(Debug)]
pub struct RestoreResponse {
    pub pending: bool,
    pub state: Vec<RGPState>,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
    pub donuts_fee_data: Option<fee::DonutsFeeData>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RGPState {
    Unknown,
    AddPeriod,
    AutoRenewPeriod,
    RenewPeriod,
    TransferPeriod,
    RedemptionPeriod,
    PendingRestore,
    PendingDelete,
}

#[derive(Debug)]
pub struct RestoreReportRequest {
    pub(super) name: String,
    pub(super) pre_data: String,
    pub(super) post_data: String,
    pub(super) deletion_time: DateTime<Utc>,
    pub(super) restore_time: DateTime<Utc>,
    pub(super) restore_reason: String,
    pub(super) statement_1: String,
    pub(super) statement_2: String,
    pub(super) other_information: Option<String>,
    pub(super) donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub return_path: Sender<RestoreReportResponse>,
}

#[derive(Debug)]
pub struct RestoreReportResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
    pub donuts_fee_data: Option<fee::DonutsFeeData>,
}

/// Requests the restartion of a deleted domain name
///
/// # Arguments
/// * `domain` - The domain to restore
/// * `donuts_fee_agreement` - Donuts fee information
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn request(
    domain: &str,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<RestoreResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::RestoreRequest(Box::new(RestoreRequest {
            name: domain.to_string(),
            donuts_fee_agreement,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

pub struct RestoreReportInfo<'a> {
    pub domain: &'a str,
    pub pre_data: &'a str,
    pub post_data: &'a str,
    pub deletion_time: DateTime<Utc>,
    pub restore_time: DateTime<Utc>,
    pub restore_reason: &'a str,
    pub statement_1: &'a str,
    pub statement_2: &'a str,
    pub other_information: Option<&'a str>,
    pub donuts_fee_agreement: Option<fee::DonutsFeeData>,
}

/// Files a follow up report about a domain restore request
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn report(
    info: RestoreReportInfo<'_>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<RestoreReportResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::RestoreReport(Box::new(RestoreReportRequest {
            name: info.domain.to_string(),
            pre_data: info.pre_data.to_string(),
            post_data: info.post_data.to_string(),
            deletion_time: info.deletion_time,
            restore_time: info.restore_time,
            restore_reason: info.restore_reason.to_string(),
            statement_1: info.statement_1.to_string(),
            statement_2: info.statement_2.to_string(),
            other_information: info.other_information.map(|s| s.to_string()),
            donuts_fee_agreement: info.donuts_fee_agreement,
            return_path: sender,
        })),
        receiver,
    )
    .await
}
