mod domain;
mod contact;
mod host;

use crate::client;

pub struct RPPProxy {
    pub auth: Box<dyn crate::auth::Auth + Send + Sync>,
    pub client_router: super::Router,
}

impl RPPProxy {
    pub async fn launch(self, addr: std::net::SocketAddr) {
        let figment = rocket::Config::figment()
            .merge(("address", addr.ip()))
            .merge(("port", addr.port()));

        rocket::custom(figment)
            .manage(self)
            .mount("/", rocket::routes![
                domain::domain_check,
                domain::domain_create,
                domain::domain_delete,

                contact::contact_check,
                contact::contact_create,
                contact::contact_delete,

                host::host_check,
                host::host_create,
                host::host_delete,
            ])
            .launch()
            .await
            .unwrap();
    }
}

#[derive(Debug)]
struct HeaderInfo {
    client_transaction_id: Option<String>,
    authorization_info: Option<String>,
    roid: Option<String>,
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for HeaderInfo {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        rocket::request::Outcome::Success(Self {
            client_transaction_id: request.headers().get_one("RPP-Cltrid").map(|s| s.to_owned()),
            authorization_info: request.headers().get_one("RPP-AuthInfo").map(|s| s.to_owned()),
            roid: request.headers().get_one("RPP-Roid").map(|s| s.to_owned()),
        })
    }
}

#[derive(Debug)]
struct Response<R> {
    server_transaction_id: String,
    client_transaction_id: String,
    response_code: crate::proto::EPPResultCode,
    check_availability: Option<bool>,
    queue_size: Option<u32>,
    body: Option<R>,
}

impl<R, T> From<&client::CommandResponse<T>> for Response<R> {
    fn from(resp: &client::CommandResponse<T>) -> Self {
        Response {
            client_transaction_id: resp.transaction_id.as_ref().map(|t| t.client.clone()).unwrap_or_default(),
            server_transaction_id: resp.transaction_id.as_ref().map(|t| t.server.clone()).unwrap_or_default(),
            response_code: resp.result_code,
            check_availability: None,
            queue_size: None,
            body: None,
        }
    }
}

impl<'r, R: rocket::serde::Serialize> rocket::response::Responder<'r, 'static> for Response<R> {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut resp = match self.body {
            Some(b) => {
                let mut r = rocket::Response::build_from(rocket::serde::json::Json(b).respond_to(req)?);
                r.header(rocket::http::ContentType::new("application", "rpp+json"));
                r
            },
            None => {
                let mut r = rocket::Response::build();
                r.status(rocket::http::Status::NoContent);
                r
            }
        };
        resp.raw_header("RPP-Svtrid", self.server_transaction_id);
        resp.raw_header("RPP-Cltrid", self.client_transaction_id);
        resp.raw_header("RPP-Code", u16::from(self.response_code).to_string());
        if let Some(check_availability) = self.check_availability {
            resp.raw_header("RPP-Check-Avail", check_availability.to_string());
        }
        if let Some(queue_size) = self.queue_size {
            resp.raw_header("RPP-Queue-Size", queue_size.to_string());
        }
        match self.response_code {
            crate::proto::EPPResultCode::Success => resp.status(rocket::http::Status::Ok),
            crate::proto::EPPResultCode::SuccessActionPending => resp.status(rocket::http::Status::Accepted),
            crate::proto::EPPResultCode::SuccessNoMessages => resp.status(rocket::http::Status::Ok),
            crate::proto::EPPResultCode::SuccessAckToDequeue => resp.status(rocket::http::Status::Ok),
            crate::proto::EPPResultCode::SuccessEndingSession => resp.status(rocket::http::Status::Ok),
            crate::proto::EPPResultCode::UnknownCommand => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::CommandSyntaxError => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::CommandUseError => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::RequiredParameterMissing => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::ParameterValueRangeError => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::ParameterValueSyntaxError => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::UnimplementedProtocolVersion => resp.status(rocket::http::Status::NotImplemented),
            crate::proto::EPPResultCode::UnimplementedCommand => resp.status(rocket::http::Status::NotImplemented),
            crate::proto::EPPResultCode::UnimplementedOption => resp.status(rocket::http::Status::NotImplemented),
            crate::proto::EPPResultCode::UnimplementedExtension => resp.status(rocket::http::Status::NotImplemented),
            crate::proto::EPPResultCode::BillingFailure => resp.status(rocket::http::Status::PaymentRequired),
            crate::proto::EPPResultCode::ObjectNotEligibleForRenewal => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::ObjectNotEligibleForTransfer => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::AuthenticationError => resp.status(rocket::http::Status::Forbidden),
            crate::proto::EPPResultCode::AuthorizationError => resp.status(rocket::http::Status::Forbidden),
            crate::proto::EPPResultCode::InvalidAuthorization => resp.status(rocket::http::Status::Forbidden),
            crate::proto::EPPResultCode::ObjectPendingTransfer => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::ObjectNotPendingTransfer => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::ObjectExists => resp.status(rocket::http::Status::Conflict),
            crate::proto::EPPResultCode::ObjectDoesNotExist => resp.status(rocket::http::Status::NotFound),
            crate::proto::EPPResultCode::ObjectStatusProhibitsOperation => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::ObjectAssociationProhibitsOperation => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::ParameterValuePolicyError => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::UnimplementedObjectService => resp.status(rocket::http::Status::NotImplemented),
            crate::proto::EPPResultCode::DataManagementPolicyViolation => resp.status(rocket::http::Status::BadRequest),
            crate::proto::EPPResultCode::CommandFailed => resp.status(rocket::http::Status::InternalServerError),
            crate::proto::EPPResultCode::CommandFailedServerClosingConnection => resp.status(rocket::http::Status::InternalServerError),
            crate::proto::EPPResultCode::AuthenticationServerClosingConnection => resp.status(rocket::http::Status::InternalServerError),
            crate::proto::EPPResultCode::SessionLimitExceededServerClosingConnection => resp.status(rocket::http::Status::InternalServerError),
            crate::proto::EPPResultCode::Other(_) => resp.status(rocket::http::Status::Ok),
        };
        resp.ok()
    }
}

struct Authorized;

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for Authorized {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let rpp_proxy = request.guard::<&rocket::State<RPPProxy>>().await.unwrap();
        let auth_header = match request.headers().get_one("authorization") {
            Some(auth_header) => auth_header.trim(),
            None => return rocket::request::Outcome::Error((rocket::http::Status::Unauthorized, ()))
        };
        let auth_token = if let Some(auth_token) = auth_header.strip_prefix("Bearer ") {
            auth_token.trim()
        } else {
            return rocket::request::Outcome::Error((rocket::http::Status::Unauthorized, ()))
        };
        if rpp_proxy.auth.auth(auth_token).await {
            rocket::request::Outcome::Success(Authorized)
        } else {
            rocket::request::Outcome::Error((rocket::http::Status::Unauthorized, ()))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct AuthInfo {
    pw: String,
}

#[macro_export]
macro_rules! rpp_method {
    (
        name = $name:ident,
        method = $method:ident,
        url = $url:literal,
        return_type = $ret:ty,
        handler = $call:expr
    ) => {
        crate::rpp_method!(name = $name, method = $method, url = $url, return_type = $ret, handler = $call, args = );
    };
    (
        name = $name:ident,
        method = $method:ident,
        url = $url:literal,
        data_type = $body:ty,
        return_type = $ret:ty,
        handler = $call:expr
    ) => {
        crate::rpp_method!(name = $name, method = $method, url = $url, data_type = $body, return_type = $ret, handler = $call, args = );
    };
    (
        name = $name:ident,
        method = $method:ident,
        url = $url:literal,
        return_type = $ret:ty,
        handler = $call:expr,
        args = $($param:ident: $param_type:ty),*
    ) => {
        #[rocket::route($method, uri = $url)]
        pub async fn $name(
            client: &rocket::State<crate::rpp::RPPProxy>, _auth: crate::rpp::Authorized, header_info: crate::rpp::HeaderInfo, registry_id: &str,
            $($param: $param_type,)*
        ) -> Result<Response<$ret>, rocket::http::Status> {
            let client = match client.client_router.client_by_id(registry_id) {
                Some(client) => client,
                None => return Err(rocket::http::Status::NotFound),
            };
            match ($call)(client, header_info, $($param,)*).await {
                Ok(r) => Ok(r),
                Err(e) => {
                    match e {
                        client::Error::NotReady => Err(rocket::http::Status::ServiceUnavailable),
                        client::Error::Unsupported => Err(rocket::http::Status::NotImplemented),
                        client::Error::ServerInternal => Err(rocket::http::Status::InternalServerError),
                        client::Error::Timeout => Err(rocket::http::Status::GatewayTimeout),
                        client::Error::InvalidRequest(e) => {
                            warn!("Invalid request: {}", e);
                            Err(rocket::http::Status::BadRequest)
                        },
                        client::Error::Err { result_code, transaction_id, .. } => Ok(Response {
                            client_transaction_id: transaction_id.as_ref().map(|t| t.client.clone()).unwrap_or_default(),
                            server_transaction_id: transaction_id.as_ref().map(|t| t.client.clone()).unwrap_or_default(),
                            response_code: result_code,
                            check_availability: None,
                            queue_size: None,
                            body: None,
                        })
                    }
                }
            }
        }
    };
    (
        name = $name:ident,
        method = $method:ident,
        url = $url:literal,
        data_type = $body:ty,
        return_type = $ret:ty,
        handler = $call:expr,
        args = $($param:ident: $param_type:ty),*
    ) => {
        #[rocket::route($method, uri = $url, data = "<request>")]
        pub async fn $name(
            client: &rocket::State<crate::rpp::RPPProxy>, _auth: crate::rpp::Authorized, header_info: crate::rpp::HeaderInfo, registry_id: &str,
            request: rocket::serde::json::Json<$body>,
            $($param: $param_type,)*
        ) -> Result<Response<$ret>, rocket::http::Status> {
            let client = match client.client_router.client_by_id(registry_id) {
                Some(client) => client,
                None => return Err(rocket::http::Status::NotFound),
            };
            match ($call)(client, header_info, request.into_inner(), $($param,)*).await {
                Ok(r) => Ok(r),
                Err(e) => {
                    match e {
                        client::Error::NotReady => Err(rocket::http::Status::ServiceUnavailable),
                        client::Error::Unsupported => Err(rocket::http::Status::NotImplemented),
                        client::Error::ServerInternal => Err(rocket::http::Status::InternalServerError),
                        client::Error::Timeout => Err(rocket::http::Status::GatewayTimeout),
                        client::Error::InvalidRequest(e) => {
                            warn!("Invalid request: {}", e);
                            Err(rocket::http::Status::BadRequest)
                        },
                        client::Error::Err { result_code, transaction_id, .. } => Ok(Response {
                            client_transaction_id: transaction_id.as_ref().map(|t| t.client.clone()).unwrap_or_default(),
                            server_transaction_id: transaction_id.as_ref().map(|t| t.client.clone()).unwrap_or_default(),
                            response_code: result_code,
                            check_availability: None,
                            queue_size: None,
                            body: None,
                        })
                    }
                }
            }
        }
    };
}



