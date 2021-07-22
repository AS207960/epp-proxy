use super::super::tmch::{
    TrexInfo, TrexTLD, TrexActivateRequest, TrexActivateResponse,
    TrexRenewRequest, TrexRenewResponse,
};
use super::tmch_proto;
use super::super::{Error, Response};
use super::router::HandleReqReturn;
use std::convert::TryInto;

impl From<tmch_proto::trex::TrexInfo> for TrexInfo {
    fn from(from: tmch_proto::trex::TrexInfo) -> Self {
        TrexInfo {
            enabled: from.enable,
            until: from.until,
            tlds: from.tlds.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<tmch_proto::trex::TLDInfo> for TrexTLD {
    fn from(from: tmch_proto::trex::TLDInfo) -> Self {
        TrexTLD {
            tld: from.tld,
            comment: from.comment,
            status: from.status.into(),
        }
    }
}

pub fn handle_trex_activate(_client: &(), req: &TrexActivateRequest) -> HandleReqReturn<TrexActivateResponse> {
    super::mark::check_mark_id(&req.id)?;
    let command = tmch_proto::TMCHUpdate {
        id: req.id.clone(),
        case: None,
        add: None,
        remove: None,
        change: Some(tmch_proto::TMCHChange {
            mark: None,
            labels: req.labels.iter().map(|l| Ok(tmch_proto::TMCHLabel {
                label: l.label.clone(),
                smd_inclusion: None,
                claims_notify: None,
                trex_activate: Some(tmch_proto::trex::Activate {
                    period: match l.period.as_ref() {
                        Some(r) => Some(r.try_into().map_err(Result::Err)?),
                        None => None
                    }
                }),
                trex_renew: None,
            })).collect::<Result<Vec<_>, _>>()?,
            case: None,
        }),
    };
    Ok(tmch_proto::TMCHCommandType::Update(command))
}

pub fn handle_trex_activate_response(response: tmch_proto::TMCHResponse) -> Response<TrexActivateResponse> {
    match response.data {
        Some(_) => Err(Error::ServerInternal),
        None => Response::Ok(TrexActivateResponse {}),
    }
}

pub fn handle_trex_renew(_client: &(), req: &TrexRenewRequest) -> HandleReqReturn<TrexRenewResponse> {
    super::mark::check_mark_id(&req.id)?;
    let command = tmch_proto::TMCHUpdate {
        id: req.id.clone(),
        case: None,
        add: None,
        remove: None,
        change: Some(tmch_proto::TMCHChange {
            mark: None,
            labels: req.labels.iter().map(|l| Ok(tmch_proto::TMCHLabel {
                label: l.label.clone(),
                smd_inclusion: None,
                claims_notify: None,
                trex_activate: None,
                trex_renew: Some(tmch_proto::trex::Renew {
                    current_expiry_date: l.current_expiry_date,
                    period: match l.period.as_ref() {
                        Some(r) => Some(r.try_into().map_err(Result::Err)?),
                        None => None
                    }
                }),
            })).collect::<Result<Vec<_>, _>>()?,
            case: None,
        }),
    };
    Ok(tmch_proto::TMCHCommandType::Update(command))
}

pub fn handle_trex_renew_response(response: tmch_proto::TMCHResponse) -> Response<TrexRenewResponse> {
    match response.data {
        Some(_) => Err(Error::ServerInternal),
        None => Response::Ok(TrexRenewResponse {}),
    }
}