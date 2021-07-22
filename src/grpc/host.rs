use super::super::client;
use super::epp_proto;

pub fn host_status_from_i32(from: i32) -> Option<client::host::Status> {
    epp_proto::host::HostStatus::from_i32(from).map(|e| match e {
        epp_proto::host::HostStatus::ClientDeleteProhibited => {
            client::host::Status::ClientDeleteProhibited
        }
        epp_proto::host::HostStatus::ClientUpdateProhibited => {
            client::host::Status::ClientUpdateProhibited
        }
        epp_proto::host::HostStatus::Linked => client::host::Status::Linked,
        epp_proto::host::HostStatus::Ok => client::host::Status::Ok,
        epp_proto::host::HostStatus::PendingCreate => client::host::Status::PendingCreate,
        epp_proto::host::HostStatus::PendingDelete => client::host::Status::PendingDelete,
        epp_proto::host::HostStatus::PendingTransfer => client::host::Status::PendingTransfer,
        epp_proto::host::HostStatus::PendingUpdate => client::host::Status::PendingUpdate,
        epp_proto::host::HostStatus::ServerDeleteProhibited => {
            client::host::Status::ServerDeleteProhibited
        }
        epp_proto::host::HostStatus::ServerUpdateProhibited => {
            client::host::Status::ServerUpdateProhibited
        }
    })
}

impl From<client::host::InfoResponse> for epp_proto::host::HostInfoReply {
    fn from(res: client::host::InfoResponse) -> Self {
        epp_proto::host::HostInfoReply {
            name: res.name,
            registry_id: res.registry_id,
            statuses: res
                .statuses
                .into_iter()
                .map(|s| match s {
                    client::host::Status::ClientDeleteProhibited => {
                        epp_proto::host::HostStatus::ClientDeleteProhibited.into()
                    }
                    client::host::Status::ClientUpdateProhibited => {
                        epp_proto::host::HostStatus::ClientUpdateProhibited.into()
                    }
                    client::host::Status::Linked => epp_proto::host::HostStatus::Linked.into(),
                    client::host::Status::Ok => epp_proto::host::HostStatus::Ok.into(),
                    client::host::Status::PendingCreate => {
                        epp_proto::host::HostStatus::PendingCreate.into()
                    }
                    client::host::Status::PendingDelete => {
                        epp_proto::host::HostStatus::PendingDelete.into()
                    }
                    client::host::Status::PendingTransfer => {
                        epp_proto::host::HostStatus::PendingTransfer.into()
                    }
                    client::host::Status::PendingUpdate => {
                        epp_proto::host::HostStatus::PendingUpdate.into()
                    }
                    client::host::Status::ServerDeleteProhibited => {
                        epp_proto::host::HostStatus::ServerDeleteProhibited.into()
                    }
                    client::host::Status::ServerUpdateProhibited => {
                        epp_proto::host::HostStatus::ServerUpdateProhibited.into()
                    }
                })
                .collect(),
            addresses: res
                .addresses
                .into_iter()
                .map(|a| epp_proto::common::IpAddress {
                    address: a.address,
                    r#type: match a.ip_version {
                        client::host::AddressVersion::IPv4 => {
                            epp_proto::common::ip_address::IpVersion::IPv4.into()
                        }
                        client::host::AddressVersion::IPv6 => {
                            epp_proto::common::ip_address::IpVersion::IPv6.into()
                        }
                    },
                })
                .collect(),
            client_id: res.client_id,
            client_created_id: res.client_created_id,
            creation_date: super::utils::chrono_to_proto(res.creation_date),
            last_updated_client: res.last_updated_client,
            last_updated_date: super::utils::chrono_to_proto(res.last_updated_date),
            last_transfer_date: super::utils::chrono_to_proto(res.last_transfer_date),
            cmd_resp: None
        }
    }
}