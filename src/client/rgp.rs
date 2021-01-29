//! EPP commands relating to nominet specific features

use super::router::HandleReqReturn;
use super::{proto, fee, EPPClientServerFeatures, Error, Request, Response, Sender};

#[derive(Debug)]
pub struct RestoreRequest {
    name: String,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
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
        return Err(Err(Error::Unsupported));
    }
    super::domain::check_domain(&req.name)?;
    if client.has_erratum("traficom") {
        let command = proto::EPPDelete::Domain(proto::domain::EPPDomainCheck {
            name: req.name.clone(),
            auth_info: None
        });
        let ext = proto::traficom::EPPDomainDelete::Cancel {};
        Ok((
            proto::EPPCommandType::Delete(command),
            Some(vec![proto::EPPCommandExtensionType::TraficomDelete(ext)]),
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
        let mut exts = vec![proto::EPPCommandExtensionType::EPPRGPUpdate(
            proto::rgp::EPPRGPUpdate {
                restore: proto::rgp::EPPRGPRestore {
                    operation: proto::rgp::EPPRGPRestoreOperation::Request,
                    report: None,
                },
            },
        )];
        super::verisign::handle_verisign_namestore_erratum(client, &mut exts);
        super::fee::handle_donuts_fee_agreement(client, &req.donuts_fee_agreement, &mut exts)?;

        Ok((proto::EPPCommandType::Update(Box::new(command)), Some(exts)))
    }
}

pub fn handle_restore_response(response: proto::EPPResponse) -> Response<RestoreResponse> {
    let fee_data = match &response.extension {
        Some(ext) => {
            let fee10 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee10UpdateData(i) => Some(i),
                _ => None,
            });
            let fee09 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee09UpdateData(i) => Some(i),
                _ => None,
            });
            let fee08 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee08UpdateData(i) => Some(i),
                _ => None,
            });
            let fee07 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee07UpdateData(i) => Some(i),
                _ => None,
            });
            let fee05 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee05UpdateData(i) => Some(i),
                _ => None,
            });

            if let Some(f) = fee10 {
                Some(f.into())
            } else if let Some(f) = fee09 {
                Some(f.into())
            } else if let Some(f) = fee08 {
                Some(f.into())
            } else if let Some(f) = fee07 {
                Some(f.into())
            } else if let Some(f) = fee05 {
                Some(f.into())
            } else {
                None
            }
        },
        None => None,
    };

    match &response.extension {
        Some(value) => match &value.value.first() {
            Some(proto::EPPResponseExtensionType::EPPRGPUpdate(rgp_info)) => {
                Response::Ok(RestoreResponse {
                    pending: response.is_pending(),
                    transaction_id: response.transaction_id.server_transaction_id.unwrap_or_default(),
                    state: rgp_info.state.iter().map(|s| (&s.state).into()).collect(),
                    fee_data,
                })
            }
            _ => Response::Ok(RestoreResponse {
                pending: response.is_pending(),
                transaction_id: response.transaction_id.server_transaction_id.unwrap_or_default(),
                state: vec![],
                fee_data,
            }),
        },
        None => Response::Ok(RestoreResponse {
            pending: response.is_pending(),
            transaction_id: response.transaction_id.server_transaction_id.unwrap_or_default(),
            state: vec![],
            fee_data,
        }),
    }
}

/// Checks if a domain name
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn request(
    domain: &str,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<RestoreResponse, super::Error> {
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
