use super::super::client;
use super::epp_proto;

pub fn i32_from_restore_status(from: client::rgp::RGPState) -> i32 {
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