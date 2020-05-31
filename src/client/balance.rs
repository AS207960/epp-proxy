//! EPP commands relating to balance enquiries

use super::router::HandleReqReturn;
use super::{proto, EPPClientServerFeatures, Request, Response, Error, Sender};

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
    pub credit_threshold: Option<CreditThreshold>
}

#[derive(Debug)]
pub enum CreditThreshold {
    Fixed(String),
    Percentage(u8)
}

pub fn handle_balance(
    client: &EPPClientServerFeatures,
    _req: &BalanceRequest,
) -> HandleReqReturn<BalanceResponse> {
    if client.switch_balance {
        Ok((proto::EPPCommandType::Info(proto::EPPInfo::SwitchBalace {}), None))
    } else if client.verisign_balance {
        Ok((proto::EPPCommandType::Info(proto::EPPInfo::VerisignBalace {}), None))
    } else {
        Err(Err(Error::Unsupported))
    }
}

pub fn handle_balance_response(response: proto::EPPResponse) -> Response<BalanceResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::SwitchBalanceInfoResult(switch_balance) => {
                Response::Ok(BalanceResponse {
                    balance: switch_balance.balance,
                    currency: switch_balance.currency,
                    credit_limit: None,
                    available_credit: None,
                    credit_threshold: None
                })
            },
            proto::EPPResultDataValue::VerisignBalanceInfoResult(verisign_balance) => {
                Response::Ok(BalanceResponse {
                    balance: verisign_balance.balance,
                    currency: "USD".to_string(),
                    credit_limit: Some(verisign_balance.credit_limit),
                    available_credit: Some(verisign_balance.available_credit),
                    credit_threshold: Some(match verisign_balance.credit_threshold {
                        proto::verisign::EPPCreditThreshold::Fixed(f) => CreditThreshold::Fixed(f),
                        proto::verisign::EPPCreditThreshold::Percentage(p) => CreditThreshold::Percentage(p)
                    })
                })
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

/// Makes a balance enquiry to the registry
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn balance_info(
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<BalanceResponse, super::Error> {
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
