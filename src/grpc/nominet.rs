use super::super::client;
use super::epp_proto;

impl From<client::nominet::CancelData> for epp_proto::nominet::DomainCancel {
    fn from(res: client::nominet::CancelData) -> Self {
        epp_proto::nominet::DomainCancel {
            name: res.domain_name,
            originator: res.originator,
        }
    }
}

impl From<client::nominet::ReleaseData> for epp_proto::nominet::DomainRelease {
    fn from(res: client::nominet::ReleaseData) -> Self {
        epp_proto::nominet::DomainRelease {
            account_id: res.account_id,
            account_moved: res.account_moved,
            from: res.from,
            registrar_tag: res.registrar_tag,
            domains: res.domains,
        }
    }
}

impl From<client::nominet::RegistrarChangeData> for epp_proto::nominet::DomainRegistrarChange {
    fn from(res: client::nominet::RegistrarChangeData) -> Self {
        epp_proto::nominet::DomainRegistrarChange {
            originator: res.originator,
            registrar_tag: res.registrar_tag,
            case_id: res.case_id,
            domains: res.domains.into_iter().map(Into::into).collect(),
            contact: Some(res.contact.into()),
        }
    }
}

impl From<client::nominet::HostCancelData> for epp_proto::nominet::HostCancel {
    fn from(res: client::nominet::HostCancelData) -> Self {
        epp_proto::nominet::HostCancel {
            host_objects: res.host_objects,
            domain_names: res.domain_names,
        }
    }
}

impl From<client::nominet::ProcessData> for epp_proto::nominet::Process {
    fn from(res: client::nominet::ProcessData) -> Self {
        epp_proto::nominet::Process {
            stage: match res.stage {
                client::nominet::ProcessStage::Initial => {
                    epp_proto::nominet::process::ProcessStage::Initial.into()
                }
                client::nominet::ProcessStage::Updated => {
                    epp_proto::nominet::process::ProcessStage::Updated.into()
                }
            },
            contact: Some(res.contact.into()),
            process_type: res.process_type,
            suspend_date: super::utils::chrono_to_proto(res.suspend_date),
            cancel_date: super::utils::chrono_to_proto(res.cancel_date),
            domain_names: res.domain_names,
        }
    }
}

impl From<client::nominet::SuspendData> for epp_proto::nominet::Suspend {
    fn from(res: client::nominet::SuspendData) -> Self {
        epp_proto::nominet::Suspend {
            reason: res.reason,
            cancel_date: super::utils::chrono_to_proto(res.cancel_date),
            domain_names: res.domain_names,
        }
    }
}

impl From<client::nominet::DomainFailData> for epp_proto::nominet::DomainFail {
    fn from(res: client::nominet::DomainFailData) -> Self {
        epp_proto::nominet::DomainFail {
            domain: res.domain_name,
            reason: res.reason,
        }
    }
}

impl From<client::nominet::RegistrantTransferData> for epp_proto::nominet::RegistrantTransfer {
    fn from(res: client::nominet::RegistrantTransferData) -> Self {
        epp_proto::nominet::RegistrantTransfer {
            originator: res.originator,
            account_id: res.account_id,
            old_account_id: res.old_account_id,
            case_id: res.case_id,
            domain_names: res.domain_names.into_iter().map(Into::into).collect(),
            contact: Some(res.contact.into()),
        }
    }
}

impl From<client::nominet::HandshakeResponse> for epp_proto::nominet::HandshakeReply {
    fn from(res: client::nominet::HandshakeResponse) -> Self {
        epp_proto::nominet::HandshakeReply {
            case_id: res.case_id,
            domains: res.domains,
            cmd_resp: None,
        }
    }
}

impl From<client::nominet::DomainInfo> for epp_proto::nominet_ext::DomainInfo {
    fn from(value: client::nominet::DomainInfo) -> Self {
        epp_proto::nominet_ext::DomainInfo {
            registration_status: match value.registration_status {
                client::nominet::RegistrationStatus::RegisteredUntilExpiry => epp_proto::nominet_ext::RegistrationStatus::RegisteredUntilExpiry.into(),
                client::nominet::RegistrationStatus::RenewalRequired => epp_proto::nominet_ext::RegistrationStatus::RenewalRequired.into(),
                client::nominet::RegistrationStatus::NoLongerRequired => epp_proto::nominet_ext::RegistrationStatus::NoLongerRequired.into(),
            },
            first_bill: match value.first_bill {
                Some(client::nominet::BillType::BillRegistrar) => epp_proto::nominet_ext::BillType::BillRegistrar.into(),
                Some(client::nominet::BillType::BillCustomer) => epp_proto::nominet_ext::BillType::BillCustomer.into(),
                None => epp_proto::nominet_ext::BillType::Unspecified.into(),
            },
            recur_bill: match value.recur_bill {
                Some(client::nominet::BillType::BillRegistrar) => epp_proto::nominet_ext::BillType::BillRegistrar.into(),
                Some(client::nominet::BillType::BillCustomer) => epp_proto::nominet_ext::BillType::BillCustomer.into(),
                None => epp_proto::nominet_ext::BillType::Unspecified.into(),
            },
            auto_bill: value.auto_bill,
            next_bill: value.next_bill,
            auto_period: value.auto_period,
            next_period: value.next_period,
            renewal_not_required: value.renew_not_required.unwrap_or(false),
            notes: value.notes,
            reseller: value.reseller,
        }
    }
}

impl From<epp_proto::nominet_ext::DomainCreate> for client::nominet::DomainCreate {
    fn from(value: epp_proto::nominet_ext::DomainCreate) -> Self {
        client::nominet::DomainCreate {
            first_bill: match epp_proto::nominet_ext::BillType::from_i32(value.first_bill) {
                Some(epp_proto::nominet_ext::BillType::BillRegistrar) => Some(client::nominet::BillType::BillRegistrar),
                Some(epp_proto::nominet_ext::BillType::BillCustomer) => Some(client::nominet::BillType::BillCustomer),
                Some(epp_proto::nominet_ext::BillType::Unspecified) => None,
                None => None
            },
            recur_bill: match epp_proto::nominet_ext::BillType::from_i32(value.recur_bill) {
                Some(epp_proto::nominet_ext::BillType::BillRegistrar) => Some(client::nominet::BillType::BillRegistrar),
                Some(epp_proto::nominet_ext::BillType::BillCustomer) => Some(client::nominet::BillType::BillCustomer),
                Some(epp_proto::nominet_ext::BillType::Unspecified) => None,
                None => None
            },
            auto_bill: value.auto_bill,
            next_bill: value.next_bill,
            auto_period: value.auto_period,
            next_period: value.next_period,
            notes: value.notes,
            reseller: value.reseller,
        }
    }
}

impl From<epp_proto::nominet_ext::DomainUpdate> for client::nominet::DomainUpdate {
    fn from(value: epp_proto::nominet_ext::DomainUpdate) -> Self {
        client::nominet::DomainUpdate {
            first_bill: match epp_proto::nominet_ext::BillType::from_i32(value.first_bill) {
                Some(epp_proto::nominet_ext::BillType::BillRegistrar) => Some(client::nominet::BillType::BillRegistrar),
                Some(epp_proto::nominet_ext::BillType::BillCustomer) => Some(client::nominet::BillType::BillCustomer),
                Some(epp_proto::nominet_ext::BillType::Unspecified) => None,
                None => None
            },
            recur_bill: match epp_proto::nominet_ext::BillType::from_i32(value.recur_bill) {
                Some(epp_proto::nominet_ext::BillType::BillRegistrar) => Some(client::nominet::BillType::BillRegistrar),
                Some(epp_proto::nominet_ext::BillType::BillCustomer) => Some(client::nominet::BillType::BillCustomer),
                Some(epp_proto::nominet_ext::BillType::Unspecified) => None,
                None => None
            },
            auto_bill: value.auto_bill,
            next_bill: value.next_bill,
            auto_period: value.auto_period,
            next_period: value.next_period,
            renew_not_required: value.renewal_not_required,
            notes: value.notes,
            reseller: value.reseller,
        }
    }
}