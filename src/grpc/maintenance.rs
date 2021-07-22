use super::super::client;
use super::epp_proto;

impl From<client::maintenance::InfoResponse> for epp_proto::maintenance::MaintenanceInfoReply {
    fn from(from: client::maintenance::InfoResponse) -> Self {
        epp_proto::maintenance::MaintenanceInfoReply {
            id: from.id,
            name: from.name,
            item_type: from.item_type,
            poll_type: match from.poll_type {
                None => epp_proto::maintenance::PollType::NotSet.into(),
                Some(client::maintenance::PollType::Create) => {
                    epp_proto::maintenance::PollType::Create.into()
                }
                Some(client::maintenance::PollType::Update) => {
                    epp_proto::maintenance::PollType::Update.into()
                }
                Some(client::maintenance::PollType::Delete) => {
                    epp_proto::maintenance::PollType::Delete.into()
                }
                Some(client::maintenance::PollType::Courtesy) => {
                    epp_proto::maintenance::PollType::Courtesy.into()
                }
                Some(client::maintenance::PollType::End) => {
                    epp_proto::maintenance::PollType::End.into()
                }
            },
            environment: match from.environment {
                client::maintenance::Environment::Production => {
                    epp_proto::maintenance::Environment::Production.into()
                }
                client::maintenance::Environment::Ote => {
                    epp_proto::maintenance::Environment::Ote.into()
                }
                client::maintenance::Environment::Staging => {
                    epp_proto::maintenance::Environment::Staging.into()
                }
                client::maintenance::Environment::Development => {
                    epp_proto::maintenance::Environment::Development.into()
                }
                client::maintenance::Environment::Custom(_) => {
                    epp_proto::maintenance::Environment::Custom.into()
                }
            },
            environment_name: match from.environment {
                client::maintenance::Environment::Custom(e) => Some(e),
                _ => None,
            },
            systems: from
                .systems
                .into_iter()
                .map(|s| epp_proto::maintenance::System {
                    name: s.name,
                    host: s.host,
                    impact: match s.impact {
                        client::maintenance::Impact::Full => {
                            epp_proto::maintenance::Impact::Full.into()
                        }
                        client::maintenance::Impact::Partial => {
                            epp_proto::maintenance::Impact::Partial.into()
                        }
                        client::maintenance::Impact::None => {
                            epp_proto::maintenance::Impact::None.into()
                        }
                    },
                })
                .collect(),
            start: super::utils::chrono_to_proto(Some(from.start)),
            end: super::utils::chrono_to_proto(Some(from.end)),
            created: super::utils::chrono_to_proto(Some(from.created)),
            updated: super::utils::chrono_to_proto(from.updated),
            reason: match from.reason {
                client::maintenance::Reason::Planned => {
                    epp_proto::maintenance::Reason::Planned.into()
                }
                client::maintenance::Reason::Emergency => {
                    epp_proto::maintenance::Reason::Emergency.into()
                }
            },
            detail_url: from.detail_url,
            descriptions: from
                .descriptions
                .into_iter()
                .map(|d| epp_proto::maintenance::Description {
                    description: Some(match d {
                        client::maintenance::Description::Plain(p) => {
                            epp_proto::maintenance::description::Description::Plain(p)
                        }
                        client::maintenance::Description::Html(p) => {
                            epp_proto::maintenance::description::Description::Html(p)
                        }
                    }),
                })
                .collect(),
            tlds: from.tlds,
            intervention: from
                .intervention
                .map(|i| epp_proto::maintenance::Intervention {
                    connection: i.connection,
                    implementation: i.implementation,
                }),
            cmd_resp: None,
        }
    }
}