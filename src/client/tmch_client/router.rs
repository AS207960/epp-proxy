use paste::paste;

pub use super::super::{router, Error, Response};

pub type HandleReqReturn<T> = Result<super::tmch_proto::TMCHCommandType, Response<T>>;

macro_rules! router {
    ($($n:ident, $req_handle:path, $res_handle:path);*) => {
        #[derive(Default, Debug)]
        pub struct Router {}

        impl router::InnerRouter<()> for Router {
            type Request = super::tmch_proto::TMCHCommandType;
            type Response = super::tmch_proto::TMCHResponse;

            paste! {
                $(fn [<$n _request>](&mut self, client: &(), req: &router::[<$n Request>], _command_id: uuid::Uuid) -> HandleReqReturn<router::[<$n Response>]> {
                    $req_handle(client, &req)
                })*

                $(fn [<$n _response>](
                    &mut self, return_path: router::Sender<router::[<$n Response>]>,
                    response: Self::Response, metrics: &crate::metrics::ScopedMetrics
                ) {
                    let _ = if !response.is_success() {
                        if response.is_server_error() {
                            return_path.send(Err(Error::Err(format!("Server error: {}", response.response_msg()))))
                        } else {
                            return_path.send(Err(Error::Err(response.response_msg())))
                        }
                    } else {
                        let trans_id = router::CommandTransactionID {
                            client: response.transaction_id.client_transaction_id.as_deref().unwrap_or_default().to_owned(),
                            server: response.transaction_id.server_transaction_id.to_owned(),
                        };
                        match $res_handle(response, metrics) {
                            Ok(r) => return_path.send(Ok(router::CommandResponse {
                                response: r,
                                extra_values: vec![],
                                transaction_id: Some(trans_id),
                            })),
                            Err(e) => return_path.send(Err(e))
                        }
                    };
                })*
            }
        }
    }
}

fn request_nop<T, R>(_client: &(), _req: &T) -> HandleReqReturn<R> {
    Err(Response::Err(Error::Unsupported))
}

fn response_nop<T, R>(_response: T, _metrics: &crate::metrics::ScopedMetrics) -> Result<R, Error> {
    Err(Error::Unsupported)
}

router!(
    Logout,                      super::handle_logout,                      super::handle_logout_response;
    Poll,                        super::poll::handle_poll,                  super::poll::handle_poll_response;
    PollAck,                     super::poll::handle_poll_ack,              super::poll::handle_poll_ack_response;
    DomainCheck,                 request_nop,                               response_nop;
    DomainClaimsCheck,           request_nop,                               response_nop;
    DomainTrademarkCheck,        request_nop,                               response_nop;
    DomainInfo,                  request_nop,                               response_nop;
    DomainCreate,                request_nop,                               response_nop;
    DomainDelete,                request_nop,                               response_nop;
    DomainUpdate,                request_nop,                               response_nop;
    DomainRenew,                 request_nop,                               response_nop;
    DomainTransferQuery,         request_nop,                               response_nop;
    DomainTransferRequest,       request_nop,                               response_nop;
    DomainTransferCancel,        request_nop,                               response_nop;
    DomainTransferAccept,        request_nop,                               response_nop;
    DomainTransferReject,        request_nop,                               response_nop;
    VerisignSync,                request_nop,                               response_nop;
    EmailForwardCheck,           request_nop,                               response_nop;
    EmailForwardInfo,            request_nop,                               response_nop;
    EmailForwardCreate,          request_nop,                               response_nop;
    EmailForwardDelete,          request_nop,                               response_nop;
    EmailForwardUpdate,          request_nop,                               response_nop;
    EmailForwardRenew,           request_nop,                               response_nop;
    EmailForwardTransferQuery,   request_nop,                               response_nop;
    EmailForwardTransferRequest, request_nop,                               response_nop;
    EmailForwardTransferCancel,  request_nop,                               response_nop;
    EmailForwardTransferAccept,  request_nop,                               response_nop;
    EmailForwardTransferReject,  request_nop,                               response_nop;
    RestoreRequest,              request_nop,                               response_nop;
    RestoreReport,               request_nop,                               response_nop;
    HostCheck,                   request_nop,                               response_nop;
    HostInfo,                    request_nop,                               response_nop;
    HostCreate,                  request_nop,                               response_nop;
    HostDelete,                  request_nop,                               response_nop;
    HostUpdate,                  request_nop,                               response_nop;
    ContactCheck,                request_nop,                               response_nop;
    ContactInfo,                 request_nop,                               response_nop;
    ContactCreate,               request_nop,                               response_nop;
    ContactDelete,               request_nop,                               response_nop;
    ContactUpdate,               request_nop,                               response_nop;
    ContactTransferQuery,        request_nop,                               response_nop;
    ContactTransferRequest,      request_nop,                               response_nop;
    ContactTransferAccept,       request_nop,                               response_nop;
    ContactTransferReject,       request_nop,                               response_nop;
    NominetTagList,              request_nop,                               response_nop;
    NominetAccept,               request_nop,                               response_nop;
    NominetReject,               request_nop,                               response_nop;
    NominetRelease,              request_nop,                               response_nop;
    NominetContactValidate,      request_nop,                               response_nop;
    NominetLock,                 request_nop,                               response_nop;
    NominetUnlock,               request_nop,                               response_nop;
    Balance,                     request_nop,                               response_nop;
    MaintenanceList,             request_nop,                               response_nop;
    MaintenanceInfo,             request_nop,                               response_nop;
    EURIDHitPoints,              request_nop,                               response_nop;
    EURIDRegistrationLimit,      request_nop,                               response_nop;
    EURIDDNSSECEligibility,      request_nop,                               response_nop;
    EURIDDNSQuality,             request_nop,                               response_nop;
    TMCHCheck,                   super::mark::handle_check,                 super::mark::handle_check_response;
    TMCHCreate,                  super::mark::handle_create,                super::mark::handle_create_response;
    TMCHMarkInfo,                super::mark::handle_mark_info,             super::mark::handle_mark_info_response;
    TMCHMarkSMDInfo,             super::mark::handle_mark_smd_info,         super::mark::handle_mark_smd_info_response;
    TMCHMarkEncodedSMDInfo,      super::mark::handle_mark_encoded_smd_info, super::mark::handle_mark_encoded_smd_info_response;
    TMCHMarkFileInfo,            super::mark::handle_mark_file_info,        super::mark::handle_mark_file_info_response;
    TMCHUpdate,                  super::mark::handle_update,                super::mark::handle_update_response;
    TMCHRenew,                   super::mark::handle_renew,                 super::mark::handle_renew_response;
    TMCHTransferInitiate,        super::mark::handle_transfer_initiate,     super::mark::handle_transfer_initiate_response;
    TMCHTransfer,                super::mark::handle_transfer,              super::mark::handle_transfer_response;
    TMCHTrexActivate,            super::trex::handle_trex_activate,         super::trex::handle_trex_activate_response;
    TMCHTrexRenew,               super::trex::handle_trex_renew,            super::trex::handle_trex_renew_response;
    DACDomain,                   request_nop,                               response_nop;
    DACUsage,                    request_nop,                               response_nop;
    DACLimits,                   request_nop,                               response_nop;
    Hello,                       request_nop,                               response_nop
);
