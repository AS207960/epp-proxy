//! EPP commands relating to draft-ietf-regext-epp-registry-maintenance

use super::super::maintenance::{
    Description, Environment, Impact, InfoRequest, InfoResponse, Intervention, ListRequest,
    ListResponse, ListResponseItem, PollType, Reason, System,
};
use super::super::{proto, Error, Response};
use super::router::HandleReqReturn;
use super::ServerFeatures;

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
                proto::maintenance::EPPMaintenanceEnvironmentEnum::Production => {
                    Environment::Production
                }
                proto::maintenance::EPPMaintenanceEnvironmentEnum::Ote => Environment::Ote,
                proto::maintenance::EPPMaintenanceEnvironmentEnum::Staging => Environment::Staging,
                proto::maintenance::EPPMaintenanceEnvironmentEnum::Development => {
                    Environment::Development
                }
                proto::maintenance::EPPMaintenanceEnvironmentEnum::Custom => {
                    Environment::Custom(from.environment.name.unwrap_or_default())
                }
            },
            systems: from
                .systems
                .systems
                .into_iter()
                .map(|s| System {
                    name: s.name,
                    host: s.host,
                    impact: match s.impact {
                        proto::maintenance::EPPMaintenanceImpact::Full => Impact::Full,
                        proto::maintenance::EPPMaintenanceImpact::Partial => Impact::Partial,
                        proto::maintenance::EPPMaintenanceImpact::None => Impact::None,
                    },
                })
                .collect(),
            start: from.start,
            end: from.end,
            created: from.created_date,
            updated: from.update_date,
            reason: match from.reason {
                proto::maintenance::EPPMaintenanceReason::Planned => Reason::Planned,
                proto::maintenance::EPPMaintenanceReason::Emergency => Reason::Emergency,
            },
            detail_url: from.detail,
            descriptions: from
                .description
                .into_iter()
                .map(|d| match d.description_type {
                    proto::maintenance::EPPMaintenanceDescriptionType::Plain => {
                        Description::Plain(d.description)
                    }
                    proto::maintenance::EPPMaintenanceDescriptionType::Html => {
                        Description::Html(d.description)
                    }
                })
                .collect(),
            tlds: match from.tlds {
                Some(t) => t.tlds,
                None => vec![],
            },
            intervention: from.intervention.map(|i| Intervention {
                connection: i.connection,
                implementation: i.implementation,
            }),
        }
    }
}

pub fn handle_list(client: &ServerFeatures, _req: &ListRequest) -> HandleReqReturn<ListResponse> {
    if !client.maintenance_supported {
        return Err(Err(Error::Unsupported));
    }
    Ok((
        proto::EPPCommandType::Info(proto::EPPInfo::Maintenance(
            proto::maintenance::EPPMaintenanceInfo {
                id: None,
                list: Some(proto::maintenance::EPPMaintenanceInfoList {}),
            },
        )),
        None,
    ))
}

pub fn handle_list_response(response: proto::EPPResponse) -> Response<ListResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPMaintenanceInfo(maint_info) => {
                if let Some(list) = maint_info.list {
                    Response::Ok(ListResponse {
                        items: list
                            .list
                            .into_iter()
                            .map(|i| ListResponseItem {
                                id: i.id.id,
                                name: i.id.name,
                                start: i.start,
                                end: i.end,
                                created: i.created_date,
                                updated: i.update_date,
                            })
                            .collect(),
                    })
                } else {
                    Err(Error::ServerInternal)
                }
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_info(client: &ServerFeatures, req: &InfoRequest) -> HandleReqReturn<InfoResponse> {
    if !client.maintenance_supported {
        return Err(Err(Error::Unsupported));
    }
    Ok((
        proto::EPPCommandType::Info(proto::EPPInfo::Maintenance(
            proto::maintenance::EPPMaintenanceInfo {
                id: Some(req.id.clone()),
                list: None,
            },
        )),
        None,
    ))
}

pub fn handle_info_response(response: proto::EPPResponse) -> Response<InfoResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPMaintenanceInfo(maint_info) => {
                if let Some(item) = maint_info.item {
                    Response::Ok(item.into())
                } else {
                    Err(Error::ServerInternal)
                }
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}
