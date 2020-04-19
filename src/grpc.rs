//! Implements the gRPC interface for the EPP client

use super::client;
use futures::sink::SinkExt;

pub mod epp_proto {
    tonic::include_proto!("epp");
    pub mod common {
        tonic::include_proto!("epp.common");
    }
    pub mod domain {
        tonic::include_proto!("epp.domain");
    }
    pub mod host {
        tonic::include_proto!("epp.host");
    }
    pub mod contact {
        tonic::include_proto!("epp.contact");
    }
    pub mod rgp {
        tonic::include_proto!("epp.rgp");
    }
}

/// Helper function to convert chrono times to protobuf well-known type times
fn chrono_to_proto<T: chrono::TimeZone>(
    time: Option<chrono::DateTime<T>>,
) -> Option<prost_types::Timestamp> {
    time.map(|t| prost_types::Timestamp {
        seconds: t.timestamp(),
        nanos: t.timestamp_subsec_nanos() as i32,
    })
}

fn proto_to_chrono(
    time: Option<prost_types::Timestamp>,
) -> Option<chrono::DateTime<chrono::Utc>> {
    use chrono::offset::TimeZone;
    match time {
        Some(t) => chrono::Utc.timestamp_opt(t.seconds, t.nanos as u32).single(),
        None => None
    }
}

#[derive(Debug)]
pub struct EPPProxy {
    pub client_router: super::Router,
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
            client::Error::InternalServerError => tonic::Status::internal("internal server error"),
        }
    }
}

fn entity_type_from_i32(from: i32) -> Option<client::contact::EntityType> {
    match epp_proto::contact::EntityType::from_i32(from) {
        Some(e) => match e {
            epp_proto::contact::EntityType::UkLimitedCompany => Some(client::contact::EntityType::UkLimitedCompany),
            epp_proto::contact::EntityType::UkPublicLimitedCompany => Some(client::contact::EntityType::UkPublicLimitedCompany),
            epp_proto::contact::EntityType::UkPartnership => Some(client::contact::EntityType::UkPartnership),
            epp_proto::contact::EntityType::UkSoleTrader => Some(client::contact::EntityType::UkSoleTrader),
            epp_proto::contact::EntityType::UkLimitedLiabilityPartnership => Some(client::contact::EntityType::UkLimitedLiabilityPartnership),
            epp_proto::contact::EntityType::UkIndustrialProvidentRegisteredCompany => Some(client::contact::EntityType::UkIndustrialProvidentRegisteredCompany),
            epp_proto::contact::EntityType::UkIndividual => Some(client::contact::EntityType::UkIndividual),
            epp_proto::contact::EntityType::UkSchool => Some(client::contact::EntityType::UkSchool),
            epp_proto::contact::EntityType::UkRegisteredCharity => Some(client::contact::EntityType::UkRegisteredCharity),
            epp_proto::contact::EntityType::UkGovernmentBody => Some(client::contact::EntityType::UkGovernmentBody),
            epp_proto::contact::EntityType::UkCorporationByRoyalCharter => Some(client::contact::EntityType::UkCorporationByRoyalCharter),
            epp_proto::contact::EntityType::UkStatutoryBody => Some(client::contact::EntityType::UkStatutoryBody),
            epp_proto::contact::EntityType::NonUkIndividual => Some(client::contact::EntityType::NonUkIndividual),
            epp_proto::contact::EntityType::NonUkCompany => Some(client::contact::EntityType::NonUkCompany),
            epp_proto::contact::EntityType::OtherUkEntity => Some(client::contact::EntityType::OtherUkEntity),
            epp_proto::contact::EntityType::OtherNonUkEntity => Some(client::contact::EntityType::OtherNonUkEntity),
            epp_proto::contact::EntityType::UnknownEntity => Some(client::contact::EntityType::Unknown),
            epp_proto::contact::EntityType::NotSet => None
        }
        None => None
    }
}

fn period_unit_from_i32(from: i32) -> client::domain::PeriodUnit {
    match epp_proto::domain::period::Unit::from_i32(from) {
        Some(e) => match e {
            epp_proto::domain::period::Unit::Months => client::domain::PeriodUnit::Months,
            epp_proto::domain::period::Unit::Years => client::domain::PeriodUnit::Years,
        }
        None => client::domain::PeriodUnit::Years
    }
}

fn disclosure_type_from_i32(from: Vec<i32>) -> Vec<client::contact::DisclosureType> {
    let mut out = vec![];
    for i in from {
        if let Some(e) = epp_proto::contact::DisclosureType::from_i32(i) {
           out.push(match e {
                epp_proto::contact::DisclosureType::LocalName => client::contact::DisclosureType::LocalName,
                epp_proto::contact::DisclosureType::InternationalisedName => client::contact::DisclosureType::InternationalisedAddress,
                epp_proto::contact::DisclosureType::LocalOrganisation => client::contact::DisclosureType::LocalOrganisation,
                epp_proto::contact::DisclosureType::InternationalisedOrganisation => client::contact::DisclosureType::InternationalisedOrganisation,
                epp_proto::contact::DisclosureType::LocalAddress => client::contact::DisclosureType::LocalAddress,
                epp_proto::contact::DisclosureType::InternationalisedAddress => client::contact::DisclosureType::InternationalisedAddress,
                epp_proto::contact::DisclosureType::Voice => client::contact::DisclosureType::Voice,
                epp_proto::contact::DisclosureType::Fax => client::contact::DisclosureType::Fax,
                epp_proto::contact::DisclosureType::Email => client::contact::DisclosureType::Email,
            })
        }
    }
    out
}

fn contact_status_from_i32(from: Vec<i32>) -> Vec<client::contact::Status> {
    let mut out = vec![];
    for i in from {
        if let Some(e) =  epp_proto::contact::ContactStatus::from_i32(i) {
            out.push(match e {
                epp_proto::contact::ContactStatus::ClientDeleteProhibited => client::contact::Status::ClientDeleteProhibited,
                epp_proto::contact::ContactStatus::ClientTransferProhibited => client::contact::Status::ClientTransferProhibited,
                epp_proto::contact::ContactStatus::ClientUpdateProhibited => client::contact::Status::ClientUpdateProhibited,
                epp_proto::contact::ContactStatus::Linked => client::contact::Status::Linked,
                epp_proto::contact::ContactStatus::Ok => client::contact::Status::Ok,
                epp_proto::contact::ContactStatus::PendingCreate => client::contact::Status::PendingCreate,
                epp_proto::contact::ContactStatus::PendingDelete => client::contact::Status::PendingDelete,
                epp_proto::contact::ContactStatus::PendingTransfer => client::contact::Status::PendingTransfer,
                epp_proto::contact::ContactStatus::PendingUpdate => client::contact::Status::PendingUpdate,
                epp_proto::contact::ContactStatus::ServerDeleteProhibited => client::contact::Status::ServerDeleteProhibited,
                epp_proto::contact::ContactStatus::ServerTransferProhibited => client::contact::Status::ServerTransferProhibited,
                epp_proto::contact::ContactStatus::ServerUpdateProhibited => client::contact::Status::ServerUpdateProhibited,
            })
        }
    }
    out
}

fn host_status_from_i32(from: i32) -> Option<client::host::Status> {
    match epp_proto::host::HostStatus::from_i32(from) {
        Some(e) => Some(match e {
            epp_proto::host::HostStatus::ClientDeleteProhibited => client::host::Status::ClientDeleteProhibited,
            epp_proto::host::HostStatus::ClientUpdateProhibited => client::host::Status::ClientUpdateProhibited,
            epp_proto::host::HostStatus::Linked => client::host::Status::Linked,
            epp_proto::host::HostStatus::Ok => client::host::Status::Ok,
            epp_proto::host::HostStatus::PendingCreate => client::host::Status::PendingCreate,
            epp_proto::host::HostStatus::PendingDelete => client::host::Status::PendingDelete,
            epp_proto::host::HostStatus::PendingTransfer => client::host::Status::PendingTransfer,
            epp_proto::host::HostStatus::PendingUpdate => client::host::Status::PendingUpdate,
            epp_proto::host::HostStatus::ServerDeleteProhibited => client::host::Status::ServerDeleteProhibited,
            epp_proto::host::HostStatus::ServerUpdateProhibited => client::host::Status::ServerUpdateProhibited,
        }),
        None => None
    }
}

fn domain_status_from_i32(from: i32) -> Option<client::domain::Status> {
    match epp_proto::domain::DomainStatus::from_i32(from) {
        Some(e) => Some(match e {
            epp_proto::domain::DomainStatus::ClientDeleteProhibited => client::domain::Status::ClientDeleteProhibited,
            epp_proto::domain::DomainStatus::ClientHold => client::domain::Status::ClientHold,
            epp_proto::domain::DomainStatus::ClientRenewProhibited => client::domain::Status::ClientRenewProhibited,
            epp_proto::domain::DomainStatus::ClientTransferProhibited => client::domain::Status::ClientTransferProhibited,
            epp_proto::domain::DomainStatus::ClientUpdateProhibited => client::domain::Status::ClientUpdateProhibited,
            epp_proto::domain::DomainStatus::Inactive => client::domain::Status::Inactive,
            epp_proto::domain::DomainStatus::Ok => client::domain::Status::Ok,
            epp_proto::domain::DomainStatus::PendingCreate => client::domain::Status::PendingCreate,
            epp_proto::domain::DomainStatus::PendingDelete => client::domain::Status::PendingDelete,
            epp_proto::domain::DomainStatus::PendingRenew => client::domain::Status::PendingRenew,
            epp_proto::domain::DomainStatus::PendingTransfer => client::domain::Status::PendingTransfer,
            epp_proto::domain::DomainStatus::PendingUpdate => client::domain::Status::PendingUpdate,
            epp_proto::domain::DomainStatus::ServerDeleteProhibited => client::domain::Status::ServerDeleteProhibited,
            epp_proto::domain::DomainStatus::ServerHold => client::domain::Status::ServerHold,
            epp_proto::domain::DomainStatus::ServerRenewProhibited => client::domain::Status::ServerRenewProhibited,
            epp_proto::domain::DomainStatus::ServerTransferProhibited => client::domain::Status::ServerTransferProhibited,
            epp_proto::domain::DomainStatus::ServerUpdateProhibited => client::domain::Status::ServerUpdateProhibited,
        }),
        None => None
    }
}

fn i32_from_transfer_status(from: client::TransferStatus) -> i32 {
    match from {
        client::TransferStatus::ClientApproved => epp_proto::common::TransferStatus::ClientApproved.into(),
        client::TransferStatus::ClientCancelled => epp_proto::common::TransferStatus::ClientCancelled.into(),
        client::TransferStatus::ClientRejected => epp_proto::common::TransferStatus::ClientRejected.into(),
        client::TransferStatus::Pending => epp_proto::common::TransferStatus::Pending.into(),
        client::TransferStatus::ServerApproved => epp_proto::common::TransferStatus::ServerApproved.into(),
        client::TransferStatus::ServerCancelled => epp_proto::common::TransferStatus::ServerCancelled.into(),
    }
}

fn i32_from_restore_status(from: client::rgp::RGPState) -> i32 {
    match from {
        client::rgp::RGPState::Unknown => epp_proto::rgp::RgpState::Unknown.into(),
        client::rgp::RGPState::AddPeriod => epp_proto::rgp::RgpState::AddPeriod.into(),
        client::rgp::RGPState::AutoRenewPeriod => epp_proto::rgp::RgpState::AutoRenewPeriod.into(),
        client::rgp::RGPState::RenewPeriod => epp_proto::rgp::RgpState::RenewPeriod.into(),
        client::rgp::RGPState::TransferPeriod => epp_proto::rgp::RgpState::TransferPeriod.into(),
        client::rgp::RGPState::RedemptionPeriod => epp_proto::rgp::RgpState::RedemptionPeriod.into(),
        client::rgp::RGPState::PendingRestore => epp_proto::rgp::RgpState::PendingRestore.into(),
        client::rgp::RGPState::PendingDelete => epp_proto::rgp::RgpState::PendingDelete.into(),
    }
}

fn client_by_domain(router: &super::Router, domain: &str) -> Result<(client::RequestSender, String), tonic::Status> {
    match router.client_by_domain(domain) {
        Some(c) => Ok(c),
        None => Err(tonic::Status::invalid_argument("unsupported domain"))
    }
}

fn client_by_id(router: &super::Router, id: &str) -> Result<client::RequestSender, tonic::Status> {
    match router.client_by_id(id) {
        Some(c) => Ok(c),
        None => Err(tonic::Status::not_found("unknown registry"))
    }
}

#[tonic::async_trait]
impl epp_proto::epp_proxy_server::EppProxy for EPPProxy {
    async fn domain_check(
        &self,
        request: tonic::Request<epp_proto::domain::DomainCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainCheckReply>, tonic::Status> {
        let name: String = request.into_inner().name;
        let (mut sender, registry_name) = client_by_domain(&self.client_router,&name)?;
        let res = client::domain::check(&name, &mut sender).await?;

        let reply = epp_proto::domain::DomainCheckReply {
            available: res.avail,
            reason: res.reason,
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_info(
        &self,
        request: tonic::Request<epp_proto::domain::DomainInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainInfoReply>, tonic::Status> {
        let name: String = request.into_inner().name;
        let (mut sender, registry_name) = client_by_domain(&self.client_router,&name)?;
        let res = client::domain::info(&name, &mut sender).await?;

        let reply = epp_proto::domain::DomainInfoReply {
            name: res.name,
            registry_id: res.registry_id,
            statuses: res.statuses.into_iter().map(|s| match s {
                client::domain::Status::ClientDeleteProhibited => epp_proto::domain::DomainStatus::ClientDeleteProhibited.into(),
                client::domain::Status::ClientHold => epp_proto::domain::DomainStatus::ClientHold.into(),
                client::domain::Status::ClientRenewProhibited => epp_proto::domain::DomainStatus::ClientRenewProhibited.into(),
                client::domain::Status::ClientTransferProhibited => epp_proto::domain::DomainStatus::ClientTransferProhibited.into(),
                client::domain::Status::ClientUpdateProhibited => epp_proto::domain::DomainStatus::ClientUpdateProhibited.into(),
                client::domain::Status::Inactive => epp_proto::domain::DomainStatus::Inactive.into(),
                client::domain::Status::Ok => epp_proto::domain::DomainStatus::Ok.into(),
                client::domain::Status::PendingCreate => epp_proto::domain::DomainStatus::PendingCreate.into(),
                client::domain::Status::PendingDelete => epp_proto::domain::DomainStatus::PendingDelete.into(),
                client::domain::Status::PendingRenew => epp_proto::domain::DomainStatus::PendingRenew.into(),
                client::domain::Status::PendingTransfer => epp_proto::domain::DomainStatus::PendingTransfer.into(),
                client::domain::Status::PendingUpdate => epp_proto::domain::DomainStatus::PendingUpdate.into(),
                client::domain::Status::ServerDeleteProhibited => epp_proto::domain::DomainStatus::ServerDeleteProhibited.into(),
                client::domain::Status::ServerHold => epp_proto::domain::DomainStatus::ServerHold.into(),
                client::domain::Status::ServerRenewProhibited => epp_proto::domain::DomainStatus::ServerRenewProhibited.into(),
                client::domain::Status::ServerTransferProhibited => epp_proto::domain::DomainStatus::ServerTransferProhibited.into(),
                client::domain::Status::ServerUpdateProhibited => epp_proto::domain::DomainStatus::ServerUpdateProhibited.into(),
            }).collect(),
            registrant: res.registrant,
            contacts: res.contacts.into_iter().map(|c| epp_proto::domain::Contact {
                id: c.contact_id,
                r#type: c.contact_type,
            }).collect(),
            nameservers: res.nameservers.into_iter().map(|n| match n {
                client::domain::InfoNameserver::HostOnly(h) => {
                    epp_proto::domain::NameServer {
                        host: h,
                        address: None,
                    }
                }
                client::domain::InfoNameserver::HostAndAddress {
                    host,
                    address,
                    ip_version,
                } => epp_proto::domain::NameServer {
                    host,
                    address: Some(epp_proto::common::IpAddress {
                        address,
                        r#type: match ip_version {
                            client::domain::InfoNameserverAddressVersion::IPv4 => {
                                epp_proto::common::ip_address::IpVersion::IPv4.into()
                            }
                            client::domain::InfoNameserverAddressVersion::IPv6 => {
                                epp_proto::common::ip_address::IpVersion::IPv6.into()
                            }
                        },
                    }),
                },
            }).collect(),
            hosts: res.hosts,
            client_id: res.client_id,
            client_created_id: res.client_created_id,
            creation_date: chrono_to_proto(res.creation_date),
            expiry_date: chrono_to_proto(res.expiry_date),
            last_updated_client: res.last_updated_client,
            last_updated_date: chrono_to_proto(res.last_updated_date),
            last_transfer_date: chrono_to_proto(res.last_transfer_date),
            registry_name,
            rgp_state: i32_from_restore_status(res.rgp_state)
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_create(
        &self,
        request: tonic::Request<epp_proto::domain::DomainCreateRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainCreateReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) = client_by_domain(&self.client_router,&request.name)?;
        let res = client::domain::create(&request.name, request.period.map(|p| client::domain::Period {
            unit: period_unit_from_i32(p.unit),
            value: p.value
        }), &request.registrant, request.contacts.into_iter().map(|c| client::domain::InfoContact {
            contact_id: c.id,
            contact_type: c.r#type
        }).collect(), request.nameservers.iter().map(|n| Ok(match &n.address {
            None => client::domain::InfoNameserver::HostOnly(n.host.clone()),
            Some(addr) => client::domain::InfoNameserver::HostAndAddress {
                host: n.host.clone(),
                address: addr.address.clone(),
                ip_version:  match epp_proto::common::ip_address::IpVersion::from_i32(addr.r#type) {
                    Some(epp_proto::common::ip_address::IpVersion::IPv4) => {
                        client::domain::InfoNameserverAddressVersion::IPv4
                    }
                    Some(epp_proto::common::ip_address::IpVersion::IPv6) => {
                        client::domain::InfoNameserverAddressVersion::IPv6
                    }
                    None | Some(epp_proto::common::ip_address::IpVersion::Unknown) => {
                        return Err(tonic::Status::invalid_argument(
                            "unknown IP address version",
                        ));
                    }
                },
            }
        })).collect::<Result<Vec<client::domain::InfoNameserver>, tonic::Status>>()?, &request.auth_info,&mut sender).await?;

        let reply = epp_proto::domain::DomainCreateReply {
            pending: res.pending,
            creation_date: chrono_to_proto(Some(res.creation_date)),
            expiry_date: chrono_to_proto(res.expiration_date),
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_delete(
        &self,
        request: tonic::Request<epp_proto::domain::DomainDeleteRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainDeleteReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) = client_by_domain(&self.client_router,&request.name)?;
        let res = client::domain::delete(&request.name, &mut sender).await?;

        let reply = epp_proto::domain::DomainDeleteReply {
            pending: res.pending,
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_update(
        &self,
        request: tonic::Request<epp_proto::domain::DomainUpdateRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainUpdateReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) = client_by_domain(&self.client_router,&request.name)?;

        let mut add = vec![];
        let mut rem = vec![];

        let map_ns = |n: epp_proto::domain::NameServer| Ok(match &n.address {
            None => client::domain::InfoNameserver::HostOnly(n.host.clone()),
            Some(addr) => client::domain::InfoNameserver::HostAndAddress {
                host: n.host.clone(),
                address: addr.address.clone(),
                ip_version:  match epp_proto::common::ip_address::IpVersion::from_i32(addr.r#type) {
                    Some(epp_proto::common::ip_address::IpVersion::IPv4) => {
                        client::domain::InfoNameserverAddressVersion::IPv4
                    }
                    Some(epp_proto::common::ip_address::IpVersion::IPv6) => {
                        client::domain::InfoNameserverAddressVersion::IPv6
                    }
                    None | Some(epp_proto::common::ip_address::IpVersion::Unknown) => {
                        return Err(tonic::Status::invalid_argument(
                            "unknown IP address version",
                        ));
                    }
                },
            }
        });
        let map_param =
            |p: epp_proto::domain::domain_update_request::Param, l: &mut Vec<client::domain::UpdateObject>| -> Result<(), tonic::Status> {
            match p.param {
                Some(epp_proto::domain::domain_update_request::param::Param::Contact(c)) => {
                    l.push(client::domain::UpdateObject::Contact(client::domain::InfoContact {
                        contact_id: c.id,
                        contact_type: c.r#type
                    }));
                },
                Some(epp_proto::domain::domain_update_request::param::Param::Nameserver(n)) => {
                    l.push(client::domain::UpdateObject::Nameserver(map_ns(n)?));
                },
                Some(epp_proto::domain::domain_update_request::param::Param::State(s)) => {
                    if let Some(s) = domain_status_from_i32(s) {
                        l.push(client::domain::UpdateObject::Status(s));
                    }
                },
                None => {}
            }
            Ok(())
        };

        for p in request.add {
            map_param(p, &mut add)?;
        }
        for p in request.remove {
            map_param(p, &mut rem)?;
        }

        let res = client::domain::update(
            &request.name,
            add,
            rem,
            request.new_registrant.as_deref(),
            request.new_auth_info.as_deref(),
            &mut sender
        ).await?;

        let reply = epp_proto::domain::DomainUpdateReply {
            pending: res.pending,
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_renew(
        &self,
        request: tonic::Request<epp_proto::domain::DomainRenewRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainRenewReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) = client_by_domain(&self.client_router,&request.name)?;

        let cur_expiry_date = proto_to_chrono(request.current_expiry_date);
        if cur_expiry_date.is_none() {
            return Err(tonic::Status::invalid_argument("current_expiry_date must be specified"))
        }

        let res = client::domain::renew(
            &request.name,
            request.period.map(|p| client::domain::Period {
                unit: period_unit_from_i32(p.unit),
                value: p.value
            }),
            cur_expiry_date.unwrap(),
            &mut sender
        ).await?;

        let reply = epp_proto::domain::DomainRenewReply {
            pending: res.pending,
            expiry_date: chrono_to_proto(res.new_expiry_date),
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_transfer_query(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTransferQueryRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainTransferReply>, tonic::Status> {
        let name: String = request.into_inner().name;
        let (mut sender, registry_name) = client_by_domain(&self.client_router,&name)?;
        let res = client::domain::transfer_query(&name, &mut sender).await?;

        let reply = epp_proto::domain::DomainTransferReply {
            pending: res.pending,
            status: i32_from_transfer_status(res.status),
            requested_client_id: res.requested_client_id,
            requested_date: chrono_to_proto(Some(res.requested_date)),
            act_client_id: res.act_client_id,
            act_date: chrono_to_proto(Some(res.act_date)),
            expiry_date: chrono_to_proto(res.expiry_date),
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_transfer_request(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTransferRequestRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) = client_by_domain(&self.client_router,&request.name)?;
        let res = client::domain::transfer_request(
            &request.name,
            request.period.map(|p| client::domain::Period {
                unit: period_unit_from_i32(p.unit),
                value: p.value
            }),
            &request.auth_info,
            &mut sender
        ).await?;

        let reply = epp_proto::domain::DomainTransferReply {
            pending: res.pending,
            status: i32_from_transfer_status(res.status),
            requested_client_id: res.requested_client_id,
            requested_date: chrono_to_proto(Some(res.requested_date)),
            act_client_id: res.act_client_id,
            act_date: chrono_to_proto(Some(res.act_date)),
            expiry_date: chrono_to_proto(res.expiry_date),
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_restore_request(
        &self,
        request: tonic::Request<epp_proto::rgp::RequestRequest>,
    ) -> Result<tonic::Response<epp_proto::rgp::RestoreReply>, tonic::Status> {
        let name: String = request.into_inner().name;
        let (mut sender, registry_name) = client_by_domain(&self.client_router,&name)?;
        let res = client::rgp::request(&name, &mut sender).await?;

        let reply = epp_proto::rgp::RestoreReply {
            pending: res.pending,
            state: i32_from_restore_status(res.state),
            registry_name,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_check(
        &self,
        request: tonic::Request<epp_proto::host::HostCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostCheckReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = client_by_id(&self.client_router,&request.registry_name)?;
        let res = client::host::check(&name, &mut sender).await?;

        let reply = epp_proto::host::HostCheckReply {
            available: res.avail,
            reason: res.reason,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_info(
        &self,
        request: tonic::Request<epp_proto::host::HostInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostInfoReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = client_by_id(&self.client_router,&request.registry_name)?;
        let res = client::host::info(&name, &mut sender).await?;

        let reply = epp_proto::host::HostInfoReply {
            name: res.name,
            registry_id: res.registry_id,
            statuses: res.statuses.into_iter().map(|s| match s {
                client::host::Status::ClientDeleteProhibited => epp_proto::host::HostStatus::ClientDeleteProhibited.into(),
                client::host::Status::ClientUpdateProhibited => epp_proto::host::HostStatus::ClientUpdateProhibited.into(),
                client::host::Status::Linked => epp_proto::host::HostStatus::Linked.into(),
                client::host::Status::Ok => epp_proto::host::HostStatus::Ok.into(),
                client::host::Status::PendingCreate => epp_proto::host::HostStatus::PendingCreate.into(),
                client::host::Status::PendingDelete => epp_proto::host::HostStatus::PendingDelete.into(),
                client::host::Status::PendingTransfer => epp_proto::host::HostStatus::PendingTransfer.into(),
                client::host::Status::PendingUpdate => epp_proto::host::HostStatus::PendingUpdate.into(),
                client::host::Status::ServerDeleteProhibited => epp_proto::host::HostStatus::ServerDeleteProhibited.into(),
                client::host::Status::ServerUpdateProhibited => epp_proto::host::HostStatus::ServerUpdateProhibited.into(),
            }).collect(),
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
            creation_date: chrono_to_proto(res.creation_date),
            last_updated_client: res.last_updated_client,
            last_updated_date: chrono_to_proto(res.last_updated_date),
            last_transfer_date: chrono_to_proto(res.last_transfer_date),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_create(
        &self,
        request: tonic::Request<epp_proto::host::HostCreateRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostCreateReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let addresses: Vec<client::host::Address> = request
            .addresses
            .into_iter()
            .map(|a| {
                Ok(client::host::Address {
                    address: a.address,
                    ip_version: match epp_proto::common::ip_address::IpVersion::from_i32(a.r#type) {
                        Some(epp_proto::common::ip_address::IpVersion::IPv4) => {
                            client::host::AddressVersion::IPv4
                        }
                        Some(epp_proto::common::ip_address::IpVersion::IPv6) => {
                            client::host::AddressVersion::IPv6
                        }
                        None | Some(epp_proto::common::ip_address::IpVersion::Unknown) => {
                            return Err(tonic::Status::invalid_argument(
                                "unknown IP address version",
                            ));
                        }
                    },
                })
            })
            .collect::<Result<Vec<client::host::Address>, tonic::Status>>()?;
        let mut sender = client_by_id(&self.client_router,&request.registry_name)?;
        let res = client::host::create(&name, addresses, &mut sender).await?;

        let reply = epp_proto::host::HostCreateReply {
            pending: res.pending,
            creation_date: chrono_to_proto(res.creation_date),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_delete(
        &self,
        request: tonic::Request<epp_proto::host::HostDeleteRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostDeleteReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = client_by_id(&self.client_router,&request.registry_name)?;
        let res = client::host::delete(&name, &mut sender).await?;

        let reply = epp_proto::host::HostDeleteReply {
            pending: res.pending,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_update(
        &self,
        request: tonic::Request<epp_proto::host::HostUpdateRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostUpdateReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = client_by_id(&self.client_router,&request.registry_name)?;

        let mut add = vec![];
        let mut remove = vec![];

        let map_addr = |addr: epp_proto::common::IpAddress| {
            Ok(client::host::Address {
                address: addr.address,
                ip_version: match epp_proto::common::ip_address::IpVersion::from_i32(addr.r#type) {
                    Some(epp_proto::common::ip_address::IpVersion::IPv4) => {
                        client::host::AddressVersion::IPv4
                    }
                    Some(epp_proto::common::ip_address::IpVersion::IPv6) => {
                        client::host::AddressVersion::IPv6
                    }
                    None | Some(epp_proto::common::ip_address::IpVersion::Unknown) => {
                        return Err(tonic::Status::invalid_argument(
                            "unknown IP address version",
                        ));
                    }
                },
            })
        };

        for a in request.add {
            match a.param {
                Some(epp_proto::host::host_update_request::param::Param::Address(addr)) => {
                    add.push(client::host::UpdateObject::Address(map_addr(addr)?))
                }
                Some(epp_proto::host::host_update_request::param::Param::State(s)) => {
                    if let Some(s) = host_status_from_i32(s) {
                        add.push(client::host::UpdateObject::Status(s))
                    }
                }
                None => {}
            }
        }
        for r in request.remove {
            match r.param {
                Some(epp_proto::host::host_update_request::param::Param::Address(addr)) => {
                    remove.push(client::host::UpdateObject::Address(map_addr(addr)?))
                }
                Some(epp_proto::host::host_update_request::param::Param::State(s)) => {
                    if let Some(s) = host_status_from_i32(s) {
                        remove.push(client::host::UpdateObject::Status(s))
                    }
                }
                None => {}
            }
        }

        let res = client::host::update(&name, add, remove, request.new_name, &mut sender).await?;

        let reply = epp_proto::host::HostUpdateReply {
            pending: res.pending,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_check(
        &self,
        request: tonic::Request<epp_proto::contact::ContactCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactCheckReply>, tonic::Status> {
        let request = request.into_inner();
        let id: String = request.id;
        let mut sender = client_by_id(&self.client_router,&request.registry_name)?;
        let res = client::contact::check(&id, &mut sender).await?;

        let reply = epp_proto::contact::ContactCheckReply {
            available: res.avail,
            reason: res.reason,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_info(
        &self,
        request: tonic::Request<epp_proto::contact::ContactInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactInfoReply>, tonic::Status> {
        let request = request.into_inner();
        let id: String = request.id;
        let mut sender = client_by_id(&self.client_router,&request.registry_name)?;
        let res = client::contact::info(&id, &mut sender).await?;

        let map_addr = |a: client::contact::Address| epp_proto::contact::PostalAddress {
            name: a.name,
            organisation: a.organisation,
            streets: a.streets,
            city: a.city,
            province: a.province,
            postal_code: a.postal_code,
            country_code: a.country_code,
        };

        let reply = epp_proto::contact::ContactInfoReply {
            id: res.id,
            registry_id: res.registry_id,
            statuses: res.statuses.into_iter().map(|s| match s {
                client::contact::Status::ClientDeleteProhibited => epp_proto::contact::ContactStatus::ClientDeleteProhibited.into(),
                client::contact::Status::ClientTransferProhibited => epp_proto::contact::ContactStatus::ClientTransferProhibited.into(),
                client::contact::Status::ClientUpdateProhibited => epp_proto::contact::ContactStatus::ClientUpdateProhibited.into(),
                client::contact::Status::Linked => epp_proto::contact::ContactStatus::Linked.into(),
                client::contact::Status::Ok => epp_proto::contact::ContactStatus::Ok.into(),
                client::contact::Status::PendingCreate => epp_proto::contact::ContactStatus::PendingCreate.into(),
                client::contact::Status::PendingDelete => epp_proto::contact::ContactStatus::PendingDelete.into(),
                client::contact::Status::PendingTransfer => epp_proto::contact::ContactStatus::PendingTransfer.into(),
                client::contact::Status::PendingUpdate => epp_proto::contact::ContactStatus::PendingUpdate.into(),
                client::contact::Status::ServerDeleteProhibited => epp_proto::contact::ContactStatus::ServerDeleteProhibited.into(),
                client::contact::Status::ServerTransferProhibited => epp_proto::contact::ContactStatus::ServerTransferProhibited.into(),
                client::contact::Status::ServerUpdateProhibited => epp_proto::contact::ContactStatus::ServerUpdateProhibited.into(),
            }).collect(),
            local_address: res.local_address.map(map_addr),
            internationalised_address: res.internationalised_address.map(map_addr),
            phone: res.phone,
            fax: res.fax,
            email: res.email,
            client_id: res.client_id,
            client_created_id: res.client_created_id,
            creation_date: chrono_to_proto(res.creation_date),
            last_updated_client: res.last_updated_client,
            last_updated_date: chrono_to_proto(res.last_updated_date),
            last_transfer_date: chrono_to_proto(res.last_transfer_date),
            entity_type: match res.entity_type {
                client::contact::EntityType::UkLimitedCompany =>  epp_proto::contact::EntityType::UkLimitedCompany.into(),
                client::contact::EntityType::UkPublicLimitedCompany =>  epp_proto::contact::EntityType::UkLimitedCompany.into(),
                client::contact::EntityType::UkPartnership =>  epp_proto::contact::EntityType::UkPartnership.into(),
                client::contact::EntityType::UkSoleTrader =>  epp_proto::contact::EntityType::UkSoleTrader.into(),
                client::contact::EntityType::UkLimitedLiabilityPartnership =>  epp_proto::contact::EntityType::UkLimitedLiabilityPartnership.into(),
                client::contact::EntityType::UkIndustrialProvidentRegisteredCompany =>  epp_proto::contact::EntityType::UkIndustrialProvidentRegisteredCompany.into(),
                client::contact::EntityType::UkIndividual =>  epp_proto::contact::EntityType::UkIndividual.into(),
                client::contact::EntityType::UkSchool =>  epp_proto::contact::EntityType::UkSchool.into(),
                client::contact::EntityType::UkRegisteredCharity =>  epp_proto::contact::EntityType::UkRegisteredCharity.into(),
                client::contact::EntityType::UkGovernmentBody =>  epp_proto::contact::EntityType::UkGovernmentBody.into(),
                client::contact::EntityType::UkCorporationByRoyalCharter =>  epp_proto::contact::EntityType::UkCorporationByRoyalCharter.into(),
                client::contact::EntityType::UkStatutoryBody =>  epp_proto::contact::EntityType::UkStatutoryBody.into(),
                client::contact::EntityType::NonUkIndividual =>  epp_proto::contact::EntityType::NonUkIndividual.into(),
                client::contact::EntityType::NonUkCompany =>  epp_proto::contact::EntityType::NonUkCompany.into(),
                client::contact::EntityType::OtherUkEntity =>  epp_proto::contact::EntityType::OtherUkEntity.into(),
                client::contact::EntityType::OtherNonUkEntity =>  epp_proto::contact::EntityType::OtherNonUkEntity.into(),
                client::contact::EntityType::Unknown =>  epp_proto::contact::EntityType::UnknownEntity.into()
            },
            trading_name: res.trading_name,
            company_number: res.company_number,
            disclosure: res.disclosure.into_iter().map(|d| match d {
                client::contact::DisclosureType::LocalName => epp_proto::contact::DisclosureType::LocalName.into(),
                client::contact::DisclosureType::InternationalisedName => epp_proto::contact::DisclosureType::InternationalisedName.into(),
                client::contact::DisclosureType::LocalOrganisation => epp_proto::contact::DisclosureType::LocalOrganisation.into(),
                client::contact::DisclosureType::InternationalisedOrganisation => epp_proto::contact::DisclosureType::InternationalisedOrganisation.into(),
                client::contact::DisclosureType::LocalAddress => epp_proto::contact::DisclosureType::LocalAddress.into(),
                client::contact::DisclosureType::InternationalisedAddress => epp_proto::contact::DisclosureType::InternationalisedAddress.into(),
                client::contact::DisclosureType::Voice => epp_proto::contact::DisclosureType::Voice.into(),
                client::contact::DisclosureType::Fax => epp_proto::contact::DisclosureType::Fax.into(),
                client::contact::DisclosureType::Email => epp_proto::contact::DisclosureType::Email.into(),
            }).collect()
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_create(
        &self,
        request: tonic::Request<epp_proto::contact::ContactCreateRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactCreateReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router,&request.registry_name)?;

        let addr_map = |a: epp_proto::contact::PostalAddress| client::contact::Address {
            name: a.name,
            organisation: a.organisation,
            streets: a.streets,
            city: a.city,
            province: a.province,
            postal_code: a.postal_code,
            country_code: a.country_code,
        };

        let res = client::contact::create(
            &request.id,
            client::contact::NewContactData {
                local_address: request.local_address.map(addr_map),
                internationalised_address: request.internationalised_address.map(addr_map),
                phone: request.phone,
                fax: request.fax,
                email: request.email,
                entity_type: entity_type_from_i32(request.entity_type),
                trading_name: request.trading_name,
                company_number: request.company_number,
                disclosure: request.disclosure.map(|d| disclosure_type_from_i32(d.disclosure)),
            },
            &mut sender,
        )
        .await?;

        let reply = epp_proto::contact::ContactCreateReply {
            pending: res.pending,
            creation_date: chrono_to_proto(res.creation_date),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_delete(
        &self,
        request: tonic::Request<epp_proto::contact::ContactDeleteRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactDeleteReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router,&request.registry_name)?;

        let res = client::contact::delete(&request.id, &mut sender).await?;

        let reply = epp_proto::contact::ContactDeleteReply {
            pending: res.pending,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_update(
        &self,
        request: tonic::Request<epp_proto::contact::ContactUpdateRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactUpdateReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router,&request.registry_name)?;

        let addr_map = |a: epp_proto::contact::PostalAddress| client::contact::Address {
            name: a.name,
            organisation: a.organisation,
            streets: a.streets,
            city: a.city,
            province: a.province,
            postal_code: a.postal_code,
            country_code: a.country_code,
        };

        let res = client::contact::update(
            &request.id,
            contact_status_from_i32(request.add_statuses),
            contact_status_from_i32(request.remove_statuses),
            client::contact::UpdateContactData {
                local_address: request.new_local_address.map(addr_map),
                internationalised_address: request.new_internationalised_address.map(addr_map),
                phone: request.new_phone,
                fax: request.new_fax,
                email: request.new_email,
                entity_type: entity_type_from_i32(request.new_entity_type),
                trading_name: request.new_trading_name,
                company_number: request.new_company_number,
                disclosure: request.disclosure.map(|d| disclosure_type_from_i32(d.disclosure)),
            },
            &mut sender,
        )
        .await?;

        let reply = epp_proto::contact::ContactUpdateReply {
            pending: res.pending,
        };

        Ok(tonic::Response::new(reply))
    }

    type PollStream = futures::channel::mpsc::Receiver<Result<epp_proto::PollReply, tonic::Status>>;

    async fn poll(
        &self,
        request: tonic::Request<epp_proto::RegistryInfo>,
    ) -> Result<tonic::Response<Self::PollStream>, tonic::Status> {
        let request = request.into_inner();
        let (mut tx, rx) = futures::channel::mpsc::channel(4);
        let mut sender = client_by_id(&self.client_router,&request.registry_name)?;

        tokio::spawn(async move {
            let should_delay = true;
            loop {
                match client::poll::poll(&mut sender).await {
                    Ok(resp) => if let Some(message) = resp {
                        //                            if message.count > 0 {
                        //                                delay_time = 0;
                        //                            }
                        match tx
                            .send(Ok(epp_proto::PollReply {
                                msg_id: message.id.clone(),
                                enqueue_date: chrono_to_proto(Some(message.enqueue_time)),
                                message: message.message,
                            }))
                            .await
                        {
                            Ok(_) => {
                                match client::poll::poll_ack(&message.id, &mut sender).await {
                                    Ok(_) => {},
                                    Err(err) => match tx.send(Err(err.into())).await {
                                        Ok(_) => {}
                                        Err(_) => break,
                                    }
                                }
                            }
                            Err(_) => break,
                        }
                    } else if tx.is_closed() {
                        break
                    },
                    Err(err) => match tx.send(Err(err.into())).await {
                        Ok(_) => {}
                        Err(_) => break,
                    },
                }
                if should_delay {
                    tokio::time::delay_for(tokio::time::Duration::new(15, 0)).await;
                }
            }
        });

        Ok(tonic::Response::new(rx))
    }

    async fn nominet_tag_list(
        &self,
        request: tonic::Request<epp_proto::RegistryInfo>,
    ) -> Result<tonic::Response<epp_proto::NominetTagListReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router,&request.registry_name)?;

        let res = client::nominet::tag_list(&mut sender).await?;

        let reply = epp_proto::NominetTagListReply {
            tags: res.tags.into_iter().map(|t| epp_proto::nominet_tag_list_reply::Tag {
                tag: t.tag,
                name: t.name,
                trading_name: t.trading_name,
                handshake: t.handshake
            }).collect()
        };

        Ok(tonic::Response::new(reply))
    }
}
