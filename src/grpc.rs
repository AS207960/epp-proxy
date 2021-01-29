//! Implements the gRPC interface for the EPP client

use std::convert::{TryFrom, TryInto};

use futures::sink::SinkExt;

use super::client;

pub mod epp_proto {
    tonic::include_proto!("epp");
    pub mod common {
        tonic::include_proto!("epp.common");
    }

    pub mod domain {
        tonic::include_proto!("epp.domain");
    }

    pub mod host {
        tonic::include_proto!("epp.host");
    }

    pub mod contact {
        tonic::include_proto!("epp.contact");
    }

    pub mod rgp {
        tonic::include_proto!("epp.rgp");
    }

    pub mod nominet {
        tonic::include_proto!("epp.nominet");
    }

    pub mod fee {
        tonic::include_proto!("epp.fee");
    }

    pub mod launch {
        tonic::include_proto!("epp.launch");
    }
}

/// Helper function to convert chrono times to protobuf well-known type times
fn chrono_to_proto<T: chrono::TimeZone>(
    time: Option<chrono::DateTime<T>>,
) -> Option<prost_types::Timestamp> {
    time.map(|t| prost_types::Timestamp {
        seconds: t.timestamp(),
        nanos: t.timestamp_subsec_nanos() as i32,
    })
}

fn proto_to_chrono(time: Option<prost_types::Timestamp>) -> Option<chrono::DateTime<chrono::Utc>> {
    use chrono::offset::TimeZone;
    match time {
        Some(t) => chrono::Utc
            .timestamp_opt(t.seconds, t.nanos as u32)
            .single(),
        None => None,
    }
}

#[derive(Debug)]
pub struct EPPProxy {
    pub client_router: super::Router,
}

impl From<client::Error> for tonic::Status {
    fn from(err: client::Error) -> Self {
        match err {
            client::Error::Err(s) => tonic::Status::invalid_argument(s),
            client::Error::NotReady => tonic::Status::unavailable("not yet ready"),
            client::Error::Unsupported => {
                tonic::Status::unimplemented("unsupported operation for registrar")
            }
            client::Error::Timeout => {
                tonic::Status::deadline_exceeded("registrar didn't respond in time")
            }
            client::Error::InternalServerError => tonic::Status::internal("internal server error"),
        }
    }
}

fn entity_type_from_i32(from: i32) -> Option<client::contact::EntityType> {
    match epp_proto::contact::EntityType::from_i32(from) {
        Some(e) => match e {
            epp_proto::contact::EntityType::UkLimitedCompany => {
                Some(client::contact::EntityType::UkLimitedCompany)
            }
            epp_proto::contact::EntityType::UkPublicLimitedCompany => {
                Some(client::contact::EntityType::UkPublicLimitedCompany)
            }
            epp_proto::contact::EntityType::UkPartnership => {
                Some(client::contact::EntityType::UkPartnership)
            }
            epp_proto::contact::EntityType::UkSoleTrader => {
                Some(client::contact::EntityType::UkSoleTrader)
            }
            epp_proto::contact::EntityType::UkLimitedLiabilityPartnership => {
                Some(client::contact::EntityType::UkLimitedLiabilityPartnership)
            }
            epp_proto::contact::EntityType::UkIndustrialProvidentRegisteredCompany => {
                Some(client::contact::EntityType::UkIndustrialProvidentRegisteredCompany)
            }
            epp_proto::contact::EntityType::UkIndividual => {
                Some(client::contact::EntityType::UkIndividual)
            }
            epp_proto::contact::EntityType::UkSchool => Some(client::contact::EntityType::UkSchool),
            epp_proto::contact::EntityType::UkRegisteredCharity => {
                Some(client::contact::EntityType::UkRegisteredCharity)
            }
            epp_proto::contact::EntityType::UkGovernmentBody => {
                Some(client::contact::EntityType::UkGovernmentBody)
            }
            epp_proto::contact::EntityType::UkCorporationByRoyalCharter => {
                Some(client::contact::EntityType::UkCorporationByRoyalCharter)
            }
            epp_proto::contact::EntityType::UkStatutoryBody => {
                Some(client::contact::EntityType::UkStatutoryBody)
            }
            epp_proto::contact::EntityType::UkPoliticalParty => {
                Some(client::contact::EntityType::UkPoliticalParty)
            }
            epp_proto::contact::EntityType::OtherUkEntity => {
                Some(client::contact::EntityType::OtherUkEntity)
            }
            epp_proto::contact::EntityType::FinnishIndividual => {
                Some(client::contact::EntityType::FinnishIndividual)
            }
            epp_proto::contact::EntityType::FinnishCompany => {
                Some(client::contact::EntityType::FinnishCompany)
            }
            epp_proto::contact::EntityType::FinnishAssociation => {
                Some(client::contact::EntityType::FinnishAssociation)
            }
            epp_proto::contact::EntityType::FinnishInstitution => {
                Some(client::contact::EntityType::FinnishInstitution)
            }
            epp_proto::contact::EntityType::FinnishPoliticalParty => {
                Some(client::contact::EntityType::FinnishPoliticalParty)
            }
            epp_proto::contact::EntityType::FinnishMunicipality => {
                Some(client::contact::EntityType::FinnishMunicipality)
            }
            epp_proto::contact::EntityType::FinnishGovernment => {
                Some(client::contact::EntityType::FinnishGovernment)
            }
            epp_proto::contact::EntityType::FinnishPublicCommunity => {
                Some(client::contact::EntityType::FinnishPublicCommunity)
            }
            epp_proto::contact::EntityType::OtherIndividual => {
                Some(client::contact::EntityType::OtherIndividual)
            }
            epp_proto::contact::EntityType::OtherCompany => {
                Some(client::contact::EntityType::OtherCompany)
            }
            epp_proto::contact::EntityType::OtherAssociation => {
                Some(client::contact::EntityType::OtherAssociation)
            }
            epp_proto::contact::EntityType::OtherInstitution => {
                Some(client::contact::EntityType::OtherInstitution)
            }
            epp_proto::contact::EntityType::OtherPoliticalParty => {
                Some(client::contact::EntityType::OtherPoliticalParty)
            }
            epp_proto::contact::EntityType::OtherMunicipality => {
                Some(client::contact::EntityType::OtherMunicipality)
            }
            epp_proto::contact::EntityType::OtherGovernment => {
                Some(client::contact::EntityType::OtherGovernment)
            }
            epp_proto::contact::EntityType::OtherPublicCommunity => {
                Some(client::contact::EntityType::OtherPublicCommunity)
            }
            epp_proto::contact::EntityType::UnknownEntity => {
                Some(client::contact::EntityType::Unknown)
            }
            epp_proto::contact::EntityType::NotSet => None,
        },
        None => None,
    }
}

fn period_unit_from_i32(from: i32) -> client::domain::PeriodUnit {
    match epp_proto::common::period::Unit::from_i32(from) {
        Some(e) => match e {
            epp_proto::common::period::Unit::Months => client::domain::PeriodUnit::Months,
            epp_proto::common::period::Unit::Years => client::domain::PeriodUnit::Years,
        },
        None => client::domain::PeriodUnit::Years,
    }
}

fn i32_from_period_unit(from: client::domain::PeriodUnit) -> i32 {
    match from {
        client::domain::PeriodUnit::Months => epp_proto::common::period::Unit::Months.into(),
        client::domain::PeriodUnit::Years => epp_proto::common::period::Unit::Years.into()
    }
}

fn disclosure_type_from_i32(from: Vec<i32>) -> Vec<client::contact::DisclosureType> {
    let mut out = vec![];
    for i in from {
        if let Some(e) = epp_proto::contact::DisclosureType::from_i32(i) {
            out.push(match e {
                epp_proto::contact::DisclosureType::LocalName => {
                    client::contact::DisclosureType::LocalName
                }
                epp_proto::contact::DisclosureType::InternationalisedName => {
                    client::contact::DisclosureType::InternationalisedName
                }
                epp_proto::contact::DisclosureType::LocalOrganisation => {
                    client::contact::DisclosureType::LocalOrganisation
                }
                epp_proto::contact::DisclosureType::InternationalisedOrganisation => {
                    client::contact::DisclosureType::InternationalisedOrganisation
                }
                epp_proto::contact::DisclosureType::LocalAddress => {
                    client::contact::DisclosureType::LocalAddress
                }
                epp_proto::contact::DisclosureType::InternationalisedAddress => {
                    client::contact::DisclosureType::InternationalisedAddress
                }
                epp_proto::contact::DisclosureType::Voice => client::contact::DisclosureType::Voice,
                epp_proto::contact::DisclosureType::Fax => client::contact::DisclosureType::Fax,
                epp_proto::contact::DisclosureType::Email => client::contact::DisclosureType::Email,
            })
        }
    }
    out
}

fn contact_status_from_i32(from: Vec<i32>) -> Vec<client::contact::Status> {
    let mut out = vec![];
    for i in from {
        if let Some(e) = epp_proto::contact::ContactStatus::from_i32(i) {
            out.push(match e {
                epp_proto::contact::ContactStatus::ClientDeleteProhibited => {
                    client::contact::Status::ClientDeleteProhibited
                }
                epp_proto::contact::ContactStatus::ClientTransferProhibited => {
                    client::contact::Status::ClientTransferProhibited
                }
                epp_proto::contact::ContactStatus::ClientUpdateProhibited => {
                    client::contact::Status::ClientUpdateProhibited
                }
                epp_proto::contact::ContactStatus::Linked => client::contact::Status::Linked,
                epp_proto::contact::ContactStatus::Ok => client::contact::Status::Ok,
                epp_proto::contact::ContactStatus::PendingCreate => {
                    client::contact::Status::PendingCreate
                }
                epp_proto::contact::ContactStatus::PendingDelete => {
                    client::contact::Status::PendingDelete
                }
                epp_proto::contact::ContactStatus::PendingTransfer => {
                    client::contact::Status::PendingTransfer
                }
                epp_proto::contact::ContactStatus::PendingUpdate => {
                    client::contact::Status::PendingUpdate
                }
                epp_proto::contact::ContactStatus::ServerDeleteProhibited => {
                    client::contact::Status::ServerDeleteProhibited
                }
                epp_proto::contact::ContactStatus::ServerTransferProhibited => {
                    client::contact::Status::ServerTransferProhibited
                }
                epp_proto::contact::ContactStatus::ServerUpdateProhibited => {
                    client::contact::Status::ServerUpdateProhibited
                }
            })
        }
    }
    out
}

fn host_status_from_i32(from: i32) -> Option<client::host::Status> {
    match epp_proto::host::HostStatus::from_i32(from) {
        Some(e) => Some(match e {
            epp_proto::host::HostStatus::ClientDeleteProhibited => {
                client::host::Status::ClientDeleteProhibited
            }
            epp_proto::host::HostStatus::ClientUpdateProhibited => {
                client::host::Status::ClientUpdateProhibited
            }
            epp_proto::host::HostStatus::Linked => client::host::Status::Linked,
            epp_proto::host::HostStatus::Ok => client::host::Status::Ok,
            epp_proto::host::HostStatus::PendingCreate => client::host::Status::PendingCreate,
            epp_proto::host::HostStatus::PendingDelete => client::host::Status::PendingDelete,
            epp_proto::host::HostStatus::PendingTransfer => client::host::Status::PendingTransfer,
            epp_proto::host::HostStatus::PendingUpdate => client::host::Status::PendingUpdate,
            epp_proto::host::HostStatus::ServerDeleteProhibited => {
                client::host::Status::ServerDeleteProhibited
            }
            epp_proto::host::HostStatus::ServerUpdateProhibited => {
                client::host::Status::ServerUpdateProhibited
            }
        }),
        None => None,
    }
}

fn domain_status_from_i32(from: i32) -> Option<client::domain::Status> {
    match epp_proto::domain::DomainStatus::from_i32(from) {
        Some(e) => Some(match e {
            epp_proto::domain::DomainStatus::ClientDeleteProhibited => {
                client::domain::Status::ClientDeleteProhibited
            }
            epp_proto::domain::DomainStatus::ClientHold => client::domain::Status::ClientHold,
            epp_proto::domain::DomainStatus::ClientRenewProhibited => {
                client::domain::Status::ClientRenewProhibited
            }
            epp_proto::domain::DomainStatus::ClientTransferProhibited => {
                client::domain::Status::ClientTransferProhibited
            }
            epp_proto::domain::DomainStatus::ClientUpdateProhibited => {
                client::domain::Status::ClientUpdateProhibited
            }
            epp_proto::domain::DomainStatus::Inactive => client::domain::Status::Inactive,
            epp_proto::domain::DomainStatus::Ok => client::domain::Status::Ok,
            epp_proto::domain::DomainStatus::PendingCreate => client::domain::Status::PendingCreate,
            epp_proto::domain::DomainStatus::PendingDelete => client::domain::Status::PendingDelete,
            epp_proto::domain::DomainStatus::PendingRenew => client::domain::Status::PendingRenew,
            epp_proto::domain::DomainStatus::PendingTransfer => {
                client::domain::Status::PendingTransfer
            }
            epp_proto::domain::DomainStatus::PendingUpdate => client::domain::Status::PendingUpdate,
            epp_proto::domain::DomainStatus::ServerDeleteProhibited => {
                client::domain::Status::ServerDeleteProhibited
            }
            epp_proto::domain::DomainStatus::ServerHold => client::domain::Status::ServerHold,
            epp_proto::domain::DomainStatus::ServerRenewProhibited => {
                client::domain::Status::ServerRenewProhibited
            }
            epp_proto::domain::DomainStatus::ServerTransferProhibited => {
                client::domain::Status::ServerTransferProhibited
            }
            epp_proto::domain::DomainStatus::ServerUpdateProhibited => {
                client::domain::Status::ServerUpdateProhibited
            }
        }),
        None => None,
    }
}

fn i32_from_transfer_status(from: client::TransferStatus) -> i32 {
    match from {
        client::TransferStatus::ClientApproved => {
            epp_proto::common::TransferStatus::ClientApproved.into()
        }
        client::TransferStatus::ClientCancelled => {
            epp_proto::common::TransferStatus::ClientCancelled.into()
        }
        client::TransferStatus::ClientRejected => {
            epp_proto::common::TransferStatus::ClientRejected.into()
        }
        client::TransferStatus::Pending => epp_proto::common::TransferStatus::Pending.into(),
        client::TransferStatus::ServerApproved => {
            epp_proto::common::TransferStatus::ServerApproved.into()
        }
        client::TransferStatus::ServerCancelled => {
            epp_proto::common::TransferStatus::ServerCancelled.into()
        }
    }
}

fn i32_from_restore_status(from: client::rgp::RGPState) -> i32 {
    match from {
        client::rgp::RGPState::Unknown => epp_proto::rgp::RgpState::Unknown.into(),
        client::rgp::RGPState::AddPeriod => epp_proto::rgp::RgpState::AddPeriod.into(),
        client::rgp::RGPState::AutoRenewPeriod => epp_proto::rgp::RgpState::AutoRenewPeriod.into(),
        client::rgp::RGPState::RenewPeriod => epp_proto::rgp::RgpState::RenewPeriod.into(),
        client::rgp::RGPState::TransferPeriod => epp_proto::rgp::RgpState::TransferPeriod.into(),
        client::rgp::RGPState::RedemptionPeriod => {
            epp_proto::rgp::RgpState::RedemptionPeriod.into()
        }
        client::rgp::RGPState::PendingRestore => epp_proto::rgp::RgpState::PendingRestore.into(),
        client::rgp::RGPState::PendingDelete => epp_proto::rgp::RgpState::PendingDelete.into(),
    }
}

impl From<epp_proto::contact::Phone> for client::contact::Phone {
    fn from(from: epp_proto::contact::Phone) -> Self {
        client::contact::Phone {
            number: from.number,
            extension: from.extension,
        }
    }
}

impl From<client::contact::Phone> for epp_proto::contact::Phone {
    fn from(from: client::contact::Phone) -> Self {
        epp_proto::contact::Phone {
            number: from.number,
            extension: from.extension,
        }
    }
}

fn fee_command_from_i32(from: i32) -> client::fee::Command {
    match epp_proto::fee::Command::from_i32(from) {
        Some(e) => match e {
            epp_proto::fee::Command::Create => client::fee::Command::Create,
            epp_proto::fee::Command::Renew => client::fee::Command::Renew,
            epp_proto::fee::Command::Transfer => client::fee::Command::Transfer,
            epp_proto::fee::Command::Delete => client::fee::Command::Delete,
            epp_proto::fee::Command::Restore => client::fee::Command::Restore,
            epp_proto::fee::Command::Update => client::fee::Command::Update,
            epp_proto::fee::Command::Check => client::fee::Command::Check,
            epp_proto::fee::Command::Info => client::fee::Command::Info,
            epp_proto::fee::Command::Custom => client::fee::Command::Custom,
        },
        None => client::fee::Command::Create,
    }
}

fn i32_from_fee_command(from: client::fee::Command) -> i32 {
    match from {
        client::fee::Command::Create => epp_proto::fee::Command::Create.into(),
        client::fee::Command::Renew => epp_proto::fee::Command::Renew.into(),
        client::fee::Command::Transfer => epp_proto::fee::Command::Transfer.into(),
        client::fee::Command::Delete => epp_proto::fee::Command::Delete.into(),
        client::fee::Command::Restore => epp_proto::fee::Command::Restore.into(),
        client::fee::Command::Update => epp_proto::fee::Command::Update.into(),
        client::fee::Command::Check => epp_proto::fee::Command::Check.into(),
        client::fee::Command::Info => epp_proto::fee::Command::Info.into(),
        client::fee::Command::Custom => epp_proto::fee::Command::Custom.into(),
    }
}

impl From<epp_proto::fee::FeeCheck> for client::fee::FeeCheck {
    fn from(from: epp_proto::fee::FeeCheck) -> Self {
        client::fee::FeeCheck {
            currency: from.currency,
            commands: from.commands.into_iter().map(|c| client::fee::FeeCheckCommand {
                command: fee_command_from_i32(c.command),
                period: c.period.map(|p| client::domain::Period {
                    unit: period_unit_from_i32(p.unit),
                    value: p.value,
                }),
            }).collect(),
        }
    }
}

impl From<client::fee::FeeCheckData> for epp_proto::fee::FeeCheckData {
    fn from(from: client::fee::FeeCheckData) -> Self {
        epp_proto::fee::FeeCheckData {
            available: from.available,
            commands: from.commands.into_iter().map(|c| epp_proto::fee::fee_check_data::FeeCommand {
                command: i32_from_fee_command(c.command),
                standard: c.standard,
                period: c.period.map(|p| epp_proto::common::Period {
                    unit: i32_from_period_unit(p.unit),
                    value: p.value,
                }),
                currency: c.currency,
                fees: c.fees.into_iter().map(Into::into).collect(),
                credits: c.credits.into_iter().map(Into::into).collect(),
                class: c.class,
                reason: c.reason,
            }).collect(),
            reason: from.reason,
        }
    }
}

impl From<client::fee::FeeData> for epp_proto::fee::FeeData {
    fn from(from: client::fee::FeeData) -> Self {
        epp_proto::fee::FeeData {
            period: from.period.map(|p| epp_proto::common::Period {
                unit: i32_from_period_unit(p.unit),
                value: p.value,
            }),
            currency: from.currency,
            fees: from.fees.into_iter().map(Into::into).collect(),
            credits: from.credits.into_iter().map(Into::into).collect(),
            balance: from.balance,
            credit_limit: from.credit_limit,
        }
    }
}

impl From<client::fee::Fee> for epp_proto::fee::Fee {
    fn from(from: client::fee::Fee) -> Self {
        epp_proto::fee::Fee {
            value: from.value,
            description: from.description,
            refundable: from.refundable,
            grace_period: from.grace_period,
            applied: match from.applied {
                client::fee::Applied::Immediate => epp_proto::fee::Applied::Immediate.into(),
                client::fee::Applied::Delayed => epp_proto::fee::Applied::Delayed.into(),
            },
        }
    }
}

impl From<client::fee::Credit> for epp_proto::fee::Credit {
    fn from(from: client::fee::Credit) -> Self {
        epp_proto::fee::Credit {
            value: from.value,
            description: from.description,
        }
    }
}

impl From<client::fee::DonutsFeeData> for epp_proto::fee::DonutsFeeData {
    fn from(from: client::fee::DonutsFeeData) -> Self {
        epp_proto::fee::DonutsFeeData {
            fees: from.sets.into_iter().map(|f| epp_proto::fee::DonutsFeeSet {
                category: Some(epp_proto::fee::DonutsCategory {
                    name: f.category.name,
                    value: f.category.category,
                }),
                fee_type: Some(epp_proto::fee::DonutsFeeType {
                    fee_type: match f.fee_type.fee_type {
                        client::fee::DonutsFeeTypes::Fee => epp_proto::fee::donuts_fee_type::FeeTypes::Fee.into(),
                        client::fee::DonutsFeeTypes::Price => epp_proto::fee::donuts_fee_type::FeeTypes::Price.into(),
                        client::fee::DonutsFeeTypes::Custom => epp_proto::fee::donuts_fee_type::FeeTypes::Custom.into(),
                    },
                    name: f.fee_type.name,
                }),
                fees: f.fees.into_iter().map(|a| epp_proto::fee::DonutsAmount {
                    command: i32_from_fee_command(a.command),
                    name: a.command_name,
                    value: a.value,
                }).collect(),
            }).collect()
        }
    }
}

impl TryFrom<epp_proto::fee::DonutsFeeData> for client::fee::DonutsFeeData {
    type Error = tonic::Status;

    fn try_from(from: epp_proto::fee::DonutsFeeData) -> Result<Self, Self::Error> {
        Ok(client::fee::DonutsFeeData {
            sets: from.fees.into_iter().map(|f| Ok(client::fee::DonutsFeeSet {
                category: match f.category {
                    Some(c) => client::fee::DonutsCategory {
                        name: c.name,
                        category: c.value,
                    },
                    None => return Err(tonic::Status::invalid_argument(
                        "Category must be specified",
                    ))
                },
                fee_type: match f.fee_type {
                    Some(f) => client::fee::DonutsFeeType {
                        fee_type: match epp_proto::fee::donuts_fee_type::FeeTypes::from_i32(f.fee_type) {
                            Some(epp_proto::fee::donuts_fee_type::FeeTypes::Fee) => client::fee::DonutsFeeTypes::Fee,
                            Some(epp_proto::fee::donuts_fee_type::FeeTypes::Price) => client::fee::DonutsFeeTypes::Price,
                            Some(epp_proto::fee::donuts_fee_type::FeeTypes::Custom) => client::fee::DonutsFeeTypes::Custom,
                            None => return Err(tonic::Status::invalid_argument(
                                "Unknown fee type",
                            ))
                        },
                        name: f.name,
                    },
                    None => return Err(tonic::Status::invalid_argument(
                        "Fee type must be specified",
                    ))
                },
                fees: f.fees.into_iter().map(|a| client::fee::DonutsAmount {
                    command: fee_command_from_i32(a.command),
                    command_name: a.name,
                    value: a.value,
                }).collect(),
            })).collect::<Result<Vec<_>, _>>()?
        })
    }
}

impl From<client::domain::InfoResponse> for epp_proto::domain::DomainInfoReply {
    fn from(res: client::domain::InfoResponse) -> Self {
        epp_proto::domain::DomainInfoReply {
            name: res.name,
            registry_id: res.registry_id,
            statuses: res
                .statuses
                .into_iter()
                .map(|s| match s {
                    client::domain::Status::ClientDeleteProhibited => {
                        epp_proto::domain::DomainStatus::ClientDeleteProhibited.into()
                    }
                    client::domain::Status::ClientHold => {
                        epp_proto::domain::DomainStatus::ClientHold.into()
                    }
                    client::domain::Status::ClientRenewProhibited => {
                        epp_proto::domain::DomainStatus::ClientRenewProhibited.into()
                    }
                    client::domain::Status::ClientTransferProhibited => {
                        epp_proto::domain::DomainStatus::ClientTransferProhibited.into()
                    }
                    client::domain::Status::ClientUpdateProhibited => {
                        epp_proto::domain::DomainStatus::ClientUpdateProhibited.into()
                    }
                    client::domain::Status::Inactive => {
                        epp_proto::domain::DomainStatus::Inactive.into()
                    }
                    client::domain::Status::Ok => epp_proto::domain::DomainStatus::Ok.into(),
                    client::domain::Status::PendingCreate => {
                        epp_proto::domain::DomainStatus::PendingCreate.into()
                    }
                    client::domain::Status::PendingDelete => {
                        epp_proto::domain::DomainStatus::PendingDelete.into()
                    }
                    client::domain::Status::PendingRenew => {
                        epp_proto::domain::DomainStatus::PendingRenew.into()
                    }
                    client::domain::Status::PendingTransfer => {
                        epp_proto::domain::DomainStatus::PendingTransfer.into()
                    }
                    client::domain::Status::PendingUpdate => {
                        epp_proto::domain::DomainStatus::PendingUpdate.into()
                    }
                    client::domain::Status::ServerDeleteProhibited => {
                        epp_proto::domain::DomainStatus::ServerDeleteProhibited.into()
                    }
                    client::domain::Status::ServerHold => {
                        epp_proto::domain::DomainStatus::ServerHold.into()
                    }
                    client::domain::Status::ServerRenewProhibited => {
                        epp_proto::domain::DomainStatus::ServerRenewProhibited.into()
                    }
                    client::domain::Status::ServerTransferProhibited => {
                        epp_proto::domain::DomainStatus::ServerTransferProhibited.into()
                    }
                    client::domain::Status::ServerUpdateProhibited => {
                        epp_proto::domain::DomainStatus::ServerUpdateProhibited.into()
                    }
                })
                .collect(),
            registrant: res.registrant,
            contacts: res
                .contacts
                .into_iter()
                .map(|c| epp_proto::domain::Contact {
                    id: c.contact_id,
                    r#type: c.contact_type,
                })
                .collect(),
            nameservers: res
                .nameservers
                .into_iter()
                .map(|n| match n {
                    client::domain::InfoNameserver::HostOnly(h) => epp_proto::domain::NameServer {
                        server: Some(epp_proto::domain::name_server::Server::HostObj(h)),
                        addresses: vec![],
                    },
                    client::domain::InfoNameserver::HostAndAddress { host, addresses } => {
                        epp_proto::domain::NameServer {
                            server: Some(epp_proto::domain::name_server::Server::HostName(host)),
                            addresses: addresses
                                .iter()
                                .map(|addr| epp_proto::common::IpAddress {
                                    address: addr.address.clone(),
                                    r#type: match addr.ip_version {
                                        client::host::AddressVersion::IPv4 => {
                                            epp_proto::common::ip_address::IpVersion::IPv4.into()
                                        }
                                        client::host::AddressVersion::IPv6 => {
                                            epp_proto::common::ip_address::IpVersion::IPv6.into()
                                        }
                                    },
                                })
                                .collect(),
                        }
                    }
                })
                .collect(),
            hosts: res.hosts,
            client_id: res.client_id,
            client_created_id: res.client_created_id,
            creation_date: chrono_to_proto(res.creation_date),
            expiry_date: chrono_to_proto(res.expiry_date),
            last_updated_client: res.last_updated_client,
            last_updated_date: chrono_to_proto(res.last_updated_date),
            last_transfer_date: chrono_to_proto(res.last_transfer_date),
            registry_name: String::new(),
            rgp_state: res.rgp_state.into_iter().map(i32_from_restore_status).collect(),
            auth_info: res.auth_info,
            sec_dns: res.sec_dns.map(|sec_dns| epp_proto::domain::SecDnsData {
                max_sig_life: sec_dns.max_sig_life,
                data: Some(match sec_dns.data {
                    client::domain::SecDNSDataType::DSData(ds_data) => {
                        epp_proto::domain::sec_dns_data::Data::DsData(
                            epp_proto::domain::SecDnsdsData {
                                data: ds_data
                                    .into_iter()
                                    .map(|d| epp_proto::domain::SecDnsdsDatum {
                                        key_tag: d.key_tag as u32,
                                        algorithm: d.algorithm as u32,
                                        digest_type: d.digest_type as u32,
                                        digest: d.digest,
                                        key_data: d.key_data.map(|k| {
                                            epp_proto::domain::SecDnsKeyDatum {
                                                flags: k.flags as u32,
                                                protocol: k.protocol as u32,
                                                algorithm: k.algorithm as u32,
                                                public_key: k.public_key,
                                            }
                                        }),
                                    })
                                    .collect(),
                            },
                        )
                    }
                    client::domain::SecDNSDataType::KeyData(key_data) => {
                        epp_proto::domain::sec_dns_data::Data::KeyData(
                            epp_proto::domain::SecDnsKeyData {
                                data: key_data
                                    .into_iter()
                                    .map(|k| epp_proto::domain::SecDnsKeyDatum {
                                        flags: k.flags as u32,
                                        protocol: k.protocol as u32,
                                        algorithm: k.algorithm as u32,
                                        public_key: k.public_key,
                                    })
                                    .collect(),
                            },
                        )
                    }
                }),
            }),
            launch_info: res.launch_info.map(Into::into),
            donuts_fee_data: res.donuts_fee_data.map(Into::into),
        }
    }
}

impl From<client::domain::CreateResponse> for epp_proto::domain::DomainCreateReply {
    fn from(res: client::domain::CreateResponse) -> Self {
        epp_proto::domain::DomainCreateReply {
            name: res.data.name,
            pending: res.pending,
            transaction_id: res.transaction_id,
            creation_date: chrono_to_proto(res.data.creation_date),
            expiry_date: chrono_to_proto(res.data.expiration_date),
            fee_data: res.fee_data.map(Into::into),
            donuts_fee_data: res.donuts_fee_data.map(Into::into),
            registry_name: String::new(),
            launch_data: res.launch_create.map(Into::into),
        }
    }
}

impl From<client::domain::RenewResponse> for epp_proto::domain::DomainRenewReply {
    fn from(res: client::domain::RenewResponse) -> Self {
        epp_proto::domain::DomainRenewReply {
            name: res.data.name,
            pending: res.pending,
            transaction_id: res.transaction_id,
            expiry_date: chrono_to_proto(res.data.new_expiry_date),
            fee_data: res.fee_data.map(Into::into),
            donuts_fee_data: res.donuts_fee_data.map(Into::into),
            registry_name: String::new(),
        }
    }
}

impl From<client::domain::TransferResponse> for epp_proto::domain::DomainTransferReply {
    fn from(res: client::domain::TransferResponse) -> Self {
        epp_proto::domain::DomainTransferReply {
            pending: res.pending,
            transaction_id: res.transaction_id,
            status: i32_from_transfer_status(res.data.status),
            requested_client_id: res.data.requested_client_id,
            requested_date: chrono_to_proto(Some(res.data.requested_date)),
            act_client_id: res.data.act_client_id,
            act_date: chrono_to_proto(Some(res.data.act_date)),
            expiry_date: chrono_to_proto(res.data.expiry_date),
            fee_data: res.fee_data.map(Into::into),
            donuts_fee_data: res.donuts_fee_data.map(Into::into),
            registry_name: String::new(),
        }
    }
}

impl From<client::domain::PanData> for epp_proto::domain::DomainPanReply {
    fn from(res: client::domain::PanData) -> Self {
        epp_proto::domain::DomainPanReply {
            name: res.name,
            result: res.result,
            server_transaction_id: res.server_transaction_id,
            client_transaction_id: res.client_transaction_id,
            date: chrono_to_proto(Some(res.date)),
        }
    }
}

impl From<client::contact::InfoResponse> for epp_proto::contact::ContactInfoReply {
    fn from(res: client::contact::InfoResponse) -> Self {
        let map_addr = |a: client::contact::Address| epp_proto::contact::PostalAddress {
            name: a.name,
            organisation: a.organisation,
            streets: a.streets,
            city: a.city,
            province: a.province,
            postal_code: a.postal_code,
            country_code: a.country_code,
            identity_number: a.identity_number,
            birth_date: chrono_to_proto(a.birth_date.map(|d| d.and_hms(0, 0, 0))),
        };

        epp_proto::contact::ContactInfoReply {
            id: res.id,
            registry_id: res.registry_id,
            statuses: res
                .statuses
                .into_iter()
                .map(|s| match s {
                    client::contact::Status::ClientDeleteProhibited => {
                        epp_proto::contact::ContactStatus::ClientDeleteProhibited.into()
                    }
                    client::contact::Status::ClientTransferProhibited => {
                        epp_proto::contact::ContactStatus::ClientTransferProhibited.into()
                    }
                    client::contact::Status::ClientUpdateProhibited => {
                        epp_proto::contact::ContactStatus::ClientUpdateProhibited.into()
                    }
                    client::contact::Status::Linked => {
                        epp_proto::contact::ContactStatus::Linked.into()
                    }
                    client::contact::Status::Ok => epp_proto::contact::ContactStatus::Ok.into(),
                    client::contact::Status::PendingCreate => {
                        epp_proto::contact::ContactStatus::PendingCreate.into()
                    }
                    client::contact::Status::PendingDelete => {
                        epp_proto::contact::ContactStatus::PendingDelete.into()
                    }
                    client::contact::Status::PendingTransfer => {
                        epp_proto::contact::ContactStatus::PendingTransfer.into()
                    }
                    client::contact::Status::PendingUpdate => {
                        epp_proto::contact::ContactStatus::PendingUpdate.into()
                    }
                    client::contact::Status::ServerDeleteProhibited => {
                        epp_proto::contact::ContactStatus::ServerDeleteProhibited.into()
                    }
                    client::contact::Status::ServerTransferProhibited => {
                        epp_proto::contact::ContactStatus::ServerTransferProhibited.into()
                    }
                    client::contact::Status::ServerUpdateProhibited => {
                        epp_proto::contact::ContactStatus::ServerUpdateProhibited.into()
                    }
                })
                .collect(),
            local_address: res.local_address.map(map_addr),
            internationalised_address: res.internationalised_address.map(map_addr),
            phone: res.phone.map(|p| p.into()),
            fax: res.fax.map(|p| p.into()),
            email: res.email,
            client_id: res.client_id,
            client_created_id: res.client_created_id,
            creation_date: chrono_to_proto(res.creation_date),
            last_updated_client: res.last_updated_client,
            last_updated_date: chrono_to_proto(res.last_updated_date),
            last_transfer_date: chrono_to_proto(res.last_transfer_date),
            entity_type: match res.entity_type {
                client::contact::EntityType::UkLimitedCompany => {
                    epp_proto::contact::EntityType::UkLimitedCompany.into()
                }
                client::contact::EntityType::UkPublicLimitedCompany => {
                    epp_proto::contact::EntityType::UkLimitedCompany.into()
                }
                client::contact::EntityType::UkPartnership => {
                    epp_proto::contact::EntityType::UkPartnership.into()
                }
                client::contact::EntityType::UkSoleTrader => {
                    epp_proto::contact::EntityType::UkSoleTrader.into()
                }
                client::contact::EntityType::UkLimitedLiabilityPartnership => {
                    epp_proto::contact::EntityType::UkLimitedLiabilityPartnership.into()
                }
                client::contact::EntityType::UkIndustrialProvidentRegisteredCompany => {
                    epp_proto::contact::EntityType::UkIndustrialProvidentRegisteredCompany.into()
                }
                client::contact::EntityType::UkIndividual => {
                    epp_proto::contact::EntityType::UkIndividual.into()
                }
                client::contact::EntityType::UkSchool => {
                    epp_proto::contact::EntityType::UkSchool.into()
                }
                client::contact::EntityType::UkRegisteredCharity => {
                    epp_proto::contact::EntityType::UkRegisteredCharity.into()
                }
                client::contact::EntityType::UkGovernmentBody => {
                    epp_proto::contact::EntityType::UkGovernmentBody.into()
                }
                client::contact::EntityType::UkCorporationByRoyalCharter => {
                    epp_proto::contact::EntityType::UkCorporationByRoyalCharter.into()
                }
                client::contact::EntityType::UkStatutoryBody => {
                    epp_proto::contact::EntityType::UkStatutoryBody.into()
                }
                client::contact::EntityType::UkPoliticalParty => {
                    epp_proto::contact::EntityType::UkPoliticalParty.into()
                }
                client::contact::EntityType::OtherUkEntity => {
                    epp_proto::contact::EntityType::OtherUkEntity.into()
                }
                client::contact::EntityType::FinnishIndividual => {
                    epp_proto::contact::EntityType::FinnishIndividual.into()
                }
                client::contact::EntityType::FinnishCompany => {
                    epp_proto::contact::EntityType::FinnishCompany.into()
                }
                client::contact::EntityType::FinnishAssociation => {
                    epp_proto::contact::EntityType::FinnishAssociation.into()
                }
                client::contact::EntityType::FinnishInstitution => {
                    epp_proto::contact::EntityType::FinnishInstitution.into()
                }
                client::contact::EntityType::FinnishPoliticalParty => {
                    epp_proto::contact::EntityType::FinnishPoliticalParty.into()
                }
                client::contact::EntityType::FinnishMunicipality => {
                    epp_proto::contact::EntityType::FinnishMunicipality.into()
                }
                client::contact::EntityType::FinnishGovernment => {
                    epp_proto::contact::EntityType::FinnishGovernment.into()
                }
                client::contact::EntityType::FinnishPublicCommunity => {
                    epp_proto::contact::EntityType::FinnishPublicCommunity.into()
                }
                client::contact::EntityType::OtherIndividual => {
                    epp_proto::contact::EntityType::OtherIndividual.into()
                }
                client::contact::EntityType::OtherCompany => {
                    epp_proto::contact::EntityType::OtherCompany.into()
                }
                client::contact::EntityType::OtherAssociation => {
                    epp_proto::contact::EntityType::OtherAssociation.into()
                }
                client::contact::EntityType::OtherInstitution => {
                    epp_proto::contact::EntityType::OtherInstitution.into()
                }
                client::contact::EntityType::OtherPoliticalParty => {
                    epp_proto::contact::EntityType::OtherPoliticalParty.into()
                }
                client::contact::EntityType::OtherMunicipality => {
                    epp_proto::contact::EntityType::OtherMunicipality.into()
                }
                client::contact::EntityType::OtherGovernment => {
                    epp_proto::contact::EntityType::OtherGovernment.into()
                }
                client::contact::EntityType::OtherPublicCommunity => {
                    epp_proto::contact::EntityType::OtherPublicCommunity.into()
                }
                client::contact::EntityType::Unknown => {
                    epp_proto::contact::EntityType::UnknownEntity.into()
                }
            },
            trading_name: res.trading_name,
            company_number: res.company_number,
            disclosure: res
                .disclosure
                .into_iter()
                .map(|d| match d {
                    client::contact::DisclosureType::LocalName => {
                        epp_proto::contact::DisclosureType::LocalName.into()
                    }
                    client::contact::DisclosureType::InternationalisedName => {
                        epp_proto::contact::DisclosureType::InternationalisedName.into()
                    }
                    client::contact::DisclosureType::LocalOrganisation => {
                        epp_proto::contact::DisclosureType::LocalOrganisation.into()
                    }
                    client::contact::DisclosureType::InternationalisedOrganisation => {
                        epp_proto::contact::DisclosureType::InternationalisedOrganisation.into()
                    }
                    client::contact::DisclosureType::LocalAddress => {
                        epp_proto::contact::DisclosureType::LocalAddress.into()
                    }
                    client::contact::DisclosureType::InternationalisedAddress => {
                        epp_proto::contact::DisclosureType::InternationalisedAddress.into()
                    }
                    client::contact::DisclosureType::Voice => {
                        epp_proto::contact::DisclosureType::Voice.into()
                    }
                    client::contact::DisclosureType::Fax => {
                        epp_proto::contact::DisclosureType::Fax.into()
                    }
                    client::contact::DisclosureType::Email => {
                        epp_proto::contact::DisclosureType::Email.into()
                    }
                })
                .collect(),
            auth_info: res.auth_info,
        }
    }
}

impl From<client::balance::BalanceResponse> for epp_proto::BalanceReply {
    fn from(res: client::balance::BalanceResponse) -> Self {
        epp_proto::BalanceReply {
            balance: res.balance,
            currency: res.currency,
            available_credit: res.available_credit,
            credit_limit: res.credit_limit,
            credit_threshold: res.credit_threshold.map(|t| match t {
                client::balance::CreditThreshold::Fixed(f) => {
                    epp_proto::balance_reply::CreditThreshold::FixedCreditThreshold(f)
                }
                client::balance::CreditThreshold::Percentage(p) => {
                    epp_proto::balance_reply::CreditThreshold::PercentageCreditThreshold(p.into())
                }
            }),
        }
    }
}

impl From<client::verisign::LowBalanceData> for epp_proto::BalanceReply {
    fn from(res: client::verisign::LowBalanceData) -> Self {
        epp_proto::BalanceReply {
            balance: String::new(),
            currency: String::new(),
            available_credit: Some(res.available_credit),
            credit_limit: Some(res.credit_limit),
            credit_threshold: Some(match res.credit_threshold {
                client::verisign::CreditThreshold::Fixed(f) => {
                    epp_proto::balance_reply::CreditThreshold::FixedCreditThreshold(f)
                }
                client::verisign::CreditThreshold::Percentage(p) => {
                    epp_proto::balance_reply::CreditThreshold::PercentageCreditThreshold(p.into())
                }
            }),
        }
    }
}

impl From<client::nominet::CancelData> for epp_proto::nominet::DomainCancelData {
    fn from(res: client::nominet::CancelData) -> Self {
        epp_proto::nominet::DomainCancelData {
            name: res.domain_name,
            originator: res.originator,
        }
    }
}

impl From<client::nominet::ReleaseData> for epp_proto::nominet::DomainReleaseData {
    fn from(res: client::nominet::ReleaseData) -> Self {
        epp_proto::nominet::DomainReleaseData {
            account_id: res.account_id,
            account_moved: res.account_moved,
            from: res.from,
            registrar_tag: res.registrar_tag,
            domains: res.domains,
        }
    }
}

impl From<client::nominet::RegistrarChangeData> for epp_proto::nominet::DomainRegistrarChangeData {
    fn from(res: client::nominet::RegistrarChangeData) -> Self {
        epp_proto::nominet::DomainRegistrarChangeData {
            originator: res.originator,
            registrar_tag: res.registrar_tag,
            case_id: res.case_id,
            domains: res.domains.into_iter().map(Into::into).collect(),
            contact: Some(res.contact.into()),
        }
    }
}

impl From<client::nominet::HostCancelData> for epp_proto::nominet::HostCancelData {
    fn from(res: client::nominet::HostCancelData) -> Self {
        epp_proto::nominet::HostCancelData {
            host_objects: res.host_objects,
            domain_names: res.domain_names,
        }
    }
}

impl From<client::nominet::ProcessData> for epp_proto::nominet::ProcessData {
    fn from(res: client::nominet::ProcessData) -> Self {
        epp_proto::nominet::ProcessData {
            stage: match res.stage {
                client::nominet::ProcessStage::Initial => epp_proto::nominet::process_data::ProcessStage::Initial.into(),
                client::nominet::ProcessStage::Updated => epp_proto::nominet::process_data::ProcessStage::Updated.into(),
            },
            contact: Some(res.contact.into()),
            process_type: res.process_type,
            suspend_date: chrono_to_proto(res.suspend_date),
            cancel_date: chrono_to_proto(res.cancel_date),
            domain_names: res.domain_names,
        }
    }
}

impl From<client::nominet::SuspendData> for epp_proto::nominet::SuspendData {
    fn from(res: client::nominet::SuspendData) -> Self {
        epp_proto::nominet::SuspendData {
            reason: res.reason,
            cancel_date: chrono_to_proto(res.cancel_date),
            domain_names: res.domain_names,
        }
    }
}

impl From<client::nominet::DomainFailData> for epp_proto::nominet::DomainFailData {
    fn from(res: client::nominet::DomainFailData) -> Self {
        epp_proto::nominet::DomainFailData {
            domain: res.domain_name,
            reason: res.reason,
        }
    }
}

impl From<client::nominet::RegistrantTransferData> for epp_proto::nominet::RegistrantTransferData {
    fn from(res: client::nominet::RegistrantTransferData) -> Self {
        epp_proto::nominet::RegistrantTransferData {
            originator: res.originator,
            account_id: res.account_id,
            old_account_id: res.old_account_id,
            case_id: res.case_id,
            domain_names: res.domain_names.into_iter().map(Into::into).collect(),
            contact: Some(res.contact.into()),
        }
    }
}

impl From<epp_proto::launch::Phase> for client::launch::LaunchPhase {
    fn from(from: epp_proto::launch::Phase) -> Self {
        client::launch::LaunchPhase {
            phase_type: match epp_proto::launch::phase::PhaseType::from_i32(from.phase_type) {
                Some(p) => match p {
                    epp_proto::launch::phase::PhaseType::Open => client::launch::PhaseType::Open,
                    epp_proto::launch::phase::PhaseType::Sunrise => client::launch::PhaseType::Sunrise,
                    epp_proto::launch::phase::PhaseType::Landrush => client::launch::PhaseType::Landrush,
                    epp_proto::launch::phase::PhaseType::Claims => client::launch::PhaseType::Claims,
                    epp_proto::launch::phase::PhaseType::Custom => client::launch::PhaseType::Custom,
                },
                None => client::launch::PhaseType::Custom,
            },
            phase_name: from.phase_name,
        }
    }
}

impl From<client::launch::LaunchPhase> for epp_proto::launch::Phase {
    fn from(from: client::launch::LaunchPhase) -> Self {
        epp_proto::launch::Phase {
            phase_type: match from.phase_type {
                client::launch::PhaseType::Open => epp_proto::launch::phase::PhaseType::Open.into(),
                client::launch::PhaseType::Sunrise => epp_proto::launch::phase::PhaseType::Sunrise.into(),
                client::launch::PhaseType::Landrush => epp_proto::launch::phase::PhaseType::Landrush.into(),
                client::launch::PhaseType::Claims => epp_proto::launch::phase::PhaseType::Claims.into(),
                client::launch::PhaseType::Custom => epp_proto::launch::phase::PhaseType::Custom.into(),
            },
            phase_name: from.phase_name,
        }
    }
}

impl From<epp_proto::launch::Phase> for client::launch::LaunchClaimsCheck {
    fn from(from: epp_proto::launch::Phase) -> Self {
        client::launch::LaunchClaimsCheck {
            phase: from.into()
        }
    }
}

impl From<client::launch::LaunchClaimKey> for epp_proto::launch::ClaimsKey {
    fn from(from: client::launch::LaunchClaimKey) -> Self {
        epp_proto::launch::ClaimsKey {
            key: from.key,
            validator_id: from.validator_id,
        }
    }
}

impl TryFrom<epp_proto::launch::LaunchInfo> for client::launch::LaunchInfo {
    type Error = tonic::Status;

    fn try_from(from: epp_proto::launch::LaunchInfo) -> Result<Self, Self::Error> {
        Ok(client::launch::LaunchInfo {
            include_mark: from.include_mark,
            phase: match from.phase {
                Some(p) => p.into(),
                None => return Err(tonic::Status::invalid_argument(
                    "Launch phase must be specified",
                ))
            },
            application_id: from.application_id,
        })
    }
}

impl TryFrom<epp_proto::launch::LaunchCreate> for client::launch::LaunchCreate {
    type Error = tonic::Status;

    fn try_from(from: epp_proto::launch::LaunchCreate) -> Result<Self, Self::Error> {
        Ok(client::launch::LaunchCreate {
            create_type: match epp_proto::launch::launch_create::CreateType::from_i32(from.create_type) {
                Some(epp_proto::launch::launch_create::CreateType::Registration) => client::launch::LaunchCreateType::Registration,
                Some(epp_proto::launch::launch_create::CreateType::Application) => client::launch::LaunchCreateType::Application,
                None => client::launch::LaunchCreateType::Registration,
            },
            phase: match from.phase {
                Some(p) => p.into(),
                None => return Err(tonic::Status::invalid_argument(
                    "Launch phase must be specified",
                ))
            },
            code_mark: from.code_mark.into_iter().map(|m| client::launch::CodeMark {
                code: m.code,
                validator: m.validator,
                mark: m.mark,
            }).collect(),
            signed_mark: from.signed_mark,
            notices: from.notices.into_iter().map(|n| Ok(client::launch::Notice {
                notice_id: n.notice_id,
                validator: n.validator,
                not_after: match proto_to_chrono(n.not_after) {
                    Some(d) => d,
                    None => return Err(tonic::Status::invalid_argument(
                        "Date must be specified",
                    ))
                },
                accepted_date: match proto_to_chrono(n.accepted_after) {
                    Some(d) => d,
                    None => return Err(tonic::Status::invalid_argument(
                        "Date must be specified",
                    ))
                },
            })).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<epp_proto::launch::LaunchData> for client::launch::LaunchUpdate {
    type Error = tonic::Status;

    fn try_from(from: epp_proto::launch::LaunchData) -> Result<Self, Self::Error> {
        Ok(client::launch::LaunchUpdate {
            phase: match from.phase {
                Some(p) => p.into(),
                None => return Err(tonic::Status::invalid_argument(
                    "Launch phase must be specified",
                ))
            },
            application_id: from.application_id,
        })
    }
}

impl From<client::launch::LaunchInfoData> for epp_proto::launch::LaunchInfoData {
    fn from(from: client::launch::LaunchInfoData) -> Self {
        epp_proto::launch::LaunchInfoData {
            phase: Some(from.phase.into()),
            application_id: from.application_id,
            status: from.status.map(|s| epp_proto::launch::Status {
                status_type: match s.status_type {
                    client::launch::LaunchStatusType::PendingValidation => epp_proto::launch::StatusType::PendingValidation.into(),
                    client::launch::LaunchStatusType::Validated => epp_proto::launch::StatusType::Validated.into(),
                    client::launch::LaunchStatusType::Invalid => epp_proto::launch::StatusType::Invalid.into(),
                    client::launch::LaunchStatusType::PendingAllocation => epp_proto::launch::StatusType::PendingAllocation.into(),
                    client::launch::LaunchStatusType::Allocated => epp_proto::launch::StatusType::Allocated.into(),
                    client::launch::LaunchStatusType::Rejected => epp_proto::launch::StatusType::Rejected.into(),
                    client::launch::LaunchStatusType::Custom => epp_proto::launch::StatusType::Custom.into(),
                },
                status_name: s.status_name,
                message: s.message,
            }),
            mark: from.mark,
        }
    }
}

impl From<client::launch::LaunchCreateData> for epp_proto::launch::LaunchData {
    fn from(from: client::launch::LaunchCreateData) -> Self {
        epp_proto::launch::LaunchData {
            phase: Some(from.phase.into()),
            application_id: from.application_id,
        }
    }
}

// fn client_by_domain(
//     router: &super::Router,
//     domain: &str,
// ) -> Result<(client::RequestSender, String), tonic::Status> {
//     match router.client_by_domain(domain) {
//         Some(c) => Ok(c),
//         None => Err(tonic::Status::invalid_argument("unsupported domain")),
//     }
// }


fn client_by_domain_or_id(
    router: &super::Router,
    domain: &str,
    registry_id: Option<String>,
) -> Result<(client::RequestSender, String), tonic::Status> {
    if let Some(r) = registry_id {
        match router.client_by_id(&r) {
            Some(c) => return Ok((c, r)),
            None => {}
        }
    }
    match router.client_by_domain(domain) {
        Some(c) => Ok(c),
        None => Err(tonic::Status::invalid_argument("unsupported domain")),
    }
}

fn client_by_id(router: &super::Router, id: &str) -> Result<client::RequestSender, tonic::Status> {
    match router.client_by_id(id) {
        Some(c) => Ok(c),
        None => Err(tonic::Status::not_found("unknown registry")),
    }
}

#[tonic::async_trait]
impl epp_proto::epp_proxy_server::EppProxy for EPPProxy {
    async fn domain_check(
        &self,
        request: tonic::Request<epp_proto::domain::DomainCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainCheckReply>, tonic::Status> {
        let res = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &res.name, res.registry_name)?;
        let res = client::domain::check(
            &res.name,
            res.fee_check.map(Into::into),
            None,
            &mut sender,
        ).await?;

        let reply = epp_proto::domain::DomainCheckReply {
            available: res.avail,
            reason: res.reason,
            fee_check: res.fee_check.map(Into::into),
            donuts_fee_check: res.donuts_fee_check.map(Into::into),
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_claims_check(
        &self,
        request: tonic::Request<epp_proto::domain::DomainClaimsCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainClaimsCheckReply>, tonic::Status> {
        let res = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &res.name, res.registry_name)?;
        let launch_check = match res.launch_check {
            Some(l) => l,
            None => return Err(tonic::Status::invalid_argument(
                "Launch check must be specified",
            ))
        };
        let res = client::domain::launch_claims_check(
            &res.name,
            launch_check.into(),
            &mut sender,
        ).await?;

        let reply = epp_proto::domain::DomainClaimsCheckReply {
            exists: res.exists,
            claims_keys: res.claims_key.into_iter().map(Into::into).collect(),
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_trademark_check(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTrademarkCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainClaimsCheckReply>, tonic::Status> {
        let res = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &res.name, res.registry_name)?;
        let res = client::domain::launch_trademark_check(
            &res.name,
            &mut sender,
        ).await?;

        let reply = epp_proto::domain::DomainClaimsCheckReply {
            exists: res.exists,
            claims_keys: res.claims_key.into_iter().map(Into::into).collect(),
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_info(
        &self,
        request: tonic::Request<epp_proto::domain::DomainInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainInfoReply>, tonic::Status> {
        let req = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &req.name, req.registry_name)?;
        let res = client::domain::info(
            &req.name,
            req.auth_info.as_deref(),
            match req.launch_info {
                Some(i) => Some(TryInto::try_into(i)?),
                None => None
            },
            &mut sender,
        ).await?;

        let mut reply: epp_proto::domain::DomainInfoReply = res.into();
        reply.registry_name = registry_name;

        Ok(tonic::Response::new(reply))
    }

    async fn domain_create(
        &self,
        request: tonic::Request<epp_proto::domain::DomainCreateRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainCreateReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;

        let mut ns = vec![];

        for n in &request.nameservers {
            match &n.server {
                Some(epp_proto::domain::name_server::Server::HostObj(h)) => {
                    ns.push(client::domain::InfoNameserver::HostOnly(h.clone()))
                }
                Some(epp_proto::domain::name_server::Server::HostName(h)) => {
                    ns.push(client::domain::InfoNameserver::HostAndAddress {
                        host: h.clone(),
                        addresses: n
                            .addresses
                            .iter()
                            .map(|addr| {
                                Ok(client::host::Address {
                                    address: addr.address.clone(),
                                    ip_version:
                                    match epp_proto::common::ip_address::IpVersion::from_i32(
                                        addr.r#type,
                                    ) {
                                        Some(
                                            epp_proto::common::ip_address::IpVersion::IPv4,
                                        ) => client::host::AddressVersion::IPv4,
                                        Some(
                                            epp_proto::common::ip_address::IpVersion::IPv6,
                                        ) => client::host::AddressVersion::IPv6,
                                        None
                                        | Some(
                                            epp_proto::common::ip_address::IpVersion::Unknown,
                                        ) => {
                                            return Err(tonic::Status::invalid_argument(
                                                "unknown IP address version",
                                            ));
                                        }
                                    },
                                })
                            })
                            .collect::<Result<Vec<client::host::Address>, tonic::Status>>()?,
                    })
                }
                None => {}
            }
        }

        let res = client::domain::create(
            client::domain::CreateInfo {
                domain: &request.name,
                period: request.period.map(|p| client::domain::Period {
                    unit: period_unit_from_i32(p.unit),
                    value: p.value,
                }),
                registrant: &request.registrant,
                contacts: request
                    .contacts
                    .into_iter()
                    .map(|c| client::domain::InfoContact {
                        contact_id: c.id,
                        contact_type: c.r#type,
                    })
                    .collect(),
                nameservers: ns,
                auth_info: &request.auth_info,
                sec_dns: match request.sec_dns {
                    Some(sec_dns) => match sec_dns.data {
                        Some(sec_dns_data) => Some(client::domain::SecDNSData {
                            max_sig_life: sec_dns.max_sig_life,
                            data: match sec_dns_data {
                                epp_proto::domain::sec_dns_data::Data::DsData(ds_data) => {
                                    client::domain::SecDNSDataType::DSData(
                                        ds_data
                                            .data
                                            .into_iter()
                                            .map(|d| client::domain::SecDNSDSData {
                                                key_tag: d.key_tag as u16,
                                                algorithm: d.algorithm as u8,
                                                digest_type: d.digest_type as u8,
                                                digest: d.digest,
                                                key_data: d.key_data.map(|k| {
                                                    client::domain::SecDNSKeyData {
                                                        flags: k.flags as u16,
                                                        protocol: k.protocol as u8,
                                                        algorithm: k.algorithm as u8,
                                                        public_key: k.public_key,
                                                    }
                                                }),
                                            })
                                            .collect(),
                                    )
                                }
                                epp_proto::domain::sec_dns_data::Data::KeyData(key_data) => {
                                    client::domain::SecDNSDataType::KeyData(
                                        key_data
                                            .data
                                            .into_iter()
                                            .map(|k| client::domain::SecDNSKeyData {
                                                flags: k.flags as u16,
                                                protocol: k.protocol as u8,
                                                algorithm: k.algorithm as u8,
                                                public_key: k.public_key,
                                            })
                                            .collect(),
                                    )
                                }
                            },
                        }),
                        None => None,
                    },
                    None => None,
                },
                launch_create: match request.launch_data {
                    Some(i) => Some(TryInto::try_into(i)?),
                    None => None
                },
                donuts_fee_agreement: request.donuts_fee_agreement.map(TryInto::try_into).map_or(Ok(None), |v| v.map(Some))?,
            },
            &mut sender,
        )
            .await?;

        let mut reply: epp_proto::domain::DomainCreateReply = res.into();
        reply.registry_name = registry_name;

        Ok(tonic::Response::new(reply))
    }

    async fn domain_delete(
        &self,
        request: tonic::Request<epp_proto::domain::DomainDeleteRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainDeleteReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;
        let res = client::domain::delete(
            &request.name,
            match request.launch_data {
                Some(i) => Some(TryInto::try_into(i)?),
                None => None
            },
            request.donuts_fee_agreement.map(TryInto::try_into).map_or(Ok(None), |v| v.map(Some))?,
            &mut sender,
        ).await?;

        let reply = epp_proto::domain::DomainDeleteReply {
            pending: res.pending,
            transaction_id: res.transaction_id,
            fee_data: res.fee_data.map(Into::into),
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_update(
        &self,
        request: tonic::Request<epp_proto::domain::DomainUpdateRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainUpdateReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;

        let mut add = vec![];
        let mut rem = vec![];

        let map_ns =
            |n: epp_proto::domain::NameServer| {
                Ok(match &n.server {
                    Some(epp_proto::domain::name_server::Server::HostObj(h)) => {
                        client::domain::InfoNameserver::HostOnly(h.clone())
                    }
                    Some(epp_proto::domain::name_server::Server::HostName(h)) => {
                        client::domain::InfoNameserver::HostAndAddress {
                            host: h.clone(),
                            addresses: n
                                .addresses
                                .iter()
                                .map(|addr| {
                                    Ok(client::host::Address {
                                        address: addr.address.clone(),
                                        ip_version: match epp_proto::common::ip_address::IpVersion::from_i32(
                                            addr.r#type,
                                        ) {
                                            Some(epp_proto::common::ip_address::IpVersion::IPv4) => {
                                                client::host::AddressVersion::IPv4
                                            }
                                            Some(epp_proto::common::ip_address::IpVersion::IPv6) => {
                                                client::host::AddressVersion::IPv6
                                            }
                                            None | Some(epp_proto::common::ip_address::IpVersion::Unknown) => {
                                                return Err(tonic::Status::invalid_argument(
                                                    "unknown IP address version",
                                                ));
                                            }
                                        },
                                    })
                                })
                                .collect::<Result<Vec<client::host::Address>, tonic::Status>>()?,
                        }
                    }
                    None => {
                        return Err(tonic::Status::invalid_argument(
                            "one of host_obj or host_name must be specified",
                        ));
                    }
                })
            };
        let map_param = |p: epp_proto::domain::domain_update_request::Param,
                         l: &mut Vec<client::domain::UpdateObject>|
                         -> Result<(), tonic::Status> {
            match p.param {
                Some(epp_proto::domain::domain_update_request::param::Param::Contact(c)) => {
                    l.push(client::domain::UpdateObject::Contact(
                        client::domain::InfoContact {
                            contact_id: c.id,
                            contact_type: c.r#type,
                        },
                    ));
                }
                Some(epp_proto::domain::domain_update_request::param::Param::Nameserver(n)) => {
                    l.push(client::domain::UpdateObject::Nameserver(map_ns(n)?));
                }
                Some(epp_proto::domain::domain_update_request::param::Param::State(s)) => {
                    if let Some(s) = domain_status_from_i32(s) {
                        l.push(client::domain::UpdateObject::Status(s));
                    }
                }
                None => {}
            }
            Ok(())
        };

        for p in request.add {
            map_param(p, &mut add)?;
        }
        for p in request.remove {
            map_param(p, &mut rem)?;
        }

        let res = client::domain::update(
            &request.name,
            add,
            rem,
            request.new_registrant.as_deref(),
            request.new_auth_info.as_deref(),
            match request.sec_dns {
                Some(sec_dns) => Some(client::domain::UpdateSecDNS {
                    urgent: sec_dns.urgent,
                    new_max_sig_life: sec_dns.new_max_sig_life,
                    add: sec_dns.add.map(|a| match a {
                        epp_proto::domain::update_sec_dns_data::Add::AddDsData(ds_data) => {
                            client::domain::SecDNSDataType::DSData(
                                ds_data
                                    .data
                                    .into_iter()
                                    .map(|d| client::domain::SecDNSDSData {
                                        key_tag: d.key_tag as u16,
                                        algorithm: d.algorithm as u8,
                                        digest_type: d.digest_type as u8,
                                        digest: d.digest,
                                        key_data: d.key_data.map(|k| {
                                            client::domain::SecDNSKeyData {
                                                flags: k.flags as u16,
                                                protocol: k.protocol as u8,
                                                algorithm: k.algorithm as u8,
                                                public_key: k.public_key,
                                            }
                                        }),
                                    })
                                    .collect(),
                            )
                        }
                        epp_proto::domain::update_sec_dns_data::Add::AddKeyData(key_data) => {
                            client::domain::SecDNSDataType::KeyData(
                                key_data
                                    .data
                                    .into_iter()
                                    .map(|k| client::domain::SecDNSKeyData {
                                        flags: k.flags as u16,
                                        protocol: k.protocol as u8,
                                        algorithm: k.algorithm as u8,
                                        public_key: k.public_key,
                                    })
                                    .collect(),
                            )
                        }
                    }),
                    remove: sec_dns.remove.map(|r| match r {
                        epp_proto::domain::update_sec_dns_data::Remove::RemoveAll(a) => {
                            client::domain::UpdateSecDNSRemove::All(a)
                        }
                        epp_proto::domain::update_sec_dns_data::Remove::RemoveDsData(ds_data) => {
                            client::domain::UpdateSecDNSRemove::Data(
                                client::domain::SecDNSDataType::DSData(
                                    ds_data
                                        .data
                                        .into_iter()
                                        .map(|d| client::domain::SecDNSDSData {
                                            key_tag: d.key_tag as u16,
                                            algorithm: d.algorithm as u8,
                                            digest_type: d.digest_type as u8,
                                            digest: d.digest,
                                            key_data: d.key_data.map(|k| {
                                                client::domain::SecDNSKeyData {
                                                    flags: k.flags as u16,
                                                    protocol: k.protocol as u8,
                                                    algorithm: k.algorithm as u8,
                                                    public_key: k.public_key,
                                                }
                                            }),
                                        })
                                        .collect(),
                                ),
                            )
                        }
                        epp_proto::domain::update_sec_dns_data::Remove::RemoveKeyData(key_data) => {
                            client::domain::UpdateSecDNSRemove::Data(
                                client::domain::SecDNSDataType::KeyData(
                                    key_data
                                        .data
                                        .into_iter()
                                        .map(|k| client::domain::SecDNSKeyData {
                                            flags: k.flags as u16,
                                            protocol: k.protocol as u8,
                                            algorithm: k.algorithm as u8,
                                            public_key: k.public_key,
                                        })
                                        .collect(),
                                ),
                            )
                        }
                    }),
                }),
                None => None,
            },
            match request.launch_data {
                Some(i) => Some(TryInto::try_into(i)?),
                None => None
            },
            request.donuts_fee_agreement.map(TryInto::try_into).map_or(Ok(None), |v| v.map(Some))?,
            &mut sender,
        )
            .await?;

        let reply = epp_proto::domain::DomainUpdateReply {
            pending: res.pending,
            transaction_id: res.transaction_id,
            fee_data: res.fee_data.map(Into::into),
            donuts_fee_data: res.donuts_fee_data.map(Into::into),
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_renew(
        &self,
        request: tonic::Request<epp_proto::domain::DomainRenewRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainRenewReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;

        let cur_expiry_date = proto_to_chrono(request.current_expiry_date);
        if cur_expiry_date.is_none() {
            return Err(tonic::Status::invalid_argument(
                "current_expiry_date must be specified",
            ));
        }

        let res = client::domain::renew(
            &request.name,
            request.period.map(|p| client::domain::Period {
                unit: period_unit_from_i32(p.unit),
                value: p.value,
            }),
            cur_expiry_date.unwrap(),
            request.donuts_fee_agreement.map(TryInto::try_into).map_or(Ok(None), |v| v.map(Some))?,
            &mut sender,
        )
            .await?;

        let mut reply: epp_proto::domain::DomainRenewReply = res.into();
        reply.registry_name = registry_name;

        Ok(tonic::Response::new(reply))
    }

    async fn domain_transfer_query(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTransferQueryRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainTransferReply>, tonic::Status> {
        let req = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &req.name, req.registry_name)?;
        let res = client::domain::transfer_query(
            &req.name, req.auth_info.as_deref(), &mut sender).await?;

        let mut reply: epp_proto::domain::DomainTransferReply = res.into();
        reply.registry_name = registry_name;

        Ok(tonic::Response::new(reply))
    }

    async fn domain_transfer_request(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTransferRequestRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;
        let res = client::domain::transfer_request(
            &request.name,
            request.period.map(|p| client::domain::Period {
                unit: period_unit_from_i32(p.unit),
                value: p.value,
            }),
            &request.auth_info,
            request.donuts_fee_agreement.map(TryInto::try_into).map_or(Ok(None), |v| v.map(Some))?,
            &mut sender,
        )
            .await?;

        let mut reply: epp_proto::domain::DomainTransferReply = res.into();
        reply.registry_name = registry_name;

        Ok(tonic::Response::new(reply))
    }

    async fn domain_transfer_accept(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTransferAcceptRejectRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;
        let res =
            client::domain::transfer_accept(
                &request.name, Some(&request.auth_info), &mut sender).await?;

        let mut reply: epp_proto::domain::DomainTransferReply = res.into();
        reply.registry_name = registry_name;

        Ok(tonic::Response::new(reply))
    }

    async fn domain_transfer_reject(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTransferAcceptRejectRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;
        let res =
            client::domain::transfer_reject(
                &request.name, Some(&request.auth_info), &mut sender).await?;

        let mut reply: epp_proto::domain::DomainTransferReply = res.into();
        reply.registry_name = registry_name;

        Ok(tonic::Response::new(reply))
    }

    async fn domain_restore_request(
        &self,
        request: tonic::Request<epp_proto::rgp::RequestRequest>,
    ) -> Result<tonic::Response<epp_proto::rgp::RestoreReply>, tonic::Status> {
        let res = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &res.name, res.registry_name)?;
        let res = client::rgp::request(
            &res.name,
            res.donuts_fee_agreement.map(TryInto::try_into).map_or(Ok(None), |v| v.map(Some))?,
            &mut sender,
        ).await?;

        let reply = epp_proto::rgp::RestoreReply {
            pending: res.pending,
            transaction_id: res.transaction_id,
            state: res.state.into_iter().map(i32_from_restore_status).collect(),
            fee_data: res.fee_data.map(Into::into),
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_check(
        &self,
        request: tonic::Request<epp_proto::host::HostCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostCheckReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let res = client::host::check(&name, &mut sender).await?;

        let reply = epp_proto::host::HostCheckReply {
            available: res.avail,
            reason: res.reason,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_info(
        &self,
        request: tonic::Request<epp_proto::host::HostInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostInfoReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let res = client::host::info(&name, &mut sender).await?;

        let reply = epp_proto::host::HostInfoReply {
            name: res.name,
            registry_id: res.registry_id,
            statuses: res
                .statuses
                .into_iter()
                .map(|s| match s {
                    client::host::Status::ClientDeleteProhibited => {
                        epp_proto::host::HostStatus::ClientDeleteProhibited.into()
                    }
                    client::host::Status::ClientUpdateProhibited => {
                        epp_proto::host::HostStatus::ClientUpdateProhibited.into()
                    }
                    client::host::Status::Linked => epp_proto::host::HostStatus::Linked.into(),
                    client::host::Status::Ok => epp_proto::host::HostStatus::Ok.into(),
                    client::host::Status::PendingCreate => {
                        epp_proto::host::HostStatus::PendingCreate.into()
                    }
                    client::host::Status::PendingDelete => {
                        epp_proto::host::HostStatus::PendingDelete.into()
                    }
                    client::host::Status::PendingTransfer => {
                        epp_proto::host::HostStatus::PendingTransfer.into()
                    }
                    client::host::Status::PendingUpdate => {
                        epp_proto::host::HostStatus::PendingUpdate.into()
                    }
                    client::host::Status::ServerDeleteProhibited => {
                        epp_proto::host::HostStatus::ServerDeleteProhibited.into()
                    }
                    client::host::Status::ServerUpdateProhibited => {
                        epp_proto::host::HostStatus::ServerUpdateProhibited.into()
                    }
                })
                .collect(),
            addresses: res
                .addresses
                .into_iter()
                .map(|a| epp_proto::common::IpAddress {
                    address: a.address,
                    r#type: match a.ip_version {
                        client::host::AddressVersion::IPv4 => {
                            epp_proto::common::ip_address::IpVersion::IPv4.into()
                        }
                        client::host::AddressVersion::IPv6 => {
                            epp_proto::common::ip_address::IpVersion::IPv6.into()
                        }
                    },
                })
                .collect(),
            client_id: res.client_id,
            client_created_id: res.client_created_id,
            creation_date: chrono_to_proto(res.creation_date),
            last_updated_client: res.last_updated_client,
            last_updated_date: chrono_to_proto(res.last_updated_date),
            last_transfer_date: chrono_to_proto(res.last_transfer_date),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_create(
        &self,
        request: tonic::Request<epp_proto::host::HostCreateRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostCreateReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let addresses: Vec<client::host::Address> = request
            .addresses
            .into_iter()
            .map(|a| {
                Ok(client::host::Address {
                    address: a.address,
                    ip_version: match epp_proto::common::ip_address::IpVersion::from_i32(a.r#type) {
                        Some(epp_proto::common::ip_address::IpVersion::IPv4) => {
                            client::host::AddressVersion::IPv4
                        }
                        Some(epp_proto::common::ip_address::IpVersion::IPv6) => {
                            client::host::AddressVersion::IPv6
                        }
                        None | Some(epp_proto::common::ip_address::IpVersion::Unknown) => {
                            return Err(tonic::Status::invalid_argument(
                                "unknown IP address version",
                            ));
                        }
                    },
                })
            })
            .collect::<Result<Vec<client::host::Address>, tonic::Status>>()?;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let res = client::host::create(&name, addresses, &mut sender).await?;

        let reply = epp_proto::host::HostCreateReply {
            name: res.name,
            pending: res.pending,
            transaction_id: res.transaction_id,
            creation_date: chrono_to_proto(res.creation_date),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_delete(
        &self,
        request: tonic::Request<epp_proto::host::HostDeleteRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostDeleteReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let res = client::host::delete(&name, &mut sender).await?;

        let reply = epp_proto::host::HostDeleteReply {
            pending: res.pending,
            transaction_id: res.transaction_id,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_update(
        &self,
        request: tonic::Request<epp_proto::host::HostUpdateRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostUpdateReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let mut add = vec![];
        let mut remove = vec![];

        let map_addr = |addr: epp_proto::common::IpAddress| {
            Ok(client::host::Address {
                address: addr.address,
                ip_version: match epp_proto::common::ip_address::IpVersion::from_i32(addr.r#type) {
                    Some(epp_proto::common::ip_address::IpVersion::IPv4) => {
                        client::host::AddressVersion::IPv4
                    }
                    Some(epp_proto::common::ip_address::IpVersion::IPv6) => {
                        client::host::AddressVersion::IPv6
                    }
                    None | Some(epp_proto::common::ip_address::IpVersion::Unknown) => {
                        return Err(tonic::Status::invalid_argument(
                            "unknown IP address version",
                        ));
                    }
                },
            })
        };

        for a in request.add {
            match a.param {
                Some(epp_proto::host::host_update_request::param::Param::Address(addr)) => {
                    add.push(client::host::UpdateObject::Address(map_addr(addr)?))
                }
                Some(epp_proto::host::host_update_request::param::Param::State(s)) => {
                    if let Some(s) = host_status_from_i32(s) {
                        add.push(client::host::UpdateObject::Status(s))
                    }
                }
                None => {}
            }
        }
        for r in request.remove {
            match r.param {
                Some(epp_proto::host::host_update_request::param::Param::Address(addr)) => {
                    remove.push(client::host::UpdateObject::Address(map_addr(addr)?))
                }
                Some(epp_proto::host::host_update_request::param::Param::State(s)) => {
                    if let Some(s) = host_status_from_i32(s) {
                        remove.push(client::host::UpdateObject::Status(s))
                    }
                }
                None => {}
            }
        }

        let res = client::host::update(&name, add, remove, request.new_name, &mut sender).await?;

        let reply = epp_proto::host::HostUpdateReply {
            pending: res.pending,
            transaction_id: res.transaction_id,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_check(
        &self,
        request: tonic::Request<epp_proto::contact::ContactCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactCheckReply>, tonic::Status> {
        let request = request.into_inner();
        let id: String = request.id;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let res = client::contact::check(&id, &mut sender).await?;

        let reply = epp_proto::contact::ContactCheckReply {
            available: res.avail,
            reason: res.reason,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_info(
        &self,
        request: tonic::Request<epp_proto::contact::ContactInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactInfoReply>, tonic::Status> {
        let request = request.into_inner();
        let id: String = request.id;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let res = client::contact::info(&id, &mut sender).await?;

        Ok(tonic::Response::new(res.into()))
    }

    async fn contact_create(
        &self,
        request: tonic::Request<epp_proto::contact::ContactCreateRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactCreateReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let addr_map = |a: epp_proto::contact::PostalAddress| client::contact::Address {
            name: a.name,
            organisation: a.organisation,
            streets: a.streets,
            city: a.city,
            province: a.province,
            postal_code: a.postal_code,
            country_code: a.country_code,
            identity_number: a.identity_number,
            birth_date: proto_to_chrono(a.birth_date).map(|d| d.date()),
        };

        let res = client::contact::create(
            &request.id,
            client::contact::NewContactData {
                local_address: request.local_address.map(addr_map),
                internationalised_address: request.internationalised_address.map(addr_map),
                phone: request.phone.map(|p| p.into()),
                fax: request.fax.map(|p| p.into()),
                email: request.email,
                entity_type: entity_type_from_i32(request.entity_type),
                trading_name: request.trading_name,
                company_number: request.company_number,
                disclosure: request
                    .disclosure
                    .map(|d| disclosure_type_from_i32(d.disclosure)),
                auth_info: request.auth_info,
            },
            &mut sender,
        )
            .await?;

        let reply = epp_proto::contact::ContactCreateReply {
            id: res.id,
            pending: res.pending,
            transaction_id: res.transaction_id,
            creation_date: chrono_to_proto(res.creation_date),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_delete(
        &self,
        request: tonic::Request<epp_proto::contact::ContactDeleteRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactDeleteReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let res = client::contact::delete(&request.id, &mut sender).await?;

        let reply = epp_proto::contact::ContactDeleteReply {
            pending: res.pending,
            transaction_id: res.transaction_id,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_update(
        &self,
        request: tonic::Request<epp_proto::contact::ContactUpdateRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactUpdateReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let addr_map = |a: epp_proto::contact::PostalAddress| client::contact::Address {
            name: a.name,
            organisation: a.organisation,
            streets: a.streets,
            city: a.city,
            province: a.province,
            postal_code: a.postal_code,
            country_code: a.country_code,
            identity_number: a.identity_number,
            birth_date: proto_to_chrono(a.birth_date).map(|d| d.date()),
        };

        let res = client::contact::update(
            &request.id,
            contact_status_from_i32(request.add_statuses),
            contact_status_from_i32(request.remove_statuses),
            client::contact::UpdateContactData {
                local_address: request.new_local_address.map(addr_map),
                internationalised_address: request.new_internationalised_address.map(addr_map),
                phone: request.new_phone.map(|p| p.into()),
                fax: request.new_fax.map(|p| p.into()),
                email: request.new_email,
                entity_type: entity_type_from_i32(request.new_entity_type),
                trading_name: request.new_trading_name,
                company_number: request.new_company_number,
                disclosure: request
                    .disclosure
                    .map(|d| disclosure_type_from_i32(d.disclosure)),
                auth_info: request.new_auth_info,
            },
            &mut sender,
        )
            .await?;

        let reply = epp_proto::contact::ContactUpdateReply {
            pending: res.pending,
            transaction_id: res.transaction_id,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_transfer_query(
        &self,
        request: tonic::Request<epp_proto::contact::ContactTransferQueryRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let res = client::contact::transfer_query(&request.id, &mut sender).await?;

        let reply = epp_proto::contact::ContactTransferReply {
            pending: res.pending,
            transaction_id: res.transaction_id,
            status: i32_from_transfer_status(res.data.status),
            requested_client_id: res.data.requested_client_id,
            requested_date: chrono_to_proto(Some(res.data.requested_date)),
            act_client_id: res.data.act_client_id,
            act_date: chrono_to_proto(Some(res.data.act_date)),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_transfer_request(
        &self,
        request: tonic::Request<epp_proto::contact::ContactTransferRequestRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let res =
            client::contact::transfer_request(&request.id, &request.auth_info, &mut sender).await?;

        let reply = epp_proto::contact::ContactTransferReply {
            pending: res.pending,
            transaction_id: res.transaction_id,
            status: i32_from_transfer_status(res.data.status),
            requested_client_id: res.data.requested_client_id,
            requested_date: chrono_to_proto(Some(res.data.requested_date)),
            act_client_id: res.data.act_client_id,
            act_date: chrono_to_proto(Some(res.data.act_date)),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_transfer_accept(
        &self,
        request: tonic::Request<epp_proto::contact::ContactTransferRequestRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let res =
            client::contact::transfer_accept(&request.id, &request.auth_info, &mut sender).await?;

        let reply = epp_proto::contact::ContactTransferReply {
            pending: res.pending,
            transaction_id: res.transaction_id,
            status: i32_from_transfer_status(res.data.status),
            requested_client_id: res.data.requested_client_id,
            requested_date: chrono_to_proto(Some(res.data.requested_date)),
            act_client_id: res.data.act_client_id,
            act_date: chrono_to_proto(Some(res.data.act_date)),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_transfer_reject(
        &self,
        request: tonic::Request<epp_proto::contact::ContactTransferRequestRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let res =
            client::contact::transfer_reject(&request.id, &request.auth_info, &mut sender).await?;

        let reply = epp_proto::contact::ContactTransferReply {
            pending: res.pending,
            transaction_id: res.transaction_id,
            status: i32_from_transfer_status(res.data.status),
            requested_client_id: res.data.requested_client_id,
            requested_date: chrono_to_proto(Some(res.data.requested_date)),
            act_client_id: res.data.act_client_id,
            act_date: chrono_to_proto(Some(res.data.act_date)),
        };

        Ok(tonic::Response::new(reply))
    }

    type PollStream = futures::channel::mpsc::Receiver<Result<epp_proto::PollReply, tonic::Status>>;

    async fn poll(
        &self,
        request: tonic::Request<epp_proto::RegistryInfo>,
    ) -> Result<tonic::Response<Self::PollStream>, tonic::Status> {
        let request = request.into_inner();
        let (mut tx, rx) = futures::channel::mpsc::channel(4);
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        tokio::spawn(async move {
            let mut should_delay = true;
            loop {
                match client::poll::poll(&mut sender).await {
                    Ok(resp) => {
                        if let Some(message) = resp {
                            if message.count > 0 {
                                should_delay = false;
                            }
                            let change_data = match message.data {
                                client::poll::PollData::DomainInfoData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::ContactInfoData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::DomainTransferData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::DomainCreateData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::DomainPanData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::DomainRenewData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::NominetDomainCancelData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::NominetDomainReleaseData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::NominetDomainRegistrarChangeData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::NominetHostCancelData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::NominetProcessData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::NominetSuspendData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::NominetDomainFailData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                client::poll::PollData::NominetRegistrantTransferData {
                                    change_data: ref c,
                                    data: _
                                } => c,
                                _ => &None
                            };
                            match tx
                                .send(Ok(epp_proto::PollReply {
                                    msg_id: message.id.clone(),
                                    enqueue_date: chrono_to_proto(Some(message.enqueue_time)),
                                    message: message.message,
                                    change_data: change_data.as_ref().map(|c| epp_proto::ChangeData {
                                        change_state: match c.state {
                                            client::poll::ChangeState::After => epp_proto::change_data::ChangeState::After.into(),
                                            client::poll::ChangeState::Before => epp_proto::change_data::ChangeState::Before.into(),
                                        },
                                        operation: Some(epp_proto::change_data::ChangeOperation {
                                            operation_type: match c.operation.op_type {
                                                client::poll::ChangeOperationType::Create => epp_proto::change_data::change_operation::ChangeOperationType::Create.into(),
                                                client::poll::ChangeOperationType::Delete => epp_proto::change_data::change_operation::ChangeOperationType::Delete.into(),
                                                client::poll::ChangeOperationType::Renew => epp_proto::change_data::change_operation::ChangeOperationType::Renew.into(),
                                                client::poll::ChangeOperationType::Transfer => epp_proto::change_data::change_operation::ChangeOperationType::Transfer.into(),
                                                client::poll::ChangeOperationType::Update => epp_proto::change_data::change_operation::ChangeOperationType::Update.into(),
                                                client::poll::ChangeOperationType::Restore => epp_proto::change_data::change_operation::ChangeOperationType::Restore.into(),
                                                client::poll::ChangeOperationType::AutoRenew => epp_proto::change_data::change_operation::ChangeOperationType::AutoRenew.into(),
                                                client::poll::ChangeOperationType::AutoDelete => epp_proto::change_data::change_operation::ChangeOperationType::AutoDelete.into(),
                                                client::poll::ChangeOperationType::AutoPurge => epp_proto::change_data::change_operation::ChangeOperationType::AutoPurge.into(),
                                                client::poll::ChangeOperationType::Custom => epp_proto::change_data::change_operation::ChangeOperationType::Custom.into(),
                                            },
                                            operation: c.operation.operation.clone(),
                                        }),
                                        date: chrono_to_proto(Some(c.date)),
                                        server_transaction_id: c.server_transaction_id.clone(),
                                        who: c.who.clone(),
                                        case_id: c.case_id.as_ref().map(|i| epp_proto::change_data::CaseId {
                                            case_id_type: match i.case_type {
                                                client::poll::ChangeCaseIdType::UDRP => epp_proto::change_data::case_id::CaseIdType::Udrp.into(),
                                                client::poll::ChangeCaseIdType::URS => epp_proto::change_data::case_id::CaseIdType::Urs.into(),
                                                client::poll::ChangeCaseIdType::Custom => epp_proto::change_data::case_id::CaseIdType::Custom.into(),
                                            },
                                            name: i.name.clone(),
                                            case_id: i.case_id.clone(),
                                        }),
                                        reason: c.reason.clone(),
                                    }),
                                    data: match message.data {
                                        client::poll::PollData::DomainInfoData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::DomainInfo((*i).into())),
                                        client::poll::PollData::ContactInfoData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::ContactInfo((*i).into())),
                                        client::poll::PollData::DomainTransferData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::DomainTransfer(i.into())),
                                        client::poll::PollData::DomainCreateData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::DomainCreate(i.into())),
                                        client::poll::PollData::DomainPanData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::DomainPan(i.into())),
                                        client::poll::PollData::NominetDomainCancelData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetDomainCancelData(i.into())),
                                        client::poll::PollData::NominetDomainReleaseData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetDomainReleaseData(i.into())),
                                        client::poll::PollData::NominetDomainRegistrarChangeData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetDomainRegistrarChangeData(i.into())),
                                        client::poll::PollData::NominetHostCancelData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetHostCancelData(i.into())),
                                        client::poll::PollData::NominetProcessData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetProcessData(i.into())),
                                        client::poll::PollData::NominetSuspendData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetSuspendData(i.into())),
                                        client::poll::PollData::NominetDomainFailData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetDomainFailData(i.into())),
                                        client::poll::PollData::NominetRegistrantTransferData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetRegistrantTransferData(i.into())),
                                        client::poll::PollData::VerisignLowBalanceData(i) =>
                                            Some(epp_proto::poll_reply::Data::VerisignLowBalanceData(i.into())),
                                        _ => None
                                    },
                                }))
                                .await
                            {
                                Ok(_) => {
                                    match client::poll::poll_ack(&message.id, &mut sender).await {
                                        Ok(_) => {}
                                        Err(err) => match tx.send(Err(err.into())).await {
                                            Ok(_) => {}
                                            Err(_) => break,
                                        },
                                    }
                                }
                                Err(_) => break,
                            }
                        } else if tx.is_closed() {
                            break;
                        }
                    }
                    Err(err) => match tx.send(Err(err.into())).await {
                        Ok(_) => {}
                        Err(_) => break,
                    },
                }
                if should_delay {
                    tokio::time::delay_for(tokio::time::Duration::new(15, 0)).await;
                }
            }
        });

        Ok(tonic::Response::new(rx))
    }

    async fn nominet_tag_list(
        &self,
        request: tonic::Request<epp_proto::RegistryInfo>,
    ) -> Result<tonic::Response<epp_proto::nominet::NominetTagListReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let res = client::nominet::tag_list(&mut sender).await?;

        let reply = epp_proto::nominet::NominetTagListReply {
            tags: res
                .tags
                .into_iter()
                .map(|t| epp_proto::nominet::nominet_tag_list_reply::Tag {
                    tag: t.tag,
                    name: t.name,
                    trading_name: t.trading_name,
                    handshake: t.handshake,
                })
                .collect(),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn balance_info(
        &self,
        request: tonic::Request<epp_proto::RegistryInfo>,
    ) -> Result<tonic::Response<epp_proto::BalanceReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let res = client::balance::balance_info(&mut sender).await?;

        Ok(tonic::Response::new(res.into()))
    }
}
