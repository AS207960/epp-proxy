use super::super::mark::{
    Address, Contact, ContactType, Court, Entitlement, Holder, Mark, Protection, TradeMark,
    TreatyOrStatute,
};
use super::super::{proto, Phone};

impl From<&Mark> for proto::mark::Mark {
    fn from(from: &Mark) -> Self {
        match from {
            Mark::TradeMark(m) => proto::mark::Mark::TradeMark(proto::mark::TradeMark {
                id: m.id.to_string(),
                mark_name: m.mark_name.to_string(),
                holders: m.holders.iter().map(Into::into).collect(),
                contacts: m.contacts.iter().map(Into::into).collect(),
                jurisdiction: m.jurisdiction.to_string(),
                classes: m.classes.clone(),
                labels: m.labels.clone(),
                goods_and_services: m.goods_and_services.to_string(),
                application_id: m.application_id.as_ref().map(Into::into),
                application_date: m.application_date,
                registration_id: m.registration_id.to_string(),
                registration_date: m.registration_date,
                expiry_date: m.expiry_date,
            }),
            Mark::TreatyOrStatute(m) => {
                proto::mark::Mark::TreatyOrStatute(proto::mark::TreatyOrStatute {
                    id: m.id.to_string(),
                    mark_name: m.mark_name.to_string(),
                    holders: m.holders.iter().map(Into::into).collect(),
                    contacts: m.contacts.iter().map(Into::into).collect(),
                    protections: m
                        .protections
                        .iter()
                        .map(|p| proto::mark::Protection {
                            country_code: p.country_code.to_string(),
                            region: p.region.as_ref().map(Into::into),
                            ruling: p.ruling.iter().map(Into::into).collect(),
                        })
                        .collect(),
                    labels: m.labels.clone(),
                    goods_and_services: m.goods_and_services.to_string(),
                    reference_number: m.reference_number.to_string(),
                    protection_date: m.protection_date,
                    title: m.title.to_string(),
                    execution_date: m.execution_date,
                })
            }
            Mark::Court(m) => proto::mark::Mark::Court(proto::mark::Court {
                id: m.id.to_string(),
                mark_name: m.mark_name.to_string(),
                holders: m.holders.iter().map(Into::into).collect(),
                contacts: m.contacts.iter().map(Into::into).collect(),
                labels: m.labels.clone(),
                goods_and_services: m.goods_and_services.to_string(),
                reference_number: m.reference_number.to_string(),
                protection_date: m.protection_date,
                country_code: m.country_code.to_string(),
                region: m.region.clone(),
                court_name: m.court_name.to_string(),
            }),
        }
    }
}

impl From<proto::mark::Mark> for Mark {
    fn from(from: proto::mark::Mark) -> Self {
        match from {
            proto::mark::Mark::TradeMark(m) => Mark::TradeMark(TradeMark {
                id: m.id,
                mark_name: m.mark_name,
                holders: m.holders.into_iter().map(Into::into).collect(),
                contacts: m.contacts.into_iter().map(Into::into).collect(),
                jurisdiction: m.jurisdiction,
                classes: m.classes,
                labels: m.labels,
                goods_and_services: m.goods_and_services,
                application_id: m.application_id,
                application_date: m.application_date,
                registration_id: m.registration_id,
                registration_date: m.registration_date,
                expiry_date: m.expiry_date,
            }),
            proto::mark::Mark::TreatyOrStatute(m) => Mark::TreatyOrStatute(TreatyOrStatute {
                id: m.id,
                mark_name: m.mark_name,
                holders: m.holders.into_iter().map(Into::into).collect(),
                contacts: m.contacts.into_iter().map(Into::into).collect(),
                protections: m
                    .protections
                    .into_iter()
                    .map(|p| Protection {
                        country_code: p.country_code,
                        region: p.region,
                        ruling: p.ruling.into_iter().map(Into::into).collect(),
                    })
                    .collect(),
                labels: m.labels,
                goods_and_services: m.goods_and_services,
                reference_number: m.reference_number,
                protection_date: m.protection_date,
                title: m.title,
                execution_date: m.execution_date,
            }),
            proto::mark::Mark::Court(m) => Mark::Court(Court {
                id: m.id,
                mark_name: m.mark_name,
                holders: m.holders.into_iter().map(Into::into).collect(),
                contacts: m.contacts.into_iter().map(Into::into).collect(),
                labels: m.labels,
                goods_and_services: m.goods_and_services,
                reference_number: m.reference_number,
                protection_date: m.protection_date,
                country_code: m.country_code,
                region: m.region,
                court_name: m.court_name,
            }),
        }
    }
}

impl From<&Holder> for proto::mark::Holder {
    fn from(from: &Holder) -> Self {
        proto::mark::Holder {
            name: from.name.as_ref().map(Into::into),
            organisation: from.organisation.as_ref().map(Into::into),
            address: (&from.address).into(),
            voice: from.voice.as_ref().map(Into::into),
            fax: from.fax.as_ref().map(Into::into),
            email: from.email.as_ref().map(Into::into),
            entitlement: match from.entitlement {
                Entitlement::Owner => proto::mark::Entitlement::Owner,
                Entitlement::Assignee => proto::mark::Entitlement::Assignee,
                Entitlement::Licensee => proto::mark::Entitlement::Licensee,
            },
        }
    }
}

impl From<proto::mark::Holder> for Holder {
    fn from(from: proto::mark::Holder) -> Self {
        Holder {
            name: from.name,
            organisation: from.organisation,
            address: from.address.into(),
            voice: from.voice.map(Into::into),
            fax: from.fax.map(Into::into),
            email: from.email,
            entitlement: match from.entitlement {
                proto::mark::Entitlement::Owner => Entitlement::Owner,
                proto::mark::Entitlement::Assignee => Entitlement::Assignee,
                proto::mark::Entitlement::Licensee => Entitlement::Licensee,
            },
        }
    }
}

impl From<&Contact> for proto::mark::Contact {
    fn from(from: &Contact) -> Self {
        proto::mark::Contact {
            name: from.name.to_owned(),
            organisation: from.organisation.to_owned(),
            address: (&from.address).into(),
            voice: (&from.voice).into(),
            fax: from.fax.as_ref().map(Into::into),
            email: from.email.to_owned(),
            contact_type: match from.contact_type {
                ContactType::Owner => proto::mark::ContactType::Owner,
                ContactType::Agent => proto::mark::ContactType::Agent,
                ContactType::ThirdParty => proto::mark::ContactType::ThirdParty,
            },
        }
    }
}

impl From<proto::mark::Contact> for Contact {
    fn from(from: proto::mark::Contact) -> Self {
        Contact {
            name: from.name,
            organisation: from.organisation,
            address: from.address.into(),
            voice: from.voice.into(),
            fax: from.fax.map(Into::into),
            email: from.email,
            contact_type: match from.contact_type {
                proto::mark::ContactType::Owner => ContactType::Owner,
                proto::mark::ContactType::Agent => ContactType::Agent,
                proto::mark::ContactType::ThirdParty => ContactType::ThirdParty,
            },
        }
    }
}

impl From<&Address> for proto::mark::Address {
    fn from(from: &Address) -> Self {
        let mut streets = vec![];
        if let Some(s) = &from.street_1 {
            streets.push(s.to_string());
            if let Some(s) = &from.street_2 {
                streets.push(s.to_string());
                if let Some(s) = &from.street_3 {
                    streets.push(s.to_string());
                }
            }
        }
        proto::mark::Address {
            street: streets,
            city: from.city.to_owned(),
            province: from.province.as_ref().map(Into::into),
            postal_code: from.postal_code.as_ref().map(Into::into),
            country_code: from.country_code.to_string(),
        }
    }
}

impl From<proto::mark::Address> for Address {
    fn from(mut from: proto::mark::Address) -> Self {
        Address {
            street_1: from.street.pop(),
            street_2: from.street.pop(),
            street_3: from.street.pop(),
            city: from.city,
            province: from.province.map(Into::into),
            postal_code: from.postal_code.map(Into::into),
            country_code: from.country_code,
        }
    }
}

impl From<proto::mark::Phone> for Phone {
    fn from(from: proto::mark::Phone) -> Self {
        Phone {
            number: from.number,
            extension: from.extension,
        }
    }
}

impl From<&Phone> for proto::mark::Phone {
    fn from(from: &Phone) -> Self {
        proto::mark::Phone {
            number: from.number.clone(),
            extension: from.extension.clone(),
        }
    }
}
