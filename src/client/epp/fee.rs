use super::super::fee::{
    Applied, Command, Credit, DonutsAmount, DonutsCategory, DonutsFeeData, DonutsFeeSet,
    DonutsFeeType, DonutsFeeTypes, Fee, FeeAgreement, FeeData,
};
use super::super::proto;
use super::ServerFeatures;

impl From<&proto::fee::EPPFeeCommand> for Command {
    fn from(from: &proto::fee::EPPFeeCommand) -> Self {
        match from {
            proto::fee::EPPFeeCommand::Create => Command::Create,
            proto::fee::EPPFeeCommand::Renew => Command::Renew,
            proto::fee::EPPFeeCommand::Transfer => Command::Transfer,
            proto::fee::EPPFeeCommand::Delete => Command::Delete,
            proto::fee::EPPFeeCommand::Restore => Command::Restore,
        }
    }
}

impl From<&proto::united_tld::EPPChargeCommand> for Command {
    fn from(from: &proto::united_tld::EPPChargeCommand) -> Self {
        match from {
            proto::united_tld::EPPChargeCommand::Check => Command::Check,
            proto::united_tld::EPPChargeCommand::Create => Command::Create,
            proto::united_tld::EPPChargeCommand::Delete => Command::Delete,
            proto::united_tld::EPPChargeCommand::Info => Command::Info,
            proto::united_tld::EPPChargeCommand::Renew => Command::Renew,
            proto::united_tld::EPPChargeCommand::Transfer => Command::Transfer,
            proto::united_tld::EPPChargeCommand::Update => Command::Update,
            proto::united_tld::EPPChargeCommand::Custom => Command::Custom,
        }
    }
}

impl From<&Command> for Option<proto::fee::EPPFeeCommand> {
    fn from(from: &Command) -> Self {
        match from {
            Command::Create => Some(proto::fee::EPPFeeCommand::Create),
            Command::Renew => Some(proto::fee::EPPFeeCommand::Renew),
            Command::Transfer => Some(proto::fee::EPPFeeCommand::Transfer),
            Command::Delete => Some(proto::fee::EPPFeeCommand::Delete),
            Command::Restore => Some(proto::fee::EPPFeeCommand::Restore),
            _ => None,
        }
    }
}

impl From<&Command> for proto::united_tld::EPPChargeCommand {
    fn from(from: &Command) -> Self {
        match from {
            Command::Create => proto::united_tld::EPPChargeCommand::Create,
            Command::Renew => proto::united_tld::EPPChargeCommand::Renew,
            Command::Transfer => proto::united_tld::EPPChargeCommand::Transfer,
            Command::Delete => proto::united_tld::EPPChargeCommand::Delete,
            Command::Restore => proto::united_tld::EPPChargeCommand::Update,
            Command::Check => proto::united_tld::EPPChargeCommand::Check,
            Command::Info => proto::united_tld::EPPChargeCommand::Info,
            Command::Update => proto::united_tld::EPPChargeCommand::Update,
            Command::Custom => proto::united_tld::EPPChargeCommand::Custom,
        }
    }
}

impl From<Option<&proto::fee::EPPFee10Applied>> for Applied {
    fn from(from: Option<&proto::fee::EPPFee10Applied>) -> Self {
        match from {
            None => Applied::Immediate,
            Some(proto::fee::EPPFee10Applied::Immediate) => Applied::Immediate,
            Some(proto::fee::EPPFee10Applied::Delayed) => Applied::Delayed,
        }
    }
}

impl From<&Applied> for Option<proto::fee::EPPFee10Applied> {
    fn from(from: &Applied) -> Self {
        match from {
            Applied::Immediate => Some(proto::fee::EPPFee10Applied::Immediate),
            Applied::Delayed => Some(proto::fee::EPPFee10Applied::Delayed),
            Applied::Unspecified => None,
        }
    }
}

impl From<&proto::fee::EPPFee05Fee> for Fee {
    fn from(from: &proto::fee::EPPFee05Fee) -> Self {
        Fee {
            value: from.value.to_owned(),
            description: from.description.to_owned(),
            refundable: Some(from.refundable),
            grace_period: from.grace_period.to_owned(),
            applied: Applied::Immediate,
        }
    }
}

impl From<&proto::fee::EPPFee08Fee> for Fee {
    fn from(from: &proto::fee::EPPFee08Fee) -> Self {
        Fee {
            value: from.value.to_owned(),
            description: from.description.to_owned(),
            refundable: Some(from.refundable),
            grace_period: from.grace_period.to_owned(),
            applied: (Some(&from.applied)).into(),
        }
    }
}

impl From<&proto::fee::EPPFee011Fee> for Fee {
    fn from(from: &proto::fee::EPPFee011Fee) -> Self {
        Fee {
            value: from.value.to_owned(),
            description: from.description.to_owned(),
            refundable: from.refundable,
            grace_period: from.grace_period.to_owned(),
            applied: from.applied.as_ref().into(),
        }
    }
}

impl From<&Fee> for proto::fee::EPPFee011Fee {
    fn from(from: &Fee) -> Self {
        proto::fee::EPPFee011Fee {
            value: from.value.to_owned(),
            description: from.description.to_owned(),
            refundable: from.refundable,
            grace_period: from.grace_period.to_owned(),
            applied: (&from.applied).into(),
        }
    }
}

impl From<&proto::fee::EPPFee10Fee> for Fee {
    fn from(from: &proto::fee::EPPFee10Fee) -> Self {
        Fee {
            value: from.value.to_owned(),
            description: from.description.to_owned(),
            refundable: from.refundable,
            grace_period: from.grace_period.to_owned(),
            applied: from.applied.as_ref().into(),
        }
    }
}

impl From<&Fee> for proto::fee::EPPFee10Fee {
    fn from(from: &Fee) -> Self {
        proto::fee::EPPFee10Fee {
            value: from.value.to_owned(),
            description: from.description.to_owned(),
            refundable: from.refundable,
            grace_period: from.grace_period.to_owned(),
            applied: (&from.applied).into(),
        }
    }
}

impl From<&proto::fee::EPPFee011Credit> for Credit {
    fn from(from: &proto::fee::EPPFee011Credit) -> Self {
        Credit {
            value: from.value.to_owned(),
            description: from.description.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee10Credit> for Credit {
    fn from(from: &proto::fee::EPPFee10Credit) -> Self {
        Credit {
            value: from.value.to_owned(),
            description: from.description.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee05TransformData> for FeeData {
    fn from(f: &proto::fee::EPPFee05TransformData) -> Self {
        FeeData {
            period: None,
            currency: f.currency.to_owned(),
            fees: f.fee.iter().map(Into::into).collect(),
            credits: vec![],
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee06TransformData> for FeeData {
    fn from(f: &proto::fee::EPPFee06TransformData) -> Self {
        FeeData {
            period: None,
            currency: f.currency.to_owned(),
            fees: f.fee.iter().map(Into::into).collect(),
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee07TransformData> for FeeData {
    fn from(f: &proto::fee::EPPFee07TransformData) -> Self {
        FeeData {
            period: None,
            currency: f.currency.to_owned(),
            fees: f.fee.iter().map(Into::into).collect(),
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee08TransformData> for FeeData {
    fn from(f: &proto::fee::EPPFee08TransformData) -> Self {
        FeeData {
            period: None,
            currency: f.currency.to_owned(),
            fees: f.fee.iter().map(Into::into).collect(),
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee09TransformData> for FeeData {
    fn from(f: &proto::fee::EPPFee09TransformData) -> Self {
        FeeData {
            period: None,
            currency: f.currency.to_owned(),
            fees: f.fee.iter().map(Into::into).collect(),
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee011TransformData> for FeeData {
    fn from(f: &proto::fee::EPPFee011TransformData) -> Self {
        FeeData {
            period: None,
            currency: f.currency.to_owned(),
            fees: f.fee.iter().map(Into::into).collect(),
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee10TransformData> for FeeData {
    fn from(f: &proto::fee::EPPFee10TransformData) -> Self {
        FeeData {
            period: None,
            currency: f.currency.to_owned(),
            fees: f.fee.iter().map(Into::into).collect(),
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee06TransferData> for FeeData {
    fn from(f: &proto::fee::EPPFee06TransferData) -> Self {
        FeeData {
            period: f.period.as_ref().map(Into::into),
            currency: f.currency.to_owned(),
            fees: f.fee.iter().map(Into::into).collect(),
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee07TransferData> for FeeData {
    fn from(f: &proto::fee::EPPFee07TransferData) -> Self {
        FeeData {
            period: f.period.as_ref().map(Into::into),
            currency: f.currency.to_owned(),
            fees: f.fee.iter().map(Into::into).collect(),
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee08TransferData> for FeeData {
    fn from(f: &proto::fee::EPPFee08TransferData) -> Self {
        FeeData {
            period: f.period.as_ref().map(Into::into),
            currency: f.currency.to_owned(),
            fees: f.fee.iter().map(Into::into).collect(),
            credits: f.credit.iter().map(Into::into).collect(),
            balance: None,
            credit_limit: None,
        }
    }
}

impl From<&proto::fee::EPPFee09TransferData> for FeeData {
    fn from(f: &proto::fee::EPPFee09TransferData) -> Self {
        FeeData {
            period: f.period.as_ref().map(Into::into),
            currency: f.currency.to_owned(),
            fees: f.fee.iter().map(Into::into).collect(),
            credits: f.credit.iter().map(Into::into).collect(),
            balance: None,
            credit_limit: None,
        }
    }
}

impl From<&proto::fee::EPPFee05DeleteData> for FeeData {
    fn from(f: &proto::fee::EPPFee05DeleteData) -> Self {
        FeeData {
            period: None,
            currency: f.currency.to_owned(),
            fees: vec![],
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee06DeleteData> for FeeData {
    fn from(f: &proto::fee::EPPFee06DeleteData) -> Self {
        FeeData {
            period: None,
            currency: f.currency.to_owned(),
            fees: vec![],
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}


impl From<&proto::fee::EPPFee07DeleteData> for FeeData {
    fn from(f: &proto::fee::EPPFee07DeleteData) -> Self {
        FeeData {
            period: None,
            currency: f.currency.to_owned(),
            fees: vec![],
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee08DeleteData> for FeeData {
    fn from(f: &proto::fee::EPPFee08DeleteData) -> Self {
        FeeData {
            period: None,
            currency: f.currency.to_owned(),
            fees: vec![],
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::fee::EPPFee09DeleteData> for FeeData {
    fn from(f: &proto::fee::EPPFee09DeleteData) -> Self {
        FeeData {
            period: None,
            currency: f.currency.to_owned(),
            fees: vec![],
            credits: f.credit.iter().map(Into::into).collect(),
            balance: f.balance.to_owned(),
            credit_limit: f.credit_limit.to_owned(),
        }
    }
}

impl From<&proto::united_tld::EPPChargeData> for DonutsFeeData {
    fn from(f: &proto::united_tld::EPPChargeData) -> Self {
        DonutsFeeData {
            sets: f.sets.iter().map(Into::into).collect(),
        }
    }
}

impl From<&proto::united_tld::EPPChargeSet> for DonutsFeeSet {
    fn from(f: &proto::united_tld::EPPChargeSet) -> Self {
        DonutsFeeSet {
            fees: f
                .amount
                .iter()
                .map(|f| DonutsAmount {
                    value: f.value.to_string(),
                    command: (&f.command).into(),
                    command_name: f.name.as_ref().map(Into::into),
                })
                .collect(),
            fee_type: DonutsFeeType {
                fee_type: match f.set_type.value {
                    proto::united_tld::EPPChargeTypes::Fee => DonutsFeeTypes::Fee,
                    proto::united_tld::EPPChargeTypes::Price => DonutsFeeTypes::Price,
                    proto::united_tld::EPPChargeTypes::Custom => DonutsFeeTypes::Custom,
                },
                name: f.set_type.name.as_ref().map(Into::into),
            },
            category: DonutsCategory {
                category: f.category.value.to_string(),
                name: f.category.name.as_ref().map(Into::into),
            },
        }
    }
}

impl From<&proto::united_tld::EPPChargeCheckDatum> for DonutsFeeData {
    fn from(f: &proto::united_tld::EPPChargeCheckDatum) -> Self {
        DonutsFeeData {
            sets: f.sets.iter().map(Into::into).collect(),
        }
    }
}

impl From<&DonutsFeeData> for proto::united_tld::EPPChargeData {
    fn from(f: &DonutsFeeData) -> Self {
        proto::united_tld::EPPChargeData {
            sets: f
                .sets
                .iter()
                .map(|s| proto::united_tld::EPPChargeSet {
                    category: proto::united_tld::EPPChargeCategory {
                        value: s.category.category.to_string(),
                        name: s.category.name.as_ref().map(Into::into),
                    },
                    set_type: proto::united_tld::EPPChargeType {
                        value: match s.fee_type.fee_type {
                            DonutsFeeTypes::Fee => proto::united_tld::EPPChargeTypes::Fee,
                            DonutsFeeTypes::Price => proto::united_tld::EPPChargeTypes::Price,
                            DonutsFeeTypes::Custom => proto::united_tld::EPPChargeTypes::Custom,
                        },
                        name: s.fee_type.name.as_ref().map(Into::into),
                    },
                    amount: s
                        .fees
                        .iter()
                        .map(|f| proto::united_tld::EPPChargeAmount {
                            value: f.value.to_string(),
                            name: f.command_name.as_ref().map(Into::into),
                            command: (&f.command).into(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

impl From<&FeeAgreement> for proto::fee::EPPFee011Agreement {
    fn from(f: &FeeAgreement) -> Self {
        proto::fee::EPPFee011Agreement {
            currency: f.currency.as_ref().map(Into::into),
            fee: f.fees.iter().map(Into::into).collect(),
        }
    }
}

impl From<&FeeAgreement> for proto::fee::EPPFee10Agreement {
    fn from(f: &FeeAgreement) -> Self {
        proto::fee::EPPFee10Agreement {
            currency: f.currency.as_ref().map(Into::into),
            fee: f.fees.iter().map(Into::into).collect(),
        }
    }
}

pub fn handle_donuts_fee_agreement<T>(
    client: &ServerFeatures,
    agreement: &Option<DonutsFeeData>,
    exts: &mut Vec<super::proto::EPPCommandExtensionType>,
) -> Result<(), super::super::router::Response<T>> {
    if let Some(agreement) = agreement {
        if client.unitedtld_charge {
            exts.push(
                super::super::proto::EPPCommandExtensionType::EPPDonutsChargeAgreement(
                    agreement.into(),
                ),
            );
            Ok(())
        } else {
            Err(Err(super::super::Error::Unsupported))
        }
    } else {
        Ok(())
    }
}
