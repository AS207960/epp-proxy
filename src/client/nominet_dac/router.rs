use paste::paste;

pub use super::super::{router, Error, Response};

pub type HandleReqReturn<T> = Result<(super::proto::DACRequest, DACEnv), Response<T>>;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum DACEnv {
    RealTime,
    TimeDelay,
    Both,
}

impl From<super::super::dac::DACEnv> for DACEnv {
    fn from(from: super::super::dac::DACEnv) -> Self {
        match from {
            super::super::dac::DACEnv::RealTime => DACEnv::RealTime,
            super::super::dac::DACEnv::TimeDelay => DACEnv::TimeDelay,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub(super) struct DACKey {
    pub env: DACEnv,
    pub cmd: String,
}

macro_rules! router {
    ($($n:ident);*) => {
        #[derive(Default, Debug)]
        pub struct Router {
            pub(super) command_map: std::collections::HashMap<DACKey, uuid::Uuid>
        }

        impl<M: crate::metrics::Metrics> router::InnerRouter<(), M> for Router {
            type Request = (super::proto::DACRequest, DACEnv);
            type Response = super::proto::DACResponse;

            fn Logout_request(&mut self, _client: &(), _req: &router::LogoutRequest, command_id: uuid::Uuid) -> HandleReqReturn<router::LogoutResponse> {
                self.command_map.insert(DACKey {
                    env: DACEnv::Both,
                    cmd: "#exit".to_string(),
                }, command_id);
                Ok((super::proto::DACRequest::Exit, DACEnv::Both))
            }

            fn Logout_response(
                &mut self, return_path: router::Sender<router::LogoutResponse>,
                _response: Self::Response, _metrics: &M
            ) {
                let _ = return_path.send(Ok(router::CommandResponse {
                    response: (),
                    extra_values: vec![],
                    transaction_id: None,
                }));
            }

            fn DomainCheck_request(&mut self, _client: &(), req: &router::DomainCheckRequest, command_id: uuid::Uuid) -> HandleReqReturn<router::DomainCheckResponse> {
                if req.fee_check.is_some() {
                    return Err(Err(Error::Unsupported));
                }
                if req.launch_check.is_some() {
                    return Err(Err(Error::Unsupported));
                }

                self.command_map.insert(DACKey {
                    env: DACEnv::RealTime,
                    cmd: req.name.clone(),
                }, command_id);
                Ok((super::proto::DACRequest::Domain(req.name.clone()), DACEnv::RealTime))
            }

            fn DomainCheck_response(
                &mut self, return_path: router::Sender<router::DomainCheckResponse>,
                response: Self::Response, _metrics: &M
            ) {
                match response {
                    super::proto::DACResponse::DomainRT(d) => {
                        let _ = return_path.send(Ok(router::CommandResponse {
                            response: router::DomainCheckResponse {
                                avail: !d.registered,
                                reason: None,
                                fee_check: None,
                                donuts_fee_check: None,
                                eurid_check: None,
                                eurid_idn: None,
                            },
                            extra_values: vec![],
                            transaction_id: None,
                        }));
                    },
                    super::proto::DACResponse::DomainTD(d) => {
                        let _ = return_path.send(Ok(router::CommandResponse {
                            response: router::DomainCheckResponse {
                                avail: match d.registered {
                                    super::proto::DomainRegistered::Registered => false,
                                    super::proto::DomainRegistered::Available => true,
                                    super::proto::DomainRegistered::NotWithinRegistry => false,
                                    super::proto::DomainRegistered::RulesPrevent => false,
                                },
                                reason: match d.registered {
                                    super::proto::DomainRegistered::Registered => Some("already registered".to_string()),
                                    super::proto::DomainRegistered::Available => None,
                                    super::proto::DomainRegistered::NotWithinRegistry => Some("not within this registry".to_string()),
                                    super::proto::DomainRegistered::RulesPrevent => Some("rules prevent registration".to_string()),
                                },
                                fee_check: None,
                                donuts_fee_check: None,
                                eurid_check: None,
                                eurid_idn: None,
                            },
                            extra_values: vec![],
                            transaction_id: None,
                        }));
                    },
                    super::proto::DACResponse::Aub(b) => {
                        let _ = return_path.send(Err(Error::Err(format!("Acceptable usage block, please try again in {} seconds", b.delay))));
                    },
                    _ => {
                        let _ = return_path.send(Err(Error::ServerInternal));
                    },
                }
            }

            fn DACDomain_request(&mut self, _client: &(), req: &router::DACDomainRequest, command_id: uuid::Uuid) -> HandleReqReturn<router::DACDomainResponse> {
                self.command_map.insert(DACKey {
                    env: req.env.into(),
                    cmd: req.domain.clone()
                }, command_id);
                Ok((super::proto::DACRequest::Domain(req.domain.clone()), req.env.into()))
            }

            fn DACDomain_response(
                &mut self, return_path: router::Sender<router::DACDomainResponse>,
                response: Self::Response, _metrics: &M
            ) {
                match response {
                    super::proto::DACResponse::DomainRT(d) => {
                        let _ = return_path.send(Ok(router::CommandResponse {
                            response: router::DACDomainResponse {
                                registration_state: if d.registered {
                                    super::super::dac::DomainState::Registered
                                } else {
                                    super::super::dac::DomainState::Available
                                },
                                detagged: d.detagged,
                                created: d.created,
                                expiry: d.expiry,
                                status: super::super::dac::DomainStatus::Unknown,
                                suspended: None,
                                tag: d.tag,
                            },
                            extra_values: vec![],
                            transaction_id: None,
                        }));
                    },
                    super::proto::DACResponse::DomainTD(d) => {
                        let _ = return_path.send(Ok(router::CommandResponse {
                            response: router::DACDomainResponse {
                                registration_state: match d.registered {
                                    super::proto::DomainRegistered::Registered => super::super::dac::DomainState::Registered,
                                    super::proto::DomainRegistered::Available => super::super::dac::DomainState::Available,
                                    super::proto::DomainRegistered::NotWithinRegistry => super::super::dac::DomainState::NotWithinRegistry,
                                    super::proto::DomainRegistered::RulesPrevent => super::super::dac::DomainState::RulesPrevent,
                                },
                                detagged: d.detagged,
                                created: d.created,
                                expiry: d.expiry,
                                status: match d.status {
                                    super::proto::DomainStatus::Unknown => super::super::dac::DomainStatus::Unknown,
                                    super::proto::DomainStatus::RegisteredUntilExpiry => super::super::dac::DomainStatus::RegisteredUntilExpiry,
                                    super::proto::DomainStatus::RenewalRequired => super::super::dac::DomainStatus::RenewalRequired,
                                    super::proto::DomainStatus::NoLongerRequired => super::super::dac::DomainStatus::NoLongerRequired,
                                },
                                suspended: Some(d.suspended),
                                tag: d.tag,
                            },
                            extra_values: vec![],
                            transaction_id: None,
                        }));
                    },
                    super::proto::DACResponse::Aub(b) => {
                        let _ = return_path.send(Err(Error::Err(format!("Acceptable usage block, please try again in {} seconds", b.delay))));
                    },
                    _ => {
                        let _ = return_path.send(Err(Error::ServerInternal));
                    },
                }
            }

            fn DACUsage_request(&mut self, _client: &(), req: &router::DACUsageRequest, command_id: uuid::Uuid) -> HandleReqReturn<router::DACUsageResponse> {
                self.command_map.insert(DACKey {
                    env: req.env.into(),
                    cmd: "#usage".to_string()
                }, command_id);
                Ok((super::proto::DACRequest::Usage, req.env.into()))
            }

            fn DACUsage_response(
                &mut self, return_path: router::Sender<router::DACUsageResponse>,
                response: Self::Response, _metrics: &M
            ) {
                match response {
                    super::proto::DACResponse::Usage(u) => {
                        let _ = return_path.send(Ok(router::CommandResponse {
                            response: router::DACUsageResponse {
                                usage_60: u.usage_60,
                                usage_24: u.usage_24,
                            },
                            extra_values: vec![],
                            transaction_id: None,
                        }));
                    },
                    super::proto::DACResponse::Aub(b) => {
                        let _ = return_path.send(Err(Error::Err(format!("Acceptable usage block, please try again in {} seconds", b.delay))));
                    },
                    _ => {
                        let _ = return_path.send(Err(Error::ServerInternal));
                    },
                }
            }

            fn DACLimits_request(&mut self, _client: &(), req: &router::DACLimitsRequest, command_id: uuid::Uuid) -> HandleReqReturn<router::DACLimitsResponse> {
                self.command_map.insert(DACKey {
                    env: req.env.into(),
                    cmd: "#limits".to_string()
                }, command_id);
                Ok((super::proto::DACRequest::Limits, req.env.into()))
            }

            fn DACLimits_response(
                &mut self, return_path: router::Sender<router::DACLimitsResponse>,
                response: Self::Response, _metrics: &M
            ) {
                match response {
                    super::proto::DACResponse::Limits(u) => {
                        let _ = return_path.send(Ok(router::CommandResponse {
                            response: router::DACLimitsResponse {
                                usage_60: u.usage_60,
                                usage_24: u.usage_24,
                            },
                            extra_values: vec![],
                            transaction_id: None,
                        }));
                    },
                    super::proto::DACResponse::Aub(b) => {
                        let _ = return_path.send(Err(Error::Err(format!("Acceptable usage block, please try again in {} seconds", b.delay))));
                    },
                    _ => {
                        let _ = return_path.send(Err(Error::ServerInternal));
                    },
                }
            }

            paste! {
                $(fn [<$n _request>](&mut self, _client: &(), _req: &router::[<$n Request>], _command_id: uuid::Uuid) -> HandleReqReturn<router::[<$n Response>]> {
                    Err(Response::Err(Error::Unsupported))
                })*

                $(fn [<$n _response>](
                    &mut self, return_path: router::Sender<router::[<$n Response>]>,
                    _response: Self::Response, _metrics: &M
                ) {
                    let _ = return_path.send(Err(Error::Unsupported));
                })*
            }
        }
    }
}

router!(
    Poll;
    PollAck;
    DomainClaimsCheck;
    DomainTrademarkCheck;
    DomainInfo;
    DomainCreate;
    DomainDelete;
    DomainUpdate;
    DomainRenew;
    DomainTransferQuery;
    DomainTransferRequest;
    DomainTransferCancel;
    DomainTransferAccept;
    DomainTransferReject;
    VerisignSync;
    EmailForwardCheck;
    EmailForwardInfo;
    EmailForwardCreate;
    EmailForwardDelete;
    EmailForwardUpdate;
    EmailForwardRenew;
    EmailForwardTransferQuery;
    EmailForwardTransferRequest;
    EmailForwardTransferCancel;
    EmailForwardTransferAccept;
    EmailForwardTransferReject;
    RestoreRequest;
    RestoreReport;
    HostCheck;
    HostInfo;
    HostCreate;
    HostDelete;
    HostUpdate;
    ContactCheck;
    ContactInfo;
    ContactCreate;
    ContactDelete;
    ContactUpdate;
    ContactTransferQuery;
    ContactTransferRequest;
    ContactTransferAccept;
    ContactTransferReject;
    NominetTagList;
    NominetAccept;
    NominetReject;
    NominetRelease;
    NominetContactValidate;
    NominetLock;
    NominetUnlock;
    Balance;
    MaintenanceList;
    MaintenanceInfo;
    EURIDHitPoints;
    EURIDRegistrationLimit;
    EURIDDNSSECEligibility;
    EURIDDNSQuality;
    TMCHCheck;
    TMCHCreate;
    TMCHMarkInfo;
    TMCHMarkSMDInfo;
    TMCHMarkEncodedSMDInfo;
    TMCHMarkFileInfo;
    TMCHUpdate;
    TMCHRenew;
    TMCHTransferInitiate;
    TMCHTransfer;
    TMCHTrexActivate;
    TMCHTrexRenew;
    Hello
);
