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
    Percentage(u8),
}

impl TryFrom<super::proto::verisign::EPPLowBalanceData> for LowBalanceData {
    type Error = super::Error;

    fn try_from(from: super::proto::verisign::EPPLowBalanceData) -> Result<Self, Self::Error> {
        Ok(LowBalanceData {
            registrar_name: from.registrar_name,
            credit_limit: from.credit_limit,
            available_credit: from.available_credit,
            credit_threshold: match from.credit_threshold.credit_type {
                super::proto::verisign::EPPLowCreditThresholdType::Percentage => {
                    CreditThreshold::Percentage(
                        match from.credit_threshold.threshold.parse::<u8>() {
                            Ok(v) => v,
                            Err(_) => return Err(super::Error::InternalServerError),
                        },
                    )
                }
                super::proto::verisign::EPPLowCreditThresholdType::Fixed => {
                    CreditThreshold::Fixed(from.credit_threshold.threshold)
                }
            },
        })
    }
}

pub fn handle_verisign_namestore_erratum(client: &super::EPPClientServerFeatures, exts: &mut Vec<super::proto::EPPCommandExtensionType>) {
    if client.has_erratum("verisign-tv") {
        exts.push(super::proto::EPPCommandExtensionType::VerisignNameStoreExt(
            super::proto::verisign::EPPNameStoreExt {
                sub_product: "dotTV".to_string(),
            },
        ));
    } else if client.has_erratum("verisign-cc") {
        exts.push(super::proto::EPPCommandExtensionType::VerisignNameStoreExt(
            super::proto::verisign::EPPNameStoreExt {
                sub_product: "dotCC".to_string(),
            },
        ));
    } else if client.has_erratum("verisign-com") {
        exts.push(super::proto::EPPCommandExtensionType::VerisignNameStoreExt(
            super::proto::verisign::EPPNameStoreExt {
                sub_product: "dotCOM".to_string(),
            },
        ));
    } else if client.has_erratum("verisign-net") {
        exts.push(super::proto::EPPCommandExtensionType::VerisignNameStoreExt(
            super::proto::verisign::EPPNameStoreExt {
                sub_product: "dotNET".to_string(),
            },
        ));
    } else if client.has_erratum("verisign-name") {
        exts.push(super::proto::EPPCommandExtensionType::VerisignNameStoreExt(
            super::proto::verisign::EPPNameStoreExt {
                sub_product: "dotNAME".to_string(),
            },
        ));
    }
}