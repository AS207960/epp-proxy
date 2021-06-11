//! EPP commands relating to draft-ietf-regext-epp-registry-maintenance

use super::{CommandResponse, Request, Sender};
use chrono::prelude::*;

#[derive(Debug)]
pub struct InfoRequest {
    pub(super) id: String,
    pub return_path: Sender<InfoResponse>,
}

#[derive(Debug)]
pub struct ListRequest {
    pub return_path: Sender<ListResponse>,
}

#[derive(Debug)]
pub struct InfoResponse {
    pub id: String,
    pub name: Option<String>,
    pub item_type: Vec<String>,
    pub poll_type: Option<PollType>,
    pub environment: Environment,
    pub systems: Vec<System>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub created: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>,
    pub reason: Reason,
    pub detail_url: Option<String>,
    pub descriptions: Vec<Description>,
    pub tlds: Vec<String>,
    pub intervention: Option<Intervention>,
}

#[derive(Debug)]
pub enum PollType {
    Create,
    Update,
    Delete,
    Courtesy,
    End,
}

#[derive(Debug)]
pub struct ListResponse {
    pub items: Vec<ListResponseItem>,
}

#[derive(Debug)]
pub struct ListResponseItem {
    pub id: String,
    pub name: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub created: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct System {
    pub name: String,
    pub host: Option<String>,
    pub impact: Impact,
}

#[derive(Debug)]
pub enum Impact {
    Full,
    Partial,
    None,
}

#[derive(Debug)]
pub struct Intervention {
    pub connection: bool,
    pub implementation: bool,
}

#[derive(Debug)]
pub enum Environment {
    Production,
    OTE,
    Staging,
    Development,
    Custom(String),
}

#[derive(Debug)]
pub enum Reason {
    Planned,
    Emergency,
}

#[derive(Debug)]
pub enum Description {
    Plain(String),
    HTML(String),
}

/// Fetches all maintenance items
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn list(
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<ListResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::MaintenanceList(Box::new(ListRequest {
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Fetches a specific maintenance item
///
/// # Arguments
/// * `id` - ID of the maintenance item to retrieve
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn info(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<InfoResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::MaintenanceInfo(Box::new(InfoRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}
