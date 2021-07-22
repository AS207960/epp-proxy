use super::super::client;
use super::epp_proto;

impl From<epp_proto::isnic::PaymentInfo> for Option<client::isnic::PaymentInfo> {
    fn from(from: epp_proto::isnic::PaymentInfo) -> Self {
        from.payment_method.map(|m| match m {
            epp_proto::isnic::payment_info::PaymentMethod::Prepaid(id) => {
                client::isnic::PaymentInfo::Prepaid(id)
            }
            epp_proto::isnic::payment_info::PaymentMethod::Card(card) => {
                client::isnic::PaymentInfo::Card {
                    id: card.id,
                    cvc: card.cvc,
                }
            }
        })
    }
}

impl From<epp_proto::isnic::HostInfo> for client::isnic::HostInfo {
    fn from(from: epp_proto::isnic::HostInfo) -> Self {
        client::isnic::HostInfo {
            zone_contact: from.zone_contact,
        }
    }
}

impl From<epp_proto::isnic::DomainUpdate> for client::isnic::DomainUpdate {
    fn from(from: epp_proto::isnic::DomainUpdate) -> Self {
        client::isnic::DomainUpdate {
            remove_all_ns: from.remove_all_ns,
            new_master_ns: from.new_master_ns,
        }
    }
}

impl From<epp_proto::isnic::ContactCreate> for client::isnic::ContactCreate {
    fn from(from: epp_proto::isnic::ContactCreate) -> Self {
        client::isnic::ContactCreate {
            mobile: from.mobile.map(Into::into),
            sid: from.sid,
            auto_update_from_national_registry: from.auto_update_from_national_registry,
            paper_invoices: from.paper_invoices,
            lang: from.lang,
        }
    }
}

impl From<epp_proto::isnic::ContactUpdate> for client::isnic::ContactUpdate {
    fn from(from: epp_proto::isnic::ContactUpdate) -> Self {
        client::isnic::ContactUpdate {
            mobile: from.mobile.map(Into::into),
            auto_update_from_national_registry: from.auto_update_from_national_registry,
            paper_invoices: from.paper_invoices,
            lang: from.lang,
            password: from.password,
        }
    }
}