//! EPP commands relating to host (nameserver) objects

use super::super::host::{
    Address, AddressVersion, CheckRequest, CheckResponse, CreateRequest, CreateResponse,
    DeleteRequest, DeleteResponse, InfoRequest, InfoResponse, Status, UpdateObject, UpdateRequest,
    UpdateResponse,
};
use super::super::{proto, Error, Response, ServerFeatures};
use super::router::HandleReqReturn;

impl From<proto::host::EPPHostStatusType> for Status {
    fn from(from: proto::host::EPPHostStatusType) -> Self {
        use proto::host::EPPHostStatusType;
        match from {
            EPPHostStatusType::ClientDeleteProhibited => Status::ClientDeleteProhibited,
            EPPHostStatusType::ClientUpdateProhibited => Status::ClientUpdateProhibited,
            EPPHostStatusType::Linked => Status::Linked,
            EPPHostStatusType::Ok => Status::Ok,
            EPPHostStatusType::PendingCreate => Status::PendingCreate,
            EPPHostStatusType::PendingDelete => Status::PendingDelete,
            EPPHostStatusType::PendingTransfer => Status::PendingTransfer,
            EPPHostStatusType::PendingUpdate => Status::PendingUpdate,
            EPPHostStatusType::ServerDeleteProhibited => Status::ServerDeleteProhibited,
            EPPHostStatusType::ServerUpdateProhibited => Status::ServerUpdateProhibited,
        }
    }
}

impl From<&Status> for proto::host::EPPHostStatusType {
    fn from(from: &Status) -> Self {
        use proto::host::EPPHostStatusType;
        match from {
            Status::ClientDeleteProhibited => EPPHostStatusType::ClientDeleteProhibited,
            Status::ClientUpdateProhibited => EPPHostStatusType::ClientUpdateProhibited,
            Status::Linked => EPPHostStatusType::Linked,
            Status::Ok => EPPHostStatusType::Ok,
            Status::PendingCreate => EPPHostStatusType::PendingCreate,
            Status::PendingDelete => EPPHostStatusType::PendingDelete,
            Status::PendingTransfer => EPPHostStatusType::PendingTransfer,
            Status::PendingUpdate => EPPHostStatusType::PendingUpdate,
            Status::ServerDeleteProhibited => EPPHostStatusType::ServerDeleteProhibited,
            Status::ServerUpdateProhibited => EPPHostStatusType::ServerUpdateProhibited,
        }
    }
}

fn check_host<T>(id: &str) -> Result<(), Response<T>> {
    if !id.is_empty() {
        Ok(())
    } else {
        Err(Err(Error::Err(
            "host name has a min length of 1".to_string(),
        )))
    }
}

pub fn handle_check(client: &ServerFeatures, req: &CheckRequest) -> HandleReqReturn<CheckResponse> {
    if !(client.host_supported || client.nsset_supported) {
        return Err(Err(Error::Unsupported));
    }
    check_host(&req.name)?;
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    let command = proto::EPPCheck::Host(proto::host::EPPHostCheck {
        name: req.name.clone(),
    });
    Ok((
        proto::EPPCommandType::Check(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
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
                    Err(Error::InternalServerError)
                }
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

pub fn handle_info(client: &ServerFeatures, req: &InfoRequest) -> HandleReqReturn<InfoResponse> {
    if !(client.host_supported || client.nsset_supported) {
        return Err(Err(Error::Unsupported));
    }
    check_host(&req.name)?;
    let command = proto::EPPInfo::Host(proto::host::EPPHostCheck {
        name: req.name.clone(),
    });
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((
        proto::EPPCommandType::Info(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_info_response(response: proto::EPPResponse) -> Response<InfoResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPHostInfoResult(host_info) => Response::Ok(InfoResponse {
                name: host_info.name,
                registry_id: host_info.registry_id.unwrap_or_default(),
                statuses: host_info
                    .statuses
                    .into_iter()
                    .map(|s| s.status.into())
                    .collect(),
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
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

pub fn handle_create(
    client: &ServerFeatures,
    req: &CreateRequest,
) -> HandleReqReturn<CreateResponse> {
    if !(client.host_supported || client.nsset_supported) {
        return Err(Err(Error::Unsupported));
    }
    check_host(&req.name)?;
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    let command = proto::EPPCreate::Host(proto::host::EPPHostCreate {
        name: req.name.clone(),
        addresses: match req
            .addresses
            .iter()
            .map(|a| {
                Ok(proto::host::EPPHostAddress {
                    address: if let 3..=45 = a.address.len() {
                        a.address.clone()
                    } else {
                        return Err(Err(Error::Err(
                            "address has a min length of 3 and a max length of 45".to_string(),
                        )));
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
    Ok((
        proto::EPPCommandType::Create(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_create_response(response: proto::EPPResponse) -> Response<CreateResponse> {
    match response.data {
        Some(ref value) => match &value.value {
            proto::EPPResultDataValue::EPPHostCreateResult(host_create) => {
                Response::Ok(CreateResponse {
                    name: host_create.name.clone(),
                    pending: response.is_pending(),
                    transaction_id: response
                        .transaction_id
                        .server_transaction_id
                        .unwrap_or_default(),
                    creation_date: host_create.creation_date,
                })
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

pub fn handle_delete(
    client: &ServerFeatures,
    req: &DeleteRequest,
) -> HandleReqReturn<DeleteResponse> {
    if !(client.host_supported || client.nsset_supported) {
        return Err(Err(Error::Unsupported));
    }
    check_host(&req.name)?;
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    let command = proto::EPPDelete::Host(proto::host::EPPHostCheck {
        name: req.name.clone(),
    });
    Ok((
        proto::EPPCommandType::Delete(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_delete_response(response: proto::EPPResponse) -> Response<DeleteResponse> {
    Response::Ok(DeleteResponse {
        pending: response.is_pending(),
        transaction_id: response
            .transaction_id
            .server_transaction_id
            .unwrap_or_default(),
    })
}

pub fn handle_update(
    client: &ServerFeatures,
    req: &UpdateRequest,
) -> HandleReqReturn<UpdateResponse> {
    if !(client.host_supported || client.nsset_supported) {
        return Err(Err(Error::Unsupported));
    }
    check_host(&req.name)?;
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    if req.add.is_empty() && req.remove.is_empty() && req.new_name.is_none() {
        return Err(Err(Error::Err(
            "at least one operation must be specified".to_string(),
        )));
    }
    match &req.new_name {
        Some(n) => {
            if n.is_empty() {
                return Err(Err(Error::Err(
                    "new host name has a min length of 1".to_string(),
                )));
            }
        }
        None => {}
    }

    let mut adds = vec![];
    let mut rems = vec![];
    let map_obj = |a: &UpdateObject| {
        Ok(match a {
            UpdateObject::Address(addr) => {
                proto::host::EPPHostUpdateParam::Address(proto::host::EPPHostAddress {
                    address: if let 3..=45 = addr.address.len() {
                        addr.address.clone()
                    } else {
                        return Err(Err(Error::Err(
                            "address has a min length of 3 and a max length of 45".to_string(),
                        )));
                    },
                    ip_version: match addr.ip_version {
                        AddressVersion::IPv4 => proto::host::EPPHostAddressVersion::IPv4,
                        AddressVersion::IPv6 => proto::host::EPPHostAddressVersion::IPv6,
                    },
                })
            }
            UpdateObject::Status(s) => {
                proto::host::EPPHostUpdateParam::Status(proto::host::EPPHostStatus {
                    status: s.into(),
                })
            }
        })
    };
    for add in &req.add {
        adds.push(map_obj(add)?);
    }
    for rem in &req.remove {
        rems.push(map_obj(rem)?);
    }

    let update_as_i32 = |u: &proto::host::EPPHostUpdateParam| match u {
        proto::host::EPPHostUpdateParam::Address(_) => 0,
        proto::host::EPPHostUpdateParam::Status(_) => 1,
    };
    adds.sort_unstable_by_key(update_as_i32);
    rems.sort_unstable_by_key(update_as_i32);

    let command = proto::EPPUpdate::Host(proto::host::EPPHostUpdate {
        name: req.name.clone(),
        add: match adds.len() {
            0 => None,
            _ => Some(proto::host::EPPHostUpdateAdd { params: adds }),
        },
        remove: match rems.len() {
            0 => None,
            _ => Some(proto::host::EPPHostUpdateRemove { params: rems }),
        },
        change: req
            .new_name
            .as_ref()
            .map(|n| proto::host::EPPHostUpdateChange { name: n.clone() }),
    });
    Ok((
        proto::EPPCommandType::Update(Box::new(command)),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_update_response(response: proto::EPPResponse) -> Response<UpdateResponse> {
    Response::Ok(UpdateResponse {
        pending: response.is_pending(),
        transaction_id: response
            .transaction_id
            .server_transaction_id
            .unwrap_or_default(),
    })
}
