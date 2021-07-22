use super::super::client;
use super::epp_proto;
use std::convert::{TryFrom, TryInto};

impl TryFrom<epp_proto::mark::Mark> for client::mark::Mark {
    type Error = tonic::Status;

    fn try_from(res: epp_proto::mark::Mark) -> Result<Self, Self::Error> {
        Ok(match res.mark {
            Some(epp_proto::mark::mark::Mark::Trademark(m)) => client::mark::Mark::TradeMark(client::mark::TradeMark {
                id: m.id,
                mark_name: m.mark_name,
                holders: m.holders.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                contacts: m.contacts.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                jurisdiction: m.jurisdiction,
                classes: m.classes,
                labels: m.labels,
                goods_and_services: m.goods_and_services,
                application_id: m.application_id,
                application_date: super::utils::proto_to_chrono(m.application_date),
                registration_id: m.registration_id,
                registration_date: match super::utils::proto_to_chrono(m.registration_date) {
                    Some(r) => r,
                    None => return Err(tonic::Status::invalid_argument(
                    "registration_date must be specified",
                    ))
                },
                expiry_date: super::utils::proto_to_chrono(m.expiry_date),
            }),
            Some(epp_proto::mark::mark::Mark::TreatyOrStatute(m)) => client::mark::Mark::TreatyOrStatute(client::mark::TreatyOrStatute {
                id: m.id,
                mark_name: m.mark_name,
                holders: m.holders.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                contacts: m.contacts.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                protections: m.protections.into_iter().map(|p| client::mark::Protection {
                    country_code: p.country_code,
                    region: p.region,
                    ruling: p.ruling,
                }).collect(),
                labels: m.labels,
                goods_and_services: m.goods_and_services,
                reference_number: m.reference_number,
                protection_date: match super::utils::proto_to_chrono(m.protection_date) {
                    Some(r) => r,
                    None => return Err(tonic::Status::invalid_argument(
                    "protection_date must be specified",
                    ))
                },
                title: m.title,
                execution_date: match super::utils::proto_to_chrono(m.execution_date) {
                    Some(r) => r,
                    None => return Err(tonic::Status::invalid_argument(
                    "execution_date must be specified",
                    ))
                },
            }),
            Some(epp_proto::mark::mark::Mark::Court(m)) => client::mark::Mark::Court(client::mark::Court {
                id: m.id,
                mark_name: m.mark_name,
                holders: m.holders.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                contacts: m.contacts.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                labels: m.labels,
                goods_and_services: m.goods_and_services,
                reference_number: m.reference_number,
                protection_date: match super::utils::proto_to_chrono(m.protection_date) {
                    Some(r) => r,
                    None => return Err(tonic::Status::invalid_argument(
                    "protection_date must be specified",
                    ))
                },
                country_code: m.country_code,
                region: m.regions,
                court_name: m.court_name,
            }),
            None => return Err(tonic::Status::invalid_argument(
            "mark must be specified",
            ))
        })
    }
}

impl From<client::mark::Mark> for epp_proto::mark::Mark {
    fn from(res: client::mark::Mark) -> Self {
        epp_proto::mark::Mark {
            mark: Some(match res {
                client::mark::Mark::TradeMark(m) => epp_proto::mark::mark::Mark::Trademark(epp_proto::mark::TradeMark {
                    id: m.id,
                    mark_name: m.mark_name,
                    holders: m.holders.into_iter().map(Into::into).collect(),
                    contacts: m.contacts.into_iter().map(Into::into).collect(),
                    jurisdiction: m.jurisdiction,
                    classes: m.classes,
                    labels: m.labels,
                    goods_and_services: m.goods_and_services,
                    application_id: m.application_id,
                    application_date: super::utils::chrono_to_proto(m.application_date),
                    registration_id: m.registration_id,
                    registration_date: super::utils::chrono_to_proto(Some(m.registration_date)),
                    expiry_date: super::utils::chrono_to_proto(m.expiry_date),
                }),
                client::mark::Mark::TreatyOrStatute(m) => epp_proto::mark::mark::Mark::TreatyOrStatute(epp_proto::mark::TreatyOrStatute {
                    id: m.id,
                    mark_name: m.mark_name,
                    holders: m.holders.into_iter().map(Into::into).collect(),
                    contacts: m.contacts.into_iter().map(Into::into).collect(),
                    protections: m.protections.into_iter().map(|p| epp_proto::mark::Protection {
                        country_code: p.country_code,
                        region: p.region,
                        ruling: p.ruling,
                    }).collect(),
                    labels: m.labels,
                    goods_and_services: m.goods_and_services,
                    reference_number: m.reference_number,
                    protection_date: super::utils::chrono_to_proto(Some(m.protection_date)),
                    title: m.title,
                    execution_date: super::utils::chrono_to_proto(Some(m.execution_date)),
                }),
                client::mark::Mark::Court(m) => epp_proto::mark::mark::Mark::Court(epp_proto::mark::Court {
                    id: m.id,
                    mark_name: m.mark_name,
                    holders: m.holders.into_iter().map(Into::into).collect(),
                    contacts: m.contacts.into_iter().map(Into::into).collect(),
                    labels: m.labels,
                    goods_and_services: m.goods_and_services,
                    reference_number: m.reference_number,
                    protection_date: super::utils::chrono_to_proto(Some(m.protection_date)),
                    country_code: m.country_code,
                    regions: m.region,
                    court_name: m.court_name,
                }),
            })
        }
    }
}

impl TryFrom<epp_proto::mark::Holder> for client::mark::Holder {
    type Error = tonic::Status;

    fn try_from(from: epp_proto::mark::Holder) -> Result<Self, Self::Error> {
        Ok(client::mark::Holder {
            name: from.name,
            organisation: from.organisation,
            address: match from.address {
                Some(a) => a.into(),
                None => return Err(tonic::Status::invalid_argument(
                    "address must be specified",
                ))
            },
            voice: from.voice.map(Into::into),
            fax: from.fax.map(Into::into),
            email: from.email,
            entitlement: match epp_proto::mark::Entitlement::from_i32(from.entitlement) {
                None => client::mark::Entitlement::Owner,
                Some(epp_proto::mark::Entitlement::Owner) => client::mark::Entitlement::Owner,
                Some(epp_proto::mark::Entitlement::Licensee) => client::mark::Entitlement::Licensee,
                Some(epp_proto::mark::Entitlement::Assignee) => client::mark::Entitlement::Assignee,
            }
        })
    }
}


impl From<client::mark::Holder> for epp_proto::mark::Holder {
    fn from(from: client::mark::Holder) -> Self {
        epp_proto::mark::Holder {
            name: from.name,
            organisation: from.organisation,
            address: Some(from.address.into()),
            voice: from.voice.map(Into::into),
            fax: from.fax.map(Into::into),
            email: from.email,
            entitlement: match from.entitlement {
                client::mark::Entitlement::Owner => epp_proto::mark::Entitlement::Owner.into(),
                client::mark::Entitlement::Licensee => epp_proto::mark::Entitlement::Licensee.into(),
                client::mark::Entitlement::Assignee => epp_proto::mark::Entitlement::Assignee.into()
            }
        }
    }
}

impl TryFrom<epp_proto::mark::Contact> for client::mark::Contact {
    type Error = tonic::Status;

    fn try_from(from: epp_proto::mark::Contact) -> Result<Self, Self::Error> {
        Ok(client::mark::Contact {
            name: from.name,
            organisation: from.organisation,
            address: match from.address {
                Some(a) => a.into(),
                None => return Err(tonic::Status::invalid_argument(
                    "address must be specified",
                ))
            },
            voice: match from.voice {
                Some(v) => v.into(),
                None => return Err(tonic::Status::invalid_argument(
                    "voice must be specified",
                ))
            },
            fax: from.fax.map(Into::into),
            email: from.email,
            contact_type:  match epp_proto::mark::ContactType::from_i32(from.contact_type) {
                None => client::mark::ContactType::Owner,
                Some(epp_proto::mark::ContactType::OwnerContact) => client::mark::ContactType::Owner,
                Some(epp_proto::mark::ContactType::Agent) => client::mark::ContactType::Agent,
                Some(epp_proto::mark::ContactType::ThirdParty) => client::mark::ContactType::ThirdParty,
            }
        })
    }
}

impl From<client::mark::Contact> for epp_proto::mark::Contact {
    fn from(from: client::mark::Contact) -> Self {
        epp_proto::mark::Contact {
            name: from.name,
            organisation: from.organisation,
            address: Some(from.address.into()),
            voice: Some(from.voice.into()),
            fax: from.fax.map(Into::into),
            email: from.email,
            contact_type: match from.contact_type {
                client::mark::ContactType::Owner => epp_proto::mark::ContactType::OwnerContact.into(),
                client::mark::ContactType::Agent => epp_proto::mark::ContactType::Agent.into(),
                client::mark::ContactType::ThirdParty => epp_proto::mark::ContactType::ThirdParty.into(),
            }
        }
    }
}

impl From<epp_proto::mark::Address> for client::mark::Address {
    fn from(from: epp_proto::mark::Address) -> Self {
        client::mark::Address {
            street_1: from.street1,
            street_2: from.street2,
            street_3: from.street3,
            city: from.city,
            province: from.province,
            postal_code: from.postal_code,
            country_code: from.country_code,
        }
    }
}

impl From<client::mark::Address> for epp_proto::mark::Address {
    fn from(from: client::mark::Address) -> Self {
        epp_proto::mark::Address {
            street1: from.street_1,
            street2: from.street_2,
            street3: from.street_3,
            city: from.city,
            province: from.province,
            postal_code: from.postal_code,
            country_code: from.country_code,
        }
    }
}