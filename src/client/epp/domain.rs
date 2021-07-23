//! EPP commands relating to domain objects

use std::convert::{TryFrom, TryInto};

use super::super::domain::{
    CheckRequest, CheckResponse, ClaimsCheckRequest, ClaimsCheckResponse, CreateData,
    CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, InfoContact, InfoHost,
    InfoNameserver, InfoRequest, InfoResponse, PanData, RenewData, RenewRequest, RenewResponse,
    SecDNSDSData, SecDNSData, SecDNSDataType, SecDNSKeyData, Status, TrademarkCheckRequest,
    TransferAcceptRejectRequest, TransferData, TransferQueryRequest, TransferRequestRequest,
    TransferResponse, UpdateObject, UpdateRequest, UpdateResponse, UpdateSecDNSRemove,
    VerisignSyncRequest,
};
use super::super::{fee, launch, proto, Error, Period, PeriodUnit, Response};
use super::router::HandleReqReturn;
use super::ServerFeatures;

impl From<proto::domain::EPPDomainStatusType> for Status {
    fn from(from: proto::domain::EPPDomainStatusType) -> Self {
        use proto::domain::EPPDomainStatusType;
        match from {
            EPPDomainStatusType::ClientDeleteProhibited => Status::ClientDeleteProhibited,
            EPPDomainStatusType::ClientHold => Status::ClientHold,
            EPPDomainStatusType::ClientRenewProhibited => Status::ClientRenewProhibited,
            EPPDomainStatusType::ClientTransferProhibited => Status::ClientTransferProhibited,
            EPPDomainStatusType::ClientUpdateProhibited => Status::ClientUpdateProhibited,
            EPPDomainStatusType::Inactive => Status::Inactive,
            EPPDomainStatusType::Ok => Status::Ok,
            EPPDomainStatusType::Granted => Status::Ok,
            EPPDomainStatusType::PendingCreate => Status::PendingCreate,
            EPPDomainStatusType::PendingDelete => Status::PendingDelete,
            EPPDomainStatusType::Terminated => Status::PendingDelete,
            EPPDomainStatusType::PendingRenew => Status::PendingRenew,
            EPPDomainStatusType::PendingTransfer => Status::PendingTransfer,
            EPPDomainStatusType::PendingUpdate => Status::PendingUpdate,
            EPPDomainStatusType::ServerDeleteProhibited => Status::ServerDeleteProhibited,
            EPPDomainStatusType::ServerHold => Status::ServerHold,
            EPPDomainStatusType::ServerRenewProhibited => Status::ServerRenewProhibited,
            EPPDomainStatusType::ServerTransferProhibited => Status::ServerTransferProhibited,
            EPPDomainStatusType::ServerUpdateProhibited => Status::ServerUpdateProhibited,
        }
    }
}

impl From<&Status> for proto::domain::EPPDomainStatusType {
    fn from(from: &Status) -> Self {
        use proto::domain::EPPDomainStatusType;
        match from {
            Status::ClientDeleteProhibited => EPPDomainStatusType::ClientDeleteProhibited,
            Status::ClientHold => EPPDomainStatusType::ClientHold,
            Status::ClientRenewProhibited => EPPDomainStatusType::ClientRenewProhibited,
            Status::ClientTransferProhibited => EPPDomainStatusType::ClientTransferProhibited,
            Status::ClientUpdateProhibited => EPPDomainStatusType::ClientUpdateProhibited,
            Status::Inactive => EPPDomainStatusType::Inactive,
            Status::Ok => EPPDomainStatusType::Ok,
            Status::PendingCreate => EPPDomainStatusType::PendingCreate,
            Status::PendingDelete => EPPDomainStatusType::PendingDelete,
            Status::PendingRenew => EPPDomainStatusType::PendingRenew,
            Status::PendingTransfer => EPPDomainStatusType::PendingTransfer,
            Status::PendingUpdate => EPPDomainStatusType::PendingUpdate,
            Status::ServerDeleteProhibited => EPPDomainStatusType::ServerDeleteProhibited,
            Status::ServerHold => EPPDomainStatusType::ServerHold,
            Status::ServerRenewProhibited => EPPDomainStatusType::ServerRenewProhibited,
            Status::ServerTransferProhibited => EPPDomainStatusType::ServerTransferProhibited,
            Status::ServerUpdateProhibited => EPPDomainStatusType::ServerUpdateProhibited,
        }
    }
}

impl From<&InfoNameserver> for proto::domain::EPPDomainInfoNameserver {
    fn from(from: &InfoNameserver) -> Self {
        match from {
            InfoNameserver::HostOnly(h) => {
                proto::domain::EPPDomainInfoNameserver::HostOnly(h.to_string())
            }
            InfoNameserver::HostAndAddress {
                host, addresses, ..
            } => proto::domain::EPPDomainInfoNameserver::HostAndAddress {
                host: host.to_string(),
                addresses: addresses
                    .iter()
                    .map(|addr| proto::domain::EPPDomainInfoNameserverAddress {
                        address: addr.address.to_string(),
                        ip_version: match addr.ip_version {
                            super::super::host::AddressVersion::IPv4 => {
                                proto::domain::EPPDomainInfoNameserverAddressVersion::IPv4
                            }
                            super::super::host::AddressVersion::IPv6 => {
                                proto::domain::EPPDomainInfoNameserverAddressVersion::IPv6
                            }
                        },
                    })
                    .collect(),
            },
        }
    }
}

impl From<&Period> for proto::domain::EPPDomainPeriod {
    fn from(from: &Period) -> Self {
        proto::domain::EPPDomainPeriod {
            unit: match from.unit {
                PeriodUnit::Months => proto::domain::EPPDomainPeriodUnit::Months,
                PeriodUnit::Years => proto::domain::EPPDomainPeriodUnit::Years,
            },
            value: std::cmp::min(99, std::cmp::max(from.value, 1)),
        }
    }
}

impl From<&proto::domain::EPPDomainPeriod> for Period {
    fn from(from: &proto::domain::EPPDomainPeriod) -> Self {
        Period {
            unit: match from.unit {
                proto::domain::EPPDomainPeriodUnit::Years => PeriodUnit::Years,
                proto::domain::EPPDomainPeriodUnit::Months => PeriodUnit::Months,
            },
            value: from.value,
        }
    }
}

impl
    TryFrom<(
        proto::domain::EPPDomainInfoData,
        &Option<proto::EPPResponseExtension>,
    )> for InfoResponse
{
    type Error = Error;

    fn try_from(
        from: (
            proto::domain::EPPDomainInfoData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (domain_info, extension) = from;
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

        let sec_dns = match extension {
            Some(ext) => match ext
                .value
                .iter()
                .find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPSecDNSInfo(i) => Some(i),
                    _ => None,
                })
                .map(|i| {
                    Ok(SecDNSData {
                        max_sig_life: i.max_signature_life,
                        data: if !i.ds_data.is_empty() {
                            SecDNSDataType::DSData(
                                i.ds_data
                                    .iter()
                                    .map(|d| SecDNSDSData {
                                        key_tag: d.key_tag,
                                        algorithm: d.algorithm,
                                        digest_type: d.digest_type,
                                        digest: d.digest.clone(),
                                        key_data: d.key_data.as_ref().map(|k| SecDNSKeyData {
                                            flags: k.flags,
                                            protocol: k.protocol,
                                            algorithm: k.algorithm,
                                            public_key: k.public_key.clone(),
                                        }),
                                    })
                                    .collect(),
                            )
                        } else if !i.key_data.is_empty() {
                            SecDNSDataType::KeyData(
                                i.key_data
                                    .iter()
                                    .map(|k| SecDNSKeyData {
                                        flags: k.flags,
                                        protocol: k.protocol,
                                        algorithm: k.algorithm,
                                        public_key: k.public_key.clone(),
                                    })
                                    .collect(),
                            )
                        } else {
                            return Err(Error::ServerInternal);
                        },
                    })
                }) {
                Some(i) => match i {
                    Ok(i) => Some(i),
                    Err(e) => return Err(e),
                },
                None => None,
            },
            None => None,
        };

        let launch_info = match extension {
            Some(ext) => ext
                .value
                .iter()
                .find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPLaunchInfoData(i) => Some(i),
                    _ => None,
                })
                .map(launch::LaunchInfoData::from),
            None => None,
        };

        let donuts_fee_data = match extension {
            Some(ext) => {
                let charge = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPDonutsChargeInfoData(i) => Some(i),
                    _ => None,
                });

                charge.map(Into::into)
            }
            None => None,
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

        let isnic_info = match extension {
            Some(ext) => {
                let i = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::ISNICDomainInfo(i) => Some(i),
                    _ => None,
                });
                i.map(Into::into)
            }
            None => None,
        };

        Ok(InfoResponse {
            eurid_idn: super::eurid::extract_eurid_idn_singular(extension, domain_info.name.as_str())?,
            name: domain_info.name,
            registry_id: domain_info.registry_id.unwrap_or_default(),
            statuses: domain_info
                .statuses
                .into_iter()
                .map(|s| s.status.into())
                .collect(),
            registrant: domain_info.registrant.unwrap_or_default(),
            contacts: domain_info
                .contacts
                .into_iter()
                .map(|c| InfoContact {
                    contact_id: c.contact_id,
                    contact_type: c.contact_type,
                })
                .collect(),
            nameservers: match domain_info.nameservers {
                None => vec![],
                Some(n) => n
                    .servers
                    .into_iter()
                    .map(|s| Ok(match s {
                        proto::domain::EPPDomainInfoNameserver::HostOnly(h) => {
                            InfoNameserver::HostOnly(h)
                        }
                        proto::domain::EPPDomainInfoNameserver::HostAndAddress {
                            host,
                            addresses,
                        } => InfoNameserver::HostAndAddress {
                            eurid_idn: super::eurid::extract_eurid_idn_singular(extension, host.as_str())?,
                            host,
                            addresses: addresses
                                .into_iter()
                                .map(|addr| {
                                    super::super::host::Address {
                                        address: addr.address,
                                        ip_version: match addr.ip_version {
                                            proto::domain::EPPDomainInfoNameserverAddressVersion::IPv4 => {
                                                super::super::host::AddressVersion::IPv4
                                            }
                                            proto::domain::EPPDomainInfoNameserverAddressVersion::IPv6 => {
                                                super::super::host::AddressVersion::IPv6
                                            }
                                        },
                                    }
                                })
                                .collect(),
                        },
                    }))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            hosts: domain_info.hosts,
            client_id: domain_info.client_id,
            client_created_id: domain_info.client_created_id,
            creation_date: domain_info.creation_date,
            expiry_date: domain_info.expiry_date,
            last_updated_client: domain_info.last_updated_client,
            last_updated_date: domain_info.last_updated_date,
            last_transfer_date: domain_info.last_transfer_date,
            rgp_state,
            auth_info: match domain_info.auth_info {
                Some(a) => a.password,
                None => None,
            },
            sec_dns,
            launch_info,
            donuts_fee_data,
            whois_info,
            isnic_info,
            eurid_data: super::eurid::extract_eurid_domain_info(extension),
        })
    }
}

impl
    TryFrom<(
        proto::domain::EPPDomainTransferData,
        &Option<proto::EPPResponseExtension>,
    )> for TransferResponse
{
    type Error = Error;

    fn try_from(
        from: (
            proto::domain::EPPDomainTransferData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (domain_transfer, extension) = from;

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

        let donuts_fee_data = match extension {
            Some(ext) => {
                let charge = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPDonutsChargeTransferData(i) => Some(i),
                    _ => None,
                });

                charge.map(Into::into)
            }
            None => None,
        };

        Ok(TransferResponse {
            pending: false,
            data: TransferData {
                name: domain_transfer.name.clone(),
                status: (&domain_transfer.transfer_status).into(),
                requested_client_id: domain_transfer.requested_client_id.clone(),
                requested_date: domain_transfer.requested_date,
                act_client_id: domain_transfer.act_client_id.clone(),
                act_date: domain_transfer.act_date,
                expiry_date: domain_transfer.expiry_date,
                eurid_data: super::eurid::extract_eurid_domain_transfer_info(extension),
                eurid_idn: super::eurid::extract_eurid_idn_singular(extension, None)?,
            },
            fee_data,
            donuts_fee_data,
        })
    }
}

impl
    TryFrom<(
        Option<proto::domain::EPPDomainCreateData>,
        &Option<proto::EPPResponseExtension>,
    )> for CreateResponse
{
    type Error = Error;

    fn try_from(
        from: (
            Option<proto::domain::EPPDomainCreateData>,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (domain_create, extension) = from;

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

        let launch_create = match extension {
            Some(ext) => ext
                .value
                .iter()
                .find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPLaunchCreateData(i) => Some(i),
                    _ => None,
                })
                .map(launch::LaunchCreateData::from),
            None => None,
        };

        let donuts_fee_data = match extension {
            Some(ext) => {
                let charge = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPDonutsChargeCreateData(i) => Some(i),
                    _ => None,
                });

                charge.map(Into::into)
            }
            None => None,
        };

        match domain_create {
            Some(domain_create) => Ok(CreateResponse {
                pending: false,
                data: CreateData {
                    eurid_idn: super::eurid::extract_eurid_idn_singular(
                        extension,
                        domain_create.name.as_str(),
                    )?,
                    name: domain_create.name.clone(),
                    creation_date: Some(domain_create.creation_date),
                    expiration_date: domain_create.expiry_date,
                },
                fee_data,
                donuts_fee_data,
                launch_create,
            }),
            None => Ok(CreateResponse {
                pending: false,
                data: CreateData {
                    eurid_idn: super::eurid::extract_eurid_idn_singular(extension, None)?,
                    name: "".to_string(),
                    creation_date: None,
                    expiration_date: None,
                },
                fee_data,
                donuts_fee_data,
                launch_create,
            }),
        }
    }
}

impl
    TryFrom<(
        proto::domain::EPPDomainRenewData,
        &Option<proto::EPPResponseExtension>,
    )> for RenewResponse
{
    type Error = Error;

    fn try_from(
        from: (
            proto::domain::EPPDomainRenewData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (domain_renew, extension) = from;

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

        let donuts_fee_data = match extension {
            Some(ext) => {
                let charge = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPDonutsChargeRenewData(i) => Some(i),
                    _ => None,
                });

                charge.map(Into::into)
            }
            None => None,
        };

        Ok(RenewResponse {
            pending: false,
            data: RenewData {
                eurid_idn: super::eurid::extract_eurid_idn_singular(
                    extension,
                    domain_renew.name.as_str(),
                )?,
                name: domain_renew.name.to_owned(),
                new_expiry_date: domain_renew.expiry_date,
                eurid_data: super::eurid::extract_eurid_domain_renew_info(extension),
            },
            fee_data,
            donuts_fee_data,
        })
    }
}

impl From<&proto::domain::EPPDomainPanData> for PanData {
    fn from(from: &proto::domain::EPPDomainPanData) -> Self {
        PanData {
            name: from.name.domain.clone(),
            result: from.name.result,
            server_transaction_id: from.transaction_id.server_transaction_id.clone(),
            client_transaction_id: from.transaction_id.client_transaction_id.clone(),
            date: from.action_date,
        }
    }
}

pub(crate) fn check_domain<T>(id: &str) -> Result<(), Response<T>> {
    if !id.is_empty() {
        Ok(())
    } else {
        Err(Err(Error::Err(
            "domain name has a min length of 1".to_string(),
        )))
    }
}

pub(crate) fn check_pass<T>(id: &str) -> Result<(), Response<T>> {
    if let 6..=16 = id.len() {
        Ok(())
    } else {
        Err(Err(Error::Err(
            "passwords have a min length of 6 and a max length of 16".to_string(),
        )))
    }
}

pub fn handle_check(client: &ServerFeatures, req: &CheckRequest) -> HandleReqReturn<CheckResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPCheck::Domain(proto::domain::EPPDomainCheck {
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
                                name: match (&c.command).into() {
                                    Some(n) => n,
                                    None => return Err(Err(Error::Unsupported)),
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
                            command: match (&c.command).into() {
                                Some(n) => n,
                                None => return Err(Err(Error::Unsupported)),
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
                                object_uri: Some("urn:ietf:params:xml:ns:domain-1.0".to_string()),
                                object_id: proto::fee::EPPFee10ObjectID {
                                    element: "name".to_string(),
                                    id: req.name.to_owned(),
                                },
                                currency: fee_check.currency.to_owned(),
                                command: match (&c.command).into() {
                                    Some(n) => n,
                                    None => return Err(Err(Error::Unsupported)),
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
                                command: match (&c.command).into() {
                                    Some(n) => n,
                                    None => return Err(Err(Error::Unsupported)),
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
                                command: match (&c.command).into() {
                                    Some(n) => n,
                                    None => return Err(Err(Error::Unsupported)),
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
                                command: match (&c.command).into() {
                                    Some(n) => n,
                                    None => return Err(Err(Error::Unsupported)),
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

    if let Some(launch_check) = &req.launch_check {
        if client.launch_supported {
            ext.push(proto::EPPCommandExtensionType::EPPLaunchCheck(
                launch_check.into(),
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
                            command: (&c.name).into(),
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
                            command: (&c.command.name).into(),
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
                            command: (&d.command).into(),
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
                            command: (&d.command).into(),
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
                            command: (&d.command).into(),
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
                            command: (&d.command).into(),
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

    let donuts_fee_check = match &response.extension {
        Some(ext) => {
            let charge = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPDonutsChargeCheckData(i) => Some(i),
                _ => None,
            });

            if let Some(c) = charge {
                let d = match c.domains.get(0) {
                    Some(o) => o,
                    None => return Err(Error::ServerInternal),
                };
                Some(d.into())
            } else {
                None
            }
        }
        None => None,
    };

    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPDomainCheckResult(domain_check) => {
                if let Some(domain_check) = domain_check.data.first() {
                    Response::Ok(CheckResponse {
                        eurid_idn: super::eurid::extract_eurid_idn_singular(
                            &response.extension,
                            domain_check.name.name.as_str(),
                        )?,
                        avail: domain_check.name.available,
                        reason: domain_check.reason.to_owned(),
                        fee_check,
                        donuts_fee_check,
                        eurid_check: super::eurid::extract_eurid_domain_check_singular(
                            &response.extension,
                        )?,
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

pub fn handle_claims_check(
    client: &ServerFeatures,
    req: &ClaimsCheckRequest,
) -> HandleReqReturn<ClaimsCheckResponse> {
    if !client.launch_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPCheck::Domain(proto::domain::EPPDomainCheck {
        name: req.name.clone(),
        auth_info: None,
    });
    let mut ext = vec![proto::EPPCommandExtensionType::EPPLaunchCheck(
        (&req.launch_check).into(),
    )];

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((
        proto::EPPCommandType::Check(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_trademark_check(
    client: &ServerFeatures,
    req: &TrademarkCheckRequest,
) -> HandleReqReturn<ClaimsCheckResponse> {
    if !client.launch_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPCheck::Domain(proto::domain::EPPDomainCheck {
        name: req.name.clone(),
        auth_info: None,
    });
    let mut ext = vec![proto::EPPCommandExtensionType::EPPLaunchCheck(
        (&launch::LaunchTrademarkCheck {}).into(),
    )];

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((
        proto::EPPCommandType::Check(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_claims_check_response(response: proto::EPPResponse) -> Response<ClaimsCheckResponse> {
    let claims_check = match response.extension {
        Some(ext) => {
            let claims = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPLaunchCheckData(i) => Some(i),
                _ => None,
            });

            if let Some(c) = claims {
                c.data.first().map(|domain_check| ClaimsCheckResponse {
                    exists: domain_check.name.exists,
                    claims_key: domain_check.claim_key.iter().map(Into::into).collect(),
                })
            } else {
                None
            }
        }
        None => None,
    };

    match response.data {
        Some(_) => Err(Error::ServerInternal),
        None => match claims_check {
            Some(c) => Response::Ok(c),
            None => Err(Error::ServerInternal),
        },
    }
}

pub fn handle_info(client: &ServerFeatures, req: &InfoRequest) -> HandleReqReturn<InfoResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPInfo::Domain(proto::domain::EPPDomainInfo {
        name: proto::domain::EPPDomainInfoName {
            name: req.name.clone(),
            hosts: req.hosts.as_ref().map(|h| match h {
                InfoHost::All => proto::domain::EPPDomainInfoHosts::All,
                InfoHost::Delegated => proto::domain::EPPDomainInfoHosts::Delegated,
                InfoHost::Subordinate => proto::domain::EPPDomainInfoHosts::Subordinate,
                InfoHost::None => proto::domain::EPPDomainInfoHosts::None,
            }),
        },
        auth_info: req
            .auth_info
            .as_ref()
            .map(|a| proto::domain::EPPDomainAuthInfo {
                password: Some(a.clone()),
            }),
    });
    let mut exts = vec![];
    if let Some(launch_info) = &req.launch_info {
        if client.launch_supported {
            exts.push(proto::EPPCommandExtensionType::EPPLaunchInfo(
                launch_info.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }
    if client.verisign_whois_info {
        exts.push(proto::EPPCommandExtensionType::VerisignWhoisInfExt(
            proto::verisign::EPPWhoisInfoExt { flag: true },
        ))
    }

    if let Some(eurid_data) = &req.eurid_data {
        if let Some(euird_auth_info) = eurid_data.into() {
            if client.eurid_auth_info_supported {
                exts.push(proto::EPPCommandExtensionType::EURIDAuthInfo(
                    euird_auth_info,
                ))
            } else {
                return Err(Err(Error::Unsupported));
            }
        }
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
            proto::EPPResultDataValue::EPPDomainInfoResult(domain_info) => {
                (*domain_info, &response.extension).try_into()
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
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let no_registrant = client.has_erratum("verisign-com")
        || client.has_erratum("verisign-net")
        || client.has_erratum("verisign-cc")
        || client.has_erratum("verisign-tv");
    if !no_registrant {
        super::contact::check_id(&req.registrant)?;
    }

    let mut exts = vec![];
    match &req.sec_dns {
        Some(sec_dns) => {
            if client.secdns_supported || client.has_erratum("pir") {
                exts.push(proto::EPPCommandExtensionType::EPPSecDNSCreate(
                    match &sec_dns.data {
                        SecDNSDataType::DSData(ds_data) => proto::secdns::EPPSecDNSData {
                            max_signature_life: sec_dns.max_sig_life,
                            key_data: vec![],
                            ds_data: ds_data
                                .iter()
                                .map(|d| proto::secdns::EPPSecDNSDSData {
                                    key_tag: d.key_tag,
                                    algorithm: d.algorithm,
                                    digest_type: d.digest_type,
                                    digest: d.digest.clone(),
                                    key_data: d.key_data.as_ref().map(|k| {
                                        proto::secdns::EPPSecDNSKeyData {
                                            flags: k.flags,
                                            protocol: k.protocol,
                                            algorithm: k.algorithm,
                                            public_key: k.public_key.clone(),
                                        }
                                    }),
                                })
                                .collect(),
                        },
                        SecDNSDataType::KeyData(key_data) => proto::secdns::EPPSecDNSData {
                            max_signature_life: sec_dns.max_sig_life,
                            ds_data: vec![],
                            key_data: key_data
                                .iter()
                                .map(|k| proto::secdns::EPPSecDNSKeyData {
                                    flags: k.flags,
                                    protocol: k.protocol,
                                    algorithm: k.algorithm,
                                    public_key: k.public_key.clone(),
                                })
                                .collect(),
                        },
                    },
                ))
            } else {
                return Err(Err(Error::Unsupported));
            }
        }
        None => {}
    }

    if let Some(launch_create) = &req.launch_create {
        if client.launch_supported {
            if !launch_create.core_nic.is_empty() {
                if !(client.corenic_mark || client.has_erratum("corenic")) {
                    return Err(Err(Error::Unsupported));
                }

                for info in launch_create.core_nic.iter() {
                    if let Some(info_type) = &info.info_type {
                        if info_type.is_empty() || info_type.len() > 64 {
                            return Err(Err(Error::Err(
                                "application info type has a min length of 1 and a max length of 64".to_string(),
                            )));
                        }
                    }
                    if info.info.is_empty() || info.info.len() > 2048 {
                        return Err(Err(Error::Err(
                            "application info has a min length of 1 and a max length of 2048"
                                .to_string(),
                        )));
                    }
                }
            }

            exts.push(proto::EPPCommandExtensionType::EPPLaunchCreate(
                launch_create.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

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

    if let Some(eurid_data) = &req.eurid_data {
        if client.eurid_domain_support {
            exts.push(proto::EPPCommandExtensionType::EURIDDomainCreate(
                eurid_data.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    match &req.isnic_payment {
        Some(e) => {
            if client.isnic_domain_supported {
                exts.push(proto::EPPCommandExtensionType::ISNICDomainCreate(e.into()))
            } else {
                return Err(Err(Error::Unsupported));
            }
        }
        None => {
            if client.isnic_domain_supported {
                return Err(Err(Error::Err(
                    "payment extension required for ISNIC".to_string(),
                )));
            }
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut exts);
    super::fee::handle_donuts_fee_agreement(client, &req.donuts_fee_agreement, &mut exts)?;

    if !req.auth_info.is_empty() {
        check_pass(&req.auth_info)?;
    }

    let command = proto::EPPCreate::Domain(proto::domain::EPPDomainCreate {
        name: req.name.clone(),
        period: req.period.as_ref().map(|p| p.into()),
        nameservers: match req.nameservers.len() {
            0 => None,
            _ => Some(proto::domain::EPPDomainInfoNameservers {
                servers: req.nameservers.iter().map(|n| n.into()).collect(),
            }),
        },
        registrant: if no_registrant {
            None
        } else {
            Some(req.registrant.to_string())
        },
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
        auth_info: proto::domain::EPPDomainAuthInfo {
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
            proto::EPPResultDataValue::EPPDomainCreateResult(domain_create) => {
                let mut res: CreateResponse =
                    (Some(domain_create), &response.extension).try_into()?;
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
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPDelete::Domain(proto::domain::EPPDomainCheck {
        name: req.name.clone(),
        auth_info: None,
    });
    let mut ext = vec![];

    if let Some(eurid_data) = &req.eurid_data {
        if client.eurid_domain_support {
            ext.push(proto::EPPCommandExtensionType::EURIDDomainDelete(
                eurid_data.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    super::fee::handle_donuts_fee_agreement(client, &req.donuts_fee_agreement, &mut ext)?;

    if let Some(launch_info) = &req.launch_info {
        if client.launch_supported {
            ext.push(proto::EPPCommandExtensionType::EPPLaunchDelete(
                launch_info.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }
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
        eurid_idn: super::eurid::extract_eurid_idn_singular(&response.extension, None)?,
    })
}

pub fn handle_update(
    client: &ServerFeatures,
    req: &UpdateRequest,
) -> HandleReqReturn<UpdateResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;

    let no_registrant = client.has_erratum("verisign-com")
        || client.has_erratum("verisign-net")
        || client.has_erratum("verisign-cc")
        || client.has_erratum("verisign-tv");

    if !no_registrant {
        if let Some(new_registrant) = &req.new_registrant {
            super::contact::check_id(new_registrant)?;
        }
    }
    let mut adds = vec![];
    let mut rems = vec![];
    let mut add_ns = vec![];
    let mut rem_ns = vec![];
    for add in &req.add {
        match add {
            UpdateObject::Status(s) => adds.push(proto::domain::EPPDomainUpdateParam::Status(
                proto::domain::EPPDomainStatus {
                    status: s.into(),
                    message: None,
                },
            )),
            UpdateObject::Contact(c) => {
                super::contact::check_id(&c.contact_id)?;
                adds.push(proto::domain::EPPDomainUpdateParam::Contact(
                    proto::domain::EPPDomainInfoContact {
                        contact_type: c.contact_type.clone(),
                        contact_id: c.contact_id.clone(),
                    },
                ))
            }
            UpdateObject::Nameserver(n) => add_ns.push(n.into()),
        }
    }
    for rem in &req.remove {
        match rem {
            UpdateObject::Status(s) => rems.push(proto::domain::EPPDomainUpdateParam::Status(
                proto::domain::EPPDomainStatus {
                    status: s.into(),
                    message: None,
                },
            )),
            UpdateObject::Contact(c) => {
                super::contact::check_id(&c.contact_id)?;
                rems.push(proto::domain::EPPDomainUpdateParam::Contact(
                    proto::domain::EPPDomainInfoContact {
                        contact_type: c.contact_type.clone(),
                        contact_id: c.contact_id.clone(),
                    },
                ))
            }
            UpdateObject::Nameserver(n) => rem_ns.push(n.into()),
        }
    }
    if !add_ns.is_empty() {
        adds.push(proto::domain::EPPDomainUpdateParam::Nameserver(
            proto::domain::EPPDomainInfoNameservers { servers: add_ns },
        ))
    }
    if !rem_ns.is_empty() {
        rems.push(proto::domain::EPPDomainUpdateParam::Nameserver(
            proto::domain::EPPDomainInfoNameservers { servers: rem_ns },
        ))
    }

    let update_as_i32 = |u: &proto::domain::EPPDomainUpdateParam| match u {
        proto::domain::EPPDomainUpdateParam::Nameserver(_) => 0,
        proto::domain::EPPDomainUpdateParam::Contact(_) => 1,
        proto::domain::EPPDomainUpdateParam::Status(_) => 2,
    };
    adds.sort_unstable_by_key(update_as_i32);
    rems.sort_unstable_by_key(update_as_i32);

    let is_not_change = req.new_registrant.is_none() && req.new_auth_info.is_none();

    let is_not_isnic_change = match &req.isnic_info {
        Some(e) => !e.remove_all_ns && e.new_master_ns.is_empty(),
        None => true,
    };
    let is_not_eurid_change = match &req.eurid_data {
        Some(e) => {
            e.remove_on_site.is_none()
                && e.remove_reseller.is_none()
                && e.add_reseller.is_none()
                && e.add_on_site.is_none()
        }
        None => true,
    };

    if req.add.is_empty()
        && req.remove.is_empty()
        && is_not_change
        && (req.sec_dns.is_none() || !client.secdns_supported)
        && is_not_eurid_change
        && is_not_isnic_change
    {
        return Err(Err(Error::Err(
            "at least one operation must be specified".to_string(),
        )));
    }

    let mut exts = vec![];
    match &req.sec_dns {
        Some(sec_dns) => {
            if client.secdns_supported || client.has_erratum("pir") {
                exts.push(proto::EPPCommandExtensionType::EPPSecDNSUpdate(
                    proto::secdns::EPPSecDNSUpdate {
                        urgent: sec_dns.urgent,
                        add: sec_dns.add.as_ref().map(|a| match a {
                            SecDNSDataType::DSData(ds_data) => proto::secdns::EPPSecDNSUpdateAdd {
                                key_data: vec![],
                                ds_data: ds_data
                                    .iter()
                                    .map(|d| proto::secdns::EPPSecDNSDSData {
                                        key_tag: d.key_tag,
                                        algorithm: d.algorithm,
                                        digest_type: d.digest_type,
                                        digest: d.digest.clone(),
                                        key_data: d.key_data.as_ref().map(|k| {
                                            proto::secdns::EPPSecDNSKeyData {
                                                flags: k.flags,
                                                protocol: k.protocol,
                                                algorithm: k.algorithm,
                                                public_key: k.public_key.clone(),
                                            }
                                        }),
                                    })
                                    .collect(),
                            },
                            SecDNSDataType::KeyData(key_data) => {
                                proto::secdns::EPPSecDNSUpdateAdd {
                                    ds_data: vec![],
                                    key_data: key_data
                                        .iter()
                                        .map(|k| proto::secdns::EPPSecDNSKeyData {
                                            flags: k.flags,
                                            protocol: k.protocol,
                                            algorithm: k.algorithm,
                                            public_key: k.public_key.clone(),
                                        })
                                        .collect(),
                                }
                            }
                        }),
                        remove: sec_dns.remove.as_ref().map(|r| match r {
                            UpdateSecDNSRemove::All(a) => proto::secdns::EPPSecDNSUpdateRemove {
                                all: Some(*a),
                                ds_data: vec![],
                                key_data: vec![],
                            },
                            UpdateSecDNSRemove::Data(d) => match d {
                                SecDNSDataType::DSData(ds_data) => {
                                    proto::secdns::EPPSecDNSUpdateRemove {
                                        all: None,
                                        key_data: vec![],
                                        ds_data: ds_data
                                            .iter()
                                            .map(|d| proto::secdns::EPPSecDNSDSData {
                                                key_tag: d.key_tag,
                                                algorithm: d.algorithm,
                                                digest_type: d.digest_type,
                                                digest: d.digest.clone(),
                                                key_data: None,
                                            })
                                            .collect(),
                                    }
                                }
                                SecDNSDataType::KeyData(key_data) => {
                                    proto::secdns::EPPSecDNSUpdateRemove {
                                        all: None,
                                        ds_data: vec![],
                                        key_data: key_data
                                            .iter()
                                            .map(|k| proto::secdns::EPPSecDNSKeyData {
                                                flags: k.flags,
                                                protocol: k.protocol,
                                                algorithm: k.algorithm,
                                                public_key: k.public_key.clone(),
                                            })
                                            .collect(),
                                    }
                                }
                            },
                        }),
                        change: sec_dns.new_max_sig_life.map(|s| {
                            proto::secdns::EPPSecDNSUpdateChange {
                                max_signature_life: Some(s),
                            }
                        }),
                    },
                ))
            } else {
                return Err(Err(Error::Unsupported));
            }
        }
        None => {}
    }

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
    super::fee::handle_donuts_fee_agreement(client, &req.donuts_fee_agreement, &mut exts)?;

    if let Some(launch_info) = &req.launch_info {
        if client.launch_supported {
            exts.push(proto::EPPCommandExtensionType::EPPLaunchUpdate(
                launch_info.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    if let Some(eurid_data) = &req.eurid_data {
        if client.eurid_domain_support {
            exts.push(proto::EPPCommandExtensionType::EURIDDomainUpdate(
                eurid_data.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    if let Some(isnic_info) = &req.isnic_info {
        if client.isnic_contact_supported {
            exts.push(proto::EPPCommandExtensionType::ISNICDomainUpdate(
                isnic_info.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    if let Some(auth_info) = &req.new_auth_info {
        if !auth_info.is_empty() {
            check_pass(auth_info)?;
        }
    }

    let command = proto::EPPUpdate::Domain(proto::domain::EPPDomainUpdate {
        name: req.name.clone(),
        add: if adds.is_empty() {
            None
        } else {
            Some(proto::domain::EPPDomainUpdateAddRemove { params: adds })
        },
        remove: if rems.is_empty() {
            None
        } else {
            Some(proto::domain::EPPDomainUpdateAddRemove { params: rems })
        },
        change: if is_not_change {
            None
        } else {
            Some(proto::domain::EPPDomainUpdateChange {
                registrant: if no_registrant {
                    None
                } else {
                    req.new_registrant.clone()
                },
                auth_info: req
                    .new_auth_info
                    .as_ref()
                    .map(|a| proto::domain::EPPDomainAuthInfo {
                        password: Some(a.clone()),
                    }),
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

pub fn handle_verisign_sync(
    client: &ServerFeatures,
    req: &VerisignSyncRequest,
) -> HandleReqReturn<UpdateResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;

    if !client.verisign_sync_supported {
        return Err(Err(Error::Unsupported));
    }

    let mut exts = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut exts);

    exts.push(proto::EPPCommandExtensionType::VerisignSyncUpdate(
        proto::verisign::EPPSyncUpdate {
            month_day: proto::verisign::EPPSyncUpdateMonthDay {
                month: req.month,
                day: req.day,
            },
        },
    ));

    let command = proto::EPPUpdate::Domain(proto::domain::EPPDomainUpdate {
        name: req.name.clone(),
        add: None,
        remove: None,
        change: Some(proto::domain::EPPDomainUpdateChange {
            registrant: None,
            auth_info: None,
        }),
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

    let donuts_fee_data = match &response.extension {
        Some(ext) => {
            let charge = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPDonutsChargeUpdateData(i) => Some(i),
                _ => None,
            });

            charge.map(Into::into)
        }
        None => None,
    };

    Response::Ok(UpdateResponse {
        pending: response.is_pending(),
        fee_data,
        donuts_fee_data,
    })
}

pub fn handle_renew(client: &ServerFeatures, req: &RenewRequest) -> HandleReqReturn<RenewResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPRenew::Domain(proto::domain::EPPDomainRenew {
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

    match &req.isnic_payment {
        Some(e) => {
            if client.isnic_domain_supported {
                ext.push(proto::EPPCommandExtensionType::ISNICDomainRenew(e.into()))
            } else {
                return Err(Err(Error::Unsupported));
            }
        }
        None => {
            if client.isnic_domain_supported {
                return Err(Err(Error::Err(
                    "payment extension required for ISNIC".to_string(),
                )));
            }
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    super::fee::handle_donuts_fee_agreement(client, &req.donuts_fee_agreement, &mut ext)?;

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
            proto::EPPResultDataValue::EPPDomainRenewResult(domain_renew) => {
                let mut res: RenewResponse = (domain_renew, &response.extension).try_into()?;
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
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.isnic_contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;

    if let Some(auth_info) = &req.auth_info {
        if !auth_info.is_empty() {
            check_pass(auth_info)?;
        }
    }

    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Query,
        command: proto::EPPTransferCommand::DomainQuery(proto::domain::EPPDomainCheck {
            name: req.name.clone(),
            auth_info: req
                .auth_info
                .as_ref()
                .map(|a| proto::domain::EPPDomainAuthInfo {
                    password: Some(a.clone()),
                }),
        }),
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
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.isnic_contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;

    if !req.auth_info.is_empty() {
        check_pass(&req.auth_info)?;
    }

    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Request,
        command: proto::EPPTransferCommand::DomainRequest(proto::domain::EPPDomainTransfer {
            name: req.name.clone(),
            period: req.add_period.as_ref().map(|p| p.into()),
            auth_info: Some(proto::domain::EPPDomainAuthInfo {
                password: Some(req.auth_info.clone()),
            }),
        }),
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
    super::fee::handle_donuts_fee_agreement(client, &req.donuts_fee_agreement, &mut ext)?;

    if let Some(eurid_data) = &req.eurid_data {
        if client.eurid_domain_support {
            ext.push(proto::EPPCommandExtensionType::EURIDDomainTransfer(
                eurid_data.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

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
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.isnic_contact_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.eurid_domain_support {
        return Err(Err(Error::Unsupported));
    }

    check_domain(&req.name)?;

    if let Some(auth_info) = &req.auth_info {
        if !auth_info.is_empty() {
            check_pass(auth_info)?;
        }
    }

    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Cancel,
        command: proto::EPPTransferCommand::DomainRequest(proto::domain::EPPDomainTransfer {
            name: req.name.clone(),
            period: None,
            auth_info: req
                .auth_info
                .as_ref()
                .map(|a| proto::domain::EPPDomainAuthInfo {
                    password: Some(a.clone()),
                }),
        }),
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
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.isnic_contact_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.eurid_domain_support {
        return Err(Err(Error::Unsupported));
    }

    check_domain(&req.name)?;

    if let Some(auth_info) = &req.auth_info {
        if !auth_info.is_empty() {
            check_pass(auth_info)?;
        }
    }

    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Accept,
        command: proto::EPPTransferCommand::DomainRequest(proto::domain::EPPDomainTransfer {
            name: req.name.clone(),
            period: None,
            auth_info: req
                .auth_info
                .as_ref()
                .map(|a| proto::domain::EPPDomainAuthInfo {
                    password: Some(a.clone()),
                }),
        }),
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
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.isnic_contact_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.eurid_domain_support {
        return Err(Err(Error::Unsupported));
    }

    check_domain(&req.name)?;

    if let Some(auth_info) = &req.auth_info {
        if !auth_info.is_empty() {
            check_pass(auth_info)?;
        }
    }

    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Reject,
        command: proto::EPPTransferCommand::DomainRequest(proto::domain::EPPDomainTransfer {
            name: req.name.clone(),
            period: None,
            auth_info: req
                .auth_info
                .as_ref()
                .map(|a| proto::domain::EPPDomainAuthInfo {
                    password: Some(a.clone()),
                }),
        }),
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
            proto::EPPResultDataValue::EPPDomainTransferResult(domain_transfer) => {
                let mut res: TransferResponse =
                    (domain_transfer, &response.extension).try_into()?;
                res.pending = pending;
                Ok(res)
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

#[cfg(test)]
mod domain_tests {
    #[test]
    fn claims_check() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1000">
     <msg>Command completed successfully</msg>
    </result>
    <extension>
     <launch:chkData
      xmlns:launch="urn:ietf:params:xml:ns:launch-1.0">
      <launch:phase>claims</launch:phase>
      <launch:cd>
        <launch:name exists="1">domain3.example</launch:name>
        <launch:claimKey validatorID="tmch">
        2013041500/2/6/9/rJ1NrDO92vDsAzf7EQzgjX4R0000000001
        </launch:claimKey>
        <launch:claimKey validatorID="custom-tmch">
        20140423200/1/2/3/rJ1Nr2vDsAzasdff7EasdfgjX4R000000002
        </launch:claimKey>
      </launch:cd>
     </launch:chkData>
    </extension>
    <trID>
     <clTRID>ABC-12345</clTRID>
     <svTRID>54321-XYZ</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_claims_check_response(*res).unwrap();
        assert_eq!(data.exists, true);
        assert_eq!(data.claims_key.len(), 2);
        let claims_key_1 = data.claims_key.get(0).unwrap();
        let claims_key_2 = data.claims_key.get(1).unwrap();
        assert_eq!(claims_key_1.validator_id.as_ref().unwrap(), "tmch");
        assert_eq!(
            claims_key_1.key,
            "2013041500/2/6/9/rJ1NrDO92vDsAzf7EQzgjX4R0000000001"
        );
        assert_eq!(claims_key_2.validator_id.as_ref().unwrap(), "custom-tmch");
        assert_eq!(
            claims_key_2.key,
            "20140423200/1/2/3/rJ1Nr2vDsAzasdff7EasdfgjX4R000000002"
        );
    }

    #[test]
    fn launch_info() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <domain:infData
       xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
        <domain:name>domain.example</domain:name>
        <domain:roid>EXAMPLE1-REP</domain:roid>
        <domain:status s="pendingCreate"/>
        <domain:registrant>jd1234</domain:registrant>
        <domain:contact type="admin">sh8013</domain:contact>
        <domain:contact type="tech">sh8013</domain:contact>
        <domain:clID>ClientX</domain:clID>
        <domain:crID>ClientY</domain:crID>
        <domain:crDate>2012-04-03T22:00:00.0Z</domain:crDate>
        <domain:authInfo>
          <domain:pw>2fooBAR</domain:pw>
        </domain:authInfo>
      </domain:infData>
    </resData>
    <extension>
      <launch:infData
       xmlns:launch="urn:ietf:params:xml:ns:launch-1.0">
        <launch:phase>sunrise</launch:phase>
          <launch:applicationID>abc123</launch:applicationID>
          <launch:status s="pendingValidation"/>
          <mark:mark
            xmlns:mark="urn:ietf:params:xml:ns:mark-1.0">
             Test
         </mark:mark>
      </launch:infData>
    </extension>
    <trID>
      <clTRID>ABC-12345</clTRID>
      <svTRID>54321-XYZ</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_info_response(*res).unwrap();
        assert_eq!(data.name, "domain.example");
        let launch_info = data.launch_info.unwrap();
        assert_eq!(
            launch_info.phase.phase_type,
            super::launch::PhaseType::Sunrise
        );
        assert_eq!(launch_info.application_id.unwrap(), "abc123");
        assert_eq!(
            launch_info.status.unwrap().status_type,
            super::launch::LaunchStatusType::PendingValidation
        );
        assert_eq!(
            launch_info.mark.unwrap(),
            r#"<mark:mark xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:launch="urn:ietf:params:xml:ns:launch-1.0" xmlns:mark="urn:ietf:params:xml:ns:mark-1.0">Test</mark:mark>"#
        );
    }

    #[test]
    fn create_info() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1001">
      <msg>Command completed successfully; action pending</msg>
    </result>
    <resData>
      <domain:creData
         xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
       <domain:name>domain.example</domain:name>
       <domain:crDate>2010-08-10T15:38:26.623854Z</domain:crDate>
      </domain:creData>
    </resData>
    <extension>
      <launch:creData
        xmlns:launch="urn:ietf:params:xml:ns:launch-1.0">
        <launch:phase>sunrise</launch:phase>
        <launch:applicationID>2393-9323-E08C-03B1
        </launch:applicationID>
      </launch:creData>
    </extension>
    <trID>
      <clTRID>ABC-12345</clTRID>
      <svTRID>54321-XYZ</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_create_response(*res).unwrap();
        assert_eq!(data.pending, true);
        let launch_create = data.launch_create.unwrap();
        assert_eq!(
            launch_create.phase.phase_type,
            super::launch::PhaseType::Sunrise
        );
        assert_eq!(launch_create.application_id.unwrap(), "2393-9323-E08C-03B1");
    }
}
