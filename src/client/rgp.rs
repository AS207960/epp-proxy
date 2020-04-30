//! EPP commands relating to nominet specific features

use super::router::HandleReqReturn;
use super::{proto, EPPClientServerFeatures, Request, Response, Sender};

#[derive(Debug)]
pub struct RestoreRequest {
    name: String,
    pub return_path: Sender<RestoreResponse>,
}

/// Response to a RGP query
#[derive(Debug)]
pub struct RestoreResponse {
    pub pending: bool,
    pub state: RGPState,
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

impl From<&proto::rgp::EPPRGPState> for RGPState {
    fn from(from: &proto::rgp::EPPRGPState) -> Self {
        use proto::rgp::EPPRGPState;
        match from {
            EPPRGPState::AddPeriod => RGPState::AddPeriod,
            EPPRGPState::AutoRenewPeriod => RGPState::AutoRenewPeriod,
            EPPRGPState::RenewPeriod => RGPState::RenewPeriod,
            EPPRGPState::TransferPeriod => RGPState::TransferPeriod,
            EPPRGPState::RedemptionPeriod => RGPState::RedemptionPeriod,
            EPPRGPState::PendingRestore => RGPState::PendingRestore,
            EPPRGPState::PendingDelete => RGPState::PendingDelete,
        }
    }
}

pub fn handle_restore(
    client: &EPPClientServerFeatures,
    req: &RestoreRequest,
) -> HandleReqReturn<RestoreResponse> {
    if !(client.rgp_supported || client.has_erratum("traficom")) {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err(
            "domain name has a min length of 1".to_string(),
        ));
    }
    if client.has_erratum("traficom") {
        let command = proto::EPPDelete::Domain(proto::domain::EPPDomainCheck {
            name: req.name.clone(),
        });
        let ext = proto::traficom::EPPDomainDelete::Cancel {};
        Ok((
            proto::EPPCommandType::Delete(command),
            Some(proto::EPPCommandExtensionType::TraficomDelete(ext)),
        ))
    } else {
        let command = proto::EPPUpdate::Domain(proto::domain::EPPDomainUpdate {
            name: req.name.clone(),
            add: None,
            remove: None,
            change: Some(proto::domain::EPPDomainUpdateChange {
                registrant: None,
                auth_info: None,
            }),
        });
        let ext = proto::rgp::EPPRGPUpdate {
            restore: proto::rgp::EPPRGPRestore {
                operation: proto::rgp::EPPRGPRestoreOperation::Request,
                report: None,
            },
        };
        Ok((
            proto::EPPCommandType::Update(Box::new(command)),
            Some(proto::EPPCommandExtensionType::EPPRGPUpdate(ext)),
        ))
    }
}

pub fn handle_restore_response(response: proto::EPPResponse) -> Response<RestoreResponse> {
    match &response.extension {
        Some(value) => match &value.value.first() {
            Some(proto::EPPResponseExtensionType::EPPRGPUpdate(rgp_info)) => {
                Response::Ok(RestoreResponse {
                    pending: response.is_pending(),
                    state: (&rgp_info.state.state).into(),
                })
            }
            _ => Response::Ok(RestoreResponse {
                pending: response.is_pending(),
                state: RGPState::Unknown,
            }),
        },
        None => Response::Ok(RestoreResponse {
            pending: response.is_pending(),
            state: RGPState::Unknown,
        }),
    }
}

/// Checks if a domain name
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn request(
    domain: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<RestoreResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::RestoreRequest(Box::new(RestoreRequest {
            name: domain.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}
