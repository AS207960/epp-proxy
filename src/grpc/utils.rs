use super::super::client;
use super::epp_proto;

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
            cmd_resp: None,
        }
    }
}

/// Helper function to convert chrono times to protobuf well-known type times
pub fn chrono_to_proto<T: chrono::TimeZone>(
    time: Option<chrono::DateTime<T>>,
) -> Option<prost_types::Timestamp> {
    time.map(|t| prost_types::Timestamp {
        seconds: t.timestamp(),
        nanos: t.timestamp_subsec_nanos() as i32,
    })
}

pub fn proto_to_chrono(time: Option<prost_types::Timestamp>) -> Option<chrono::DateTime<chrono::Utc>> {
    use chrono::offset::TimeZone;
    match time {
        Some(t) => chrono::Utc
            .timestamp_opt(t.seconds, t.nanos as u32)
            .single(),
        None => None,
    }
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
            client::Error::ServerInternal => tonic::Status::internal("internal server error"),
        }
    }
}

pub fn i32_from_transfer_status(from: client::TransferStatus) -> i32 {
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
            cmd_resp: None,
        }
    }
}

pub fn map_command_response<T>(
    from: client::CommandResponse<T>,
) -> (T, epp_proto::common::CommandResponse) {
    (
        from.response,
        epp_proto::common::CommandResponse {
            extra_values: from
                .extra_values
                .into_iter()
                .map(|e| epp_proto::common::CommandExtraValue {
                    reason: e.reason,
                    value: e.value,
                })
                .collect(),
            transaction_id: from
                .transaction_id
                .map(|t| epp_proto::common::CommandTransactionId {
                    client: t.client,
                    server: t.server,
                }),
        },
    )
}


pub fn period_unit_from_i32(from: i32) -> client::PeriodUnit {
    match epp_proto::common::period::Unit::from_i32(from) {
        Some(e) => match e {
            epp_proto::common::period::Unit::Months => client::PeriodUnit::Months,
            epp_proto::common::period::Unit::Years => client::PeriodUnit::Years,
        },
        None => client::PeriodUnit::Years,
    }
}

pub fn i32_from_period_unit(from: client::PeriodUnit) -> i32 {
    match from {
        client::PeriodUnit::Months => epp_proto::common::period::Unit::Months.into(),
        client::PeriodUnit::Years => epp_proto::common::period::Unit::Years.into(),
    }
}

impl From<epp_proto::common::Period> for client::Period {
    fn from(from: epp_proto::common::Period) -> Self {
        client::Period {
            unit: period_unit_from_i32(from.unit),
            value: from.value,
        }
    }
}


impl From<epp_proto::common::Phone> for client::Phone {
    fn from(from: epp_proto::common::Phone) -> Self {
        client::Phone {
            number: from.number,
            extension: from.extension,
        }
    }
}

impl From<client::Phone> for epp_proto::common::Phone {
    fn from(from: client::Phone) -> Self {
        epp_proto::common::Phone {
            number: from.number,
            extension: from.extension,
        }
    }
}

