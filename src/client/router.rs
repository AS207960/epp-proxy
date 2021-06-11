pub use super::Error;
use paste::paste;

use std::collections::HashMap;
pub type Response<T> = Result<T, Error>;
pub type Sender<T> = futures::channel::oneshot::Sender<Result<CommandResponse<T>, Error>>;
pub type RequestSender = futures::channel::mpsc::Sender<Request>;

#[derive(Debug)]
pub struct CommandExtraValue {
    pub value: String,
    pub reason: String,
}

#[derive(Debug)]
pub struct CommandTransactionID {
    pub client: String,
    pub server: String,
}

#[derive(Debug)]
pub struct CommandResponse<T> {
    pub response: T,
    pub extra_values: Vec<CommandExtraValue>,
    pub transaction_id: Option<CommandTransactionID>,
}

macro_rules! router {
    ($($n:ident, $req:ty, $res:ty);*) => {
        #[derive(Debug)]
        pub enum Request {
            $($n(Box<$req>),)*
        }

        #[allow(non_snake_case)]
        #[derive(Default, Debug)]
        pub struct Router<I: InnerRouter> {
            inner: Box<I>,
            $($n: HashMap<uuid::Uuid, Sender<$res>>,)*
        }

        paste! {
            $(pub type [<$n Request>] = $req;)*
            $(pub type [<$n Response>] = $res;)*
        }

        paste! {
            #[allow(non_snake_case)]
            pub trait InnerRouter {
                type Request;
                type Response;

                $(fn [<$n _request>](&mut self, client: &super::ServerFeatures, req: &$req, command_id: uuid::Uuid) -> Result<Self::Request, Response<$res>>;)*
                $(fn [<$n _response>](&mut self, return_path: Sender<$res>, response: Self::Response);)*
            }
        }

        impl<I: InnerRouter> Router<I> {
            pub fn reject_request(req: Request) {
                match req {
                    $(Request::$n(req) => {let _ = req.return_path.send(Err(Error::NotReady));},)*
                };
            }

            pub fn drain(&mut self) {
                $(for r in self.$n.drain() {
                    let _ = r.1.send(Err(Error::NotReady));
                })*
            }

            pub fn handle_request(&mut self, client: &super::ServerFeatures, req: Request) ->
             Option<(I::Request, uuid::Uuid)> {
                match req {
                    $(Request::$n(req) => {
                        let command_id = uuid::Uuid::new_v4();
                        paste! {
                            let res = match I::[<$n _request>](&mut self.inner, client, &req, command_id.clone()) {
                                Ok(c) => c,
                                Err(e) => {
                                    let _ = req.return_path.send(match e {
                                        Ok(r) => Ok(CommandResponse {
                                            response: r,
                                            extra_values: vec![],
                                            transaction_id: None
                                        }),
                                        Err(e) => Err(e)
                                    });
                                    return None
                                }
                            };
                        }
                        self.$n.insert(command_id, req.return_path);
                        Some((res, command_id))
                    },)*
                }
            }

            pub fn handle_response(&mut self, transaction_id: &uuid::Uuid, response: I::Response) {
                $(if let Some(return_path) = self.$n.remove(transaction_id) {
                    paste! {
                        I::[<$n _response>](&mut self.inner, return_path, response);
                    }
                } else)* {}
            }
        }
    }
}

router!(
    Logout,                 super::LogoutRequest,                       ();
    Poll,                   super::poll::PollRequest,                   Option<super::poll::PollResponse>;
    PollAck,                super::poll::PollAckRequest,                super::poll::PollAckResponse;
    DomainCheck,            super::domain::CheckRequest,                super::domain::CheckResponse;
    DomainClaimsCheck,      super::domain::ClaimsCheckRequest,          super::domain::ClaimsCheckResponse;
    DomainTrademarkCheck,   super::domain::TrademarkCheckRequest,       super::domain::ClaimsCheckResponse;
    DomainInfo,             super::domain::InfoRequest,                 super::domain::InfoResponse;
    DomainCreate,           super::domain::CreateRequest,               super::domain::CreateResponse;
    DomainDelete,           super::domain::DeleteRequest,               super::domain::DeleteResponse;
    DomainUpdate,           super::domain::UpdateRequest,               super::domain::UpdateResponse;
    DomainRenew,            super::domain::RenewRequest,                super::domain::RenewResponse;
    DomainTransferQuery,    super::domain::TransferQueryRequest,        super::domain::TransferResponse;
    DomainTransferRequest,  super::domain::TransferRequestRequest,      super::domain::TransferResponse;
    DomainTransferCancel,   super::domain::TransferAcceptRejectRequest, super::domain::TransferResponse;
    DomainTransferAccept,   super::domain::TransferAcceptRejectRequest, super::domain::TransferResponse;
    DomainTransferReject,   super::domain::TransferAcceptRejectRequest, super::domain::TransferResponse;
    VerisignSync,           super::domain::VerisignSyncRequest,         super::domain::UpdateResponse;
    RestoreRequest,         super::rgp::RestoreRequest,                 super::rgp::RestoreResponse;
    HostCheck,              super::host::CheckRequest,                  super::host::CheckResponse;
    HostInfo,               super::host::InfoRequest,                   super::host::InfoResponse;
    HostCreate,             super::host::CreateRequest,                 super::host::CreateResponse;
    HostDelete,             super::host::DeleteRequest,                 super::host::DeleteResponse;
    HostUpdate,             super::host::UpdateRequest,                 super::host::UpdateResponse;
    ContactCheck,           super::contact::CheckRequest,               super::contact::CheckResponse;
    ContactInfo,            super::contact::InfoRequest,                super::contact::InfoResponse;
    ContactCreate,          super::contact::CreateRequest,              super::contact::CreateResponse;
    ContactDelete,          super::contact::DeleteRequest,              super::contact::DeleteResponse;
    ContactUpdate,          super::contact::UpdateRequest,              super::contact::UpdateResponse;
    ContactTransferQuery,   super::contact::TransferQueryRequest,       super::contact::TransferResponse;
    ContactTransferRequest, super::contact::TransferRequestRequest,     super::contact::TransferResponse;
    ContactTransferAccept,  super::contact::TransferRequestRequest,     super::contact::TransferResponse;
    ContactTransferReject,  super::contact::TransferRequestRequest,     super::contact::TransferResponse;
    NominetTagList,         super::nominet::TagListRequest,             super::nominet::TagListResponse;
    Balance,                super::balance::BalanceRequest,             super::balance::BalanceResponse;
    MaintenanceList,        super::maintenance::ListRequest,            super::maintenance::ListResponse;
    MaintenanceInfo,        super::maintenance::InfoRequest,            super::maintenance::InfoResponse;
    EURIDHitPoints,         super::eurid::HitPointsRequest,             super::eurid::HitPointsResponse;
    EURIDRegistrationLimit, super::eurid::RegistrationLimitRequest,     super::eurid::RegistrationLimitResponse;
    EURIDDNSSECEligibility, super::eurid::DNSSECEligibilityRequest,     super::eurid::DNSSECEligibilityResponse;
    EURIDDNSQuality,        super::eurid::DNSQualityRequest,            super::eurid::DNSQualityResponse
);
