//! EPP commands relating to domain objects

use chrono::prelude::*;

use super::{CommandResponse, RequestMessage, Sender};

#[derive(Debug)]
pub struct PollRequest {
    pub return_path: Sender<Option<PollResponse>>,
}

/// Response to a poll query
#[derive(Debug)]
pub struct PollResponse {
    /// Messages in the queue
    pub count: u64,
    /// ID of the message
    pub id: String,
    /// Time the message was enqueued into the server
    pub enqueue_time: DateTime<Utc>,
    /// Human readable message
    pub message: String,
    pub data: PollData,
}

#[derive(Debug)]
pub enum PollData {
    DomainInfoData {
        data: Box<super::domain::InfoResponse>,
        change_data: Option<ChangeData>,
    },
    ContactInfoData {
        data: Box<super::contact::InfoResponse>,
        change_data: Option<ChangeData>,
    },
    HostInfoData {
        data: Box<super::host::InfoResponse>,
        change_data: Option<ChangeData>,
    },
    DomainTransferData {
        data: super::domain::TransferResponse,
        change_data: Option<ChangeData>,
    },
    ContactTransferData {
        data: super::contact::TransferResponse,
        change_data: Option<ChangeData>,
    },
    DomainCreateData {
        data: super::domain::CreateResponse,
        change_data: Option<ChangeData>,
    },
    DomainPanData {
        data: super::domain::PanData,
        change_data: Option<ChangeData>,
    },
    ContactPanData {
        data: super::contact::PanData,
        change_data: Option<ChangeData>,
    },
    DomainRenewData {
        data: super::domain::RenewResponse,
        change_data: Option<ChangeData>,
    },
    NominetDomainCancelData {
        data: super::nominet::CancelData,
        change_data: Option<ChangeData>,
    },
    NominetDomainReleaseData {
        data: super::nominet::ReleaseData,
        change_data: Option<ChangeData>,
    },
    NominetDomainRegistrarChangeData {
        data: super::nominet::RegistrarChangeData,
        change_data: Option<ChangeData>,
    },
    NominetHostCancelData {
        data: super::nominet::HostCancelData,
        change_data: Option<ChangeData>,
    },
    NominetProcessData {
        data: super::nominet::ProcessData,
        change_data: Option<ChangeData>,
    },
    NominetSuspendData {
        data: super::nominet::SuspendData,
        change_data: Option<ChangeData>,
    },
    NominetDomainFailData {
        data: super::nominet::DomainFailData,
        change_data: Option<ChangeData>,
    },
    NominetRegistrantTransferData {
        data: super::nominet::RegistrantTransferData,
        change_data: Option<ChangeData>,
    },
    VerisignLowBalanceData(super::verisign::LowBalanceData),
    TraficomTrnData(super::traficom::TrnData),
    MaintenanceData(super::maintenance::InfoResponse),
    EURIDPoll(super::eurid::PollResponse),
    None,
}

#[derive(Debug)]
pub struct ChangeData {
    pub state: ChangeState,
    pub operation: ChangeOperation,
    pub date: DateTime<Utc>,
    pub server_transaction_id: String,
    pub who: String,
    pub case_id: Option<ChangeCaseId>,
    pub reason: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum ChangeState {
    Before,
    After,
}

#[derive(Debug)]
pub struct ChangeOperation {
    pub operation: Option<String>,
    pub op_type: ChangeOperationType,
}

#[derive(PartialEq, Debug)]
pub enum ChangeOperationType {
    Create,
    Delete,
    Renew,
    Transfer,
    Update,
    Restore,
    AutoRenew,
    AutoDelete,
    AutoPurge,
    Custom,
}

#[derive(Debug)]
pub struct ChangeCaseId {
    pub case_type: ChangeCaseIdType,
    pub name: Option<String>,
    pub case_id: String,
}

#[derive(Debug)]
pub enum ChangeCaseIdType {
    Udrp,
    Urs,
    Custom,
}

#[derive(Debug)]
pub struct PollAckRequest {
    pub(super) id: String,
    pub return_path: Sender<PollAckResponse>,
}

/// Response to a poll query
#[derive(Debug)]
pub struct PollAckResponse {
    /// Messages in the queue
    pub count: Option<u64>,
    /// ID of the message next in the queue
    pub next_id: Option<String>,
}

/// Polls a single message from the server.
///
/// Return `Some()` if a message was available from the server, `None` otherwise
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn poll(
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<Option<PollResponse>>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::Poll(Box::new(PollRequest {
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Acknowledges and dequeues a message previously retrieved via poll
///
/// # Arguments
/// * `id` - ID of the message to ack
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn poll_ack(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<PollAckResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::PollAck(Box::new(PollAckRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}
