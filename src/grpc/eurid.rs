use super::super::client;
use super::epp_proto;

fn i32_from_eurid_contact_type(from: client::eurid::ContactType) -> i32 {
    match from {
        client::eurid::ContactType::Registrant => epp_proto::eurid::ContactType::Registrant.into(),
        client::eurid::ContactType::Tech => epp_proto::eurid::ContactType::Tech.into(),
        client::eurid::ContactType::Billing => epp_proto::eurid::ContactType::Billing.into(),
        client::eurid::ContactType::OnSite => epp_proto::eurid::ContactType::OnSite.into(),
        client::eurid::ContactType::Reseller => epp_proto::eurid::ContactType::Reseller.into(),
    }
}

fn eurid_contact_type_from_i32(from: i32) -> client::eurid::ContactType {
    match epp_proto::eurid::ContactType::from_i32(from) {
        Some(e) => match e {
            epp_proto::eurid::ContactType::Registrant => client::eurid::ContactType::Registrant,
            epp_proto::eurid::ContactType::Tech => client::eurid::ContactType::Tech,
            epp_proto::eurid::ContactType::Billing => client::eurid::ContactType::Billing,
            epp_proto::eurid::ContactType::OnSite => client::eurid::ContactType::OnSite,
            epp_proto::eurid::ContactType::Reseller => client::eurid::ContactType::Reseller,
        },
        None => client::eurid::ContactType::Registrant,
    }
}

impl From<client::eurid::Idn> for epp_proto::eurid::Idn {
    fn from(from: client::eurid::Idn) -> Self {
        epp_proto::eurid::Idn {
            ace: from.ace,
            unicode: from.unicode,
        }
    }
}

impl From<client::eurid::ContactExtension> for epp_proto::eurid::ContactExtension {
    fn from(from: client::eurid::ContactExtension) -> Self {
        epp_proto::eurid::ContactExtension {
            contact_type: i32_from_eurid_contact_type(from.contact_type),
            citizenship_country: from.citizenship_country,
            vat: from.vat,
            language: from.language,
            whois_email: from.whois_email,
        }
    }
}

impl From<epp_proto::eurid::ContactExtension> for client::eurid::ContactExtension {
    fn from(from: epp_proto::eurid::ContactExtension) -> Self {
        client::eurid::ContactExtension {
            contact_type: eurid_contact_type_from_i32(from.contact_type),
            citizenship_country: from.citizenship_country,
            vat: from.vat,
            language: from.language,
            whois_email: from.whois_email,
        }
    }
}

impl From<epp_proto::eurid::ContactUpdateExtension> for client::eurid::ContactExtensionUpdate {
    fn from(from: epp_proto::eurid::ContactUpdateExtension) -> Self {
        client::eurid::ContactExtensionUpdate {
            citizenship_country: from.new_citizenship_country,
            vat: from.new_vat,
            language: from.new_language,
            whois_email: from.new_whois_email,
        }
    }
}

impl From<epp_proto::eurid::DomainCreateExtension> for client::eurid::DomainCreate {
    fn from(from: epp_proto::eurid::DomainCreateExtension) -> Self {
        client::eurid::DomainCreate {
            on_site: from.on_site,
            reseller: from.reseller,
        }
    }
}

impl From<epp_proto::eurid::DomainUpdateExtension> for client::eurid::DomainUpdate {
    fn from(from: epp_proto::eurid::DomainUpdateExtension) -> Self {
        client::eurid::DomainUpdate {
            remove_on_site: from.remove_on_site,
            remove_reseller: from.remove_reseller,
            add_on_site: from.add_on_site,
            add_reseller: from.add_reseller,
        }
    }
}

impl From<epp_proto::eurid::DomainTransferExtension> for client::eurid::DomainTransfer {
    fn from(from: epp_proto::eurid::DomainTransferExtension) -> Self {
        client::eurid::DomainTransfer {
            on_site: from.on_site,
            reseller: from.reseller,
            technical: from.technical,
            billing: from.billing,
            registrant: from.registrant,
        }
    }
}

impl From<epp_proto::eurid::DomainDeleteExtension> for Option<client::eurid::DomainDelete> {
    fn from(from: epp_proto::eurid::DomainDeleteExtension) -> Self {
        match from.delete {
            Some(epp_proto::eurid::domain_delete_extension::Delete::Schedule(t)) => {
                super::utils::proto_to_chrono(Some(t)).map(client::eurid::DomainDelete::Schedule)
            }
            Some(epp_proto::eurid::domain_delete_extension::Delete::Cancel(_)) => {
                Some(client::eurid::DomainDelete::Cancel)
            }
            None => None,
        }
    }
}

impl From<epp_proto::eurid::DomainInfoRequest> for client::eurid::DomainInfoRequest {
    fn from(from: epp_proto::eurid::DomainInfoRequest) -> Self {
        client::eurid::DomainInfoRequest {
            auth_info: match from.auth_info {
                None => None,
                Some(epp_proto::eurid::domain_info_request::AuthInfo::Request(_)) => {
                    Some(client::eurid::DomainAuthInfo::Request)
                }
                Some(epp_proto::eurid::domain_info_request::AuthInfo::Cancel(_)) => {
                    Some(client::eurid::DomainAuthInfo::Cancel)
                }
            },
        }
    }
}

impl From<client::eurid::PollResponse> for epp_proto::eurid::PollReply {
    fn from(from: client::eurid::PollResponse) -> Self {
        epp_proto::eurid::PollReply {
            context: from.context,
            object_type: from.object_type,
            object: from.object,
            object_unicode: from.object_unicode,
            action: from.action,
            code: from.code,
            detail: from.detail,
            registrar: from.registrar,
        }
    }
}
