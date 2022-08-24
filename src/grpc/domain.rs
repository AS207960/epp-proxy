use super::super::client;
use super::epp_proto;

pub fn domain_status_from_i32(from: i32) -> Option<client::domain::Status> {
    epp_proto::domain_common::DomainStatus::from_i32(from).map(|e| match e {
        epp_proto::domain_common::DomainStatus::ClientDeleteProhibited => {
            client::domain::Status::ClientDeleteProhibited
        }
        epp_proto::domain_common::DomainStatus::ClientHold => client::domain::Status::ClientHold,
        epp_proto::domain_common::DomainStatus::ClientRenewProhibited => {
            client::domain::Status::ClientRenewProhibited
        }
        epp_proto::domain_common::DomainStatus::ClientTransferProhibited => {
            client::domain::Status::ClientTransferProhibited
        }
        epp_proto::domain_common::DomainStatus::ClientUpdateProhibited => {
            client::domain::Status::ClientUpdateProhibited
        }
        epp_proto::domain_common::DomainStatus::Inactive => client::domain::Status::Inactive,
        epp_proto::domain_common::DomainStatus::Ok => client::domain::Status::Ok,
        epp_proto::domain_common::DomainStatus::PendingCreate => client::domain::Status::PendingCreate,
        epp_proto::domain_common::DomainStatus::PendingDelete => client::domain::Status::PendingDelete,
        epp_proto::domain_common::DomainStatus::PendingRenew => client::domain::Status::PendingRenew,
        epp_proto::domain_common::DomainStatus::PendingTransfer => client::domain::Status::PendingTransfer,
        epp_proto::domain_common::DomainStatus::PendingUpdate => client::domain::Status::PendingUpdate,
        epp_proto::domain_common::DomainStatus::ServerDeleteProhibited => {
            client::domain::Status::ServerDeleteProhibited
        }
        epp_proto::domain_common::DomainStatus::ServerHold => client::domain::Status::ServerHold,
        epp_proto::domain_common::DomainStatus::ServerRenewProhibited => {
            client::domain::Status::ServerRenewProhibited
        }
        epp_proto::domain_common::DomainStatus::ServerTransferProhibited => {
            client::domain::Status::ServerTransferProhibited
        }
        epp_proto::domain_common::DomainStatus::ServerUpdateProhibited => {
            client::domain::Status::ServerUpdateProhibited
        }
    })
}

fn i32_from_domain_status(from: client::domain::Status) -> i32 {
    match from {
        client::domain::Status::ClientDeleteProhibited => {
            epp_proto::domain_common::DomainStatus::ClientDeleteProhibited.into()
        }
        client::domain::Status::ClientHold => epp_proto::domain_common::DomainStatus::ClientHold.into(),
        client::domain::Status::ClientRenewProhibited => {
            epp_proto::domain_common::DomainStatus::ClientRenewProhibited.into()
        }
        client::domain::Status::ClientTransferProhibited => {
            epp_proto::domain_common::DomainStatus::ClientTransferProhibited.into()
        }
        client::domain::Status::ClientUpdateProhibited => {
            epp_proto::domain_common::DomainStatus::ClientUpdateProhibited.into()
        }
        client::domain::Status::Inactive => epp_proto::domain_common::DomainStatus::Inactive.into(),
        client::domain::Status::Ok => epp_proto::domain_common::DomainStatus::Ok.into(),
        client::domain::Status::PendingCreate => {
            epp_proto::domain_common::DomainStatus::PendingCreate.into()
        }
        client::domain::Status::PendingDelete => {
            epp_proto::domain_common::DomainStatus::PendingDelete.into()
        }
        client::domain::Status::PendingRenew => {
            epp_proto::domain_common::DomainStatus::PendingRenew.into()
        }
        client::domain::Status::PendingTransfer => {
            epp_proto::domain_common::DomainStatus::PendingTransfer.into()
        }
        client::domain::Status::PendingUpdate => {
            epp_proto::domain_common::DomainStatus::PendingUpdate.into()
        }
        client::domain::Status::ServerDeleteProhibited => {
            epp_proto::domain_common::DomainStatus::ServerDeleteProhibited.into()
        }
        client::domain::Status::ServerHold => epp_proto::domain_common::DomainStatus::ServerHold.into(),
        client::domain::Status::ServerRenewProhibited => {
            epp_proto::domain_common::DomainStatus::ServerRenewProhibited.into()
        }
        client::domain::Status::ServerTransferProhibited => {
            epp_proto::domain_common::DomainStatus::ServerTransferProhibited.into()
        }
        client::domain::Status::ServerUpdateProhibited => {
            epp_proto::domain_common::DomainStatus::ServerUpdateProhibited.into()
        }
    }
}

impl From<client::domain::CheckResponse> for epp_proto::domain::DomainCheckReply {
    fn from(res: client::domain::CheckResponse) -> Self {
        epp_proto::domain::DomainCheckReply {
            available: res.avail,
            reason: res.reason,
            fee_check: res.fee_check.map(Into::into),
            donuts_fee_check: res.donuts_fee_check.map(Into::into),
            registry_name: String::new(),
            cmd_resp: None,
            eurid_idn: res.eurid_idn.map(Into::into),
            eurid_data: res.eurid_check.map(|c| epp_proto::eurid::DomainCheckData {
                available_date: super::utils::chrono_to_proto(c.available_date),
                status: c.status.into_iter().map(i32_from_domain_status).collect(),
            }),
        }
    }
}

impl From<client::domain::InfoResponse> for epp_proto::domain::DomainInfoReply {
    fn from(res: client::domain::InfoResponse) -> Self {
        epp_proto::domain::DomainInfoReply {
            name: res.name,
            registry_id: res.registry_id,
            statuses: res
                .statuses
                .into_iter()
                .map(i32_from_domain_status)
                .collect(),
            registrant: res.registrant,
            contacts: res
                .contacts
                .into_iter()
                .map(|c| epp_proto::domain::Contact {
                    id: c.contact_id,
                    r#type: c.contact_type,
                })
                .collect(),
            nameservers: res
                .nameservers
                .into_iter()
                .map(|n| match n {
                    client::domain::InfoNameserver::HostOnly(h) => epp_proto::domain::NameServer {
                        server: Some(epp_proto::domain::name_server::Server::HostObj(h)),
                        addresses: vec![],
                        eurid_idn: None,
                    },
                    client::domain::InfoNameserver::HostAndAddress {
                        host,
                        addresses,
                        eurid_idn,
                    } => epp_proto::domain::NameServer {
                        server: Some(epp_proto::domain::name_server::Server::HostName(host)),
                        addresses: addresses
                            .iter()
                            .map(|addr| epp_proto::common::IpAddress {
                                address: addr.address.clone(),
                                r#type: match addr.ip_version {
                                    client::host::AddressVersion::IPv4 => {
                                        epp_proto::common::ip_address::IpVersion::IPv4.into()
                                    }
                                    client::host::AddressVersion::IPv6 => {
                                        epp_proto::common::ip_address::IpVersion::IPv6.into()
                                    }
                                },
                            })
                            .collect(),
                        eurid_idn: eurid_idn.map(Into::into),
                    },
                })
                .collect(),
            hosts: res.hosts,
            client_id: res.client_id,
            client_created_id: res.client_created_id,
            creation_date: super::utils::chrono_to_proto(res.creation_date),
            expiry_date: super::utils::chrono_to_proto(res.expiry_date),
            last_updated_client: res.last_updated_client,
            last_updated_date: super::utils::chrono_to_proto(res.last_updated_date),
            last_transfer_date: super::utils::chrono_to_proto(res.last_transfer_date),
            registry_name: String::new(),
            rgp_state: res
                .rgp_state
                .into_iter()
                .map(super::rgp::i32_from_restore_status)
                .collect(),
            auth_info: res.auth_info,
            sec_dns: res.sec_dns.map(|sec_dns| epp_proto::domain::SecDnsData {
                max_sig_life: sec_dns.max_sig_life,
                data: Some(match sec_dns.data {
                    client::domain::SecDNSDataType::DSData(ds_data) => {
                        epp_proto::domain::sec_dns_data::Data::DsData(
                            epp_proto::domain::SecDnsdsData {
                                data: ds_data
                                    .into_iter()
                                    .map(|d| epp_proto::domain::SecDnsdsDatum {
                                        key_tag: d.key_tag as u32,
                                        algorithm: d.algorithm as u32,
                                        digest_type: d.digest_type as u32,
                                        digest: d.digest,
                                        key_data: d.key_data.map(|k| {
                                            epp_proto::domain::SecDnsKeyDatum {
                                                flags: k.flags as u32,
                                                protocol: k.protocol as u32,
                                                algorithm: k.algorithm as u32,
                                                public_key: k.public_key,
                                            }
                                        }),
                                    })
                                    .collect(),
                            },
                        )
                    }
                    client::domain::SecDNSDataType::KeyData(key_data) => {
                        epp_proto::domain::sec_dns_data::Data::KeyData(
                            epp_proto::domain::SecDnsKeyData {
                                data: key_data
                                    .into_iter()
                                    .map(|k| epp_proto::domain::SecDnsKeyDatum {
                                        flags: k.flags as u32,
                                        protocol: k.protocol as u32,
                                        algorithm: k.algorithm as u32,
                                        public_key: k.public_key,
                                    })
                                    .collect(),
                            },
                        )
                    }
                }),
            }),
            launch_info: res.launch_info.map(Into::into),
            donuts_fee_data: res.donuts_fee_data.map(Into::into),
            verisign_whois_info: res
                .whois_info
                .map(|i| epp_proto::domain::VerisignWhoisInfo {
                    registrar: i.registrar,
                    whois_server: i.whois_server,
                    url: i.url,
                    iris_server: i.iris_server,
                }),
            cmd_resp: None,
            eurid_idn: res.eurid_idn.map(Into::into),
            eurid_data: res.eurid_data.map(|d| epp_proto::eurid::DomainInfo {
                on_hold: d.on_hold,
                quarantined: d.quarantined,
                suspended: d.suspended,
                delayed: d.delayed,
                seized: d.seized,
                deletion_date: super::utils::chrono_to_proto(d.deletion_date),
                on_site: d.on_site,
                reseller: d.reseller,
                max_extension_period: d.max_extension_period,
                registrant_country: d.registrant_country,
                registrant_country_of_citizenship: d.registrant_country_of_citizenship,
                auth_info_valid_until: super::utils::chrono_to_proto(d.auth_info_valid_until),
            }),
            isnic_info: res.isnic_info.map(|d| epp_proto::isnic::DomainInfo {
                zone_contact: d.zone_contact,
            }),
            personal_registration: res.personal_registration.map(|p| {
                epp_proto::personal_registration::PersonalRegistrationInfo {
                    consent_id: p.consent_id,
                }
            }),
            keysys: res.keysys.map(Into::into),
        }
    }
}

impl From<client::domain::CreateResponse> for epp_proto::domain::DomainCreateReply {
    fn from(res: client::domain::CreateResponse) -> Self {
        epp_proto::domain::DomainCreateReply {
            name: res.data.name,
            pending: res.pending,
            creation_date: super::utils::chrono_to_proto(res.data.creation_date),
            expiry_date: super::utils::chrono_to_proto(res.data.expiration_date),
            fee_data: res.fee_data.map(Into::into),
            donuts_fee_data: res.donuts_fee_data.map(Into::into),
            registry_name: String::new(),
            launch_data: res.launch_create.map(Into::into),
            cmd_resp: None,
            eurid_idn: res.data.eurid_idn.map(Into::into),
            personal_registration: res.data.personal_registration.map(|p| {
                epp_proto::personal_registration::PersonalRegistrationCreate {
                    bundled_rate: p.bundled_rate,
                }
            }),
        }
    }
}

impl From<client::domain::RenewResponse> for epp_proto::domain::DomainRenewReply {
    fn from(res: client::domain::RenewResponse) -> Self {
        epp_proto::domain::DomainRenewReply {
            name: res.data.name,
            pending: res.pending,
            expiry_date: super::utils::chrono_to_proto(res.data.new_expiry_date),
            fee_data: res.fee_data.map(Into::into),
            donuts_fee_data: res.donuts_fee_data.map(Into::into),
            registry_name: String::new(),
            cmd_resp: None,
            eurid_idn: res.data.eurid_idn.map(Into::into),
            eurid_data: res
                .data
                .eurid_data
                .map(|d| epp_proto::eurid::DomainRenewInfo {
                    removed_deletion: d.removed_deletion,
                }),
            personal_registration: res.data.personal_registration.map(|p| {
                epp_proto::personal_registration::PersonalRegistrationCreate {
                    bundled_rate: p.bundled_rate,
                }
            }),
        }
    }
}

impl From<client::domain::TransferResponse> for epp_proto::domain::DomainTransferReply {
    fn from(res: client::domain::TransferResponse) -> Self {
        epp_proto::domain::DomainTransferReply {
            pending: res.pending,
            name: res.data.name,
            status: super::utils::i32_from_transfer_status(res.data.status),
            requested_client_id: res.data.requested_client_id,
            requested_date: super::utils::chrono_to_proto(Some(res.data.requested_date)),
            act_client_id: res.data.act_client_id,
            act_date: super::utils::chrono_to_proto(Some(res.data.act_date)),
            expiry_date: super::utils::chrono_to_proto(res.data.expiry_date),
            fee_data: res.fee_data.map(Into::into),
            donuts_fee_data: res.donuts_fee_data.map(Into::into),
            registry_name: String::new(),
            cmd_resp: None,
            eurid_idn: res.data.eurid_idn.map(Into::into),
            eurid_data: res
                .data
                .eurid_data
                .map(|d| epp_proto::eurid::DomainTransferInfo {
                    on_hold: d.on_hold,
                    quarantined: d.quarantined,
                    reason: d.reason,
                    delayed: d.delayed,
                    registrant: d.registrant,
                    billing: d.billing,
                    technical: d.technical,
                    on_site: d.on_site,
                    reseller: d.reseller,
                }),
            personal_registration: res.data.personal_registration.map(|p| {
                epp_proto::personal_registration::PersonalRegistrationCreate {
                    bundled_rate: p.bundled_rate,
                }
            }),
        }
    }
}

impl From<client::domain::PanData> for epp_proto::domain::DomainPanReply {
    fn from(res: client::domain::PanData) -> Self {
        epp_proto::domain::DomainPanReply {
            name: res.name,
            result: res.result,
            server_transaction_id: res.server_transaction_id,
            client_transaction_id: res.client_transaction_id,
            date: super::utils::chrono_to_proto(Some(res.date)),
        }
    }
}

impl From<client::domain::UpdateResponse> for epp_proto::domain::DomainUpdateReply {
    fn from(res: client::domain::UpdateResponse) -> Self {
        epp_proto::domain::DomainUpdateReply {
            pending: res.pending,
            fee_data: res.fee_data.map(Into::into),
            donuts_fee_data: res.donuts_fee_data.map(Into::into),
            registry_name: String::new(),
            cmd_resp: None,
        }
    }
}
