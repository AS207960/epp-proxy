//! EPP commands relating to contact objects

use super::{proto, EPPClientServerFeatures, Request, Response, Sender};
use chrono::prelude::*;

#[derive(Debug)]
pub struct CheckRequest {
    id: String,
    pub return_path: Sender<CheckResponse>,
}

/// Response to a contact check query
#[derive(Debug)]
pub struct CheckResponse {
    /// Is the contact available for creation commands
    pub avail: bool,
    /// An optional reason for the ID's status
    pub reason: Option<String>,
}

#[derive(Debug)]
pub struct InfoRequest {
    id: String,
    pub return_path: Sender<InfoResponse>,
}

/// Response to a contact info query
#[derive(Debug)]
pub struct InfoResponse {
    /// The contact's ID
    pub id: String,
    /// The contact's internal registry ID
    pub registry_id: String,
    /// Statuses currently set on the contact
    pub statuses: Vec<String>,
    /// The localised address of the contact
    pub local_address: Option<Address>,
    /// The internationalised address of the contact
    pub internationalised_addresses: Option<Address>,
    /// Voice phone number of the contact
    pub phone: Option<String>,
    /// Fax number of the contact
    pub fax: Option<String>,
    /// Email address of the contact
    pub email: String,
    /// Sponsoring client ID
    pub client_id: String,
    /// ID of the client that created the contact
    pub client_created_id: Option<String>,
    /// Date of creation
    pub creation_date: Option<DateTime<Utc>>,
    /// ID of the client that last updated the contact
    pub last_updated_client: Option<String>,
    /// Date of last update
    pub last_updated_date: Option<DateTime<Utc>>,
    /// Date of last transfer
    pub last_transfer_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct Address {
    /// Name of the contact
    pub name: String,
    /// Organisation of the contact
    pub organisation: Option<String>,
    /// 1-3 street address lines
    pub streets: Vec<String>,
    pub city: String,
    /// Province or state
    pub province: Option<String>,
    pub postal_code: Option<String>,
    /// ISO 2 letter country code
    pub country_code: String,
}

//#[derive(Debug)]
//pub enum AddressVersion {
//    IPv4,
//    IPv6
//}
//
//#[derive(Debug)]
//pub struct CreateRequest {
//    name: String,
//    addresses: Vec<Address>,
//    pub return_path: Sender<CreateResponse>
//}
//
//#[derive(Debug)]
//pub struct CreateResponse {
//    pub pending: bool,
//    pub creation_date: Option<DateTime<Utc>>,
//}
//
//#[derive(Debug)]
//pub struct DeleteRequest {
//    name: String,
//    pub return_path: Sender<DeleteResponse>
//}
//
//#[derive(Debug)]
//pub struct DeleteResponse {
//    pub pending: bool,
//}
//
//#[derive(Debug)]
//pub struct UpdateRequest {
//    name: String,
//    add: Vec<UpdateObject>,
//    remove: Vec<UpdateObject>,
//    new_name: Option<String>,
//    pub return_path: Sender<UpdateResponse>
//}
//
//#[derive(Debug)]
//pub enum UpdateObject {
//    Address(Address),
//    Status(String)
//}
//
//#[derive(Debug)]
//pub struct UpdateResponse {
//    pub pending: bool,
//}

pub fn handle_check(
    client: &EPPClientServerFeatures,
    req: &CheckRequest,
) -> Result<proto::EPPCommandType, Response<CheckResponse>> {
    if !client.contact_supported {
        return Err(Response::Unsupported);
    }
    if let 3..=16 = req.id.len() {
    } else {
        return Err(Response::Err(
            "contact id has a min length of 3 and a max length of 16".to_string(),
        ));
    }
    let command = proto::EPPCheck::Contact(proto::contact::EPPContactCheck { id: req.id.clone() });
    Ok(proto::EPPCommandType::Check(command))
}

pub fn handle_check_response(response: proto::EPPResponse) -> Response<CheckResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPContactCheckResult(contact_check) => {
                if let Some(contact_check) = contact_check.data.first() {
                    Response::Ok(CheckResponse {
                        avail: contact_check.id.available,
                        reason: contact_check.reason.to_owned(),
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
    if !client.contact_supported {
        return Err(Response::Unsupported);
    }
    if let 3..=16 = req.id.len() {
    } else {
        return Err(Response::Err(
            "contact id has a min length of 3 and a max length of 16".to_string(),
        ));
    }
    let command = proto::EPPInfo::Contact(proto::contact::EPPContactCheck { id: req.id.clone() });
    Ok(proto::EPPCommandType::Info(command))
}

pub fn handle_info_response(response: proto::EPPResponse) -> Response<InfoResponse> {
    match response.data {
        Some(value) => {
            match value.value {
                proto::EPPResultDataValue::EPPContactInfoResult(contact_info) => {
                    let map_addr = |a: Option<&proto::contact::EPPContactPostalInfo>| match a {
                        Some(p) => Some(Address {
                            name: p.name.clone(),
                            organisation: p.organisation.clone(),
                            streets: p.address.streets.clone(),
                            city: p.address.city.clone(),
                            province: p.address.province.clone(),
                            postal_code: p.address.postal_code.clone(),
                            country_code: p.address.country_code.clone(),
                        }),
                        None => None,
                    };
                    Response::Ok(InfoResponse {
                    id: contact_info.id,
                    statuses: contact_info.statuses.into_iter().map(|s| s.status).collect(),
                    registry_id: contact_info.registry_id,
                    local_address: map_addr(
                        contact_info.postal_info.iter().filter(|p| p.addr_type == proto::contact::EPPContactPostalInfoType::Local).next()
                    ),
                    internationalised_addresses: map_addr(
                        contact_info.postal_info.iter().filter(|p| p.addr_type == proto::contact::EPPContactPostalInfoType::Internationalised).next()
                    ),
                    phone: contact_info.phone,
                    fax: contact_info.fax,
                    email: contact_info.email,
                    client_id: contact_info.client_id,
                    client_created_id: contact_info.client_created_id,
                    creation_date: contact_info.creation_date,
                    last_updated_client: contact_info.last_updated_client,
                    last_updated_date: contact_info.last_updated_date,
                    last_transfer_date: contact_info.last_transfer_date,
                })
                }
                _ => Response::InternalServerError,
            }
        }
        None => Response::InternalServerError,
    }
}
//
//pub async fn handle_create(client: &mut EPPClient, sock: Sock<'_>, req: CreateRequest) -> Result<(), ()> {
//    if !client.host_supported {
//        let _ = req.return_path.send(Response::Unsupported);
//        return Ok(())
//    }
//    if req.name.len() < 1 {
//        let _ = req.return_path.send(Response::Err("host name has a min length of 1".to_string()));
//        return Ok(())
//    }
//    let command = proto::EPPCreate::Host(proto::host::EPPHostCreate {
//        name: req.name,
//        addresses: match req.addresses.into_iter().map(|a| Ok(proto::host::EPPHostAddressSer {
//            address: if let 3..=45 = a.address.len() {
//                a.address
//            } else {
//                return Err(Response::Err("address has a min length of 3 and a max length of 45".to_string()));
//            },
//            ip_version: match a.ip_version {
//                AddressVersion::IPv4 => proto::host::EPPHostAddressVersion::IPv4,
//                AddressVersion::IPv6 => proto::host::EPPHostAddressVersion::IPv6,
//            }
//        })).collect() {
//            Ok(a) => a,
//            Err(e) => {
//                let _ = req.return_path.send(e);
//                return Ok(())
//            }
//        }
//    });
//    let command_id = match client._send_command(proto::EPPCommandType::Create(command), sock).await {
//        Ok(i) => i,
//        Err(_) => return Err(())
//    };
//    client.pending_host_create_responses.insert(command_id, req.return_path);
//    Ok(())
//}
//
//pub async fn handle_create_response(return_path: Sender<CreateResponse>, response: proto::EPPResponse) -> Result<(), ()> {
//    let _ = if !response.is_success() {
//        if response.is_server_error() {
//            return_path.send(Response::InternalServerError)
//        } else {
//            return_path.send(Response::Err(response.response_msg()))
//        }
//    } else {
//        match response.data {
//            Some(ref value) => match &value.value {
//                proto::EPPResultDataValue::EPPHostCreateResult(host_create) => {
//                    return_path.send(Response::Ok(CreateResponse {
//                        pending: response.is_pending(),
//                        creation_date: host_create.creation_date
//                    }))
//                },
//                _ => return_path.send(Response::InternalServerError)
//            },
//            None => {
//                return_path.send(Response::InternalServerError)
//            }
//        }
//    };
//    Ok(())
//}
//
//pub async fn handle_delete(client: &mut EPPClient, sock: Sock<'_>, req: DeleteRequest) -> Result<(), ()> {
//    if !client.host_supported {
//        let _ = req.return_path.send(Response::Unsupported);
//        return Ok(())
//    }
//    if req.name.len() < 1 {
//        let _ = req.return_path.send(Response::Err("host name has a min length of 1".to_string()));
//        return Ok(())
//    }
//    let command = proto::EPPDelete::Host(proto::host::EPPHostDelete {
//        name: req.name,
//    });
//    let command_id = match client._send_command(proto::EPPCommandType::Delete(command), sock).await {
//        Ok(i) => i,
//        Err(_) => return Err(())
//    };
//    client.pending_host_delete_responses.insert(command_id, req.return_path);
//    Ok(())
//}
//
//pub async fn handle_delete_response(return_path: Sender<DeleteResponse>, response: proto::EPPResponse) -> Result<(), ()> {
//    let _ = if !response.is_success() {
//        if response.is_server_error() {
//            return_path.send(Response::InternalServerError)
//        } else {
//            return_path.send(Response::Err(response.response_msg()))
//        }
//    } else {
//         return_path.send(Response::Ok(DeleteResponse {
//            pending: response.is_pending(),
//        }))
//    };
//    Ok(())
//}
//
//pub async fn handle_update(client: &mut EPPClient, sock: Sock<'_>, req: UpdateRequest) -> Result<(), ()> {
//    if !client.host_supported {
//        let _ = req.return_path.send(Response::Unsupported);
//        return Ok(())
//    }
//    if req.name.len() < 1 {
//        let _ = req.return_path.send(Response::Err("host name has a min length of 1".to_string()));
//        return Ok(())
//    }
//    if req.add.len() < 1 && req.remove.len() < 1 && req.new_name.is_none() {
//        let _ = req.return_path.send(Response::Err("at least one operation must be specified".to_string()));
//        return Ok(())
//    }
//    match &req.new_name {
//        Some(n) => {
//            if n.len() < 1 {
//                let _ = req.return_path.send(Response::Err("new host name has a min length of 1".to_string()));
//                return Ok(())
//            }
//        },
//        None => {}
//    }
//    let map_obj = |a| Ok(match a {
//        UpdateObject::Address(addr) => proto::host::EPPHostUpdateParam::Address(proto::host::EPPHostAddressSer {
//            address: if let 3..=45 = addr.address.len() {
//                addr.address
//            } else {
//               return Err(Response::Err("address has a min length of 3 and a max length of 45".to_string()));
//            },
//            ip_version: match addr.ip_version {
//                AddressVersion::IPv4 => proto::host::EPPHostAddressVersion::IPv4,
//                AddressVersion::IPv6 => proto::host::EPPHostAddressVersion::IPv6,
//            }
//        }),
//        UpdateObject::Status(s) => proto::host::EPPHostUpdateParam::Status(proto::host::EPPHostStatusSer {
//            status: s
//        })
//    });
//    let command = proto::EPPUpdate::Host(proto::host::EPPHostUpdate {
//        name: req.name,
//        add: match req.add.len() {
//            0 => None,
//            _ => Some(proto::host::EPPHostUpdateAdd {
//                params: match req.add.into_iter().map(map_obj).collect() {
//                    Ok(p) => p,
//                    Err(e) => {
//                        let _ = req.return_path.send(e);
//                        return Ok(())
//                    }
//                }
//            })
//        },
//        remove: match req.remove.len() {
//            0 => None,
//            _ => Some(proto::host::EPPHostUpdateRemove {
//                params: match req.remove.into_iter().map(map_obj).collect() {
//                    Ok(p) => p,
//                    Err(e) => {
//                        let _ = req.return_path.send(e);
//                        return Ok(())
//                    }
//                }
//            })
//        },
//        change: req.new_name.map(|n| proto::host::EPPHostUpdateChange {
//            name: n
//        })
//    });
//    let command_id = match client._send_command(proto::EPPCommandType::Update(command), sock).await {
//        Ok(i) => i,
//        Err(_) => return Err(())
//    };
//    client.pending_host_update_responses.insert(command_id, req.return_path);
//    Ok(())
//}
//
//pub async fn handle_update_response(return_path: Sender<UpdateResponse>, response: proto::EPPResponse) -> Result<(), ()> {
//    let _ = if !response.is_success() {
//        if response.is_server_error() {
//            return_path.send(Response::InternalServerError)
//        } else {
//            return_path.send(Response::Err(response.response_msg()))
//        }
//    } else {
//        return_path.send(Response::Ok(UpdateResponse {
//            pending: response.is_pending(),
//        }))
//    };
//    Ok(())
//}

/// Checks if a contact ID exists
///
/// # Arguments
/// * `id` - The ID in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn check(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CheckResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::ContactCheck(CheckRequest {
            id: id.to_string(),
            return_path: sender,
        }),
        receiver,
    )
    .await
}

/// Fetches information about a specific contact
///
/// # Arguments
/// * `id` - The ID in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn info(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<InfoResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::ContactInfo(InfoRequest {
            id: id.to_string(),
            return_path: sender,
        }),
        receiver,
    )
    .await
}

//pub async fn create(host: &str, addresses: Vec<Address>, client_sender: &mut futures::channel::mpsc::Sender<Request>) -> Result<CreateResponse, super::Error> {
//    let (sender, receiver) = futures::channel::oneshot::channel();
//    super::send_epp_client_request(client_sender, Request::HostCreate(CreateRequest {
//        name: host.to_string(),
//        addresses,
//        return_path: sender
//    }), receiver).await
//}
//
//pub async fn delete(host: &str, client_sender: &mut futures::channel::mpsc::Sender<Request>) -> Result<DeleteResponse, super::Error> {
//    let (sender, receiver) = futures::channel::oneshot::channel();
//    super::send_epp_client_request(client_sender, Request::HostDelete(DeleteRequest {
//        name: host.to_string(),
//        return_path: sender
//    }), receiver).await
//}
//
//pub async fn update<N: Into<Option<String>>>(host: &str, add: Vec<UpdateObject>, remove: Vec<UpdateObject>, new_name: N, client_sender: &mut futures::channel::mpsc::Sender<Request>) -> Result<UpdateResponse, super::Error> {
//    let (sender, receiver) = futures::channel::oneshot::channel();
//    super::send_epp_client_request(client_sender, Request::HostUpdate(UpdateRequest {
//        name: host.to_string(),
//        add,
//        remove,
//        new_name: new_name.into(),
//        return_path: sender
//    }), receiver).await
//}
