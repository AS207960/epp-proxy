//! Routes requests into and out of the EPP client by keeping track of

use std::collections::HashMap;

/// Responses from EPP client, see [`super::Error`] for explanations of errors
#[derive(Debug)]
pub enum Response<T> {
    Ok(T),
    Err(String),
    NotReady,
    Unsupported,
    InternalServerError,
}

macro_rules! router {
    ($($n:ident, $req:path, $res:path, $req_handle:path, $res_handle:path);*) => {
        /// Request into the EPP client, see sibling modules for explanation of requests
        #[derive(Debug)]
        pub enum Request {
            $($n($req),)*
        }

        #[allow(non_snake_case)]
        #[derive(Default, Debug)]
        pub struct Router {
            $($n: HashMap<uuid::Uuid, super::Sender<$res>>,)*
        }

        impl Router {
            pub fn reject_request(req: Request) {
                match req {
                    $(Request::$n(req) => {let _ = req.return_path.send(super::Response::NotReady);},)*
                };
            }

            pub async fn handle_request(&mut self, client: &super::EPPClientServerFeatures, req: Request) -> Option<(super::proto::EPPCommandType, uuid::Uuid)> {
                match req {
                    $(Request::$n(req) => {
                        let command_id = uuid::Uuid::new_v4();
                        let command = match $req_handle(client, &req) {
                            Ok(c) => c,
                            Err(e) => {
                                let _ = req.return_path.send(e);
                                return None
                            }
                        };
                        self.$n.insert(command_id, req.return_path);
                        Some((command, command_id))
                    },)*
                }
            }

            pub async fn handle_response(&mut self, transaction_id: &uuid::Uuid, response: super::proto::EPPResponse) -> Result<(), ()> {
                $(if let Some(return_path) = self.$n.remove(transaction_id) {
                    let _ = if !response.is_success() {
                        if response.is_server_error() {
                            return_path.send(Response::InternalServerError)
                        } else {
                            return_path.send(Response::Err(response.response_msg()))
                        }
                    } else {
                        return_path.send($res_handle(response))
                    };
                } else)* {}
                Ok(())
            }

            pub fn drain(&mut self) {
                $(for r in self.$n.drain() {
                    let _ = r.1.send(super::Response::NotReady);
                })*
            }
        }
    }
}

router!(
    DomainCheck,  super::domain::CheckRequest,  super::domain::CheckResponse,  super::domain::handle_check,  super::domain::handle_check_response;
    DomainInfo,   super::domain::InfoRequest,   super::domain::InfoResponse,   super::domain::handle_info,   super::domain::handle_info_response;
    HostCheck,    super::host::CheckRequest,    super::host::CheckResponse,    super::host::handle_check,    super::host::handle_check_response;
    HostInfo,     super::host::InfoRequest,     super::host::InfoResponse,     super::host::handle_info,     super::host::handle_info_response;
    HostCreate,   super::host::CreateRequest,   super::host::CreateResponse,   super::host::handle_create,   super::host::handle_create_response;
    HostDelete,   super::host::DeleteRequest,   super::host::DeleteResponse,   super::host::handle_delete,   super::host::handle_delete_response;
    HostUpdate,   super::host::UpdateRequest,   super::host::UpdateResponse,   super::host::handle_update,   super::host::handle_update_response;
    ContactCheck, super::contact::CheckRequest, super::contact::CheckResponse, super::contact::handle_check, super::contact::handle_check_response;
    ContactInfo,  super::contact::InfoRequest,  super::contact::InfoResponse,  super::contact::handle_info,  super::contact::handle_info_response
);
