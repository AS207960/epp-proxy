use super::proto;

#[derive(Debug)]
pub struct FeeCheck {
    pub currency: Option<String>,
    pub commands: Vec<FeeCheckCommand>
}

#[derive(Debug)]
pub enum Command {
    Create,
    Renew,
    Transfer,
    Delete,
    Restore,
}

#[derive(Debug)]
pub enum Applied {
    Immediate,
    Delayed,
}

#[derive(Debug)]
pub struct FeeCheckCommand {
    pub command: Command,
    pub period: Option<super::domain::Period>
}

#[derive(Debug)]
pub struct FeeCheckData {
    pub available: bool,
    pub commands: Vec<FeeCommand>,
    pub reason: Option<String>,
}

#[derive(Debug)]
pub struct FeeCommand {
    pub command: Command,
    pub period: Option<super::domain::Period>,
    pub standard: Option<bool>,
    pub currency: String,
    pub fees: Vec<Fee>,
    pub credits: Vec<Credit>,
    pub reason: Option<String>,
    pub class: Option<String>,
}

#[derive(Debug)]
pub struct Fee {
    pub value: String,
    pub description: Option<String>,
    pub refundable: Option<bool>,
    pub grace_period: Option<String>,
    pub applied: Applied
}

#[derive(Debug)]
pub struct Credit {
    pub value: String,
    pub description: Option<String>
}

#[derive(Debug)]
pub struct FeeData {
    pub currency: String,
    pub period: Option<super::domain::Period>,
    pub fees: Vec<Fee>,
    pub credits: Vec<Credit>,
    pub balance: Option<String>,
    pub credit_limit: Option<String>,
}


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

impl From<&Command> for proto::fee::EPPFeeCommand {
    fn from(from: &Command) -> Self {
        match from {
            Command::Create => proto::fee::EPPFeeCommand::Create,
            Command::Renew => proto::fee::EPPFeeCommand::Renew,
            Command::Transfer => proto::fee::EPPFeeCommand::Transfer,
            Command::Delete => proto::fee::EPPFeeCommand::Delete,
            Command::Restore => proto::fee::EPPFeeCommand::Restore,
        }
    }
}

impl From<&proto::fee::EPPFee10Applied> for Applied {
    fn from(from: &proto::fee::EPPFee10Applied) -> Self {
        match from {
            proto::fee::EPPFee10Applied::Immediate => Applied::Immediate,
            proto::fee::EPPFee10Applied::Delayed => Applied::Delayed,
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
            applied: (&from.applied).into(),
        }
    }
}

impl From<&proto::fee::EPPFee10Credit> for Credit {
    fn from(from: &proto::fee::EPPFee10Credit) -> Self {
        Credit {
            value: from.value.to_owned(),
            description: from.description.to_owned()
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
            credit_limit: f.credit_limit.to_owned()
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
            credit_limit: f.credit_limit.to_owned()
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
            credit_limit: f.credit_limit.to_owned()
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
            credit_limit: f.credit_limit.to_owned()
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
            credit_limit: f.credit_limit.to_owned()
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
            credit_limit: None
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
            credit_limit: f.credit_limit.to_owned()
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
            credit_limit: f.credit_limit.to_owned()
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
            credit_limit: f.credit_limit.to_owned()
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
            credit_limit: f.credit_limit.to_owned()
        }
    }
}