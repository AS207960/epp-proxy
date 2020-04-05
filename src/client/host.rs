//! EPP commands relating to host (nameserver) objects

use super::{proto, EPPClientServerFeatures, Request, Response, Sender};
use chrono::prelude::*;

#[derive(Debug)]
pub struct CheckRequest {
    name: String,
    pub return_path: Sender<CheckResponse>,
}

#[derive(Debug)]
pub struct CheckResponse {
    pub avail: bool,
    pub reason: Option<String>,
}

#[derive(Debug)]
pub struct InfoRequest {
    name: String,
    pub return_path: Sender<InfoResponse>,
}

#[derive(Debug)]
pub struct InfoResponse {
    pub name: String,
    pub registry_id: String,
    pub statuses: Vec<String>,
    pub addresses: Vec<Address>,
    pub client_id: String,
    pub client_created_id: Option<String>,
    pub creation_date: Option<DateTime<Utc>>,
    pub last_updated_client: Option<String>,
    pub last_updated_date: Option<DateTime<Utc>>,
    pub last_transfer_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct Address {
    pub address: String,
    pub ip_version: AddressVersion,
}

#[derive(Debug)]
pub enum AddressVersion {
    IPv4,
    IPv6,
}

#[derive(Debug)]
pub struct CreateRequest {
    name: String,
    addresses: Vec<Address>,
    pub return_path: Sender<CreateResponse>,
}

#[derive(Debug)]
pub struct CreateResponse {
    pub pending: bool,
    pub creation_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct DeleteRequest {
    name: String,
    pub return_path: Sender<DeleteResponse>,
}

#[derive(Debug)]
pub struct DeleteResponse {
    pub pending: bool,
}

#[derive(Debug)]
pub struct UpdateRequest {
    name: String,
    add: Vec<UpdateObject>,
    remove: Vec<UpdateObject>,
    new_name: Option<String>,
    pub return_path: Sender<UpdateResponse>,
}

#[derive(Debug)]
pub enum UpdateObject {
    Address(Address),
    Status(String),
}

#[derive(Debug)]
pub struct UpdateResponse {
    pub pending: bool,
}

pub fn handle_check(
    client: &EPPClientServerFeatures,
    req: &CheckRequest,
) -> Result<proto::EPPCommandType, Response<CheckResponse>> {
    if !client.host_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err("host name has a min length of 1".to_string()));
    }
    let command = proto::EPPCheck::Host(proto::host::EPPHostCheck {
        name: req.name.clone(),
    });
    Ok(proto::EPPCommandType::Check(command))
}

pub fn handle_check_response(response: proto::EPPResponse) -> Response<CheckResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPHostCheckResult(host_check) => {
                if let Some(host_check) = host_check.data.first() {
                    Response::Ok(CheckResponse {
                        avail: host_check.name.available,
                        reason: host_check.reason.to_owned(),
                    })
                } else {
                    Response::InternalServerError
                }
            }
            _ => Response::InternalServerError,
        },
        None => Response::InternalServerError,
    }
}

pub fn handle_info(
    client: &EPPClientServerFeatures,
    req: &InfoRequest,
) -> Result<proto::EPPCommandType, Response<InfoResponse>> {
    if !client.host_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err("host name has a min length of 1".to_string()));
    }
    let command = proto::EPPInfo::Host(proto::host::EPPHostCheck {
        name: req.name.clone(),
    });
    Ok(proto::EPPCommandType::Info(command))
}

pub fn handle_info_response(response: proto::EPPResponse) -> Response<InfoResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPHostInfoResult(host_info) => Response::Ok(InfoResponse {
                name: host_info.name,
                registry_id: host_info.registry_id,
                statuses: host_info.statuses.into_iter().map(|s| s.status).collect(),
                addresses: host_info
                    .addresses
                    .into_iter()
                    .map(|a| Address {
                        address: a.address,
                        ip_version: match a.ip_version {
                            proto::host::EPPHostAddressVersion::IPv4 => AddressVersion::IPv4,
                            proto::host::EPPHostAddressVersion::IPv6 => AddressVersion::IPv6,
                        },
                    })
                    .collect(),
                client_id: host_info.client_id,
                client_created_id: host_info.client_created_id,
                creation_date: host_info.creation_date,
                last_updated_client: host_info.last_updated_client,
                last_updated_date: host_info.last_updated_date,
                last_transfer_date: host_info.last_transfer_date,
            }),
            _ => Response::InternalServerError,
        },
        None => Response::InternalServerError,
    }
}

pub fn handle_create(
    client: &EPPClientServerFeatures,
    req: &CreateRequest,
) -> Result<proto::EPPCommandType, Response<CreateResponse>> {
    if !client.host_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err("host name has a min length of 1".to_string()));
    }
    let command = proto::EPPCreate::Host(proto::host::EPPHostCreate {
        name: req.name.clone(),
        addresses: match req
            .addresses
            .iter()
            .map(|a| {
                Ok(proto::host::EPPHostAddressSer {
                    address: if let 3..=45 = a.address.len() {
                        a.address.clone()
                    } else {
                        return Err(Response::Err(
                            "address has a min length of 3 and a max length of 45".to_string(),
                        ));
                    },
                    ip_version: match a.ip_version {
                        AddressVersion::IPv4 => proto::host::EPPHostAddressVersion::IPv4,
                        AddressVersion::IPv6 => proto::host::EPPHostAddressVersion::IPv6,
                    },
                })
            })
            .collect()
        {
            Ok(a) => a,
            Err(e) => return Err(e),
        },
    });
    Ok(proto::EPPCommandType::Create(command))
}

pub fn handle_create_response(response: proto::EPPResponse) -> Response<CreateResponse> {
    match response.data {
        Some(ref value) => match &value.value {
            proto::EPPResultDataValue::EPPHostCreateResult(host_create) => {
                Response::Ok(CreateResponse {
                    pending: response.is_pending(),
                    creation_date: host_create.creation_date,
                })
            }
            _ => Response::InternalServerError,
        },
        None => Response::InternalServerError,
    }
}

pub fn handle_delete(
    client: &EPPClientServerFeatures,
    req: &DeleteRequest,
) -> Result<proto::EPPCommandType, Response<DeleteResponse>> {
    if !client.host_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err("host name has a min length of 1".to_string()));
    }
    let command = proto::EPPDelete::Host(proto::host::EPPHostDelete {
        name: req.name.clone(),
    });
    Ok(proto::EPPCommandType::Delete(command))
}

pub fn handle_delete_response(response: proto::EPPResponse) -> Response<DeleteResponse> {
    Response::Ok(DeleteResponse {
        pending: response.is_pending(),
    })
}

pub fn handle_update(
    client: &EPPClientServerFeatures,
    req: &UpdateRequest,
) -> Result<proto::EPPCommandType, Response<UpdateResponse>> {
    if !client.host_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err("host name has a min length of 1".to_string()));
    }
    if req.add.is_empty() && req.remove.is_empty() && req.new_name.is_none() {
        return Err(Response::Err(
            "at least one operation must be specified".to_string(),
        ));
    }
    match &req.new_name {
        Some(n) => {
            if n.is_empty() {
                return Err(Response::Err(
                    "new host name has a min length of 1".to_string(),
                ));
            }
        }
        None => {}
    }
    let map_obj = |a: &UpdateObject| {
        Ok(match a {
            UpdateObject::Address(addr) => {
                proto::host::EPPHostUpdateParam::Address(proto::host::EPPHostAddressSer {
                    address: if let 3..=45 = addr.address.len() {
                        addr.address.clone()
                    } else {
                        return Err(Response::Err(
                            "address has a min length of 3 and a max length of 45".to_string(),
                        ));
                    },
                    ip_version: match addr.ip_version {
                        AddressVersion::IPv4 => proto::host::EPPHostAddressVersion::IPv4,
                        AddressVersion::IPv6 => proto::host::EPPHostAddressVersion::IPv6,
                    },
                })
            }
            UpdateObject::Status(s) => {
                proto::host::EPPHostUpdateParam::Status(proto::host::EPPHostStatusSer {
                    status: s.to_string(),
                })
            }
        })
    };
    let command = proto::EPPUpdate::Host(proto::host::EPPHostUpdate {
        name: req.name.clone(),
        add: match req.add.len() {
            0 => None,
            _ => Some(proto::host::EPPHostUpdateAdd {
                params: req
                    .add
                    .iter()
                    .map(map_obj)
                    .collect::<Result<_, Response<UpdateResponse>>>()?,
            }),
        },
        remove: match req.remove.len() {
            0 => None,
            _ => Some(proto::host::EPPHostUpdateRemove {
                params: req
                    .remove
                    .iter()
                    .map(map_obj)
                    .collect::<Result<_, Response<UpdateResponse>>>()?,
            }),
        },
        change: req
            .new_name
            .as_ref()
            .map(|n| proto::host::EPPHostUpdateChange { name: n.clone() }),
    });
    Ok(proto::EPPCommandType::Update(command))
}

pub fn handle_update_response(response: proto::EPPResponse) -> Response<UpdateResponse> {
    Response::Ok(UpdateResponse {
        pending: response.is_pending(),
    })
}

pub async fn check(
    host: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CheckResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::HostCheck(CheckRequest {
            name: host.to_string(),
            return_path: sender,
        }),
        receiver,
    )
    .await
}

pub async fn info(
    host: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<InfoResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::HostInfo(InfoRequest {
            name: host.to_string(),
            return_path: sender,
        }),
        receiver,
    )
    .await
}

pub async fn create(
    host: &str,
    addresses: Vec<Address>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CreateResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::HostCreate(CreateRequest {
            name: host.to_string(),
            addresses,
            return_path: sender,
        }),
        receiver,
    )
    .await
}

pub async fn delete(
    host: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<DeleteResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::HostDelete(DeleteRequest {
            name: host.to_string(),
            return_path: sender,
        }),
        receiver,
    )
    .await
}

pub async fn update<N: Into<Option<String>>>(
    host: &str,
    add: Vec<UpdateObject>,
    remove: Vec<UpdateObject>,
    new_name: N,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<UpdateResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::HostUpdate(UpdateRequest {
            name: host.to_string(),
            add,
            remove,
            new_name: new_name.into(),
            return_path: sender,
        }),
        receiver,
    )
    .await
}
