//! Commands relating to balance enquiries

use super::{CommandResponse, Request, Sender};

#[derive(Debug)]
pub struct BalanceRequest {
    pub return_path: Sender<BalanceResponse>,
}

#[derive(Debug)]
pub struct BalanceResponse {
    pub balance: String,
    pub currency: String,
    pub credit_limit: Option<String>,
    pub available_credit: Option<String>,
    pub credit_threshold: Option<CreditThreshold>,
}

#[derive(Debug, PartialEq)]
pub enum CreditThreshold {
    Fixed(String),
    Percentage(u8),
}

/// Makes a balance enquiry to the registry
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn balance_info(
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<BalanceResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::Balance(Box::new(BalanceRequest {
            return_path: sender,
        })),
        receiver,
    )
    .await
}
