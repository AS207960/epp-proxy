//! EPP commands relating to nominet specific features

use super::super::rgp::{
    RGPState, RestoreReportRequest, RestoreReportResponse, RestoreRequest, RestoreResponse,
};
use super::super::{proto, Error, Response};
use super::router::HandleReqReturn;
use super::ServerFeatures;

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
    client: &ServerFeatures,
    req: &RestoreRequest,
) -> HandleReqReturn<RestoreResponse> {
    if !(client.rgp_supported || client.has_erratum("traficom")) {
        return Err(Err(Error::Unsupported));
    }
    super::domain::check_domain(&req.name)?;
    if client.has_erratum("traficom") {
        let command = proto::EPPDelete::Domain(proto::domain::EPPDomainCheck {
            name: req.name.clone(),
            auth_info: None,
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

pub fn handle_restore_response<M: crate::metrics::Metrics>(
    response: proto::EPPResponse, _metrics: &M
) -> Response<RestoreResponse> {
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
            } else {
                fee05.map(|f| f.into())
            }
        }
        None => None,
    };

    let donuts_fee_data = match &response.extension {
        Some(ext) => {
            let charge = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPDonutsChargeInfoData(i) => Some(i),
                _ => None,
            });

            charge.map(Into::into)
        }
        None => None,
    };

    match &response.extension {
        Some(value) => match &value.value.first() {
            Some(proto::EPPResponseExtensionType::EPPRGPUpdate(rgp_info)) => {
                Response::Ok(RestoreResponse {
                    pending: response.is_pending(),
                    state: rgp_info.state.iter().map(|s| (&s.state).into()).collect(),
                    fee_data,
                    donuts_fee_data,
                })
            }
            _ => Response::Ok(RestoreResponse {
                pending: response.is_pending(),
                state: vec![],
                fee_data,
                donuts_fee_data,
            }),
        },
        None => Response::Ok(RestoreResponse {
            pending: response.is_pending(),
            state: vec![],
            fee_data,
            donuts_fee_data,
        }),
    }
}

pub fn handle_restore_report(
    client: &ServerFeatures,
    req: &RestoreReportRequest,
) -> HandleReqReturn<RestoreReportResponse> {
    if !(client.rgp_supported) {
        return Err(Err(Error::Unsupported));
    }
    super::domain::check_domain(&req.name)?;
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
                operation: proto::rgp::EPPRGPRestoreOperation::Report,
                report: Some(proto::rgp::EPPRGPReport {
                    pre_data: req.pre_data.clone(),
                    post_data: req.post_data.clone(),
                    delete_time: req.deletion_time,
                    restore_time: req.restore_time,
                    restore_reason: req.restore_reason.clone(),
                    statement: vec![req.statement_1.clone(), req.statement_2.clone()],
                    other: req.other_information.clone(),
                }),
            },
        },
    )];
    super::verisign::handle_verisign_namestore_erratum(client, &mut exts);
    super::fee::handle_donuts_fee_agreement(client, &req.donuts_fee_agreement, &mut exts)?;

    Ok((proto::EPPCommandType::Update(Box::new(command)), Some(exts)))
}

pub fn handle_restore_report_response<M: crate::metrics::Metrics>(
    response: proto::EPPResponse, _metrics: &M
) -> Response<RestoreReportResponse> {
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
            } else {
                fee05.map(|f| f.into())
            }
        }
        None => None,
    };

    let donuts_fee_data = match &response.extension {
        Some(ext) => {
            let charge = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPDonutsChargeInfoData(i) => Some(i),
                _ => None,
            });

            charge.map(Into::into)
        }
        None => None,
    };

    Response::Ok(RestoreReportResponse {
        pending: response.is_pending(),
        fee_data,
        donuts_fee_data,
    })
}
