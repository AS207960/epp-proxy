use super::super::poll::{PollAckRequest, PollAckResponse, PollData, PollRequest, PollResponse};
use super::super::{Error, Response};
use super::router::HandleReqReturn;
use super::tmch_proto;
use chrono::prelude::*;

pub fn handle_poll(_client: &(), _req: &PollRequest) -> HandleReqReturn<Option<PollResponse>> {
    let command = tmch_proto::TMCHPoll {
        operation: tmch_proto::TMCHPollOperation::Request,
        message_id: None,
    };
    Ok(tmch_proto::TMCHCommandType::Poll(command))
}

pub fn handle_poll_response<M: crate::metrics::Metrics>(
    response: tmch_proto::TMCHResponse, _metrics: &M
) -> Response<Option<PollResponse>> {
    match response.results.first() {
        Some(result) => match result.code {
            tmch_proto::TMCHResultCode::SuccessNoMessages => Response::Ok(None),
            tmch_proto::TMCHResultCode::SuccessAckToDequeue => match response.message_queue {
                Some(value) => Response::Ok(Some(PollResponse {
                    count: value.count,
                    id: value.id,
                    enqueue_time: value.enqueue_date.unwrap_or_else(Utc::now),
                    message: value.message.unwrap_or_default(),
                    data: match response.data {
                        Some(_value) => return Err(Error::ServerInternal),
                        None => PollData::None,
                    },
                })),
                None => Err(Error::ServerInternal),
            },
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_poll_ack(_client: &(), req: &PollAckRequest) -> HandleReqReturn<PollAckResponse> {
    let command = tmch_proto::TMCHPoll {
        operation: tmch_proto::TMCHPollOperation::Acknowledge,
        message_id: Some(req.id.clone()),
    };
    Ok(tmch_proto::TMCHCommandType::Poll(command))
}

pub fn handle_poll_ack_response<M: crate::metrics::Metrics>(
    response: tmch_proto::TMCHResponse, _metrics: &M
) -> Response<PollAckResponse> {
    match response.message_queue {
        Some(value) => Response::Ok(PollAckResponse {
            count: Some(value.count),
            next_id: Some(value.id),
        }),
        None => Response::Ok(PollAckResponse {
            count: None,
            next_id: None,
        }),
    }
}
