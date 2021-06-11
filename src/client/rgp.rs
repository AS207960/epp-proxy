//! EPP commands relating to nominet specific features

use super::{fee, CommandResponse, Request, Sender};

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
    pub transaction_id: String,
    pub state: Vec<RGPState>,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
}

#[derive(Debug)]
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

/// Checks if a domain name
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn request(
    domain: &str,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<RestoreResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::RestoreRequest(Box::new(RestoreRequest {
            name: domain.to_string(),
            donuts_fee_agreement,
            return_path: sender,
        })),
        receiver,
    )
    .await
}
