use paste::paste;

pub use super::super::{router, Error, Response};

pub type HandleReqReturn<T> = Result<
    (
        super::proto::EPPCommandType,
        Option<Vec<super::proto::EPPCommandExtensionType>>,
    ),
    Response<T>,
>;

macro_rules! router {
    ($($n:ident, $req_handle:path, $res_handle:path);*) => {
        #[derive(Default, Debug)]
        pub struct Router {}

        impl router::InnerRouter for Router {
            type Request = (super::proto::EPPCommandType, Option<Vec<super::proto::EPPCommandExtensionType>>);
            type Response = super::proto::EPPResponse;

            paste! {
                $(fn [<$n _request>](&mut self, client: &super::ServerFeatures, req: &router::[<$n Request>], _command_id: uuid::Uuid) -> HandleReqReturn<router::[<$n Response>]> {
                    $req_handle(client, &req)
                })*

                $(fn [<$n _response>](&mut self, return_path: router::Sender<router::[<$n Response>]>, response: Self::Response) {
                    let _ = if !response.is_success() {
                        if response.is_server_error() {
                            return_path.send(Err(Error::Err(format!("Server error: {}", response.response_msg()))))
                        } else {
                            return_path.send(Err(Error::Err(response.response_msg())))
                        }
                    } else {
                        let trans_id = router::CommandTransactionID {
                            client: response.transaction_id.client_transaction_id.as_deref().unwrap_or_default().to_owned(),
                            server: response.transaction_id.server_transaction_id.as_deref().unwrap_or_default().to_owned(),
                        };
                        match $res_handle(response) {
                            Ok(r) =>  return_path.send(Ok(router::CommandResponse {
                                response: r,
                                extra_values: vec![],
                                 transaction_id: Some(trans_id)
                            })),
                            Err(e) => return_path.send(Err(e))
                        }
                    };
                })*
            }
        }
    }
}

router!(
    Logout,                 super::handle_logout,                     super::handle_logout_response;
    Poll,                   super::poll::handle_poll,                 super::poll::handle_poll_response;
    PollAck,                super::poll::handle_poll_ack,             super::poll::handle_poll_ack_response;
    DomainCheck,            super::domain::handle_check,              super::domain::handle_check_response;
    DomainClaimsCheck,      super::domain::handle_claims_check,       super::domain::handle_claims_check_response;
    DomainTrademarkCheck,   super::domain::handle_trademark_check,    super::domain::handle_claims_check_response;
    DomainInfo,             super::domain::handle_info,               super::domain::handle_info_response;
    DomainCreate,           super::domain::handle_create,             super::domain::handle_create_response;
    DomainDelete,           super::domain::handle_delete,             super::domain::handle_delete_response;
    DomainUpdate,           super::domain::handle_update,             super::domain::handle_update_response;
    DomainRenew,            super::domain::handle_renew,              super::domain::handle_renew_response;
    DomainTransferQuery,    super::domain::handle_transfer_query,     super::domain::handle_transfer_response;
    DomainTransferRequest,  super::domain::handle_transfer_request,   super::domain::handle_transfer_response;
    DomainTransferCancel,   super::domain::handle_transfer_cancel,    super::domain::handle_transfer_response;
    DomainTransferAccept,   super::domain::handle_transfer_accept,    super::domain::handle_transfer_response;
    DomainTransferReject,   super::domain::handle_transfer_reject,    super::domain::handle_transfer_response;
    VerisignSync,           super::domain::handle_verisign_sync,      super::domain::handle_update_response;
    RestoreRequest,         super::rgp::handle_restore,               super::rgp::handle_restore_response;
    HostCheck,              super::host::handle_check,                super::host::handle_check_response;
    HostInfo,               super::host::handle_info,                 super::host::handle_info_response;
    HostCreate,             super::host::handle_create,               super::host::handle_create_response;
    HostDelete,             super::host::handle_delete,               super::host::handle_delete_response;
    HostUpdate,             super::host::handle_update,               super::host::handle_update_response;
    ContactCheck,           super::contact::handle_check,             super::contact::handle_check_response;
    ContactInfo,            super::contact::handle_info,              super::contact::handle_info_response;
    ContactCreate,          super::contact::handle_create,            super::contact::handle_create_response;
    ContactDelete,          super::contact::handle_delete,            super::contact::handle_delete_response;
    ContactUpdate,          super::contact::handle_update,            super::contact::handle_update_response;
    ContactTransferQuery,   super::contact::handle_transfer_query,    super::contact::handle_transfer_response;
    ContactTransferRequest, super::contact::handle_transfer_request,  super::contact::handle_transfer_response;
    ContactTransferAccept,  super::contact::handle_transfer_accept,   super::contact::handle_transfer_response;
    ContactTransferReject,  super::contact::handle_transfer_reject,   super::contact::handle_transfer_response;
    NominetTagList,         super::nominet::handle_tag_list,          super::nominet::handle_tag_list_response;
    NominetAccept,          super::nominet::handle_accept,            super::nominet::handle_handshake_response;
    NominetReject,          super::nominet::handle_reject,            super::nominet::handle_handshake_response;
    NominetRelease,         super::nominet::handle_release,           super::nominet::handle_release_response;
    Balance,                super::balance::handle_balance,           super::balance::handle_balance_response;
    MaintenanceList,        super::maintenance::handle_list,          super::maintenance::handle_list_response;
    MaintenanceInfo,        super::maintenance::handle_info,          super::maintenance::handle_info_response;
    EURIDHitPoints,         super::eurid::handle_hit_points,          super::eurid::handle_hit_points_response;
    EURIDRegistrationLimit, super::eurid::handle_registration_limits, super::eurid::handle_registration_limits_response;
    EURIDDNSSECEligibility, super::eurid::handle_dnssec_eligibility,  super::eurid::handle_dnssec_eligibility_response;
    EURIDDNSQuality,        super::eurid::handle_dns_quality,         super::eurid::handle_dns_quality_response
);
