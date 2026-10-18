use crate::client;

pub fn from_poll_response_to_grpc(
    message: client::poll::PollResponse, cmd_resp: super::epp_proto::common::CommandResponse
) -> super::epp_proto::PollReply {
    let change_data = match message.data {
        client::poll::PollData::DomainInfoData {
            change_data: ref c,
            data: _,
        } => c,
        client::poll::PollData::ContactInfoData {
            change_data: ref c,
            data: _,
        } => c,
        client::poll::PollData::DomainTransferData {
            change_data: ref c,
            data: _,
            keysys_data: _
        } => c,
        client::poll::PollData::DomainCreateData {
            change_data: ref c,
            data: _,
        } => c,
        client::poll::PollData::DomainPanData {
            change_data: ref c,
            data: _,
            keysys_data: _,
        } => c,
        client::poll::PollData::DomainRenewData {
            change_data: ref c,
            data: _,
            keysys_data: _,
        } => c,
        client::poll::PollData::NominetDomainCancelData {
            change_data: ref c,
            data: _,
        } => c,
        client::poll::PollData::NominetDomainReleaseData {
            change_data: ref c,
            data: _,
        } => c,
        client::poll::PollData::NominetDomainRegistrarChangeData {
            change_data: ref c,
            data: _,
        } => c,
        client::poll::PollData::NominetHostCancelData {
            change_data: ref c,
            data: _,
        } => c,
        client::poll::PollData::NominetProcessData {
            change_data: ref c,
            data: _,
        } => c,
        client::poll::PollData::NominetSuspendData {
            change_data: ref c,
            data: _,
        } => c,
        client::poll::PollData::NominetDomainFailData {
            change_data: ref c,
            data: _,
        } => c,
        client::poll::PollData::NominetRegistrantTransferData {
            change_data: ref c,
            data: _,
        } => c,
        _ => &None,
    };
    let keysys_data = match message.data {
        client::poll::PollData::DomainTransferData {
            change_data: _,
            data: _,
            keysys_data: ref c,
        } => c,
        client::poll::PollData::DomainPanData {
            change_data: _,
            data: _,
            keysys_data: ref c,
        } => c,
        client::poll::PollData::DomainRenewData {
            change_data: _,
            data: _,
            keysys_data: ref c,
        } => c,
        _ => &None,
    };
    super::epp_proto::PollReply {
        msg_id: message.id.clone(),
        count: message.count,
        enqueue_date: super::utils::chrono_to_proto(Some(message.enqueue_time)),
        message: message.message,
        cmd_resp: Some(cmd_resp),
        change_data: change_data.as_ref().map(|c| super::epp_proto::ChangeData {
            change_state: match c.state {
                client::poll::ChangeState::After => super::epp_proto::change_data::ChangeState::After.into(),
                client::poll::ChangeState::Before => super::epp_proto::change_data::ChangeState::Before.into(),
            },
            operation: Some(super::epp_proto::change_data::ChangeOperation {
                operation_type: match c.operation.op_type {
                    client::poll::ChangeOperationType::Create => super::epp_proto::change_data::change_operation::ChangeOperationType::Create.into(),
                    client::poll::ChangeOperationType::Delete => super::epp_proto::change_data::change_operation::ChangeOperationType::Delete.into(),
                    client::poll::ChangeOperationType::Renew => super::epp_proto::change_data::change_operation::ChangeOperationType::Renew.into(),
                    client::poll::ChangeOperationType::Transfer => super::epp_proto::change_data::change_operation::ChangeOperationType::Transfer.into(),
                    client::poll::ChangeOperationType::Update => super::epp_proto::change_data::change_operation::ChangeOperationType::Update.into(),
                    client::poll::ChangeOperationType::Restore => super::epp_proto::change_data::change_operation::ChangeOperationType::Restore.into(),
                    client::poll::ChangeOperationType::AutoRenew => super::epp_proto::change_data::change_operation::ChangeOperationType::AutoRenew.into(),
                    client::poll::ChangeOperationType::AutoDelete => super::epp_proto::change_data::change_operation::ChangeOperationType::AutoDelete.into(),
                    client::poll::ChangeOperationType::AutoPurge => super::epp_proto::change_data::change_operation::ChangeOperationType::AutoPurge.into(),
                    client::poll::ChangeOperationType::Custom => super::epp_proto::change_data::change_operation::ChangeOperationType::Custom.into(),
                },
                operation: c.operation.operation.clone(),
            }),
            date: super::utils::chrono_to_proto(Some(c.date)),
            server_transaction_id: c.server_transaction_id.clone(),
            who: c.who.clone(),
            case_id: c.case_id.as_ref().map(|i| super::epp_proto::change_data::CaseId {
                case_id_type: match i.case_type {
                    client::poll::ChangeCaseIdType::Udrp => super::epp_proto::change_data::case_id::CaseIdType::Udrp.into(),
                    client::poll::ChangeCaseIdType::Urs => super::epp_proto::change_data::case_id::CaseIdType::Urs.into(),
                    client::poll::ChangeCaseIdType::Custom => super::epp_proto::change_data::case_id::CaseIdType::Custom.into(),
                },
                name: i.name.clone(),
                case_id: i.case_id.clone(),
            }),
            reason: c.reason.clone(),
        }),
        keysys: keysys_data.as_ref().map(|d| super::epp_proto::KeysysPoll {
            info: d.info.clone(),
            data: d.data.clone()
        }),
        data: match message.data {
            client::poll::PollData::DomainInfoData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::DomainInfo((*i).into())),
            client::poll::PollData::ContactInfoData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::ContactInfo((*i).into())),
            client::poll::PollData::HostInfoData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::HostInfo((*i).into())),
            client::poll::PollData::DomainTransferData {
                change_data: _,
                keysys_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::DomainTransfer(i.into())),
            client::poll::PollData::DomainRenewData {
                change_data: _,
                keysys_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::DomainRenew(i.into())),
            client::poll::PollData::ContactTransferData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::ContactTransfer(i.into())),
            client::poll::PollData::DomainCreateData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::DomainCreate(i.into())),
            client::poll::PollData::DomainPanData {
                change_data: _,
                keysys_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::DomainPan(i.into())),
            client::poll::PollData::ContactPanData {
                change_data: _,
                keysys_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::ContactPan(i.into())),
            client::poll::PollData::NominetDomainCancelData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::NominetDomainCancel(i.into())),
            client::poll::PollData::NominetDomainReleaseData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::NominetDomainRelease(i.into())),
            client::poll::PollData::NominetDomainRegistrarChangeData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::NominetDomainRegistrarChange(i.into())),
            client::poll::PollData::NominetHostCancelData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::NominetHostCancel(i.into())),
            client::poll::PollData::NominetProcessData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::NominetProcess(i.into())),
            client::poll::PollData::NominetSuspendData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::NominetSuspend(i.into())),
            client::poll::PollData::NominetDomainFailData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::NominetDomainFail(i.into())),
            client::poll::PollData::NominetRegistrantTransferData {
                change_data: _,
                data: i
            } => Some(super::epp_proto::poll_reply::Data::NominetRegistrantTransfer(i.into())),
            client::poll::PollData::VerisignLowBalanceData(i) =>
                Some(super::epp_proto::poll_reply::Data::VerisignLowBalance(i.into())),
            client::poll::PollData::TraficomTrnData(i) =>
                Some(super::epp_proto::poll_reply::Data::TraficomTrn(i.into())),
            client::poll::PollData::EURIDPoll(i) =>
                Some(super::epp_proto::poll_reply::Data::EuridPoll(i.into())),
            client::poll::PollData::MaintenanceData(i) =>
                Some(super::epp_proto::poll_reply::Data::MaintenanceInfo(i.into())),
            client::poll::PollData::None => None
        },
    }
}