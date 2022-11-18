//! EPP commands relating to domain objects

use std::convert::{TryFrom, TryInto};

use super::super::domain::InfoContact;
use super::super::email_forward::{
    CheckRequest, CheckResponse, CreateData, CreateRequest, CreateResponse, DeleteRequest,
    DeleteResponse, InfoRequest, InfoResponse, PanData, RenewData, RenewRequest, RenewResponse,
    TransferAcceptRejectRequest, TransferData, TransferQueryRequest, TransferRequestRequest,
    TransferResponse, UpdateObject, UpdateRequest, UpdateResponse,
};
use super::super::{fee, proto, Error, Response};
use super::router::HandleReqReturn;
use super::ServerFeatures;

impl
    TryFrom<(
        proto::email_forward::EPPEmailForwardInfoData,
        &Option<proto::EPPResponseExtension>,
    )> for InfoResponse
{
    type Error = Error;

    fn try_from(
        from: (
            proto::email_forward::EPPEmailForwardInfoData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (email_forward_info, extension) = from;
        let rgp_state = match extension {
            Some(ext) => match ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPRGPInfo(i) => Some(i),
                _ => None,
            }) {
                Some(e) => e.state.iter().map(|s| (&s.state).into()).collect(),
                None => vec![],
            },
            None => vec![],
        };

        let whois_info = match extension {
            Some(ext) => {
                let i = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::VerisignWhoisInfo(i) => Some(i),
                    _ => None,
                });
                i.map(Into::into)
            }
            None => None,
        };

        let personal_registration = match extension {
            Some(ext) => {
                let i = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::PersonalRegistrationInfoData(i) => Some(i),
                    _ => None,
                });
                i.map(Into::into)
            }
            None => None,
        };

        Ok(InfoResponse {
            name: email_forward_info.name,
            registry_id: email_forward_info.registry_id,
            statuses: email_forward_info
                .statuses
                .into_iter()
                .map(|s| s.status.into())
                .collect(),
            registrant: email_forward_info.registrant.unwrap_or_default(),
            contacts: email_forward_info
                .contacts
                .into_iter()
                .map(|c| InfoContact {
                    contact_id: c.contact_id,
                    contact_type: c.contact_type,
                })
                .collect(),

            forward_to: email_forward_info.forward_to.unwrap_or_default(),
            client_id: email_forward_info.client_id,
            client_created_id: email_forward_info.client_created_id,
            creation_date: email_forward_info.creation_date,
            expiry_date: email_forward_info.expiry_date,
            last_updated_client: email_forward_info.last_updated_client,
            last_updated_date: email_forward_info.last_updated_date,
            last_transfer_date: email_forward_info.last_transfer_date,
            auth_info: match email_forward_info.auth_info {
                Some(a) => a.password,
                None => None,
            },
            rgp_state,
            whois_info,
            personal_registration,
        })
    }
}

impl
    TryFrom<(
        proto::email_forward::EPPEmailForwardTransferData,
        &Option<proto::EPPResponseExtension>,
    )> for TransferResponse
{
    type Error = Error;

    fn try_from(
        from: (
            proto::email_forward::EPPEmailForwardTransferData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (email_forward_transfer, extension) = from;

        let fee_data = match extension {
            Some(ext) => {
                let fee10 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee10TransferData(i) => Some(i),
                    _ => None,
                });
                let fee011 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee011TransferData(i) => Some(i),
                    _ => None,
                });
                let fee09 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee09TransferData(i) => Some(i),
                    _ => None,
                });
                let fee08 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee08TransferData(i) => Some(i),
                    _ => None,
                });
                let fee07 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee07TransferData(i) => Some(i),
                    _ => None,
                });
                let fee05 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee05TransferData(i) => Some(i),
                    _ => None,
                });

                if let Some(f) = fee10 {
                    Some(f.into())
                } else if let Some(f) = fee011 {
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

        let personal_registration = match extension {
            Some(ext) => {
                let i = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::PersonalRegistrationTransferData(i) => Some(i),
                    _ => None,
                });
                i.map(Into::into)
            }
            None => None,
        };

        Ok(TransferResponse {
            pending: false,
            data: TransferData {
                name: email_forward_transfer.name.clone(),
                status: (&email_forward_transfer.transfer_status).into(),
                requested_client_id: email_forward_transfer.requested_client_id.clone(),
                requested_date: email_forward_transfer.requested_date,
                act_client_id: email_forward_transfer.act_client_id.clone(),
                act_date: email_forward_transfer.act_date,
                expiry_date: email_forward_transfer.expiry_date,
                personal_registration,
            },
            fee_data,
        })
    }
}

impl
    TryFrom<(
        Option<proto::email_forward::EPPEmailForwardCreateData>,
        &Option<proto::EPPResponseExtension>,
    )> for CreateResponse
{
    type Error = Error;

    fn try_from(
        from: (
            Option<proto::email_forward::EPPEmailForwardCreateData>,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (email_forward_create, extension) = from;

        let fee_data = match extension {
            Some(ext) => {
                let fee10 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee10CreateData(i) => Some(i),
                    _ => None,
                });
                let fee011 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee011CreateData(i) => Some(i),
                    _ => None,
                });
                let fee09 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee09CreateData(i) => Some(i),
                    _ => None,
                });
                let fee08 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee08CreateData(i) => Some(i),
                    _ => None,
                });
                let fee07 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee07CreateData(i) => Some(i),
                    _ => None,
                });
                let fee05 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee05CreateData(i) => Some(i),
                    _ => None,
                });

                if let Some(f) = fee10 {
                    Some(f.into())
                } else if let Some(f) = fee011 {
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

        let personal_registration = match extension {
            Some(ext) => {
                let i = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::PersonalRegistrationCreateData(i) => Some(i),
                    _ => None,
                });
                i.map(Into::into)
            }
            None => None,
        };

        match email_forward_create {
            Some(email_forward_create) => Ok(CreateResponse {
                pending: false,
                data: CreateData {
                    name: email_forward_create.name.clone(),
                    creation_date: Some(email_forward_create.creation_date),
                    expiration_date: email_forward_create.expiry_date,
                    personal_registration,
                },
                fee_data,
            }),
            None => Ok(CreateResponse {
                pending: false,
                data: CreateData {
                    name: "".to_string(),
                    creation_date: None,
                    expiration_date: None,
                    personal_registration,
                },
                fee_data,
            }),
        }
    }
}

impl
    TryFrom<(
        proto::email_forward::EPPEmailForwardRenewData,
        &Option<proto::EPPResponseExtension>,
    )> for RenewResponse
{
    type Error = Error;

    fn try_from(
        from: (
            proto::email_forward::EPPEmailForwardRenewData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (email_forward_renew, extension) = from;

        let fee_data = match extension {
            Some(ext) => {
                let fee10 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee10RenewData(i) => Some(i),
                    _ => None,
                });
                let fee011 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee011RenewData(i) => Some(i),
                    _ => None,
                });
                let fee09 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee09RenewData(i) => Some(i),
                    _ => None,
                });
                let fee08 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee08RenewData(i) => Some(i),
                    _ => None,
                });
                let fee07 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee07RenewData(i) => Some(i),
                    _ => None,
                });
                let fee05 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee05RenewData(i) => Some(i),
                    _ => None,
                });

                if let Some(f) = fee10 {
                    Some(f.into())
                } else if let Some(f) = fee011 {
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

        let personal_registration = match extension {
            Some(ext) => {
                let i = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::PersonalRegistrationRenewData(i) => Some(i),
                    _ => None,
                });
                i.map(Into::into)
            }
            None => None,
        };

        Ok(RenewResponse {
            pending: false,
            data: RenewData {
                name: email_forward_renew.name.to_owned(),
                new_expiry_date: email_forward_renew.expiry_date,
                personal_registration,
            },
            fee_data,
        })
    }
}

impl From<&proto::email_forward::EPPEmailForwardPanData> for PanData {
    fn from(from: &proto::email_forward::EPPEmailForwardPanData) -> Self {
        PanData {
            name: from.name.domain.clone(),
            result: from.name.result,
            server_transaction_id: from.transaction_id.server_transaction_id.clone(),
            client_transaction_id: from.transaction_id.client_transaction_id.clone(),
            date: from.action_date,
        }
    }
}

pub(crate) fn check_email<T>(id: &str) -> Result<(), Response<T>> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new("^.+@.+$").unwrap();
    }
    if RE.is_match(id) {
        Ok(())
    } else {
        Err(Err(Error::Err("invalid email".to_string())))
    }
}

pub(crate) fn check_pass<T>(id: &str) -> Result<(), Response<T>> {
    if let 6..=32 = id.len() {
        Ok(())
    } else {
        Err(Err(Error::Err(
            "passwords have a min length of 6 and a max length of 32".to_string(),
        )))
    }
}

pub fn handle_check(client: &ServerFeatures, req: &CheckRequest) -> HandleReqReturn<CheckResponse> {
    if !client.email_forward_supported {
        return Err(Err(Error::Unsupported));
    }
    check_email(&req.name)?;
    let command = proto::EPPCheck::EmailForward(proto::email_forward::EPPEmailForwardCheck {
        name: req.name.clone(),
        auth_info: None,
    });
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    if let Some(fee_check) = &req.fee_check {
        if client.fee_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee10Check(
                proto::fee::EPPFee10Check {
                    currency: fee_check.currency.to_owned(),
                    commands: fee_check
                        .commands
                        .iter()
                        .map(|c| {
                            Ok(proto::fee::EPPFee10CheckCommand {
                                name: proto::fee::EPPFeeCommand {
                                    command: match (&c.command).into() {
                                        Some(n) => n,
                                        None => return Err(Err(Error::Unsupported)),
                                    },
                                    phase: None,
                                    subphase: None
                                },
                                period: c.period.as_ref().map(Into::into),
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                },
            ))
        } else if client.fee_011_supported {
            fee_check
                .commands
                .iter()
                .map(|c| {
                    ext.push(proto::EPPCommandExtensionType::EPPFee011Check(
                        proto::fee::EPPFee011Check {
                            currency: fee_check.currency.to_owned(),
                            command: proto::fee::EPPFeeCommand {
                                command: match (&c.command).into() {
                                    Some(n) => n,
                                    None => return Err(Err(Error::Unsupported)),
                                },
                                phase: None,
                                subphase: None
                            },
                            period: c.period.as_ref().map(Into::into),
                        },
                    ));
                    Ok(())
                })
                .collect::<Result<Vec<_>, _>>()?;
        } else if client.fee_09_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee09Check(
                proto::fee::EPPFee09Check {
                    objects: fee_check
                        .commands
                        .iter()
                        .map(|c| {
                            Ok(proto::fee::EPPFee09CheckObject {
                                object_uri: Some(
                                    "http://www.nic.name/epp/emailFwd-1.0".to_string(),
                                ),
                                object_id: proto::fee::EPPFee10ObjectID {
                                    element: "name".to_string(),
                                    id: req.name.to_owned(),
                                },
                                currency: fee_check.currency.to_owned(),
                                command: proto::fee::EPPFeeCommand {
                                    command: match (&c.command).into() {
                                        Some(n) => n,
                                        None => return Err(Err(Error::Unsupported)),
                                    },
                                    phase: None,
                                    subphase: None
                                },
                                period: c.period.as_ref().map(Into::into),
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                },
            ))
        } else if client.fee_08_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee08Check(
                proto::fee::EPPFee08Check {
                    domains: fee_check
                        .commands
                        .iter()
                        .map(|c| {
                            Ok(proto::fee::EPPFee08CheckDomain {
                                name: req.name.to_owned(),
                                currency: fee_check.currency.to_owned(),
                                command: proto::fee::EPPFeeCommand {
                                    command: match (&c.command).into() {
                                        Some(n) => n,
                                        None => return Err(Err(Error::Unsupported)),
                                    },
                                    phase: None,
                                    subphase: None
                                },
                                period: c.period.as_ref().map(Into::into),
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                },
            ))
        } else if client.fee_07_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee07Check(
                proto::fee::EPPFee07Check {
                    domains: fee_check
                        .commands
                        .iter()
                        .map(|c| {
                            Ok(proto::fee::EPPFee07CheckDomain {
                                name: req.name.to_owned(),
                                currency: fee_check.currency.to_owned(),
                                command: proto::fee::EPPFeeCommand {
                                    command: match (&c.command).into() {
                                        Some(n) => n,
                                        None => return Err(Err(Error::Unsupported)),
                                    },
                                    phase: None,
                                    subphase: None
                                },
                                period: c.period.as_ref().map(Into::into),
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                },
            ))
        } else if client.fee_05_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee05Check(
                proto::fee::EPPFee05Check {
                    domains: fee_check
                        .commands
                        .iter()
                        .map(|c| {
                            Ok(proto::fee::EPPFee05CheckDomain {
                                name: req.name.to_owned(),
                                currency: fee_check.currency.to_owned(),
                                command: proto::fee::EPPFeeCommand {
                                    command: match (&c.command).into() {
                                        Some(n) => n,
                                        None => return Err(Err(Error::Unsupported)),
                                    },
                                    phase: None,
                                    subphase: None
                                },
                                period: c.period.as_ref().map(Into::into),
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                },
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    Ok((
        proto::EPPCommandType::Check(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_check_response(response: proto::EPPResponse) -> Response<CheckResponse> {
    let fee_check = match &response.extension {
        Some(ext) => {
            let fee10 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee10CheckData(i) => Some(i),
                _ => None,
            });
            let fee011 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee011CheckData(i) => Some(i),
                _ => None,
            });
            let fee09 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee09CheckData(i) => Some(i),
                _ => None,
            });
            let fee08 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee08CheckData(i) => Some(i),
                _ => None,
            });
            let fee07 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee07CheckData(i) => Some(i),
                _ => None,
            });
            let fee05 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee05CheckData(i) => Some(i),
                _ => None,
            });

            if let Some(f) = fee10 {
                let d = match f.objects.get(0) {
                    Some(o) => o,
                    None => return Err(Error::ServerInternal),
                };
                Some(fee::FeeCheckData {
                    available: d.available,
                    commands: d
                        .commands
                        .iter()
                        .map(|c| fee::FeeCommand {
                            command: (&c.name.command).into(),
                            period: c.period.as_ref().map(Into::into),
                            standard: Some(c.standard),
                            currency: f.currency.to_owned(),
                            fees: c.fee.iter().map(Into::into).collect(),
                            credits: c.credit.iter().map(Into::into).collect(),
                            reason: c.reason.to_owned(),
                            class: d.class.to_owned(),
                        })
                        .collect(),
                    reason: d.reason.to_owned(),
                })
            } else if let Some(f) = fee011 {
                let d = match f.objects.get(0) {
                    Some(o) => o,
                    None => return Err(Error::ServerInternal),
                };
                Some(fee::FeeCheckData {
                    available: d.available,
                    commands: f
                        .objects
                        .iter()
                        .map(|c| fee::FeeCommand {
                            command: (&c.command.name.command).into(),
                            period: c.period.as_ref().map(Into::into),
                            standard: Some(c.command.standard),
                            currency: c.currency.to_owned(),
                            fees: c.fee.iter().map(Into::into).collect(),
                            credits: c.credit.iter().map(Into::into).collect(),
                            reason: c.reason.to_owned(),
                            class: c.class.to_owned(),
                        })
                        .collect(),
                    reason: d.reason.to_owned(),
                })
            } else if let Some(f) = fee09 {
                Some(fee::FeeCheckData {
                    available: true,
                    commands: f
                        .objects
                        .iter()
                        .map(|d| fee::FeeCommand {
                            command: (&d.command.command).into(),
                            period: d.period.as_ref().map(Into::into),
                            standard: None,
                            currency: d.currency.to_owned(),
                            fees: d.fee.iter().map(Into::into).collect(),
                            credits: d.credit.iter().map(Into::into).collect(),
                            class: d.class.to_owned(),
                            reason: None,
                        })
                        .collect(),
                    reason: None,
                })
            } else if let Some(f) = fee08 {
                Some(fee::FeeCheckData {
                    available: true,
                    commands: f
                        .domains
                        .iter()
                        .map(|d| fee::FeeCommand {
                            command: (&d.command.command).into(),
                            period: d.period.as_ref().map(Into::into),
                            standard: None,
                            currency: d.currency.to_owned(),
                            fees: d.fee.iter().map(Into::into).collect(),
                            credits: d.credit.iter().map(Into::into).collect(),
                            class: d.class.to_owned(),
                            reason: None,
                        })
                        .collect(),
                    reason: None,
                })
            } else if let Some(f) = fee07 {
                Some(fee::FeeCheckData {
                    available: true,
                    commands: f
                        .domains
                        .iter()
                        .map(|d| fee::FeeCommand {
                            command: (&d.command.command).into(),
                            period: d.period.as_ref().map(Into::into),
                            standard: None,
                            currency: d.currency.to_owned().unwrap_or_default(),
                            fees: d.fee.iter().map(Into::into).collect(),
                            credits: d.credit.iter().map(Into::into).collect(),
                            class: d.class.to_owned(),
                            reason: None,
                        })
                        .collect(),
                    reason: None,
                })
            } else {
                fee05.map(|f| fee::FeeCheckData {
                    available: true,
                    commands: f
                        .domains
                        .iter()
                        .map(|d| fee::FeeCommand {
                            command: (&d.command.command).into(),
                            period: Some((&d.period).into()),
                            standard: None,
                            currency: d.currency.to_owned(),
                            fees: d.fee.iter().map(Into::into).collect(),
                            class: d.class.to_owned(),
                            credits: vec![],
                            reason: None,
                        })
                        .collect(),
                    reason: None,
                })
            }
        }
        None => None,
    };

    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPEmailForwardCheckResult(email_forward_check) => {
                if let Some(email_forward_check) = email_forward_check.data.first() {
                    Response::Ok(CheckResponse {
                        avail: email_forward_check.name.available,
                        reason: email_forward_check.reason.to_owned(),
                        fee_check,
                    })
                } else {
                    Err(Error::ServerInternal)
                }
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_info(client: &ServerFeatures, req: &InfoRequest) -> HandleReqReturn<InfoResponse> {
    if !client.email_forward_supported {
        return Err(Err(Error::Unsupported));
    }
    check_email(&req.name)?;
    let command = proto::EPPInfo::EmailForward(proto::email_forward::EPPEmailForwardCheck {
        name: req.name.clone(),
        auth_info: req
            .auth_info
            .as_ref()
            .map(|a| proto::email_forward::EPPEmailForwardAuthInfo {
                password: Some(a.clone()),
            }),
    });
    let mut exts = vec![];
    if client.verisign_whois_info {
        exts.push(proto::EPPCommandExtensionType::VerisignWhoisInfExt(
            proto::verisign::EPPWhoisInfoExt { flag: true },
        ))
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut exts);
    Ok((
        proto::EPPCommandType::Info(command),
        match exts.is_empty() {
            true => None,
            false => Some(exts),
        },
    ))
}

pub fn handle_info_response(response: proto::EPPResponse) -> Response<InfoResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPEmailForwardInfoResult(email_forward_info) => {
                (*email_forward_info, &response.extension).try_into()
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_create(
    client: &ServerFeatures,
    req: &CreateRequest,
) -> HandleReqReturn<CreateResponse> {
    if !client.email_forward_supported {
        return Err(Err(Error::Unsupported));
    }
    check_email(&req.name)?;
    check_email(&req.forward_to)?;
    let no_registrant = client.has_erratum("verisign-com")
        || client.has_erratum("verisign-net")
        || client.has_erratum("verisign-cc")
        || client.has_erratum("verisign-tv");
    if !no_registrant {
        super::contact::check_id(&req.registrant)?;
    }

    let mut exts = vec![];

    if let Some(fee_agreement) = &req.fee_agreement {
        if client.fee_supported {
            exts.push(proto::EPPCommandExtensionType::EPPFee10Create(
                fee_agreement.into(),
            ));
        } else if client.fee_011_supported {
            exts.push(proto::EPPCommandExtensionType::EPPFee011Create(
                fee_agreement.into(),
            ));
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    if let Some(personal_registration_data) = &req.personal_registration {
        if client.personal_registration_supported {
            exts.push(proto::EPPCommandExtensionType::PersonalRegistrationCreate(
                personal_registration_data.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut exts);

    if !req.auth_info.is_empty() {
        check_pass(&req.auth_info)?;
    }

    let command = proto::EPPCreate::EmailForward(proto::email_forward::EPPEmailForwardCreate {
        name: req.name.clone(),
        period: req.period.as_ref().map(|p| p.into()),
        registrant: if no_registrant {
            None
        } else {
            Some(req.registrant.to_string())
        },
        forward_to: req.forward_to.to_string(),
        contacts: req
            .contacts
            .iter()
            .map(|c| {
                super::contact::check_id(&c.contact_id)?;
                Ok(proto::domain::EPPDomainInfoContact {
                    contact_type: c.contact_type.to_string(),
                    contact_id: c.contact_id.to_string(),
                })
            })
            .collect::<Result<Vec<_>, super::router::Response<CreateResponse>>>()?,
        auth_info: proto::email_forward::EPPEmailForwardAuthInfo {
            password: Some(req.auth_info.to_string()),
        },
    });
    Ok((
        proto::EPPCommandType::Create(command),
        match exts.len() {
            0 => None,
            _ => Some(exts),
        },
    ))
}

pub fn handle_create_response(response: proto::EPPResponse) -> Response<CreateResponse> {
    let pending = response.is_pending();
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPEmailForwardCreateResult(email_forward_create) => {
                let mut res: CreateResponse =
                    (Some(email_forward_create), &response.extension).try_into()?;
                res.pending = pending;
                Ok(res)
            }
            _ => Err(Error::ServerInternal),
        },
        None => {
            if response.is_pending() {
                let mut res: CreateResponse = (None, &response.extension).try_into()?;
                res.pending = response.is_pending();
                Ok(res)
            } else {
                Err(Error::ServerInternal)
            }
        }
    }
}

pub fn handle_delete(
    client: &ServerFeatures,
    req: &DeleteRequest,
) -> HandleReqReturn<DeleteResponse> {
    if !client.email_forward_supported {
        return Err(Err(Error::Unsupported));
    }
    check_email(&req.name)?;
    let command = proto::EPPDelete::EmailForward(proto::email_forward::EPPEmailForwardCheck {
        name: req.name.clone(),
        auth_info: None,
    });
    let mut ext = vec![];

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);

    Ok((
        proto::EPPCommandType::Delete(command),
        match ext.len() {
            0 => None,
            _ => Some(ext),
        },
    ))
}

pub fn handle_delete_response(response: proto::EPPResponse) -> Response<DeleteResponse> {
    let fee_data = match &response.extension {
        Some(ext) => {
            let fee10 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee10DeleteData(i) => Some(i),
                _ => None,
            });
            let fee09 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee09DeleteData(i) => Some(i),
                _ => None,
            });
            let fee08 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee08DeleteData(i) => Some(i),
                _ => None,
            });
            let fee07 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee07DeleteData(i) => Some(i),
                _ => None,
            });
            let fee05 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee05DeleteData(i) => Some(i),
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

    Response::Ok(DeleteResponse {
        pending: response.is_pending(),
        fee_data,
    })
}

pub fn handle_update(
    client: &ServerFeatures,
    req: &UpdateRequest,
) -> HandleReqReturn<UpdateResponse> {
    if !client.email_forward_supported {
        return Err(Err(Error::Unsupported));
    }
    check_email(&req.name)?;

    let no_registrant = client.has_erratum("verisign-com")
        || client.has_erratum("verisign-net")
        || client.has_erratum("verisign-cc")
        || client.has_erratum("verisign-tv");

    if !no_registrant {
        if let Some(new_registrant) = &req.new_registrant {
            super::contact::check_id(new_registrant)?;
        }
    }
    if let Some(new_forward_to) = &req.new_forward_to {
        check_email(new_forward_to)?;
    }
    let mut adds = vec![];
    let mut rems = vec![];
    for add in &req.add {
        match add {
            UpdateObject::Status(s) => {
                adds.push(proto::email_forward::EPPEmailForwardUpdateParam::Status(
                    proto::domain::EPPDomainStatus {
                        status: s.into(),
                        message: None,
                    },
                ))
            }
            UpdateObject::Contact(c) => {
                super::contact::check_id(&c.contact_id)?;
                adds.push(proto::email_forward::EPPEmailForwardUpdateParam::Contact(
                    proto::domain::EPPDomainInfoContact {
                        contact_type: c.contact_type.clone(),
                        contact_id: c.contact_id.clone(),
                    },
                ))
            }
        }
    }
    for rem in &req.remove {
        match rem {
            UpdateObject::Status(s) => {
                rems.push(proto::email_forward::EPPEmailForwardUpdateParam::Status(
                    proto::domain::EPPDomainStatus {
                        status: s.into(),
                        message: None,
                    },
                ))
            }
            UpdateObject::Contact(c) => {
                super::contact::check_id(&c.contact_id)?;
                rems.push(proto::email_forward::EPPEmailForwardUpdateParam::Contact(
                    proto::domain::EPPDomainInfoContact {
                        contact_type: c.contact_type.clone(),
                        contact_id: c.contact_id.clone(),
                    },
                ))
            }
        }
    }

    let update_as_i32 = |u: &proto::email_forward::EPPEmailForwardUpdateParam| match u {
        proto::email_forward::EPPEmailForwardUpdateParam::Contact(_) => 0,
        proto::email_forward::EPPEmailForwardUpdateParam::Status(_) => 1,
    };
    adds.sort_unstable_by_key(update_as_i32);
    rems.sort_unstable_by_key(update_as_i32);

    let is_not_change =
        req.new_registrant.is_none() && req.new_auth_info.is_none() && req.new_forward_to.is_none();

    if req.add.is_empty() && req.remove.is_empty() && is_not_change {
        return Err(Err(Error::Err(
            "at least one operation must be specified".to_string(),
        )));
    }

    let mut exts = vec![];

    if let Some(fee_agreement) = &req.fee_agreement {
        if client.fee_supported {
            exts.push(proto::EPPCommandExtensionType::EPPFee10Update(
                fee_agreement.into(),
            ));
        } else if client.fee_011_supported {
            exts.push(proto::EPPCommandExtensionType::EPPFee011Update(
                fee_agreement.into(),
            ));
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut exts);

    if let Some(auth_info) = &req.new_auth_info {
        if !auth_info.is_empty() {
            check_pass(auth_info)?;
        }
    }

    let command = proto::EPPUpdate::EmailForward(proto::email_forward::EPPEmailForwardUpdate {
        name: req.name.clone(),
        add: if adds.is_empty() {
            None
        } else {
            Some(proto::email_forward::EPPEmailForwardUpdateAddRemove { params: adds })
        },
        remove: if rems.is_empty() {
            None
        } else {
            Some(proto::email_forward::EPPEmailForwardUpdateAddRemove { params: rems })
        },
        change: if is_not_change {
            None
        } else {
            Some(proto::email_forward::EPPEmailForwardUpdateChange {
                registrant: if no_registrant {
                    None
                } else {
                    req.new_registrant.clone()
                },
                auth_info: req.new_auth_info.as_ref().map(|a| {
                    proto::email_forward::EPPEmailForwardAuthInfo {
                        password: Some(a.clone()),
                    }
                }),
                forward_to: req.new_forward_to.clone(),
            })
        },
    });
    Ok((
        proto::EPPCommandType::Update(Box::new(command)),
        match exts.len() {
            0 => None,
            _ => Some(exts),
        },
    ))
}

pub fn handle_update_response(response: proto::EPPResponse) -> Response<UpdateResponse> {
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

    Response::Ok(UpdateResponse {
        pending: response.is_pending(),
        fee_data,
    })
}

pub fn handle_renew(client: &ServerFeatures, req: &RenewRequest) -> HandleReqReturn<RenewResponse> {
    if !client.email_forward_supported {
        return Err(Err(Error::Unsupported));
    }
    check_email(&req.name)?;
    let command = proto::EPPRenew::EmailForward(proto::email_forward::EPPEmailForwardRenew {
        name: req.name.clone(),
        period: req.add_period.as_ref().map(Into::into),
        current_expiry_date: req.cur_expiry_date.date(),
    });
    let mut ext = vec![];

    if let Some(fee_agreement) = &req.fee_agreement {
        if client.fee_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee10Renew(
                fee_agreement.into(),
            ));
        } else if client.fee_011_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee011Renew(
                fee_agreement.into(),
            ));
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);

    Ok((
        proto::EPPCommandType::Renew(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_renew_response(response: proto::EPPResponse) -> Response<RenewResponse> {
    let pending = response.is_pending();
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPEmailForwardRenewResult(email_forward_renew) => {
                let mut res: RenewResponse =
                    (email_forward_renew, &response.extension).try_into()?;
                res.pending = pending;
                Ok(res)
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_transfer_query(
    client: &ServerFeatures,
    req: &TransferQueryRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.email_forward_supported {
        return Err(Err(Error::Unsupported));
    }
    check_email(&req.name)?;

    if let Some(auth_info) = &req.auth_info {
        if !auth_info.is_empty() {
            check_pass(auth_info)?;
        }
    }

    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Query,
        command: proto::EPPTransferCommand::EmailForwardQuery(
            proto::email_forward::EPPEmailForwardCheck {
                name: req.name.clone(),
                auth_info: req.auth_info.as_ref().map(|a| {
                    proto::email_forward::EPPEmailForwardAuthInfo {
                        password: Some(a.clone()),
                    }
                }),
            },
        ),
    };
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((
        proto::EPPCommandType::Transfer(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_transfer_request(
    client: &ServerFeatures,
    req: &TransferRequestRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.email_forward_supported {
        return Err(Err(Error::Unsupported));
    }
    check_email(&req.name)?;

    if !req.auth_info.is_empty() {
        check_pass(&req.auth_info)?;
    }

    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Request,
        command: proto::EPPTransferCommand::EmailForwardRequest(
            proto::email_forward::EPPEmailForwardTransfer {
                name: req.name.clone(),
                period: req.add_period.as_ref().map(|p| p.into()),
                auth_info: Some(proto::email_forward::EPPEmailForwardAuthInfo {
                    password: Some(req.auth_info.clone()),
                }),
            },
        ),
    };
    let mut ext = vec![];

    if let Some(fee_agreement) = &req.fee_agreement {
        if client.fee_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee10Transfer(
                fee_agreement.into(),
            ));
        } else if client.fee_011_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee011Transfer(
                fee_agreement.into(),
            ));
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);

    Ok((
        proto::EPPCommandType::Transfer(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_transfer_cancel(
    client: &ServerFeatures,
    req: &TransferAcceptRejectRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.email_forward_supported {
        return Err(Err(Error::Unsupported));
    }

    check_email(&req.name)?;

    if let Some(auth_info) = &req.auth_info {
        if !auth_info.is_empty() {
            check_pass(auth_info)?;
        }
    }

    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Cancel,
        command: proto::EPPTransferCommand::EmailForwardRequest(
            proto::email_forward::EPPEmailForwardTransfer {
                name: req.name.clone(),
                period: None,
                auth_info: req.auth_info.as_ref().map(|a| {
                    proto::email_forward::EPPEmailForwardAuthInfo {
                        password: Some(a.clone()),
                    }
                }),
            },
        ),
    };
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((
        proto::EPPCommandType::Transfer(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_transfer_accept(
    client: &ServerFeatures,
    req: &TransferAcceptRejectRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.email_forward_supported {
        return Err(Err(Error::Unsupported));
    }

    check_email(&req.name)?;

    if let Some(auth_info) = &req.auth_info {
        if !auth_info.is_empty() {
            check_pass(auth_info)?;
        }
    }

    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Accept,
        command: proto::EPPTransferCommand::EmailForwardRequest(
            proto::email_forward::EPPEmailForwardTransfer {
                name: req.name.clone(),
                period: None,
                auth_info: req.auth_info.as_ref().map(|a| {
                    proto::email_forward::EPPEmailForwardAuthInfo {
                        password: Some(a.clone()),
                    }
                }),
            },
        ),
    };
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((
        proto::EPPCommandType::Transfer(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_transfer_reject(
    client: &ServerFeatures,
    req: &TransferAcceptRejectRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.email_forward_supported {
        return Err(Err(Error::Unsupported));
    }

    check_email(&req.name)?;

    if let Some(auth_info) = &req.auth_info {
        if !auth_info.is_empty() {
            check_pass(auth_info)?;
        }
    }

    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Reject,
        command: proto::EPPTransferCommand::EmailForwardRequest(
            proto::email_forward::EPPEmailForwardTransfer {
                name: req.name.clone(),
                period: None,
                auth_info: req.auth_info.as_ref().map(|a| {
                    proto::email_forward::EPPEmailForwardAuthInfo {
                        password: Some(a.clone()),
                    }
                }),
            },
        ),
    };
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);

    Ok((
        proto::EPPCommandType::Transfer(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_transfer_response(response: proto::EPPResponse) -> Response<TransferResponse> {
    let pending = response.is_pending();
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPEmailForwardTransferResult(email_forward_transfer) => {
                let mut res: TransferResponse =
                    (email_forward_transfer, &response.extension).try_into()?;
                res.pending = pending;
                Ok(res)
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}
