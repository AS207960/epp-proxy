//! Implements the gRPC interface for the EPP client

use std::convert::TryInto;

use futures::sink::SinkExt;

use super::client;

mod contact;
mod dac;
mod domain;
mod eurid;
mod fee;
mod host;
mod isnic;
mod launch;
mod maintenance;
mod mark;
mod nominet;
mod rgp;
mod tmch;
mod utils;

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

        pub mod qualified_lawyer {
            tonic::include_proto!("epp.contact.qualified_lawyer");
        }
    }

    pub mod rgp {
        tonic::include_proto!("epp.rgp");
    }

    pub mod nominet {
        tonic::include_proto!("epp.nominet");
    }

    pub mod nominet_ext {
        tonic::include_proto!("epp.nominet_ext");
    }

    pub mod traficom {
        tonic::include_proto!("epp.traficom");
    }

    pub mod fee {
        tonic::include_proto!("epp.fee");
    }

    pub mod launch {
        tonic::include_proto!("epp.launch");
    }

    pub mod maintenance {
        tonic::include_proto!("epp.maintenance");
    }

    pub mod eurid {
        tonic::include_proto!("epp.eurid");
    }

    pub mod isnic {
        tonic::include_proto!("epp.isnic");
    }

    pub mod marks {
        tonic::include_proto!("epp.marks");
    }

    pub mod tmch {
        tonic::include_proto!("epp.tmch");
    }

    pub mod dac {
        tonic::include_proto!("epp.dac");
    }

    pub mod personal_registration {
        tonic::include_proto!("epp.personal_registration");
    }
}

#[derive(Debug)]
pub struct EPPProxy {
    pub client_router: super::Router,
}

impl From<client::traficom::TrnData> for epp_proto::traficom::TrnData {
    fn from(from: client::traficom::TrnData) -> Self {
        epp_proto::traficom::TrnData { name: from.name }
    }
}

// fn client_by_domain(
//     router: &super::Router,
//     domain: &str,
// ) -> Result<(client::RequestSender, String), tonic::Status> {
//     match router.client_by_domain(domain) {
//         Some(c) => Ok(c),
//         None => Err(tonic::Status::invalid_argument("unsupported domain")),
//     }
// }

fn client_by_domain_or_id(
    router: &super::Router,
    domain: &str,
    registry_id: Option<String>,
) -> Result<(client::RequestSender, String), tonic::Status> {
    if let Some(r) = registry_id {
        if let Some(c) = router.client_by_id(&r) {
            return Ok((c, r));
        }
    }
    match router.client_by_domain(domain) {
        Some(c) => Ok(c),
        None => Err(tonic::Status::invalid_argument("unsupported domain")),
    }
}

fn client_by_id(router: &super::Router, id: &str) -> Result<client::RequestSender, tonic::Status> {
    match router.client_by_id(id) {
        Some(c) => Ok(c),
        None => Err(tonic::Status::not_found("unknown registry")),
    }
}

#[tonic::async_trait]
impl epp_proto::epp_proxy_server::EppProxy for EPPProxy {
    async fn domain_check(
        &self,
        request: tonic::Request<epp_proto::domain::DomainCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainCheckReply>, tonic::Status> {
        let res = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &res.name, res.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::domain::check(&res.name, res.fee_check.map(Into::into), None, &mut sender)
                .await?,
        );

        let mut reply: epp_proto::domain::DomainCheckReply = res.into();
        reply.registry_name = registry_name;
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn domain_claims_check(
        &self,
        request: tonic::Request<epp_proto::domain::DomainClaimsCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainClaimsCheckReply>, tonic::Status> {
        let res = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &res.name, res.registry_name)?;
        let launch_check = match res.launch_check {
            Some(l) => l,
            None => {
                return Err(tonic::Status::invalid_argument(
                    "Launch check must be specified",
                ));
            }
        };
        let (res, cmd_resp) = utils::map_command_response(
            client::domain::launch_claims_check(&res.name, launch_check.into(), &mut sender)
                .await?,
        );

        let reply = epp_proto::domain::DomainClaimsCheckReply {
            exists: res.exists,
            claims_keys: res.claims_key.into_iter().map(Into::into).collect(),
            registry_name,
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_trademark_check(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTrademarkCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainClaimsCheckReply>, tonic::Status> {
        let res = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &res.name, res.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::domain::launch_trademark_check(&res.name, &mut sender).await?,
        );

        let reply = epp_proto::domain::DomainClaimsCheckReply {
            exists: res.exists,
            claims_keys: res.claims_key.into_iter().map(Into::into).collect(),
            registry_name,
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_info(
        &self,
        request: tonic::Request<epp_proto::domain::DomainInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainInfoReply>, tonic::Status> {
        let req = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &req.name, req.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::domain::info(
                &req.name,
                req.auth_info.as_deref(),
                req.hosts.map(
                    |h| match epp_proto::domain::DomainHostsType::from_i32(h.hosts) {
                        Some(epp_proto::domain::DomainHostsType::All) => {
                            client::domain::InfoHost::All
                        }
                        Some(epp_proto::domain::DomainHostsType::Delegated) => {
                            client::domain::InfoHost::Delegated
                        }
                        Some(epp_proto::domain::DomainHostsType::Subordinate) => {
                            client::domain::InfoHost::Subordinate
                        }
                        Some(epp_proto::domain::DomainHostsType::None) => {
                            client::domain::InfoHost::None
                        }
                        None => client::domain::InfoHost::All,
                    },
                ),
                match req.launch_info {
                    Some(i) => Some(TryInto::try_into(i)?),
                    None => None,
                },
                req.eurid_data.map(Into::into),
                &mut sender,
            )
            .await?,
        );

        let mut reply: epp_proto::domain::DomainInfoReply = res.into();
        reply.registry_name = registry_name;
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn domain_create(
        &self,
        request: tonic::Request<epp_proto::domain::DomainCreateRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainCreateReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;

        let mut ns = vec![];

        for n in &request.nameservers {
            match &n.server {
                Some(epp_proto::domain::name_server::Server::HostObj(h)) => {
                    ns.push(client::domain::InfoNameserver::HostOnly(h.clone()))
                }
                Some(epp_proto::domain::name_server::Server::HostName(h)) => {
                    ns.push(client::domain::InfoNameserver::HostAndAddress {
                        eurid_idn: None,
                        host: h.clone(),
                        addresses: n
                            .addresses
                            .iter()
                            .map(|addr| {
                                Ok(client::host::Address {
                                    address: addr.address.clone(),
                                    ip_version:
                                        match epp_proto::common::ip_address::IpVersion::from_i32(
                                            addr.r#type,
                                        ) {
                                            Some(
                                                epp_proto::common::ip_address::IpVersion::IPv4,
                                            ) => client::host::AddressVersion::IPv4,
                                            Some(
                                                epp_proto::common::ip_address::IpVersion::IPv6,
                                            ) => client::host::AddressVersion::IPv6,
                                            None
                                            | Some(
                                                epp_proto::common::ip_address::IpVersion::Unknown,
                                            ) => {
                                                return Err(tonic::Status::invalid_argument(
                                                    "unknown IP address version",
                                                ));
                                            }
                                        },
                                })
                            })
                            .collect::<Result<Vec<client::host::Address>, tonic::Status>>()?,
                    })
                }
                None => {}
            }
        }

        let (res, cmd_resp) = utils::map_command_response(
            client::domain::create(
                client::domain::CreateInfo {
                    domain: &request.name,
                    period: request.period.map(Into::into),
                    registrant: &request.registrant,
                    contacts: request
                        .contacts
                        .into_iter()
                        .map(|c| client::domain::InfoContact {
                            contact_id: c.id,
                            contact_type: c.r#type,
                        })
                        .collect(),
                    nameservers: ns,
                    auth_info: &request.auth_info,
                    sec_dns: match request.sec_dns {
                        Some(sec_dns) => match sec_dns.data {
                            Some(sec_dns_data) => Some(client::domain::SecDNSData {
                                max_sig_life: sec_dns.max_sig_life,
                                data: match sec_dns_data {
                                    epp_proto::domain::sec_dns_data::Data::DsData(ds_data) => {
                                        client::domain::SecDNSDataType::DSData(
                                            ds_data
                                                .data
                                                .into_iter()
                                                .map(|d| client::domain::SecDNSDSData {
                                                    key_tag: d.key_tag as u16,
                                                    algorithm: d.algorithm as u8,
                                                    digest_type: d.digest_type as u8,
                                                    digest: d.digest,
                                                    key_data: d.key_data.map(|k| {
                                                        client::domain::SecDNSKeyData {
                                                            flags: k.flags as u16,
                                                            protocol: k.protocol as u8,
                                                            algorithm: k.algorithm as u8,
                                                            public_key: k.public_key,
                                                        }
                                                    }),
                                                })
                                                .collect(),
                                        )
                                    }
                                    epp_proto::domain::sec_dns_data::Data::KeyData(key_data) => {
                                        client::domain::SecDNSDataType::KeyData(
                                            key_data
                                                .data
                                                .into_iter()
                                                .map(|k| client::domain::SecDNSKeyData {
                                                    flags: k.flags as u16,
                                                    protocol: k.protocol as u8,
                                                    algorithm: k.algorithm as u8,
                                                    public_key: k.public_key,
                                                })
                                                .collect(),
                                        )
                                    }
                                },
                            }),
                            None => None,
                        },
                        None => None,
                    },
                    launch_create: match request.launch_data {
                        Some(i) => Some(TryInto::try_into(i)?),
                        None => None,
                    },
                    fee_agreement: request.fee_agreement.map(Into::into),
                    donuts_fee_agreement: request
                        .donuts_fee_agreement
                        .map(TryInto::try_into)
                        .map_or(Ok(None), |v| v.map(Some))?,
                    eurid_data: request.eurid_data.map(Into::into),
                    isnic_payment: request.isnic_payment.and_then(Into::into),
                    personal_registration: request.personal_registration.map(|p| {
                        client::personal_registration::PersonalRegistrationInfo {
                            consent_id: p.consent_id,
                        }
                    }),
                },
                &mut sender,
            )
            .await?,
        );

        let mut reply: epp_proto::domain::DomainCreateReply = res.into();
        reply.registry_name = registry_name;
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn domain_delete(
        &self,
        request: tonic::Request<epp_proto::domain::DomainDeleteRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainDeleteReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::domain::delete(
                &request.name,
                match request.launch_data {
                    Some(i) => Some(TryInto::try_into(i)?),
                    None => None,
                },
                request
                    .donuts_fee_agreement
                    .map(TryInto::try_into)
                    .map_or(Ok(None), |v| v.map(Some))?,
                request.eurid_data.and_then(Into::into),
                &mut sender,
            )
            .await?,
        );

        let reply = epp_proto::domain::DomainDeleteReply {
            pending: res.pending,
            fee_data: res.fee_data.map(Into::into),
            registry_name,
            cmd_resp: Some(cmd_resp),
            eurid_idn: res.eurid_idn.map(Into::into),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_update(
        &self,
        request: tonic::Request<epp_proto::domain::DomainUpdateRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainUpdateReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;

        let mut add = vec![];
        let mut rem = vec![];

        let map_ns = |n: epp_proto::domain::NameServer| {
            Ok(match &n.server {
                Some(epp_proto::domain::name_server::Server::HostObj(h)) => {
                    client::domain::InfoNameserver::HostOnly(h.clone())
                }
                Some(epp_proto::domain::name_server::Server::HostName(h)) => {
                    client::domain::InfoNameserver::HostAndAddress {
                        eurid_idn: None,
                        host: h.clone(),
                        addresses: n
                            .addresses
                            .iter()
                            .map(|addr| {
                                Ok(client::host::Address {
                                    address: addr.address.clone(),
                                    ip_version:
                                        match epp_proto::common::ip_address::IpVersion::from_i32(
                                            addr.r#type,
                                        ) {
                                            Some(
                                                epp_proto::common::ip_address::IpVersion::IPv4,
                                            ) => client::host::AddressVersion::IPv4,
                                            Some(
                                                epp_proto::common::ip_address::IpVersion::IPv6,
                                            ) => client::host::AddressVersion::IPv6,
                                            None
                                            | Some(
                                                epp_proto::common::ip_address::IpVersion::Unknown,
                                            ) => {
                                                return Err(tonic::Status::invalid_argument(
                                                    "unknown IP address version",
                                                ));
                                            }
                                        },
                                })
                            })
                            .collect::<Result<Vec<client::host::Address>, tonic::Status>>()?,
                    }
                }
                None => {
                    return Err(tonic::Status::invalid_argument(
                        "one of host_obj or host_name must be specified",
                    ));
                }
            })
        };
        let map_param = |p: epp_proto::domain::domain_update_request::Param,
                         l: &mut Vec<client::domain::UpdateObject>|
         -> Result<(), tonic::Status> {
            match p.param {
                Some(epp_proto::domain::domain_update_request::param::Param::Contact(c)) => {
                    l.push(client::domain::UpdateObject::Contact(
                        client::domain::InfoContact {
                            contact_id: c.id,
                            contact_type: c.r#type,
                        },
                    ));
                }
                Some(epp_proto::domain::domain_update_request::param::Param::Nameserver(n)) => {
                    l.push(client::domain::UpdateObject::Nameserver(map_ns(n)?));
                }
                Some(epp_proto::domain::domain_update_request::param::Param::State(s)) => {
                    if let Some(s) = domain::domain_status_from_i32(s) {
                        l.push(client::domain::UpdateObject::Status(s));
                    }
                }
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

        let (res, cmd_resp) = utils::map_command_response(
            client::domain::update(
                client::domain::UpdateInfo {
                    domain: &request.name,
                    add,
                    remove: rem,
                    new_registrant: request.new_registrant.as_deref(),
                    new_auth_info: request.new_auth_info.as_deref(),
                    sec_dns: match request.sec_dns {
                        Some(sec_dns) => Some(client::domain::UpdateSecDNS {
                            urgent: sec_dns.urgent,
                            new_max_sig_life: sec_dns.new_max_sig_life,
                            add: sec_dns.add.map(|a| match a {
                                epp_proto::domain::update_sec_dns_data::Add::AddDsData(ds_data) => {
                                    client::domain::SecDNSDataType::DSData(
                                        ds_data
                                            .data
                                            .into_iter()
                                            .map(|d| client::domain::SecDNSDSData {
                                                key_tag: d.key_tag as u16,
                                                algorithm: d.algorithm as u8,
                                                digest_type: d.digest_type as u8,
                                                digest: d.digest,
                                                key_data: d.key_data.map(|k| {
                                                    client::domain::SecDNSKeyData {
                                                        flags: k.flags as u16,
                                                        protocol: k.protocol as u8,
                                                        algorithm: k.algorithm as u8,
                                                        public_key: k.public_key,
                                                    }
                                                }),
                                            })
                                            .collect(),
                                    )
                                }
                                epp_proto::domain::update_sec_dns_data::Add::AddKeyData(
                                    key_data,
                                ) => client::domain::SecDNSDataType::KeyData(
                                    key_data
                                        .data
                                        .into_iter()
                                        .map(|k| client::domain::SecDNSKeyData {
                                            flags: k.flags as u16,
                                            protocol: k.protocol as u8,
                                            algorithm: k.algorithm as u8,
                                            public_key: k.public_key,
                                        })
                                        .collect(),
                                ),
                            }),
                            remove: sec_dns.remove.map(|r| match r {
                                epp_proto::domain::update_sec_dns_data::Remove::All(a) => {
                                    client::domain::UpdateSecDNSRemove::All(a)
                                }
                                epp_proto::domain::update_sec_dns_data::Remove::RemDsData(
                                    ds_data,
                                ) => client::domain::UpdateSecDNSRemove::Data(
                                    client::domain::SecDNSDataType::DSData(
                                        ds_data
                                            .data
                                            .into_iter()
                                            .map(|d| client::domain::SecDNSDSData {
                                                key_tag: d.key_tag as u16,
                                                algorithm: d.algorithm as u8,
                                                digest_type: d.digest_type as u8,
                                                digest: d.digest,
                                                key_data: d.key_data.map(|k| {
                                                    client::domain::SecDNSKeyData {
                                                        flags: k.flags as u16,
                                                        protocol: k.protocol as u8,
                                                        algorithm: k.algorithm as u8,
                                                        public_key: k.public_key,
                                                    }
                                                }),
                                            })
                                            .collect(),
                                    ),
                                ),
                                epp_proto::domain::update_sec_dns_data::Remove::RemKeyData(
                                    key_data,
                                ) => client::domain::UpdateSecDNSRemove::Data(
                                    client::domain::SecDNSDataType::KeyData(
                                        key_data
                                            .data
                                            .into_iter()
                                            .map(|k| client::domain::SecDNSKeyData {
                                                flags: k.flags as u16,
                                                protocol: k.protocol as u8,
                                                algorithm: k.algorithm as u8,
                                                public_key: k.public_key,
                                            })
                                            .collect(),
                                    ),
                                ),
                            }),
                        }),
                        None => None,
                    },
                    launch_info: match request.launch_data {
                        Some(i) => Some(TryInto::try_into(i)?),
                        None => None,
                    },
                    fee_agreement: request.fee_agreement.map(Into::into),
                    donuts_fee_agreement: request
                        .donuts_fee_agreement
                        .map(TryInto::try_into)
                        .map_or(Ok(None), |v| v.map(Some))?,
                    eurid_data: request.eurid_data.map(Into::into),
                    isnic_info: request.isnic_info.map(Into::into),
                },
                &mut sender,
            )
            .await?,
        );

        let mut reply: epp_proto::domain::DomainUpdateReply = res.into();
        reply.registry_name = registry_name;
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn domain_sync(
        &self,
        request: tonic::Request<epp_proto::domain::DomainSyncRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainUpdateReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;

        let (res, cmd_resp) = utils::map_command_response(
            client::domain::verisign_sync(&request.name, request.month, request.day, &mut sender)
                .await?,
        );

        let mut reply: epp_proto::domain::DomainUpdateReply = res.into();
        reply.registry_name = registry_name;
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn domain_renew(
        &self,
        request: tonic::Request<epp_proto::domain::DomainRenewRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainRenewReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;

        let cur_expiry_date = utils::proto_to_chrono(request.current_expiry_date);
        if cur_expiry_date.is_none() {
            return Err(tonic::Status::invalid_argument(
                "current_expiry_date must be specified",
            ));
        }

        let (res, cmd_resp) = utils::map_command_response(
            client::domain::renew(
                &request.name,
                request.period.map(Into::into),
                cur_expiry_date.unwrap(),
                request.fee_agreement.map(Into::into),
                request
                    .donuts_fee_agreement
                    .map(TryInto::try_into)
                    .map_or(Ok(None), |v| v.map(Some))?,
                request.isnic_payment.and_then(Into::into),
                &mut sender,
            )
            .await?,
        );

        let mut reply: epp_proto::domain::DomainRenewReply = res.into();
        reply.registry_name = registry_name;
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn domain_transfer_query(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTransferQueryRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainTransferReply>, tonic::Status> {
        let req = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &req.name, req.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::domain::transfer_query(&req.name, req.auth_info.as_deref(), &mut sender)
                .await?,
        );

        let mut reply: epp_proto::domain::DomainTransferReply = res.into();
        reply.registry_name = registry_name;
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn domain_transfer_request(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTransferRequestRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::domain::transfer_request(
                &request.name,
                request.period.map(Into::into),
                &request.auth_info,
                request.fee_agreement.map(Into::into),
                request
                    .donuts_fee_agreement
                    .map(TryInto::try_into)
                    .map_or(Ok(None), |v| v.map(Some))?,
                request.eurid_data.map(Into::into),
                &mut sender,
            )
            .await?,
        );

        let mut reply: epp_proto::domain::DomainTransferReply = res.into();
        reply.registry_name = registry_name;
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn domain_transfer_cancel(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTransferAcceptRejectRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::domain::transfer_cancel(&request.name, Some(&request.auth_info), &mut sender)
                .await?,
        );

        let mut reply: epp_proto::domain::DomainTransferReply = res.into();
        reply.registry_name = registry_name;
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn domain_transfer_accept(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTransferAcceptRejectRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::domain::transfer_accept(&request.name, Some(&request.auth_info), &mut sender)
                .await?,
        );

        let mut reply: epp_proto::domain::DomainTransferReply = res.into();
        reply.registry_name = registry_name;
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn domain_transfer_reject(
        &self,
        request: tonic::Request<epp_proto::domain::DomainTransferAcceptRejectRequest>,
    ) -> Result<tonic::Response<epp_proto::domain::DomainTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::domain::transfer_reject(&request.name, Some(&request.auth_info), &mut sender)
                .await?,
        );

        let mut reply: epp_proto::domain::DomainTransferReply = res.into();
        reply.registry_name = registry_name;
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn domain_restore_request(
        &self,
        request: tonic::Request<epp_proto::rgp::RequestRequest>,
    ) -> Result<tonic::Response<epp_proto::rgp::RestoreReply>, tonic::Status> {
        let res = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &res.name, res.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::rgp::request(
                &res.name,
                res.donuts_fee_agreement
                    .map(TryInto::try_into)
                    .map_or(Ok(None), |v| v.map(Some))?,
                &mut sender,
            )
            .await?,
        );

        let reply = epp_proto::rgp::RestoreReply {
            pending: res.pending,
            state: res
                .state
                .into_iter()
                .map(rgp::i32_from_restore_status)
                .collect(),
            fee_data: res.fee_data.map(Into::into),
            registry_name,
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_check(
        &self,
        request: tonic::Request<epp_proto::host::HostCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostCheckReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::host::check(&name, &mut sender).await?);

        let reply = epp_proto::host::HostCheckReply {
            available: res.avail,
            reason: res.reason,
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_info(
        &self,
        request: tonic::Request<epp_proto::host::HostInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostInfoReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::host::info(&name, &mut sender).await?);

        let mut reply: epp_proto::host::HostInfoReply = res.into();
        reply.cmd_resp = Some(cmd_resp);

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
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::host::create(
                &name,
                addresses,
                request.isnic_info.map(Into::into),
                &mut sender,
            )
            .await?,
        );

        let reply = epp_proto::host::HostCreateReply {
            name: res.name,
            pending: res.pending,
            creation_date: utils::chrono_to_proto(res.creation_date),
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_delete(
        &self,
        request: tonic::Request<epp_proto::host::HostDeleteRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostDeleteReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::host::delete(&name, &mut sender).await?);

        let reply = epp_proto::host::HostDeleteReply {
            pending: res.pending,
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_update(
        &self,
        request: tonic::Request<epp_proto::host::HostUpdateRequest>,
    ) -> Result<tonic::Response<epp_proto::host::HostUpdateReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

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
                    if let Some(s) = host::host_status_from_i32(s) {
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
                    if let Some(s) = host::host_status_from_i32(s) {
                        remove.push(client::host::UpdateObject::Status(s))
                    }
                }
                None => {}
            }
        }

        let (res, cmd_resp) = utils::map_command_response(
            client::host::update(
                &name,
                add,
                remove,
                request.new_name,
                request.isnic_info.map(Into::into),
                &mut sender,
            )
            .await?,
        );

        let reply = epp_proto::host::HostUpdateReply {
            pending: res.pending,
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_check(
        &self,
        request: tonic::Request<epp_proto::contact::ContactCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactCheckReply>, tonic::Status> {
        let request = request.into_inner();
        let id: String = request.id;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::contact::check(&id, &mut sender).await?);

        let reply = epp_proto::contact::ContactCheckReply {
            available: res.avail,
            reason: res.reason,
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_info(
        &self,
        request: tonic::Request<epp_proto::contact::ContactInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactInfoReply>, tonic::Status> {
        let request = request.into_inner();
        let id: String = request.id;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::contact::info(&id, &mut sender).await?);

        let mut reply: epp_proto::contact::ContactInfoReply = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn contact_create(
        &self,
        request: tonic::Request<epp_proto::contact::ContactCreateRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactCreateReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let addr_map = |a: epp_proto::contact::PostalAddress| client::contact::Address {
            name: a.name,
            organisation: a.organisation,
            streets: a.streets,
            city: a.city,
            province: a.province,
            postal_code: a.postal_code,
            country_code: a.country_code,
            identity_number: a.identity_number,
            birth_date: utils::proto_to_chrono(a.birth_date).map(|d| d.date()),
        };

        let (res, cmd_resp) = utils::map_command_response(
            client::contact::create(
                &request.id,
                client::contact::NewContactData {
                    local_address: request.local_address.map(addr_map),
                    internationalised_address: request.internationalised_address.map(addr_map),
                    phone: request.phone.map(|p| p.into()),
                    fax: request.fax.map(|p| p.into()),
                    email: request.email,
                    entity_type: contact::entity_type_from_i32(request.entity_type),
                    trading_name: request.trading_name,
                    company_number: request.company_number,
                    disclosure: request
                        .disclosure
                        .map(|d| contact::disclosure_type_from_i32(d.disclosure)),
                    auth_info: request.auth_info,
                    eurid_info: request.eurid_info.map(Into::into),
                    isnic_info: request.isnic_info.map(Into::into),
                    qualified_lawyer: request.qualified_lawyer.map(Into::into),
                },
                &mut sender,
            )
            .await?,
        );

        let reply = epp_proto::contact::ContactCreateReply {
            id: res.id,
            pending: res.pending,
            creation_date: utils::chrono_to_proto(res.creation_date),
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_delete(
        &self,
        request: tonic::Request<epp_proto::contact::ContactDeleteRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactDeleteReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let (res, cmd_resp) =
            utils::map_command_response(client::contact::delete(&request.id, &mut sender).await?);

        let reply = epp_proto::contact::ContactDeleteReply {
            pending: res.pending,
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_update(
        &self,
        request: tonic::Request<epp_proto::contact::ContactUpdateRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactUpdateReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let addr_map = |a: epp_proto::contact::PostalAddress| client::contact::Address {
            name: a.name,
            organisation: a.organisation,
            streets: a.streets,
            city: a.city,
            province: a.province,
            postal_code: a.postal_code,
            country_code: a.country_code,
            identity_number: a.identity_number,
            birth_date: utils::proto_to_chrono(a.birth_date).map(|d| d.date()),
        };

        let (res, cmd_resp) = utils::map_command_response(
            client::contact::update(
                &request.id,
                contact::contact_status_from_i32(request.add_statuses),
                contact::contact_status_from_i32(request.remove_statuses),
                client::contact::UpdateContactData {
                    local_address: request.new_local_address.map(addr_map),
                    internationalised_address: request.new_internationalised_address.map(addr_map),
                    phone: request.new_phone.map(|p| p.into()),
                    fax: request.new_fax.map(|p| p.into()),
                    email: request.new_email,
                    entity_type: contact::entity_type_from_i32(request.new_entity_type),
                    trading_name: request.new_trading_name,
                    company_number: request.new_company_number,
                    disclosure: request
                        .disclosure
                        .map(|d| contact::disclosure_type_from_i32(d.disclosure)),
                    auth_info: request.new_auth_info,
                    eurid_info: request.new_eurid_info.map(Into::into),
                    isnic_info: request.isnic_info.map(Into::into),
                    qualified_lawyer: request.qualified_lawyer.map(Into::into),
                },
                &mut sender,
            )
            .await?,
        );

        let reply = epp_proto::contact::ContactUpdateReply {
            pending: res.pending,
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_transfer_query(
        &self,
        request: tonic::Request<epp_proto::contact::ContactTransferQueryRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::contact::transfer_query(&request.id, &mut sender).await?,
        );

        let mut reply: epp_proto::contact::ContactTransferReply = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn contact_transfer_request(
        &self,
        request: tonic::Request<epp_proto::contact::ContactTransferRequestRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let (res, cmd_resp) = utils::map_command_response(
            client::contact::transfer_request(&request.id, &request.auth_info, &mut sender).await?,
        );

        let mut reply: epp_proto::contact::ContactTransferReply = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn contact_transfer_accept(
        &self,
        request: tonic::Request<epp_proto::contact::ContactTransferRequestRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::contact::transfer_accept(&request.id, &request.auth_info, &mut sender).await?,
        );

        let mut reply: epp_proto::contact::ContactTransferReply = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn contact_transfer_reject(
        &self,
        request: tonic::Request<epp_proto::contact::ContactTransferRequestRequest>,
    ) -> Result<tonic::Response<epp_proto::contact::ContactTransferReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::contact::transfer_reject(&request.id, &request.auth_info, &mut sender).await?,
        );

        let mut reply: epp_proto::contact::ContactTransferReply = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn maintenance_info(
        &self,
        request: tonic::Request<epp_proto::maintenance::MaintenanceInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::maintenance::MaintenanceInfoReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::maintenance::info(&request.id, &mut sender).await?);

        let mut reply: epp_proto::maintenance::MaintenanceInfoReply = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn maintenance_list(
        &self,
        request: tonic::Request<epp_proto::RegistryInfo>,
    ) -> Result<tonic::Response<epp_proto::maintenance::MaintenanceListReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::maintenance::list(&mut sender).await?);

        let reply = epp_proto::maintenance::MaintenanceListReply {
            items: res
                .items
                .into_iter()
                .map(|i| epp_proto::maintenance::maintenance_list_reply::Item {
                    id: i.id,
                    name: i.name,
                    start: utils::chrono_to_proto(Some(i.start)),
                    end: utils::chrono_to_proto(Some(i.end)),
                    created: utils::chrono_to_proto(Some(i.created)),
                    updated: utils::chrono_to_proto(i.updated),
                })
                .collect(),
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn hit_points_info(
        &self,
        request: tonic::Request<epp_proto::RegistryInfo>,
    ) -> Result<tonic::Response<epp_proto::eurid::HitPointsReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::eurid::hit_points_info(&mut sender).await?);

        let reply = epp_proto::eurid::HitPointsReply {
            hit_points: res.hit_points,
            max_hit_points: res.max_hit_points,
            blocked_until: utils::chrono_to_proto(res.blocked_until),
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn registration_limit_info(
        &self,
        request: tonic::Request<epp_proto::RegistryInfo>,
    ) -> Result<tonic::Response<epp_proto::eurid::RegistrationLimitReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::eurid::registration_limit_info(&mut sender).await?);

        let reply = epp_proto::eurid::RegistrationLimitReply {
            monthly_registrations: res.monthly_registrations,
            max_monthly_registrations: res.max_monthly_registrations,
            limited_until: utils::chrono_to_proto(res.limited_until),
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn dns_quality_info(
        &self,
        request: tonic::Request<epp_proto::eurid::DnsQualityRequest>,
    ) -> Result<tonic::Response<epp_proto::eurid::DnsQualityReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::eurid::dns_quality_info(&request.name, &mut sender).await?,
        );

        let reply = epp_proto::eurid::DnsQualityReply {
            score: res.score,
            check_time: utils::chrono_to_proto(res.check_time),
            registry_name,
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn dnssec_eligibility_info(
        &self,
        request: tonic::Request<epp_proto::eurid::DnssecEligibilityRequest>,
    ) -> Result<tonic::Response<epp_proto::eurid::DnssecEligibilityReply>, tonic::Status> {
        let request = request.into_inner();
        let (mut sender, registry_name) =
            client_by_domain_or_id(&self.client_router, &request.name, request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::eurid::dnssec_eligibility_info(&request.name, &mut sender).await?,
        );

        let reply = epp_proto::eurid::DnssecEligibilityReply {
            eligible: res.eligible,
            message: res.message,
            code: res.code,
            registry_name,
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    type PollStream = futures::channel::mpsc::Receiver<Result<epp_proto::PollReply, tonic::Status>>;

    async fn poll(
        &self,
        request: tonic::Request<tonic::Streaming<epp_proto::PollAck>>,
    ) -> Result<tonic::Response<Self::PollStream>, tonic::Status> {
        let metadata = request.metadata();
        let registry_name = match match metadata.get("registry_name") {
            Some(r) => r.to_str(),
            None => return Err(tonic::Status::invalid_argument("registry name not given")),
        } {
            Ok(r) => r.to_string(),
            Err(_) => return Err(tonic::Status::invalid_argument("invalid registry name")),
        };

        let mut request = request.into_inner();
        let (mut tx, rx) = futures::channel::mpsc::channel(4);
        let mut sender = client_by_id(&self.client_router, &registry_name)?;

        tokio::spawn(async move {
            let mut should_delay = true;
            let mut pending_acks: Vec<_> = vec![];
            loop {
                match client::poll::poll(&mut sender).await {
                    Ok(resp) => {
                        let (resp, cmd_resp) = utils::map_command_response(resp);
                        if let Some(message) = resp {
                            if message.count > 0 {
                                should_delay = false;
                            } else {
                                should_delay = true;
                            }
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
                                } => c,
                                client::poll::PollData::DomainCreateData {
                                    change_data: ref c,
                                    data: _,
                                } => c,
                                client::poll::PollData::DomainPanData {
                                    change_data: ref c,
                                    data: _,
                                } => c,
                                client::poll::PollData::DomainRenewData {
                                    change_data: ref c,
                                    data: _,
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
                            match tx
                                .send(Ok(epp_proto::PollReply {
                                    msg_id: message.id.clone(),
                                    enqueue_date: utils::chrono_to_proto(Some(message.enqueue_time)),
                                    message: message.message,
                                    cmd_resp: Some(cmd_resp),
                                    change_data: change_data.as_ref().map(|c| epp_proto::ChangeData {
                                        change_state: match c.state {
                                            client::poll::ChangeState::After => epp_proto::change_data::ChangeState::After.into(),
                                            client::poll::ChangeState::Before => epp_proto::change_data::ChangeState::Before.into(),
                                        },
                                        operation: Some(epp_proto::change_data::ChangeOperation {
                                            operation_type: match c.operation.op_type {
                                                client::poll::ChangeOperationType::Create => epp_proto::change_data::change_operation::ChangeOperationType::Create.into(),
                                                client::poll::ChangeOperationType::Delete => epp_proto::change_data::change_operation::ChangeOperationType::Delete.into(),
                                                client::poll::ChangeOperationType::Renew => epp_proto::change_data::change_operation::ChangeOperationType::Renew.into(),
                                                client::poll::ChangeOperationType::Transfer => epp_proto::change_data::change_operation::ChangeOperationType::Transfer.into(),
                                                client::poll::ChangeOperationType::Update => epp_proto::change_data::change_operation::ChangeOperationType::Update.into(),
                                                client::poll::ChangeOperationType::Restore => epp_proto::change_data::change_operation::ChangeOperationType::Restore.into(),
                                                client::poll::ChangeOperationType::AutoRenew => epp_proto::change_data::change_operation::ChangeOperationType::AutoRenew.into(),
                                                client::poll::ChangeOperationType::AutoDelete => epp_proto::change_data::change_operation::ChangeOperationType::AutoDelete.into(),
                                                client::poll::ChangeOperationType::AutoPurge => epp_proto::change_data::change_operation::ChangeOperationType::AutoPurge.into(),
                                                client::poll::ChangeOperationType::Custom => epp_proto::change_data::change_operation::ChangeOperationType::Custom.into(),
                                            },
                                            operation: c.operation.operation.clone(),
                                        }),
                                        date: utils::chrono_to_proto(Some(c.date)),
                                        server_transaction_id: c.server_transaction_id.clone(),
                                        who: c.who.clone(),
                                        case_id: c.case_id.as_ref().map(|i| epp_proto::change_data::CaseId {
                                            case_id_type: match i.case_type {
                                                client::poll::ChangeCaseIdType::Udrp => epp_proto::change_data::case_id::CaseIdType::Udrp.into(),
                                                client::poll::ChangeCaseIdType::Urs => epp_proto::change_data::case_id::CaseIdType::Urs.into(),
                                                client::poll::ChangeCaseIdType::Custom => epp_proto::change_data::case_id::CaseIdType::Custom.into(),
                                            },
                                            name: i.name.clone(),
                                            case_id: i.case_id.clone(),
                                        }),
                                        reason: c.reason.clone(),
                                    }),
                                    data: match message.data {
                                        client::poll::PollData::DomainInfoData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::DomainInfo((*i).into())),
                                        client::poll::PollData::ContactInfoData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::ContactInfo((*i).into())),
                                        client::poll::PollData::DomainTransferData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::DomainTransfer(i.into())),
                                        client::poll::PollData::ContactTransferData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::ContactTransfer(i.into())),
                                        client::poll::PollData::DomainCreateData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::DomainCreate(i.into())),
                                        client::poll::PollData::DomainPanData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::DomainPan(i.into())),
                                        client::poll::PollData::ContactPanData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::ContactPan(i.into())),
                                        client::poll::PollData::NominetDomainCancelData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetDomainCancel(i.into())),
                                        client::poll::PollData::NominetDomainReleaseData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetDomainRelease(i.into())),
                                        client::poll::PollData::NominetDomainRegistrarChangeData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetDomainRegistrarChange(i.into())),
                                        client::poll::PollData::NominetHostCancelData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetHostCancel(i.into())),
                                        client::poll::PollData::NominetProcessData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetProcess(i.into())),
                                        client::poll::PollData::NominetSuspendData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetSuspend(i.into())),
                                        client::poll::PollData::NominetDomainFailData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetDomainFail(i.into())),
                                        client::poll::PollData::NominetRegistrantTransferData {
                                            change_data: _,
                                            data: i
                                        } => Some(epp_proto::poll_reply::Data::NominetRegistrantTransfer(i.into())),
                                        client::poll::PollData::VerisignLowBalanceData(i) =>
                                            Some(epp_proto::poll_reply::Data::VerisignLowBalance(i.into())),
                                        client::poll::PollData::TraficomTrnData(i) =>
                                            Some(epp_proto::poll_reply::Data::TraficomTrn(i.into())),
                                        client::poll::PollData::EURIDPoll(i) =>
                                            Some(epp_proto::poll_reply::Data::EuridPoll(i.into())),
                                        _ => None
                                    },
                                }))
                                .await
                            {
                                Ok(_) => {
                                    let msg = if !pending_acks.is_empty() {
                                        pending_acks.pop().unwrap()
                                    } else if let Some(m) = match request.message().await {
                                        Ok(m) => m,
                                        Err(err) => match tx.send(Err(err)).await {
                                            Ok(_) => continue,
                                            Err(_) => break,
                                        },
                                    } {
                                        m
                                    } else {
                                        break;
                                    };
                                    match client::poll::poll_ack(&msg.msg_id, &mut sender).await {
                                        Ok(resp) => {
                                            let (resp, _cmd_resp) = utils::map_command_response(resp);
                                            if let Some(count) = resp.count {
                                                if count > 0 {
                                                    should_delay = false;
                                                } else {
                                                    should_delay = true;
                                                }
                                            } else {
                                                should_delay = true;
                                            }
                                        }
                                        Err(err) => match tx.send(Err(err.into())).await {
                                            Ok(_) => {
                                                pending_acks.push(msg)
                                            }
                                            Err(_) => break,
                                        },
                                    }
                                }
                                Err(_) => break,
                            }
                        } else if tx.is_closed() {
                            break;
                        } else {
                            should_delay = true;
                        }
                    }
                    Err(err) => match tx.send(Err(err.into())).await {
                        Ok(_) => {}
                        Err(_) => break,
                    },
                }
                if should_delay {
                    tokio::time::sleep(tokio::time::Duration::new(15, 0)).await;
                }
            }
        });

        Ok(tonic::Response::new(rx))
    }

    async fn nominet_tag_list(
        &self,
        request: tonic::Request<epp_proto::RegistryInfo>,
    ) -> Result<tonic::Response<epp_proto::nominet::NominetTagListReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let (resp, cmd_resp) =
            utils::map_command_response(client::nominet::tag_list(&mut sender).await?);

        let reply = epp_proto::nominet::NominetTagListReply {
            tags: resp
                .tags
                .into_iter()
                .map(|t| epp_proto::nominet::nominet_tag_list_reply::Tag {
                    tag: t.tag,
                    name: t.name,
                    trading_name: t.trading_name,
                    handshake: t.handshake,
                })
                .collect(),
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn nominet_contact_validate(
        &self,
        request: tonic::Request<epp_proto::nominet::ContactValidateRequest>,
    ) -> Result<tonic::Response<epp_proto::nominet::ContactValidateReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let (_resp, cmd_resp) = utils::map_command_response(
            client::nominet::contact_validate(&request.contact_id, &mut sender).await?,
        );

        let reply = epp_proto::nominet::ContactValidateReply {
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn nominet_lock(
        &self,
        request: tonic::Request<epp_proto::nominet::LockRequest>,
    ) -> Result<tonic::Response<epp_proto::nominet::LockReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let (_resp, cmd_resp) = utils::map_command_response(
            client::nominet::lock(
                match request.object {
                    Some(o) => match o.object {
                        Some(epp_proto::nominet::object::Object::Domain(d)) => {
                            client::nominet::Object::Domain(d)
                        }
                        Some(epp_proto::nominet::object::Object::Registrant(r)) => {
                            client::nominet::Object::Registrant(r)
                        }

                        None => {
                            return Err(tonic::Status::invalid_argument(
                                "release object must be specified",
                            ));
                        }
                    },
                    None => {
                        return Err(tonic::Status::invalid_argument(
                            "release object must be specified",
                        ));
                    }
                },
                &request.lock_type,
                &mut sender,
            )
            .await?,
        );

        let reply = epp_proto::nominet::LockReply {
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn nominet_unlock(
        &self,
        request: tonic::Request<epp_proto::nominet::LockRequest>,
    ) -> Result<tonic::Response<epp_proto::nominet::LockReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let (_resp, cmd_resp) = utils::map_command_response(
            client::nominet::unlock(
                match request.object {
                    Some(o) => match o.object {
                        Some(epp_proto::nominet::object::Object::Domain(d)) => {
                            client::nominet::Object::Domain(d)
                        }
                        Some(epp_proto::nominet::object::Object::Registrant(r)) => {
                            client::nominet::Object::Registrant(r)
                        }

                        None => {
                            return Err(tonic::Status::invalid_argument(
                                "release object must be specified",
                            ));
                        }
                    },
                    None => {
                        return Err(tonic::Status::invalid_argument(
                            "release object must be specified",
                        ));
                    }
                },
                &request.lock_type,
                &mut sender,
            )
            .await?,
        );

        let reply = epp_proto::nominet::LockReply {
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn nominet_accept(
        &self,
        request: tonic::Request<epp_proto::nominet::HandshakeAcceptRequest>,
    ) -> Result<tonic::Response<epp_proto::nominet::HandshakeReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let (resp, cmd_resp) = utils::map_command_response(
            client::nominet::handshake_accept(
                &request.case_id,
                request.registrant.as_deref(),
                &mut sender,
            )
            .await?,
        );

        let mut reply: epp_proto::nominet::HandshakeReply = resp.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn nominet_reject(
        &self,
        request: tonic::Request<epp_proto::nominet::HandshakeRejectRequest>,
    ) -> Result<tonic::Response<epp_proto::nominet::HandshakeReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let (resp, cmd_resp) = utils::map_command_response(
            client::nominet::handshake_reject(&request.case_id, &mut sender).await?,
        );

        let mut reply: epp_proto::nominet::HandshakeReply = resp.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn nominet_release(
        &self,
        request: tonic::Request<epp_proto::nominet::ReleaseRequest>,
    ) -> Result<tonic::Response<epp_proto::nominet::ReleaseReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let (resp, cmd_resp) = utils::map_command_response(
            client::nominet::release(
                &request.registrar_tag,
                match request.object {
                    Some(o) => match o.object {
                        Some(epp_proto::nominet::object::Object::Domain(d)) => {
                            client::nominet::Object::Domain(d)
                        }
                        Some(epp_proto::nominet::object::Object::Registrant(r)) => {
                            client::nominet::Object::Registrant(r)
                        }

                        None => {
                            return Err(tonic::Status::invalid_argument(
                                "release object must be specified",
                            ));
                        }
                    },
                    None => {
                        return Err(tonic::Status::invalid_argument(
                            "release object must be specified",
                        ));
                    }
                },
                &mut sender,
            )
            .await?,
        );

        let reply = epp_proto::nominet::ReleaseReply {
            pending: resp.pending,
            message: resp.message,
            cmd_resp: Some(cmd_resp),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn balance_info(
        &self,
        request: tonic::Request<epp_proto::RegistryInfo>,
    ) -> Result<tonic::Response<epp_proto::BalanceReply>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let (resp, cmd_resp) =
            utils::map_command_response(client::balance::balance_info(&mut sender).await?);

        let mut reply: epp_proto::BalanceReply = resp.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn tmch_mark_check(
        &self,
        request: tonic::Request<epp_proto::tmch::MarkCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::tmch::MarkCheckResponse>, tonic::Status> {
        let request = request.into_inner();
        let id: String = request.id;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::tmch::check(&id, &mut sender).await?);

        let mut reply: epp_proto::tmch::MarkCheckResponse = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn tmch_mark_create(
        &self,
        request: tonic::Request<epp_proto::tmch::MarkCreateRequest>,
    ) -> Result<tonic::Response<epp_proto::tmch::MarkCreateResponse>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::tmch::create(
                match request.mark {
                    Some(m) => m.try_into()?,
                    None => return Err(tonic::Status::invalid_argument("mark must be specified")),
                },
                request.period.map(Into::into),
                request.documents.into_iter().map(Into::into).collect(),
                request.labels.into_iter().map(Into::into).collect(),
                request.variations,
                &mut sender,
            )
            .await?,
        );

        let mut reply: epp_proto::tmch::MarkCreateResponse = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn tmch_mark_info(
        &self,
        request: tonic::Request<epp_proto::tmch::MarkInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::tmch::MarkInfoResponse>, tonic::Status> {
        let request = request.into_inner();
        let id: String = request.id;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::tmch::mark_info(&id, &mut sender).await?);

        let mut reply: epp_proto::tmch::MarkInfoResponse = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn tmch_mark_smd_info(
        &self,
        request: tonic::Request<epp_proto::tmch::MarkInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::tmch::MarkSmdInfoResponse>, tonic::Status> {
        let request = request.into_inner();
        let id: String = request.id;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::tmch::mark_smd_info(&id, &mut sender).await?);

        let mut reply: epp_proto::tmch::MarkSmdInfoResponse = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn tmch_mark_encoded_smd_info(
        &self,
        request: tonic::Request<epp_proto::tmch::MarkInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::tmch::MarkSmdInfoResponse>, tonic::Status> {
        let request = request.into_inner();
        let id: String = request.id;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::tmch::mark_encoded_smd_info(&id, &mut sender).await?,
        );

        let mut reply: epp_proto::tmch::MarkSmdInfoResponse = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn tmch_mark_file_info(
        &self,
        request: tonic::Request<epp_proto::tmch::MarkInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::tmch::MarkSmdInfoResponse>, tonic::Status> {
        let request = request.into_inner();
        let id: String = request.id;
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) =
            utils::map_command_response(client::tmch::mark_file_info(&id, &mut sender).await?);

        let mut reply: epp_proto::tmch::MarkSmdInfoResponse = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn tmch_mark_update(
        &self,
        request: tonic::Request<epp_proto::tmch::MarkUpdateRequest>,
    ) -> Result<tonic::Response<epp_proto::tmch::MarkUpdateResponse>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let (res, cmd_resp) = utils::map_command_response(
            client::tmch::update(
                &request.id,
                request
                    .add
                    .into_iter()
                    .filter_map(|a| match a.update {
                        Some(epp_proto::tmch::mark_update_add::Update::Document(d)) => {
                            Some(Ok(client::tmch::UpdateAdd::Document(d.into())))
                        }
                        Some(epp_proto::tmch::mark_update_add::Update::Label(l)) => {
                            Some(Ok(client::tmch::UpdateAdd::Label(l.into())))
                        }
                        Some(epp_proto::tmch::mark_update_add::Update::Variation(v)) => {
                            Some(Ok(client::tmch::UpdateAdd::Variation(v)))
                        }
                        Some(epp_proto::tmch::mark_update_add::Update::Case(c)) => {
                            match c.try_into() {
                                Ok(c) => Some(Ok(client::tmch::UpdateAdd::Case(c))),
                                Err(e) => Some(Err(e)),
                            }
                        }
                        None => None,
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                request
                    .remove
                    .into_iter()
                    .filter_map(|a| match a.update {
                        Some(epp_proto::tmch::mark_update_remove::Update::Label(l)) => {
                            Some(client::tmch::UpdateRemove::Label(l))
                        }
                        Some(epp_proto::tmch::mark_update_remove::Update::Variation(v)) => {
                            Some(client::tmch::UpdateRemove::Variation(v))
                        }
                        None => None,
                    })
                    .collect(),
                match request.new_mark {
                    Some(m) => Some(m.try_into()?),
                    None => None,
                },
                request.update_labels.into_iter().map(Into::into).collect(),
                request.update_cases.into_iter().map(Into::into).collect(),
                &mut sender,
            )
            .await?,
        );

        let mut reply: epp_proto::tmch::MarkUpdateResponse = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn tmch_mark_renew(
        &self,
        request: tonic::Request<epp_proto::tmch::MarkRenewRequest>,
    ) -> Result<tonic::Response<epp_proto::tmch::MarkRenewResponse>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;

        let cur_expiry_date = utils::proto_to_chrono(request.current_expiry_date);
        if cur_expiry_date.is_none() {
            return Err(tonic::Status::invalid_argument(
                "current_expiry_date must be specified",
            ));
        }

        let (res, cmd_resp) = utils::map_command_response(
            client::tmch::renew(
                &request.id,
                cur_expiry_date.unwrap(),
                request.add_period.map(Into::into),
                &mut sender,
            )
            .await?,
        );

        let mut reply: epp_proto::tmch::MarkRenewResponse = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn tmch_mark_transfer_initiate(
        &self,
        request: tonic::Request<epp_proto::tmch::MarkTransferInitiateRequest>,
    ) -> Result<tonic::Response<epp_proto::tmch::MarkTransferInitiateResponse>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::tmch::transfer_initiate(&request.id, &mut sender).await?,
        );

        let mut reply: epp_proto::tmch::MarkTransferInitiateResponse = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn tmch_mark_transfer(
        &self,
        request: tonic::Request<epp_proto::tmch::MarkTransferRequest>,
    ) -> Result<tonic::Response<epp_proto::tmch::MarkTransferResponse>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, cmd_resp) = utils::map_command_response(
            client::tmch::transfer(&request.id, &request.auth_info, &mut sender).await?,
        );

        let mut reply: epp_proto::tmch::MarkTransferResponse = res.into();
        reply.cmd_resp = Some(cmd_resp);

        Ok(tonic::Response::new(reply))
    }

    async fn dac_domain(
        &self,
        request: tonic::Request<epp_proto::dac::DomainRequest>,
    ) -> Result<tonic::Response<epp_proto::dac::DomainResponse>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, _) = utils::map_command_response(
            client::dac::domain(
                &request.name,
                match dac::env_from_i32(request.environment) {
                    Some(e) => e,
                    None => {
                        return Err(tonic::Status::invalid_argument("unknown environment"));
                    }
                },
                &mut sender,
            )
            .await?,
        );

        let reply: epp_proto::dac::DomainResponse = res.into();
        Ok(tonic::Response::new(reply))
    }

    async fn dac_usage(
        &self,
        request: tonic::Request<epp_proto::dac::UsageRequest>,
    ) -> Result<tonic::Response<epp_proto::dac::UsageResponse>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, _) = utils::map_command_response(
            client::dac::usage(
                match dac::env_from_i32(request.environment) {
                    Some(e) => e,
                    None => {
                        return Err(tonic::Status::invalid_argument("unknown environment"));
                    }
                },
                &mut sender,
            )
            .await?,
        );

        let reply: epp_proto::dac::UsageResponse = res.into();
        Ok(tonic::Response::new(reply))
    }

    async fn dac_limits(
        &self,
        request: tonic::Request<epp_proto::dac::UsageRequest>,
    ) -> Result<tonic::Response<epp_proto::dac::UsageResponse>, tonic::Status> {
        let request = request.into_inner();
        let mut sender = client_by_id(&self.client_router, &request.registry_name)?;
        let (res, _) = utils::map_command_response(
            client::dac::limits(
                match dac::env_from_i32(request.environment) {
                    Some(e) => e,
                    None => {
                        return Err(tonic::Status::invalid_argument("unknown environment"));
                    }
                },
                &mut sender,
            )
            .await?,
        );

        let reply: epp_proto::dac::UsageResponse = res.into();
        Ok(tonic::Response::new(reply))
    }
}
