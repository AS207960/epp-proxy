use super::super::client;
use super::epp_proto;
use std::convert::TryFrom;

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
            commands: from
                .commands
                .into_iter()
                .map(|c| client::fee::FeeCheckCommand {
                    command: fee_command_from_i32(c.command),
                    period: c.period.map(|p| client::Period {
                        unit: super::utils::period_unit_from_i32(p.unit),
                        value: p.value,
                    }),
                })
                .collect(),
        }
    }
}

impl From<client::fee::FeeCheckData> for epp_proto::fee::FeeCheckData {
    fn from(from: client::fee::FeeCheckData) -> Self {
        epp_proto::fee::FeeCheckData {
            available: from.available,
            commands: from
                .commands
                .into_iter()
                .map(|c| epp_proto::fee::fee_check_data::FeeCommand {
                    command: i32_from_fee_command(c.command),
                    standard: c.standard,
                    period: c.period.map(|p| epp_proto::common::Period {
                        unit: super::utils::i32_from_period_unit(p.unit),
                        value: p.value,
                    }),
                    currency: c.currency,
                    fees: c.fees.into_iter().map(Into::into).collect(),
                    credits: c.credits.into_iter().map(Into::into).collect(),
                    class: c.class,
                    reason: c.reason,
                })
                .collect(),
            reason: from.reason,
        }
    }
}

impl From<client::fee::FeeData> for epp_proto::fee::FeeData {
    fn from(from: client::fee::FeeData) -> Self {
        epp_proto::fee::FeeData {
            period: from.period.map(|p| epp_proto::common::Period {
                unit: super::utils::i32_from_period_unit(p.unit),
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
                client::fee::Applied::Unspecified => epp_proto::fee::Applied::Unspecified.into(),
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
            fees: from
                .sets
                .into_iter()
                .map(|f| epp_proto::fee::DonutsFeeSet {
                    category: Some(epp_proto::fee::DonutsCategory {
                        name: f.category.name,
                        value: f.category.category,
                    }),
                    fee_type: Some(epp_proto::fee::DonutsFeeType {
                        fee_type: match f.fee_type.fee_type {
                            client::fee::DonutsFeeTypes::Fee => {
                                epp_proto::fee::donuts_fee_type::FeeTypes::Fee.into()
                            }
                            client::fee::DonutsFeeTypes::Price => {
                                epp_proto::fee::donuts_fee_type::FeeTypes::Price.into()
                            }
                            client::fee::DonutsFeeTypes::Custom => {
                                epp_proto::fee::donuts_fee_type::FeeTypes::Custom.into()
                            }
                        },
                        name: f.fee_type.name,
                    }),
                    fees: f
                        .fees
                        .into_iter()
                        .map(|a| epp_proto::fee::DonutsAmount {
                            command: i32_from_fee_command(a.command),
                            name: a.command_name,
                            value: a.value,
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

impl TryFrom<epp_proto::fee::DonutsFeeData> for client::fee::DonutsFeeData {
    type Error = tonic::Status;

    fn try_from(from: epp_proto::fee::DonutsFeeData) -> Result<Self, Self::Error> {
        Ok(client::fee::DonutsFeeData {
            sets: from
                .fees
                .into_iter()
                .map(|f| {
                    Ok(client::fee::DonutsFeeSet {
                        category: match f.category {
                            Some(c) => client::fee::DonutsCategory {
                                name: c.name,
                                category: c.value,
                            },
                            None => {
                                return Err(tonic::Status::invalid_argument(
                                    "Category must be specified",
                                ))
                            }
                        },
                        fee_type: match f.fee_type {
                            Some(f) => client::fee::DonutsFeeType {
                                fee_type: match epp_proto::fee::donuts_fee_type::FeeTypes::from_i32(
                                    f.fee_type,
                                ) {
                                    Some(epp_proto::fee::donuts_fee_type::FeeTypes::Fee) => {
                                        client::fee::DonutsFeeTypes::Fee
                                    }
                                    Some(epp_proto::fee::donuts_fee_type::FeeTypes::Price) => {
                                        client::fee::DonutsFeeTypes::Price
                                    }
                                    Some(epp_proto::fee::donuts_fee_type::FeeTypes::Custom) => {
                                        client::fee::DonutsFeeTypes::Custom
                                    }
                                    None => {
                                        return Err(tonic::Status::invalid_argument(
                                            "Unknown fee type",
                                        ))
                                    }
                                },
                                name: f.name,
                            },
                            None => {
                                return Err(tonic::Status::invalid_argument(
                                    "Fee type must be specified",
                                ))
                            }
                        },
                        fees: f
                            .fees
                            .into_iter()
                            .map(|a| client::fee::DonutsAmount {
                                command: fee_command_from_i32(a.command),
                                command_name: a.name,
                                value: a.value,
                            })
                            .collect(),
                    })
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl From<epp_proto::fee::FeeAgreement> for client::fee::FeeAgreement {
    fn from(from: epp_proto::fee::FeeAgreement) -> Self {
        client::fee::FeeAgreement {
            currency: from.currency,
            fees: from
                .fees
                .into_iter()
                .map(|f| client::fee::Fee {
                    value: f.value,
                    description: f.description,
                    refundable: f.refundable,
                    grace_period: f.grace_period,
                    applied: match epp_proto::fee::Applied::from_i32(f.applied) {
                        Some(e) => match e {
                            epp_proto::fee::Applied::Immediate => client::fee::Applied::Immediate,
                            epp_proto::fee::Applied::Delayed => client::fee::Applied::Delayed,
                            epp_proto::fee::Applied::Unspecified => {
                                client::fee::Applied::Unspecified
                            }
                        },
                        None => client::fee::Applied::Unspecified,
                    },
                })
                .collect(),
        }
    }
}
