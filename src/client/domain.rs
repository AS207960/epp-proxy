use chrono::prelude::*;
use super::{proto, Request, Response, Sender, EPPClientServerFeatures};

#[derive(Debug)]
pub struct CheckRequest {
    name: String,
    pub return_path: Sender<CheckResponse>
}

#[derive(Debug)]
pub struct InfoRequest {
    name: String,
    pub return_path: Sender<InfoResponse>
}

#[derive(Debug)]
pub struct CheckResponse {
    pub avail: bool,
    pub reason: Option<String>
}

#[derive(Debug)]
pub struct InfoResponse {
    pub name: String,
    pub registry_id: String,
    pub statuses: Vec<String>,
    pub registrant: String,
    pub contacts: Vec<InfoContact>,
    pub nameservers: Vec<InfoNameserver>,
    pub hosts: Vec<String>,
    pub client_id: String,
    pub client_created_id: Option<String>,
    pub creation_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub last_updated_client: Option<String>,
    pub last_updated_date: Option<DateTime<Utc>>,
    pub last_transfer_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct InfoContact {
    pub contact_type: String,
    pub contact_id: String,
}

#[derive(Debug)]
pub enum InfoNameserver {
    HostOnly(String),
    HostAndAddress {
        host: String,
        address: String,
        ip_version: InfoNameserverAddressVersion
    }
}

#[derive(Debug)]
pub enum InfoNameserverAddressVersion {
    IPv4,
    IPv6
}

pub fn handle_check(client: &EPPClientServerFeatures, req: &CheckRequest) -> Result<proto::EPPCommandType, Response<CheckResponse>> {
    if !client.domain_supported {
        return Err(Response::Unsupported)
    }
    if req.name.len() < 1 {
        return Err(Response::Err("domain name has a min length of 1".to_string()))
    }
    let command = proto::EPPCheck::Domain(proto::domain::EPPDomainCheck {
        name: req.name.clone()
    });
    Ok(proto::EPPCommandType::Check(command))
}

pub fn handle_check_response(response: proto::EPPResponse) -> Response<CheckResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPDomainCheckResult(domain_check) => {
                if let Some(domain_check) = domain_check.data.first() {
                    Response::Ok(CheckResponse {
                        avail: domain_check.name.available,
                        reason: domain_check.reason.to_owned()
                    })
                } else {
                    Response::InternalServerError
                }
            },
            _ => Response::InternalServerError
        },
        None => {
            Response::InternalServerError
        }
    }
}

pub fn handle_info(client: &EPPClientServerFeatures, req: &InfoRequest) -> Result<proto::EPPCommandType, Response<InfoResponse>> {
    if !client.domain_supported {
        return Err(Response::Unsupported)
    }
    if req.name.len() < 1 {
        return Err(Response::Err("domain name has a min length of 1".to_string()))
    }
    let command = proto::EPPInfo::Domain(proto::domain::EPPDomainCheck {
        name: req.name.clone()
    });
    Ok(proto::EPPCommandType::Info(command))
}

pub fn handle_info_response(response: proto::EPPResponse) -> Response<InfoResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPDomainInfoResult(domain_info) => {
                Response::Ok(InfoResponse {
                    name: domain_info.name,
                    registry_id: domain_info.registry_id,
                    statuses: domain_info.statuses.into_iter().map(|s| s.status).collect(),
                    registrant: domain_info.registrant,
                    contacts: domain_info.contacts.into_iter().map(|c| InfoContact {
                        contact_id: c.contact_id,
                        contact_type: c.contact_type
                    }).collect(),
                    nameservers: domain_info.nameservers.servers.into_iter().map(|s| match s {
                        proto::domain::EPPDomainInfoNameserver::HostOnly(h) => InfoNameserver::HostOnly(h),
                        proto::domain::EPPDomainInfoNameserver::HostAndAddress {
                            host, address
                        } => InfoNameserver::HostAndAddress {
                            host,
                            address: address.address,
                            ip_version: match address.ip_version {
                                proto::domain::EPPDomainInfoNameserverAddressVersion::IPv4 => InfoNameserverAddressVersion::IPv4,
                                proto::domain::EPPDomainInfoNameserverAddressVersion::IPv6 => InfoNameserverAddressVersion::IPv6
                            }
                        },
                    }).collect(),
                    hosts: domain_info.hosts,
                    client_id: domain_info.client_id,
                    client_created_id: domain_info.client_created_id,
                    creation_date: domain_info.creation_date,
                    expiry_date: domain_info.expiry_date,
                    last_updated_client: domain_info.last_updated_client,
                    last_updated_date: domain_info.last_updated_date,
                    last_transfer_date: domain_info.last_transfer_date,
                })
            },
            _ => Response::InternalServerError
        },
        None => {
            Response::InternalServerError
        }
    }
}

pub async fn check(domain: &str, client_sender: &mut futures::channel::mpsc::Sender<Request>) -> Result<CheckResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(client_sender, Request::DomainCheck(CheckRequest {
        name: domain.to_string(),
        return_path: sender
    }), receiver).await
}

pub async fn info(domain: &str, client_sender: &mut futures::channel::mpsc::Sender<Request>) -> Result<InfoResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(client_sender, Request::DomainInfo(InfoRequest {
        name: domain.to_string(),
        return_path: sender
    }), receiver).await
}