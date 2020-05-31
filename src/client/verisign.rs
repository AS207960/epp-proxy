use std::convert::TryFrom;

#[derive(Debug)]
pub struct LowBalanceData {
    pub registrar_name: String,
    pub credit_limit: String,
    pub credit_threshold: CreditThreshold,
    pub available_credit: String,
}

#[derive(PartialEq, Debug)]
pub enum CreditThreshold {
    Fixed(String),
    Percentage(u8)
}

impl TryFrom<super::proto::verisign::EPPLowBalanceData> for LowBalanceData {
    type Error = super::Error;

    fn try_from(from: super::proto::verisign::EPPLowBalanceData) -> Result<Self, Self::Error> {
        Ok(LowBalanceData {
            registrar_name: from.registrar_name,
            credit_limit: from.credit_limit,
            available_credit: from.available_credit,
            credit_threshold: match from.credit_threshold.credit_type {
                super::proto::verisign::EPPLowCreditThresholdType::Percentage => CreditThreshold::Percentage(match from.credit_threshold.threshold.parse::<u8>() {
                    Ok(v) => v,
                    Err(_) => return Err(super::Error::InternalServerError)
                }),
                super::proto::verisign::EPPLowCreditThresholdType::Fixed => CreditThreshold::Fixed(from.credit_threshold.threshold)
            }
        })
    }
}