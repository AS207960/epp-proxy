use super::super::contact::EntityType;
use super::super::isnic::{
    ContactCreate, ContactInfo, ContactStatus, ContactUpdate, DomainInfo, DomainUpdate, HostInfo,
    PaymentInfo,
};
use super::super::proto;

impl From<&proto::isnic::DomainInfo> for DomainInfo {
    fn from(from: &proto::isnic::DomainInfo) -> Self {
        DomainInfo {
            zone_contact: from
                .contacts
                .iter()
                .find(|c| c.contact_type == proto::isnic::DomainContactType::Zone)
                .map(|c| c.contact_id.to_string()),
        }
    }
}

impl From<&PaymentInfo> for proto::isnic::DomainCreateRenew {
    fn from(from: &PaymentInfo) -> Self {
        match from {
            PaymentInfo::Prepaid(id) => proto::isnic::DomainCreateRenew {
                card_id: *id,
                card_cvc: "1".to_string(),
            },
            PaymentInfo::Card { id, cvc } => proto::isnic::DomainCreateRenew {
                card_id: *id,
                card_cvc: cvc.to_string(),
            },
        }
    }
}

impl From<&DomainUpdate> for proto::isnic::DomainUpdate {
    fn from(from: &DomainUpdate) -> Self {
        proto::isnic::DomainUpdate {
            remove: if from.remove_all_ns {
                Some(proto::isnic::DomainUpdateRemove { ns_all: true })
            } else {
                None
            },
            change: if !from.new_master_ns.is_empty() {
                Some(proto::isnic::DomainUpdateChange {
                    master_ns: Some(proto::isnic::DomainUpdateNS {
                        hosts: from.new_master_ns.iter().map(Into::into).collect(),
                    }),
                })
            } else {
                None
            },
        }
    }
}

impl From<&proto::isnic::ContactInfo> for ContactInfo {
    fn from(from: &proto::isnic::ContactInfo) -> Self {
        ContactInfo {
            statuses: from
                .status
                .iter()
                .map(|s| match s.status_type {
                    proto::isnic::ContactStatusType::Ok => ContactStatus::Ok,
                    proto::isnic::ContactStatusType::OkUnconfirmed => ContactStatus::OkUnconfirmed,
                    proto::isnic::ContactStatusType::PendingCreate => ContactStatus::PendingCreate,
                    proto::isnic::ContactStatusType::ServerExpired => ContactStatus::ServerExpired,
                    proto::isnic::ContactStatusType::ServerSuspended => {
                        ContactStatus::ServerSuspended
                    }
                })
                .collect(),
            mobile: from.mobile.as_ref().map(|p| p.into()),
            sid: from.sid.as_ref().map(Into::into),
            auto_update_from_national_registry: from
                .auto_update_from_national_registry
                .unwrap_or(false),
            paper_invoices: !from.cancel_paper.unwrap_or(true),
        }
    }
}

pub(super) fn isnic_ext_to_entity(
    ext: &proto::isnic::ContactInfo,
    addr: Option<&proto::contact::EPPContactPostalInfo>,
) -> EntityType {
    match (&ext.contact_type, addr) {
        (proto::isnic::ContactType::Person, None) => EntityType::OtherIndividual,
        (proto::isnic::ContactType::Role, None) => EntityType::Unknown,
        (t, Some(a)) => match (t, a.address.country_code.as_str()) {
            (proto::isnic::ContactType::Person, "GB") => EntityType::UkIndividual,
            (proto::isnic::ContactType::Person, "FI") => EntityType::FinnishIndividual,
            (proto::isnic::ContactType::Person, _) => EntityType::OtherIndividual,
            (proto::isnic::ContactType::Role, "GB") => EntityType::OtherUkEntity,
            (proto::isnic::ContactType::Role, "FI") => EntityType::FinnishCompany,
            (proto::isnic::ContactType::Role, _) => EntityType::Unknown,
        },
    }
}

impl From<(&ContactCreate, &Option<EntityType>)> for proto::isnic::ContactCreate {
    fn from(from: (&ContactCreate, &Option<EntityType>)) -> Self {
        let (from, entity_type) = from;
        proto::isnic::ContactCreate {
            contact_type: match entity_type {
                Some(EntityType::UkIndividual)
                | Some(EntityType::FinnishIndividual)
                | Some(EntityType::OtherIndividual)
                | None => proto::isnic::ContactType::Person,
                _ => proto::isnic::ContactType::Role,
            },
            mobile: from.mobile.as_ref().map(|p| p.into()),
            sid: from.sid.as_ref().map(Into::into),
            auto_update_from_national_registry: Some(from.auto_update_from_national_registry),
            cancel_paper: Some(!from.paper_invoices),
            lang: Some(
                from.lang
                    .as_ref()
                    .map(Into::into)
                    .unwrap_or_else(|| "en".to_string()),
            ),
        }
    }
}

impl From<&ContactUpdate> for proto::isnic::ContactUpdate {
    fn from(from: &ContactUpdate) -> Self {
        let has_updates = from.mobile.is_some()
            || from.auto_update_from_national_registry.is_some()
            || from.paper_invoices.is_some()
            || from.lang.is_some();
        proto::isnic::ContactUpdate {
            change: if has_updates {
                Some(proto::isnic::ContactUpdateChange {
                    mobile: from.mobile.as_ref().map(|p| p.into()),
                    auto_update_from_national_registry: from.auto_update_from_national_registry,
                    cancel_paper: from.paper_invoices.map(|i| !i),
                    lang: from.lang.as_ref().map(Into::into),
                })
            } else {
                None
            },
            old_password: from.password.to_string(),
        }
    }
}

impl From<&HostInfo> for proto::isnic::HostCreateUpdate {
    fn from(from: &HostInfo) -> Self {
        proto::isnic::HostCreateUpdate {
            contacts: match &from.zone_contact {
                Some(c) => vec![proto::isnic::HostContact {
                    contact_type: proto::isnic::HostContactType::Admin,
                    id: c.to_string(),
                }],
                None => vec![],
            },
        }
    }
}
