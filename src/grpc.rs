//! Implements the gRPC interface for the EPP client

use super::client;

pub mod epp_proto {
    tonic::include_proto!("epp");
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

#[derive(Debug)]
pub struct EPPProxy {
    pub client_sender: futures::channel::mpsc::Sender<client::Request>,
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

#[tonic::async_trait]
impl epp_proto::epp_proxy_server::EppProxy for EPPProxy {
    async fn domain_check(
        &self,
        request: tonic::Request<epp_proto::DomainCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::DomainCheckReply>, tonic::Status> {
        let name: String = request.into_inner().name;
        let mut sender = self.client_sender.clone();
        let res = client::domain::check(&name, &mut sender).await?;

        let reply = epp_proto::DomainCheckReply {
            available: res.avail,
            reason: res.reason.unwrap_or("".to_string()),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn domain_info(
        &self,
        request: tonic::Request<epp_proto::DomainInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::DomainInfoReply>, tonic::Status> {
        let name: String = request.into_inner().name;
        let mut sender = self.client_sender.clone();
        let res = client::domain::info(&name, &mut sender).await?;

        let reply = epp_proto::DomainInfoReply {
            name: res.name,
            registry_id: res.registry_id,
            statuses: res.statuses,
            registrant: res.registrant,
            contacts: res
                .contacts
                .into_iter()
                .map(|c| epp_proto::domain_info_reply::Contact {
                    id: c.contact_id,
                    r#type: c.contact_type,
                })
                .collect(),
            nameservers: res
                .nameservers
                .into_iter()
                .map(|n| match n {
                    client::domain::InfoNameserver::HostOnly(h) => {
                        epp_proto::domain_info_reply::NameServer {
                            host: h,
                            address: None,
                        }
                    }
                    client::domain::InfoNameserver::HostAndAddress {
                        host,
                        address,
                        ip_version,
                    } => epp_proto::domain_info_reply::NameServer {
                        host,
                        address: Some(epp_proto::Address {
                            address,
                            r#type: match ip_version {
                                client::domain::InfoNameserverAddressVersion::IPv4 => {
                                    epp_proto::IpVersion::IPv4.into()
                                }
                                client::domain::InfoNameserverAddressVersion::IPv6 => {
                                    epp_proto::IpVersion::IPv6.into()
                                }
                            },
                        }),
                    },
                })
                .collect(),
            hosts: res.hosts,
            client_id: res.client_id,
            client_created_id: res.client_created_id.unwrap_or("".to_string()),
            creation_date: chrono_to_proto(res.creation_date),
            expiry_date: chrono_to_proto(res.expiry_date),
            last_updated_client: res.last_updated_client.unwrap_or("".to_string()),
            last_updated_date: chrono_to_proto(res.last_updated_date),
            last_transfer_date: chrono_to_proto(res.last_transfer_date),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_check(
        &self,
        request: tonic::Request<epp_proto::HostCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::HostCheckReply>, tonic::Status> {
        let name: String = request.into_inner().name;
        let mut sender = self.client_sender.clone();
        let res = client::host::check(&name, &mut sender).await?;

        let reply = epp_proto::HostCheckReply {
            available: res.avail,
            reason: res.reason.unwrap_or("".to_string()),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_info(
        &self,
        request: tonic::Request<epp_proto::HostInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::HostInfoReply>, tonic::Status> {
        let name: String = request.into_inner().name;
        let mut sender = self.client_sender.clone();
        let res = client::host::info(&name, &mut sender).await?;

        let reply = epp_proto::HostInfoReply {
            name: res.name,
            registry_id: res.registry_id,
            statuses: res.statuses,
            addresses: res
                .addresses
                .into_iter()
                .map(|a| epp_proto::Address {
                    address: a.address,
                    r#type: match a.ip_version {
                        client::host::AddressVersion::IPv4 => epp_proto::IpVersion::IPv4.into(),
                        client::host::AddressVersion::IPv6 => epp_proto::IpVersion::IPv6.into(),
                    },
                })
                .collect(),
            client_id: res.client_id,
            client_created_id: res.client_created_id.unwrap_or("".to_string()),
            creation_date: chrono_to_proto(res.creation_date),
            last_updated_client: res.last_updated_client.unwrap_or("".to_string()),
            last_updated_date: chrono_to_proto(res.last_updated_date),
            last_transfer_date: chrono_to_proto(res.last_transfer_date),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_create(
        &self,
        request: tonic::Request<epp_proto::HostCreateRequest>,
    ) -> Result<tonic::Response<epp_proto::HostCreateReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut addresses: Vec<Result<client::host::Address, tonic::Status>> = request
            .addresses
            .into_iter()
            .map(|a| {
                Ok(client::host::Address {
                    address: a.address,
                    ip_version: match epp_proto::IpVersion::from_i32(a.r#type) {
                        Some(epp_proto::IpVersion::IPv4) => client::host::AddressVersion::IPv4,
                        Some(epp_proto::IpVersion::IPv6) => client::host::AddressVersion::IPv6,
                        None | Some(epp_proto::IpVersion::Unknown) => {
                            return Err(tonic::Status::invalid_argument(
                                "unknown IP address version",
                            ));
                        }
                    },
                })
            })
            .collect();
        let mut clean_addresses = vec![];
        for a in addresses.drain(..) {
            match a {
                Ok(a) => clean_addresses.push(a),
                Err(e) => return Err(e),
            }
        }
        let mut sender = self.client_sender.clone();
        let res = client::host::create(&name, clean_addresses, &mut sender).await?;

        let reply = epp_proto::HostCreateReply {
            pending: res.pending,
            creation_date: chrono_to_proto(res.creation_date),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_delete(
        &self,
        request: tonic::Request<epp_proto::HostDeleteRequest>,
    ) -> Result<tonic::Response<epp_proto::HostDeleteReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = self.client_sender.clone();
        let res = client::host::delete(&name, &mut sender).await?;

        let reply = epp_proto::HostDeleteReply {
            pending: res.pending,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn host_update(
        &self,
        request: tonic::Request<epp_proto::HostUpdateRequest>,
    ) -> Result<tonic::Response<epp_proto::HostUpdateReply>, tonic::Status> {
        let request = request.into_inner();
        let name: String = request.name;
        let mut sender = self.client_sender.clone();

        let mut add = vec![];
        let mut remove = vec![];
        let new_name = if request.new_name.len() > 0 {
            Some(request.new_name)
        } else {
            None
        };

        for a in request.add {
            match a.param {
                Some(epp_proto::host_update_request::param::Param::Address(addr)) => {
                    add.push(client::host::UpdateObject::Address(client::host::Address {
                        address: addr.address,
                        ip_version: match epp_proto::IpVersion::from_i32(addr.r#type) {
                            Some(epp_proto::IpVersion::IPv4) => client::host::AddressVersion::IPv4,
                            Some(epp_proto::IpVersion::IPv6) => client::host::AddressVersion::IPv6,
                            None | Some(epp_proto::IpVersion::Unknown) => {
                                return Err(tonic::Status::invalid_argument(
                                    "unknown IP address version",
                                ));
                            }
                        },
                    }))
                }
                Some(epp_proto::host_update_request::param::Param::State(s)) => {
                    add.push(client::host::UpdateObject::Status(s))
                }
                None => {}
            }
        }
        for r in request.remove {
            match r.param {
                Some(epp_proto::host_update_request::param::Param::Address(addr)) => {
                    remove.push(client::host::UpdateObject::Address(client::host::Address {
                        address: addr.address,
                        ip_version: match epp_proto::IpVersion::from_i32(addr.r#type) {
                            Some(epp_proto::IpVersion::IPv4) => client::host::AddressVersion::IPv4,
                            Some(epp_proto::IpVersion::IPv6) => client::host::AddressVersion::IPv6,
                            None | Some(epp_proto::IpVersion::Unknown) => {
                                return Err(tonic::Status::invalid_argument(
                                    "unknown IP address version",
                                ));
                            }
                        },
                    }))
                }
                Some(epp_proto::host_update_request::param::Param::State(s)) => {
                    remove.push(client::host::UpdateObject::Status(s))
                }
                None => {}
            }
        }

        let res = client::host::update(&name, add, remove, new_name, &mut sender).await?;

        let reply = epp_proto::HostUpdateReply {
            pending: res.pending,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_check(
        &self,
        request: tonic::Request<epp_proto::ContactCheckRequest>,
    ) -> Result<tonic::Response<epp_proto::ContactCheckReply>, tonic::Status> {
        let id: String = request.into_inner().id;
        let mut sender = self.client_sender.clone();
        let res = client::contact::check(&id, &mut sender).await?;

        let reply = epp_proto::ContactCheckReply {
            available: res.avail,
            reason: res.reason.unwrap_or("".to_string()),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn contact_info(
        &self,
        request: tonic::Request<epp_proto::ContactInfoRequest>,
    ) -> Result<tonic::Response<epp_proto::ContactInfoReply>, tonic::Status> {
        let id: String = request.into_inner().id;
        let mut sender = self.client_sender.clone();
        let res = client::contact::info(&id, &mut sender).await?;

        let map_addr = |a: client::contact::Address| epp_proto::contact_info_reply::Address {
            name: a.name,
            organisation: a.organisation.unwrap_or("".to_string()),
            streets: a.streets,
            city: a.city,
            province: a.province.unwrap_or("".to_string()),
            postal_code: a.postal_code.unwrap_or("".to_string()),
            country_code: a.country_code,
        };

        let reply = epp_proto::ContactInfoReply {
            id: res.id,
            registry_id: res.registry_id,
            statuses: res.statuses,
            local_address: res.local_address.map(map_addr),
            internationalised_address: res.internationalised_addresses.map(map_addr),
            phone: res.phone.unwrap_or("".to_string()),
            fax: res.fax.unwrap_or("".to_string()),
            email: res.email,
            client_id: res.client_id,
            client_created_id: res.client_created_id.unwrap_or("".to_string()),
            creation_date: chrono_to_proto(res.creation_date),
            last_updated_client: res.last_updated_client.unwrap_or("".to_string()),
            last_updated_date: chrono_to_proto(res.last_updated_date),
            last_transfer_date: chrono_to_proto(res.last_transfer_date),
        };

        Ok(tonic::Response::new(reply))
    }
}
