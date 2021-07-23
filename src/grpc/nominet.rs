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
