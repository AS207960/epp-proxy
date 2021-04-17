//! Routes requests into and out of the EPP client by keeping track of

use std::collections::HashMap;

pub use super::Error;

pub type Response<T> = Result<T, Error>;

pub type HandleReqReturn<T> = Result<
    (
        super::proto::EPPCommandType,
        Option<Vec<super::proto::EPPCommandExtensionType>>,
    ),
    Response<T>,
>;

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
    ($($n:ident, $req:ty, $res:ty, $req_handle:path, $res_handle:path);*) => {
        /// Request into the EPP client, see sibling modules for explanation of requests
        #[derive(Debug)]
        pub enum Request {
            $($n(Box<$req>),)*
        }

        #[allow(non_snake_case)]
        #[derive(Default, Debug)]
        pub struct Router {
            $($n: HashMap<uuid::Uuid, super::Sender<$res>>,)*
        }

        impl Router {
            pub fn reject_request(req: Request) {
                match req {
                    $(Request::$n(req) => {let _ = req.return_path.send(Err(Error::NotReady));},)*
                };
            }

            pub async fn handle_request(&mut self, client: &super::EPPClientServerFeatures, req: Request) ->
             Option<(super::proto::EPPCommandType, Option<Vec<super::proto::EPPCommandExtensionType>>, uuid::Uuid)> {
                match req {
                    $(Request::$n(req) => {
                        let command_id = uuid::Uuid::new_v4();
                        let (command, extension) = match $req_handle(client, &req) {
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
                        self.$n.insert(command_id, req.return_path);
                        Some((command, extension, command_id))
                    },)*
                }
            }

            pub async fn handle_response(&mut self, transaction_id: &uuid::Uuid, response: Box<super::proto::EPPResponse>) -> Result<(), ()> {
                $(if let Some(return_path) = self.$n.remove(transaction_id) {
                    let _ = if !response.is_success() {
                        if response.is_server_error() {
                            return_path.send(Err(Error::Err(format!("Server error: {}", response.response_msg()))))
                        } else {
                            return_path.send(Err(Error::Err(response.response_msg())))
                        }
                    } else {
                        let trans_id = CommandTransactionID {
                                    client: response.transaction_id.client_transaction_id.as_deref().unwrap_or_default().to_owned(),
                                    server: response.transaction_id.server_transaction_id.as_deref().unwrap_or_default().to_owned(),
                                };
                        match $res_handle(*response) {
                            Ok(r) =>  return_path.send(Ok(CommandResponse {
                                response: r,
                                extra_values: vec![],
                                 transaction_id: Some(trans_id)
                            })),
                            Err(e) => return_path.send(Err(e))
                        }
                    };
                } else)* {}
                Ok(())
            }

            pub fn drain(&mut self) {
                $(for r in self.$n.drain() {
                    let _ = r.1.send(Err(Error::NotReady));
                })*
            }
        }
    }
}

router!(
    Logout,                 super::LogoutRequest,                       (),                                 super::handle_logout,                    super::handle_logout_response;
    Poll,                   super::poll::PollRequest,                   Option<super::poll::PollResponse>,  super::poll::handle_poll,                super::poll::handle_poll_response;
    PollAck,                super::poll::PollAckRequest,                super::poll::PollAckResponse,       super::poll::handle_poll_ack,            super::poll::handle_poll_ack_response;
    DomainCheck,            super::domain::CheckRequest,                super::domain::CheckResponse,       super::domain::handle_check,             super::domain::handle_check_response;
    DomainClaimsCheck,      super::domain::ClaimsCheckRequest,          super::domain::ClaimsCheckResponse, super::domain::handle_claims_check,      super::domain::handle_claims_check_response;
    DomainTrademarkCheck,   super::domain::TrademarkCheckRequest,       super::domain::ClaimsCheckResponse, super::domain::handle_trademark_check,   super::domain::handle_claims_check_response;
    DomainInfo,             super::domain::InfoRequest,                 super::domain::InfoResponse,        super::domain::handle_info,              super::domain::handle_info_response;
    DomainCreate,           super::domain::CreateRequest,               super::domain::CreateResponse,      super::domain::handle_create,            super::domain::handle_create_response;
    DomainDelete,           super::domain::DeleteRequest,               super::domain::DeleteResponse,      super::domain::handle_delete,            super::domain::handle_delete_response;
    DomainUpdate,           super::domain::UpdateRequest,               super::domain::UpdateResponse,      super::domain::handle_update,            super::domain::handle_update_response;
    DomainRenew,            super::domain::RenewRequest,                super::domain::RenewResponse,       super::domain::handle_renew,             super::domain::handle_renew_response;
    DomainTransferQuery,    super::domain::TransferQueryRequest,        super::domain::TransferResponse,    super::domain::handle_transfer_query,    super::domain::handle_transfer_response;
    DomainTransferRequest,  super::domain::TransferRequestRequest,      super::domain::TransferResponse,    super::domain::handle_transfer_request,  super::domain::handle_transfer_response;
    DomainTransferCancel,   super::domain::TransferAcceptRejectRequest, super::domain::TransferResponse,    super::domain::handle_transfer_cancel,   super::domain::handle_transfer_response;
    DomainTransferAccept,   super::domain::TransferAcceptRejectRequest, super::domain::TransferResponse,    super::domain::handle_transfer_accept,   super::domain::handle_transfer_response;
    DomainTransferReject,   super::domain::TransferAcceptRejectRequest, super::domain::TransferResponse,    super::domain::handle_transfer_reject,   super::domain::handle_transfer_response;
    RestoreRequest,         super::rgp::RestoreRequest,                 super::rgp::RestoreResponse,        super::rgp::handle_restore,              super::rgp::handle_restore_response;
    HostCheck,              super::host::CheckRequest,                  super::host::CheckResponse,         super::host::handle_check,               super::host::handle_check_response;
    HostInfo,               super::host::InfoRequest,                   super::host::InfoResponse,          super::host::handle_info,                super::host::handle_info_response;
    HostCreate,             super::host::CreateRequest,                 super::host::CreateResponse,        super::host::handle_create,              super::host::handle_create_response;
    HostDelete,             super::host::DeleteRequest,                 super::host::DeleteResponse,        super::host::handle_delete,              super::host::handle_delete_response;
    HostUpdate,             super::host::UpdateRequest,                 super::host::UpdateResponse,        super::host::handle_update,              super::host::handle_update_response;
    ContactCheck,           super::contact::CheckRequest,               super::contact::CheckResponse,      super::contact::handle_check,            super::contact::handle_check_response;
    ContactInfo,            super::contact::InfoRequest,                super::contact::InfoResponse,       super::contact::handle_info,             super::contact::handle_info_response;
    ContactCreate,          super::contact::CreateRequest,              super::contact::CreateResponse,     super::contact::handle_create,           super::contact::handle_create_response;
    ContactDelete,          super::contact::DeleteRequest,              super::contact::DeleteResponse,     super::contact::handle_delete,           super::contact::handle_delete_response;
    ContactUpdate,          super::contact::UpdateRequest,              super::contact::UpdateResponse,     super::contact::handle_update,           super::contact::handle_update_response;
    ContactTransferQuery,   super::contact::TransferQueryRequest,       super::contact::TransferResponse,   super::contact::handle_transfer_query,   super::contact::handle_transfer_response;
    ContactTransferRequest, super::contact::TransferRequestRequest,     super::contact::TransferResponse,   super::contact::handle_transfer_request, super::contact::handle_transfer_response;
    ContactTransferAccept,  super::contact::TransferRequestRequest,     super::contact::TransferResponse,   super::contact::handle_transfer_accept,  super::contact::handle_transfer_response;
    ContactTransferReject,  super::contact::TransferRequestRequest,     super::contact::TransferResponse,   super::contact::handle_transfer_reject,  super::contact::handle_transfer_response;
    NominetTagList,         super::nominet::TagListRequest,             super::nominet::TagListResponse,    super::nominet::handle_tag_list,         super::nominet::handle_tag_list_response;
    Balance,                super::balance::BalanceRequest,             super::balance::BalanceResponse,    super::balance::handle_balance,          super::balance::handle_balance_response;
    MaintenanceList,        super::maintenance::ListRequest,            super::maintenance::ListResponse,   super::maintenance::handle_list,         super::maintenance::handle_list_response;
    MaintenanceInfo,        super::maintenance::InfoRequest,            super::maintenance::InfoResponse,   super::maintenance::handle_info,         super::maintenance::handle_info_response
);
