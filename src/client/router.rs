pub use super::Error;
use paste::paste;

use std::collections::HashMap;

pub type Response<T> = Result<T, Error>;
pub type Sender<T> = futures::channel::oneshot::Sender<Result<CommandResponse<T>, Error>>;
pub type RequestSender = futures::channel::mpsc::Sender<RequestMessage>;

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
        pub enum RequestMessage {
            $($n(Box<$req>),)*
        }

        #[allow(non_snake_case)]
        #[derive(Debug)]
        pub struct Router<I: InnerRouter<T, M>, T, M: crate::metrics::Metrics> {
            _marker: std::marker::PhantomData<T>,
            pub inner: Box<I>,
            metrics_registry: M,
            $($n: HashMap<uuid::Uuid, (Sender<$res>, Option<prometheus::HistogramTimer>)>,)*
        }

        paste! {
            $(pub type [<$n Request>] = $req;)*
            $(pub type [<$n Response>] = $res;)*
        }

        paste! {
            #[allow(non_snake_case)]
            pub trait InnerRouter<T, M: crate::metrics::Metrics> {
                type Request;
                type Response;

                $(fn [<$n _request>](&mut self, client: &T, req: &$req, command_id: uuid::Uuid) -> Result<Self::Request, Response<$res>>;)*
                $(fn [<$n _response>](&mut self, return_path: Sender<$res>, response: Self::Response, metrics: &M);)*
            }
        }

        impl<T, I: Default + InnerRouter<T, M>, M: crate::metrics::Metrics> Router<I, T, M> {
            pub fn new(metrics_registry: &M) -> Self {
                Self {
                    _marker: Default::default(),
                    metrics_registry: metrics_registry.clone(),
                    inner: Box::<I>::default(),
                    $($n: Default::default(),)*
                }
            }

            pub fn reject_request(req: RequestMessage) {
                match req {
                    $(RequestMessage::$n(req) => {let _ = req.return_path.send(Err(Error::NotReady));},)*
                };
            }

            pub fn drain(&mut self) {
                $(for r in self.$n.drain() {
                    let _ = r.1.0.send(Err(Error::NotReady));
                })*
            }

            pub fn handle_request(&mut self, client: &T, req: RequestMessage) ->
             Option<(I::Request, uuid::Uuid)> {
                match req {
                    $(RequestMessage::$n(req) => {
                        let command_id = uuid::Uuid::new_v4();
                        let timer =  self.metrics_registry.record_response_time(stringify!($n));
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
                        self.$n.insert(command_id, (req.return_path, timer));
                        Some((res, command_id))
                    },)*
                }
            }

            pub fn handle_response(&mut self, transaction_id: &uuid::Uuid, response: I::Response) {
                $(if let Some((return_path, timer)) = self.$n.remove(transaction_id) {
                    paste! {
                        if let Some(timer) = timer {
                            timer.observe_duration();
                        }
                        I::[<$n _response>](&mut self.inner, return_path, response, &self.metrics_registry);
                    }
                } else)* {}
            }
        }
    }
}

router!(
    Hello,                       super::BlankRequest,                               ();
    Logout,                      super::BlankRequest,                               ();
    Poll,                        super::poll::PollRequest,                          Option<super::poll::PollResponse>;
    PollAck,                     super::poll::PollAckRequest,                       super::poll::PollAckResponse;
    DomainCheck,                 super::domain::CheckRequest,                       super::domain::CheckResponse;
    DomainClaimsCheck,           super::domain::ClaimsCheckRequest,                 super::domain::ClaimsCheckResponse;
    DomainTrademarkCheck,        super::domain::TrademarkCheckRequest,              super::domain::ClaimsCheckResponse;
    DomainInfo,                  super::domain::InfoRequest,                        super::domain::InfoResponse;
    DomainCreate,                super::domain::CreateRequest,                      super::domain::CreateResponse;
    DomainDelete,                super::domain::DeleteRequest,                      super::domain::DeleteResponse;
    DomainUpdate,                super::domain::UpdateRequest,                      super::domain::UpdateResponse;
    DomainRenew,                 super::domain::RenewRequest,                       super::domain::RenewResponse;
    DomainTransferQuery,         super::domain::TransferQueryRequest,               super::domain::TransferResponse;
    DomainTransferRequest,       super::domain::TransferRequestRequest,             super::domain::TransferResponse;
    DomainTransferCancel,        super::domain::TransferAcceptRejectRequest,        super::domain::TransferResponse;
    DomainTransferAccept,        super::domain::TransferAcceptRejectRequest,        super::domain::TransferResponse;
    DomainTransferReject,        super::domain::TransferAcceptRejectRequest,        super::domain::TransferResponse;
    VerisignSync,                super::domain::VerisignSyncRequest,                super::domain::UpdateResponse;
    EmailForwardCheck,           super::email_forward::CheckRequest,                super::email_forward::CheckResponse;
    EmailForwardInfo,            super::email_forward::InfoRequest,                 super::email_forward::InfoResponse;
    EmailForwardCreate,          super::email_forward::CreateRequest,               super::email_forward::CreateResponse;
    EmailForwardDelete,          super::email_forward::DeleteRequest,               super::email_forward::DeleteResponse;
    EmailForwardUpdate,          super::email_forward::UpdateRequest,               super::email_forward::UpdateResponse;
    EmailForwardRenew,           super::email_forward::RenewRequest,                super::email_forward::RenewResponse;
    EmailForwardTransferQuery,   super::email_forward::TransferQueryRequest,        super::email_forward::TransferResponse;
    EmailForwardTransferRequest, super::email_forward::TransferRequestRequest,      super::email_forward::TransferResponse;
    EmailForwardTransferCancel,  super::email_forward::TransferAcceptRejectRequest, super::email_forward::TransferResponse;
    EmailForwardTransferAccept,  super::email_forward::TransferAcceptRejectRequest, super::email_forward::TransferResponse;
    EmailForwardTransferReject,  super::email_forward::TransferAcceptRejectRequest, super::email_forward::TransferResponse;
    RestoreRequest,              super::rgp::RestoreRequest,                        super::rgp::RestoreResponse;
    RestoreReport,               super::rgp::RestoreReportRequest,                  super::rgp::RestoreReportResponse;
    HostCheck,                   super::host::CheckRequest,                         super::host::CheckResponse;
    HostInfo,                    super::host::InfoRequest,                          super::host::InfoResponse;
    HostCreate,                  super::host::CreateRequest,                        super::host::CreateResponse;
    HostDelete,                  super::host::DeleteRequest,                        super::host::DeleteResponse;
    HostUpdate,                  super::host::UpdateRequest,                        super::host::UpdateResponse;
    ContactCheck,                super::contact::CheckRequest,                      super::contact::CheckResponse;
    ContactInfo,                 super::contact::InfoRequest,                       super::contact::InfoResponse;
    ContactCreate,               super::contact::CreateRequest,                     super::contact::CreateResponse;
    ContactDelete,               super::contact::DeleteRequest,                     super::contact::DeleteResponse;
    ContactUpdate,               super::contact::UpdateRequest,                     super::contact::UpdateResponse;
    ContactTransferQuery,        super::contact::TransferQueryRequest,              super::contact::TransferResponse;
    ContactTransferRequest,      super::contact::TransferRequestRequest,            super::contact::TransferResponse;
    ContactTransferAccept,       super::contact::TransferRequestRequest,            super::contact::TransferResponse;
    ContactTransferReject,       super::contact::TransferRequestRequest,            super::contact::TransferResponse;
    NominetTagList,              super::nominet::TagListRequest,                    super::nominet::TagListResponse;
    NominetAccept,               super::nominet::HandshakeAcceptRequest,            super::nominet::HandshakeResponse;
    NominetReject,               super::nominet::HandshakeRejectRequest,            super::nominet::HandshakeResponse;
    NominetRelease,              super::nominet::ReleaseRequest,                    super::nominet::ReleaseResponse;
    NominetContactValidate,      super::nominet::ContactValidateRequest,            super::nominet::ContactValidateResponse;
    NominetLock,                 super::nominet::LockRequest,                       super::nominet::LockResponse;
    NominetUnlock,               super::nominet::LockRequest,                       super::nominet::LockResponse;
    Balance,                     super::balance::BalanceRequest,                    super::balance::BalanceResponse;
    MaintenanceList,             super::maintenance::ListRequest,                   super::maintenance::ListResponse;
    MaintenanceInfo,             super::maintenance::InfoRequest,                   super::maintenance::InfoResponse;
    EURIDHitPoints,              super::eurid::HitPointsRequest,                    super::eurid::HitPointsResponse;
    EURIDRegistrationLimit,      super::eurid::RegistrationLimitRequest,            super::eurid::RegistrationLimitResponse;
    EURIDDNSSECEligibility,      super::eurid::DNSSECEligibilityRequest,            super::eurid::DNSSECEligibilityResponse;
    EURIDDNSQuality,             super::eurid::DNSQualityRequest,                   super::eurid::DNSQualityResponse;
    TMCHCheck,                   super::tmch::CheckRequest,                         super::tmch::CheckResponse;
    TMCHCreate,                  super::tmch::CreateRequest,                        super::tmch::CreateResponse;
    TMCHMarkInfo,                super::tmch::MarkInfoRequest,                      super::tmch::MarkInfoResponse;
    TMCHMarkSMDInfo,             super::tmch::MarkSMDInfoRequest,                   super::tmch::MarkSMDInfoResponse;
    TMCHMarkEncodedSMDInfo,      super::tmch::MarkSMDInfoRequest,                   super::tmch::MarkSMDInfoResponse;
    TMCHMarkFileInfo,            super::tmch::MarkSMDInfoRequest,                   super::tmch::MarkSMDInfoResponse;
    TMCHUpdate,                  super::tmch::UpdateRequest,                        super::tmch::UpdateResponse;
    TMCHRenew,                   super::tmch::RenewRequest,                         super::tmch::RenewResponse;
    TMCHTransferInitiate,        super::tmch::TransferInitiateRequest,              super::tmch::TransferInitiateResponse;
    TMCHTransfer,                super::tmch::TransferRequest,                      super::tmch::TransferResponse;
    TMCHTrexActivate,            super::tmch::TrexActivateRequest,                  super::tmch::TrexActivateResponse;
    TMCHTrexRenew,               super::tmch::TrexRenewRequest,                     super::tmch::TrexRenewResponse;
    DACDomain,                   super::dac::DACDomainRequest,                      super::dac::DACDomainResponse;
    DACUsage,                    super::dac::DACUsageRequest,                       super::dac::DACUsageResponse;
    DACLimits,                   super::dac::DACUsageRequest,                       super::dac::DACUsageResponse
);
