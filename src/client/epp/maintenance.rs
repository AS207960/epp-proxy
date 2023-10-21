//! EPP commands relating to draft-ietf-regext-epp-registry-maintenance

use super::super::maintenance::{
    Description, Environment, Impact, InfoRequest, InfoResponse, Intervention, ListRequest,
    ListResponse, ListResponseItem, PollType, Reason, System,
};
use super::super::{proto, Error, Response};
use super::router::HandleReqReturn;
use super::ServerFeatures;

impl From<proto::maintenance::EPPMaintenancePollType> for PollType {
    fn from(from: proto::maintenance::EPPMaintenancePollType) -> Self {
        match from {
            proto::maintenance::EPPMaintenancePollType::Create => PollType::Create,
            proto::maintenance::EPPMaintenancePollType::Update => PollType::Update,
            proto::maintenance::EPPMaintenancePollType::Delete => PollType::Delete,
            proto::maintenance::EPPMaintenancePollType::Courtesy => PollType::Courtesy,
            proto::maintenance::EPPMaintenancePollType::End => PollType::End,
        }
    }
}

impl From<proto::maintenance::EPPMaintenanceEnvironment> for Environment {
    fn from(from: proto::maintenance::EPPMaintenanceEnvironment) -> Self {
        match from.env_type {
            proto::maintenance::EPPMaintenanceEnvironmentEnum::Production => {
                Environment::Production
            }
            proto::maintenance::EPPMaintenanceEnvironmentEnum::Ote => Environment::Ote,
            proto::maintenance::EPPMaintenanceEnvironmentEnum::Staging => Environment::Staging,
            proto::maintenance::EPPMaintenanceEnvironmentEnum::Development => {
                Environment::Development
            }
            proto::maintenance::EPPMaintenanceEnvironmentEnum::Custom => {
                Environment::Custom(from.name.unwrap_or_default())
            }
        }
    }
}

impl From<proto::maintenance::EPPMaintenanceReason> for Reason {
    fn from(from: proto::maintenance::EPPMaintenanceReason) -> Self {
        match from {
            proto::maintenance::EPPMaintenanceReason::Planned => Reason::Planned,
            proto::maintenance::EPPMaintenanceReason::Emergency => Reason::Emergency,
        }
    }
}

impl From<proto::maintenance::EPPMaintenanceDescription> for Description {
    fn from(from: proto::maintenance::EPPMaintenanceDescription) -> Self {
        match from.description_type {
            proto::maintenance::EPPMaintenanceDescriptionType::Plain => {
                Description::Plain(from.description)
            }
            proto::maintenance::EPPMaintenanceDescriptionType::Html => {
                Description::Html(from.description)
            }
        }
    }
}

impl From<proto::maintenance::EPPMaintenanceItem> for InfoResponse {
    fn from(from: proto::maintenance::EPPMaintenanceItem) -> Self {
        InfoResponse {
            id: from.id.id,
            name: from.id.name,
            item_type: from.item_type,
            poll_type: from.poll_type.map(Into::into),
            environment: from.environment.into(),
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
            reason: from.reason.into(),
            detail_url: from.detail,
            descriptions: from.description.into_iter().map(Into::into).collect(),
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

impl From<proto::maintenance::EPPMaintenanceItem02> for InfoResponse {
    fn from(from: proto::maintenance::EPPMaintenanceItem02) -> Self {
        InfoResponse {
            id: from.id.id,
            name: from.id.name,
            item_type: from.item_type,
            poll_type: from.poll_type.map(Into::into),
            environment: from.environment.into(),
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
            reason: from.reason.into(),
            detail_url: from.detail,
            descriptions: from.description.into_iter().map(Into::into).collect(),
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
    if client.maintenance_supported {
        Ok((
            proto::EPPCommandType::Info(proto::EPPInfo::Maintenance(
                proto::maintenance::EPPMaintenanceInfo::List(
                    proto::maintenance::EPPMaintenanceInfoList {},
                ),
            )),
            None,
        ))
    } else if client.maintenance_02_supported {
        Ok((
            proto::EPPCommandType::Info(proto::EPPInfo::Maintenance02(
                proto::maintenance::EPPMaintenanceInfo02::List(
                    proto::maintenance::EPPMaintenanceInfoList {},
                ),
            )),
            None,
        ))
    } else {
        Err(Err(Error::Unsupported))
    }
}

pub fn handle_list_response(
    response: proto::EPPResponse, _metrics: &crate::metrics::ScopedMetrics
) -> Response<ListResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPMaintenanceInfo(
                proto::maintenance::EPPMaintenanceInfoData::List(list),
            ) => Response::Ok(ListResponse {
                items: list
                    .list
                    .into_iter()
                    .map(|i| ListResponseItem {
                        id: i.id.id,
                        name: i.id.name,
                        start: Some(i.start),
                        end: Some(i.end),
                        created: i.created_date,
                        updated: i.update_date,
                    })
                    .collect(),
            }),
            proto::EPPResultDataValue::EPPMaintenanceInfo02(
                proto::maintenance::EPPMaintenanceInfoData02::List(list),
            ) => Response::Ok(ListResponse {
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
            }),
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_info(client: &ServerFeatures, req: &InfoRequest) -> HandleReqReturn<InfoResponse> {
    if client.maintenance_supported {
        Ok((
            proto::EPPCommandType::Info(proto::EPPInfo::Maintenance(
                proto::maintenance::EPPMaintenanceInfo::Id(req.id.clone()),
            )),
            None,
        ))
    } else if client.maintenance_02_supported {
        Ok((
            proto::EPPCommandType::Info(proto::EPPInfo::Maintenance02(
                proto::maintenance::EPPMaintenanceInfo02::Id(req.id.clone()),
            )),
            None,
        ))
    } else {
        return Err(Err(Error::Unsupported));
    }
}

pub fn handle_info_response(
    response: proto::EPPResponse, _metrics: &crate::metrics::ScopedMetrics
) -> Response<InfoResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPMaintenanceInfo(
                proto::maintenance::EPPMaintenanceInfoData::Maintenance(item),
            ) => Response::Ok(item.into()),
            proto::EPPResultDataValue::EPPMaintenanceInfo02(
                proto::maintenance::EPPMaintenanceInfoData02::Maintenance(item),
            ) => Response::Ok(item.into()),
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}
