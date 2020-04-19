//! EPP commands relating to domain objects

use super::router::HandleReqReturn;
use super::{proto, EPPClientServerFeatures, Request, Response, Sender};
use chrono::prelude::*;

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
    /// Time the message was equeued into the server
    pub enqueue_time: DateTime<Utc>,
    /// Human readable messsage
    pub message: String,
}

#[derive(Debug)]
pub struct PollAckRequest {
    id: String,
    pub return_path: Sender<PollAckResponse>,
}

/// Response to a poll query
#[derive(Debug)]
pub struct PollAckResponse {
    /// Messages in the queue
    pub count: u64,
    /// ID of the message next in the queue
    pub next_id: String,
}

pub fn handle_poll(
    _client: &EPPClientServerFeatures,
    _req: &PollRequest,
) -> HandleReqReturn<Option<PollResponse>> {
    let command = proto::EPPPoll {
        operation: proto::EPPPollOperation::Request,
        message_id: None,
    };
    Ok((proto::EPPCommandType::Poll(command), None))
}

pub fn handle_poll_response(response: proto::EPPResponse) -> Response<Option<PollResponse>> {
    match response.results.first() {
        Some(result) => match result.code {
            proto::EPPResultCode::SuccessNoMessages => Response::Ok(None),
            proto::EPPResultCode::SuccessAckToDequeue => match response.message_queue {
                Some(value) => match (value.message, value.enqueue_date) {
                    (Some(message), Some(enqueue_date)) => Response::Ok(Some(PollResponse {
                        count: value.count,
                        id: value.id,
                        enqueue_time: enqueue_date,
                        message,
                    })),
                    _ => Response::InternalServerError,
                },
                None => Response::InternalServerError,
            },
            _ => Response::InternalServerError,
        },
        None => Response::InternalServerError,
    }
}

pub fn handle_poll_ack(
    _client: &EPPClientServerFeatures,
    req: &PollAckRequest,
) -> HandleReqReturn<PollAckResponse> {
    let command = proto::EPPPoll {
        operation: proto::EPPPollOperation::Acknowledge,
        message_id: Some(req.id.clone()),
    };
    Ok((proto::EPPCommandType::Poll(command), None))
}

pub fn handle_poll_ack_response(response: proto::EPPResponse) -> Response<PollAckResponse> {
    match response.message_queue {
        Some(value) => Response::Ok(PollAckResponse {
            count: value.count,
            next_id: value.id,
        }),
        None => Response::InternalServerError,
    }
}

/// Polls a single message from the server.
///
/// Return `Some()` if a message was available from the server, `None` otherwise
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn poll(
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<Option<PollResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::Poll(Box::new(PollRequest {
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
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<PollAckResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::PollAck(Box::new(PollAckRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}
