//! EPP commands relating to host (nameserver) objects

use super::{CommandResponse, RequestMessage, Sender};
use chrono::prelude::*;

#[derive(Debug)]
pub struct CheckRequest {
    pub(super) name: String,
    pub return_path: Sender<CheckResponse>,
}

#[derive(Debug)]
pub struct CheckResponse {
    pub avail: bool,
    pub reason: Option<String>,
}

#[derive(Debug)]
pub struct InfoRequest {
    pub(super) name: String,
    pub return_path: Sender<InfoResponse>,
}

#[derive(Debug)]
pub struct InfoResponse {
    pub name: String,
    pub registry_id: String,
    pub statuses: Vec<Status>,
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
    pub(super) name: String,
    pub(super) addresses: Vec<Address>,
    pub return_path: Sender<CreateResponse>,
}

#[derive(Debug)]
pub struct CreateResponse {
    pub name: String,
    pub pending: bool,
    pub transaction_id: String,
    pub creation_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct DeleteRequest {
    pub(super) name: String,
    pub return_path: Sender<DeleteResponse>,
}

#[derive(Debug)]
pub struct DeleteResponse {
    pub pending: bool,
    pub transaction_id: String,
}

#[derive(Debug)]
pub struct UpdateRequest {
    pub(super) name: String,
    pub(super) add: Vec<UpdateObject>,
    pub(super) remove: Vec<UpdateObject>,
    pub(super) new_name: Option<String>,
    pub return_path: Sender<UpdateResponse>,
}

#[derive(Debug)]
pub enum UpdateObject {
    Address(Address),
    Status(Status),
}

#[derive(Debug)]
pub struct UpdateResponse {
    pub pending: bool,
    pub transaction_id: String,
}

#[derive(Debug)]
pub enum Status {
    ClientDeleteProhibited,
    ClientUpdateProhibited,
    Linked,
    Ok,
    PendingCreate,
    PendingDelete,
    PendingTransfer,
    PendingUpdate,
    ServerDeleteProhibited,
    ServerUpdateProhibited,
}

pub async fn check(
    host: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<CheckResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::HostCheck(Box::new(CheckRequest {
            name: host.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

pub async fn info(
    host: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<InfoResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::HostInfo(Box::new(InfoRequest {
            name: host.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

pub async fn create(
    host: &str,
    addresses: Vec<Address>,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<CreateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::HostCreate(Box::new(CreateRequest {
            name: host.to_string(),
            addresses,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

pub async fn delete(
    host: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<DeleteResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::HostDelete(Box::new(DeleteRequest {
            name: host.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

pub async fn update<N: Into<Option<String>>>(
    host: &str,
    add: Vec<UpdateObject>,
    remove: Vec<UpdateObject>,
    new_name: N,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<UpdateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::HostUpdate(Box::new(UpdateRequest {
            name: host.to_string(),
            add,
            remove,
            new_name: new_name.into(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}
