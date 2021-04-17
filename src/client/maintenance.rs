//! EPP commands relating to draft-ietf-regext-epp-registry-maintenance

use super::router::HandleReqReturn;
use super::{proto, EPPClientServerFeatures, Error, Request, Response, CommandResponse, Sender};
use chrono::prelude::*;

#[derive(Debug)]
pub struct InfoRequest {
    id: String,
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
    pub intervention: Option<Intervention>
}

#[derive(Debug)]
pub enum PollType {
    Create,
    Update,
    Delete,
    Courtesy,
    End
}

#[derive(Debug)]
pub struct ListResponse {
    pub items: Vec<ListResponseItem>
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
    None
}

#[derive(Debug)]
pub struct Intervention {
    pub connection: bool,
    pub implementation: bool
}

#[derive(Debug)]
pub enum Environment {
    Production,
    OTE,
    Staging,
    Development,
    Custom(String)
}

#[derive(Debug)]
pub enum Reason {
    Planned,
    Emergency,
}

#[derive(Debug)]
pub enum Description {
    Plain(String),
    HTML(String)
}

impl From<proto::maintenance::EPPMaintenanceItem> for InfoResponse {
    fn from(from: proto::maintenance::EPPMaintenanceItem) -> Self {
        InfoResponse {
            id: from.id.id,
            name: from.id.name,
            item_type: from.item_type,
            poll_type: from.poll_type.map(|p| match p {
                proto::maintenance::EPPMaintenancePollType::Create => PollType::Create,
                proto::maintenance::EPPMaintenancePollType::Update => PollType::Update,
                proto::maintenance::EPPMaintenancePollType::Delete => PollType::Delete,
                proto::maintenance::EPPMaintenancePollType::Courtesy => PollType::Courtesy,
                proto::maintenance::EPPMaintenancePollType::End => PollType::End,
            }),
            environment: match from.environment.env_type {
                proto::maintenance::EPPMaintenanceEnvironmentEnum::Production => Environment::Production,
                proto::maintenance::EPPMaintenanceEnvironmentEnum::OTE => Environment::OTE,
                proto::maintenance::EPPMaintenanceEnvironmentEnum::Staging => Environment::Staging,
                proto::maintenance::EPPMaintenanceEnvironmentEnum::Development => Environment::Development,
                proto::maintenance::EPPMaintenanceEnvironmentEnum::Custom => Environment::Custom(from.environment.name.unwrap_or_default())
            },
            systems: from.systems.systems.into_iter().map(|s| System {
                name: s.name,
                host: s.host,
                impact: match s.impact {
                    proto::maintenance::EPPMaintenanceImpact::Full => Impact::Full,
                    proto::maintenance::EPPMaintenanceImpact::Partial => Impact::Partial,
                    proto::maintenance::EPPMaintenanceImpact::None => Impact::None,
                }
            }).collect(),
            start: from.start,
            end: from.end,
            created: from.created_date,
            updated: from.update_date,
            reason: match from.reason {
                proto::maintenance::EPPMaintenanceReason::Planned => Reason::Planned,
                proto::maintenance::EPPMaintenanceReason::Emergency => Reason::Emergency,
            },
            detail_url: from.detail,
            descriptions: from.description.into_iter().map(|d| match d.description_type {
                proto::maintenance::EPPMaintenanceDescriptionType::Plain => Description::Plain(d.description),
                proto::maintenance::EPPMaintenanceDescriptionType::HTML => Description::HTML(d.description),
            }).collect(),
            tlds: match from.tlds {
                Some(t) => t.tlds,
                None => vec![]
            },
            intervention: from.intervention.map(|i| Intervention {
                connection: i.connection,
                implementation: i.implementation,
            })
        }
    }
}

pub fn handle_list(
    client: &EPPClientServerFeatures,
    _req: &ListRequest,
) -> HandleReqReturn<ListResponse> {
    if !client.maintenance_supported {
        return Err(Err(Error::Unsupported));
    }
    Ok((proto::EPPCommandType::Info(proto::EPPInfo::Maintenance(proto::maintenance::EPPMaintenanceInfo {
        id: None,
        list: Some(proto::maintenance::EPPMaintenanceInfoList {})
    })), None))
}

pub fn handle_list_response(response: proto::EPPResponse) -> Response<ListResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPMaintenanceInfo(maint_info) => {
                if let Some(list) = maint_info.list {
                    Response::Ok(ListResponse {
                        items: list.list.into_iter().map(|i| ListResponseItem {
                            id: i.id.id,
                            name: i.id.name,
                            start: i.start,
                            end: i.end,
                            created: i.created_date,
                            updated: i.update_date,
                        }).collect()
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

pub fn handle_info(
    client: &EPPClientServerFeatures,
    req: &InfoRequest,
) -> HandleReqReturn<InfoResponse> {
    if !client.maintenance_supported {
        return Err(Err(Error::Unsupported));
    }
    Ok((proto::EPPCommandType::Info(proto::EPPInfo::Maintenance(proto::maintenance::EPPMaintenanceInfo {
        id: Some(req.id.clone()),
        list: None,
    })), None))
}

pub fn handle_info_response(response: proto::EPPResponse) -> Response<InfoResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPMaintenanceInfo(maint_info) => {
                if let Some(item) = maint_info.item {
                    Response::Ok(item.into())
                } else {
                    Err(Error::InternalServerError)
                }
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
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
