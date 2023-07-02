use super::{client, epp_proto};
use chrono::prelude::*;

pub fn env_from_i32(from: i32) -> Option<client::dac::DACEnv> {
    epp_proto::dac::Environment::from_i32(from).map(|e| match e {
        epp_proto::dac::Environment::RealTime => client::dac::DACEnv::RealTime,
        epp_proto::dac::Environment::TimeDelay => client::dac::DACEnv::TimeDelay,
    })
}

impl From<client::dac::DACUsageResponse> for epp_proto::dac::UsageResponse {
    fn from(res: client::dac::DACUsageResponse) -> Self {
        epp_proto::dac::UsageResponse {
            usage_60: res.usage_60,
            usage_24: res.usage_24,
        }
    }
}

impl From<client::dac::DACDomainResponse> for epp_proto::dac::DomainResponse {
    fn from(res: client::dac::DACDomainResponse) -> Self {
        epp_proto::dac::DomainResponse {
            registration_state: match res.registration_state {
                client::dac::DomainState::Registered => {
                    epp_proto::dac::DomainState::Registered.into()
                }
                client::dac::DomainState::Available => {
                    epp_proto::dac::DomainState::Available.into()
                }
                client::dac::DomainState::NotWithinRegistry => {
                    epp_proto::dac::DomainState::NotWithinRegistry.into()
                }
                client::dac::DomainState::RulesPrevent => {
                    epp_proto::dac::DomainState::RulesPrevent.into()
                }
            },
            detagged: res.detagged,
            created: super::utils::chrono_to_proto(Some(
                Utc.from_utc_datetime(&res.created.and_hms_opt(0, 0, 0).unwrap()),
            )),
            expiry: super::utils::chrono_to_proto(Some(
                Utc.from_utc_datetime(&res.expiry.and_hms_opt(0, 0, 0).unwrap()),
            )),
            status: match res.status {
                client::dac::DomainStatus::Unknown => epp_proto::dac::DomainStatus::Unknown.into(),
                client::dac::DomainStatus::RegisteredUntilExpiry => {
                    epp_proto::dac::DomainStatus::RegisteredUntilExpiry.into()
                }
                client::dac::DomainStatus::RenewalRequired => {
                    epp_proto::dac::DomainStatus::RenewalRequired.into()
                }
                client::dac::DomainStatus::NoLongerRequired => {
                    epp_proto::dac::DomainStatus::NoLongerRequired.into()
                }
            },
            suspended: res.suspended,
            tag: res.tag,
        }
    }
}
