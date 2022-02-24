#[derive(Debug)]
pub struct FeeCheck {
    pub currency: Option<String>,
    pub commands: Vec<FeeCheckCommand>,
}

#[derive(Debug)]
pub struct FeeAgreement {
    pub currency: Option<String>,
    pub fees: Vec<Fee>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Command {
    Create,
    Renew,
    Transfer,
    Delete,
    Restore,
    Update,
    Check,
    Info,
    Custom,
}

#[derive(Debug)]
pub enum Applied {
    Immediate,
    Delayed,
    Unspecified,
}

#[derive(Debug)]
pub struct FeeCheckCommand {
    pub command: Command,
    pub period: Option<super::Period>,
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
    pub period: Option<super::Period>,
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
    pub applied: Applied,
}

#[derive(Debug)]
pub struct Credit {
    pub value: String,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct FeeData {
    pub currency: String,
    pub period: Option<super::Period>,
    pub fees: Vec<Fee>,
    pub credits: Vec<Credit>,
    pub balance: Option<String>,
    pub credit_limit: Option<String>,
}

#[derive(Debug)]
pub struct DonutsFeeData {
    pub sets: Vec<DonutsFeeSet>,
}

#[derive(Debug)]
pub struct DonutsFeeSet {
    pub fees: Vec<DonutsAmount>,
    pub fee_type: DonutsFeeType,
    pub category: DonutsCategory,
}

#[derive(Debug, Clone)]
pub struct DonutsCategory {
    pub category: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DonutsFeeType {
    pub fee_type: DonutsFeeTypes,
    pub name: Option<String>,
}

#[derive(Debug, Copy, Clone)]
pub enum DonutsFeeTypes {
    Fee,
    Price,
    Custom,
}

#[derive(Debug)]
pub struct DonutsAmount {
    pub value: String,
    pub command: Command,
    pub command_name: Option<String>,
}
