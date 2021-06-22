use paste::paste;

pub use super::super::{router, Error, Response};

pub type HandleReqReturn<T> = Result<(), Response<T>>;

macro_rules! router {
    ($($n:ident, $req_handle:path, $res_handle:path);*) => {
        #[derive(Default, Debug)]
        pub struct Router {}

        impl router::InnerRouter for Router {
            type Request = ();
            type Response = ();

            paste! {
                $(fn [<$n _request>](&mut self, client: &super::ServerFeatures, req: &router::[<$n Request>], _command_id: uuid::Uuid) -> HandleReqReturn<router::[<$n Response>]> {
                    $req_handle(client, &req)
                })*

                $(fn [<$n _response>](&mut self, return_path: router::Sender<router::[<$n Response>]>, response: Self::Response) {
                    let _ = match $res_handle(response) {
                        Ok(r) =>  return_path.send(Ok(router::CommandResponse {
                            response: r,
                            extra_values: vec![],
                            transaction_id: None,
                        })),
                        Err(e) => return_path.send(Err(e))
                    };
                    // let _ = if !response.is_success() {
                    //     if response.is_server_error() {
                    //         return_path.send(Err(Error::Err(format!("Server error: {}", response.response_msg()))))
                    //     } else {
                    //         return_path.send(Err(Error::Err(response.response_msg())))
                    //     }
                    // } else {
                    //     let trans_id = router::CommandTransactionID {
                    //         client: response.transaction_id.client_transaction_id.as_deref().unwrap_or_default().to_owned(),
                    //         server: response.transaction_id.server_transaction_id.as_deref().unwrap_or_default().to_owned(),
                    //     };
                    // };
                })*
            }
        }
    }
}

fn request_nop<T, R>(_client: &super::ServerFeatures, _req: &T) -> HandleReqReturn<R> {
    Err(Response::Err(Error::Unsupported))
}

fn response_nop<T, R>(_response: T) -> Result<R, Error> {
    Err(Error::Unsupported)
}

router!(
    Logout,                 request_nop,  response_nop;
    Poll,                   request_nop,  response_nop;
    PollAck,                request_nop,  response_nop;
    DomainCheck,            request_nop,  response_nop;
    DomainClaimsCheck,      request_nop,  response_nop;
    DomainTrademarkCheck,   request_nop,  response_nop;
    DomainInfo,             request_nop,  response_nop;
    DomainCreate,           request_nop,  response_nop;
    DomainDelete,           request_nop,  response_nop;
    DomainUpdate,           request_nop,  response_nop;
    DomainRenew,            request_nop,  response_nop;
    DomainTransferQuery,    request_nop,  response_nop;
    DomainTransferRequest,  request_nop,  response_nop;
    DomainTransferCancel,   request_nop,  response_nop;
    DomainTransferAccept,   request_nop,  response_nop;
    DomainTransferReject,   request_nop,  response_nop;
    VerisignSync,           request_nop,  response_nop;
    RestoreRequest,         request_nop,  response_nop;
    HostCheck,              request_nop,  response_nop;
    HostInfo,               request_nop,  response_nop;
    HostCreate,             request_nop,  response_nop;
    HostDelete,             request_nop,  response_nop;
    HostUpdate,             request_nop,  response_nop;
    ContactCheck,           request_nop,  response_nop;
    ContactInfo,            request_nop,  response_nop;
    ContactCreate,          request_nop,  response_nop;
    ContactDelete,          request_nop,  response_nop;
    ContactUpdate,          request_nop,  response_nop;
    ContactTransferQuery,   request_nop,  response_nop;
    ContactTransferRequest, request_nop,  response_nop;
    ContactTransferAccept,  request_nop,  response_nop;
    ContactTransferReject,  request_nop,  response_nop;
    NominetTagList,         request_nop,  response_nop;
    NominetAccept,          request_nop,  response_nop;
    NominetReject,          request_nop,  response_nop;
    NominetRelease,         request_nop,  response_nop;
    Balance,                request_nop,  response_nop;
    MaintenanceList,        request_nop,  response_nop;
    MaintenanceInfo,        request_nop,  response_nop;
    EURIDHitPoints,         request_nop,  response_nop;
    EURIDRegistrationLimit, request_nop,  response_nop;
    EURIDDNSSECEligibility, request_nop,  response_nop;
    EURIDDNSQuality,        request_nop,  response_nop
);
