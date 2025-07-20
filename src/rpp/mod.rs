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
            .register("/rpp/v0", rocket::catchers![
                unauthorized_catcher
            ])
            .mount("/rpp/v0", rocket::routes![
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
            client_transaction_id: request.headers().get_one("rpp-cltrid").map(|s| s.to_owned()),
            authorization_info: request.headers().get_one("rpp-authinfo").map(|s| s.to_owned()),
            roid: request.headers().get_one("rpp-roid").map(|s| s.to_owned()),
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
        resp.raw_header("rpp-svtrid", self.server_transaction_id);
        resp.raw_header("rpp-cltrid", self.client_transaction_id);
        resp.raw_header("rpp-code", u16::from(self.response_code).to_string());
        if let Some(check_availability) = self.check_availability {
            resp.raw_header("rpp-check-avail", check_availability.to_string());
        }
        if let Some(queue_size) = self.queue_size {
            resp.raw_header("rpp-queue-size", queue_size.to_string());
        }
        resp.status(map_status(self.response_code));
        resp.ok()
    }
}

fn map_status(status: crate::proto::EPPResultCode) -> rocket::http::Status {
    match status {
        crate::proto::EPPResultCode::Success => rocket::http::Status::Ok,
        crate::proto::EPPResultCode::SuccessActionPending => rocket::http::Status::Accepted,
        crate::proto::EPPResultCode::SuccessNoMessages => rocket::http::Status::Ok,
        crate::proto::EPPResultCode::SuccessAckToDequeue => rocket::http::Status::Ok,
        crate::proto::EPPResultCode::SuccessEndingSession => rocket::http::Status::Ok,
        crate::proto::EPPResultCode::UnknownCommand => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::CommandSyntaxError => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::CommandUseError => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::RequiredParameterMissing => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::ParameterValueRangeError => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::ParameterValueSyntaxError => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::UnimplementedProtocolVersion => rocket::http::Status::NotImplemented,
        crate::proto::EPPResultCode::UnimplementedCommand => rocket::http::Status::NotImplemented,
        crate::proto::EPPResultCode::UnimplementedOption => rocket::http::Status::NotImplemented,
        crate::proto::EPPResultCode::UnimplementedExtension => rocket::http::Status::NotImplemented,
        crate::proto::EPPResultCode::BillingFailure => rocket::http::Status::PaymentRequired,
        crate::proto::EPPResultCode::ObjectNotEligibleForRenewal => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::ObjectNotEligibleForTransfer => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::AuthenticationError => rocket::http::Status::Forbidden,
        crate::proto::EPPResultCode::AuthorizationError => rocket::http::Status::Forbidden,
        crate::proto::EPPResultCode::InvalidAuthorization => rocket::http::Status::Forbidden,
        crate::proto::EPPResultCode::ObjectPendingTransfer => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::ObjectNotPendingTransfer => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::ObjectExists => rocket::http::Status::Conflict,
        crate::proto::EPPResultCode::ObjectDoesNotExist => rocket::http::Status::NotFound,
        crate::proto::EPPResultCode::ObjectStatusProhibitsOperation => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::ObjectAssociationProhibitsOperation => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::ParameterValuePolicyError => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::UnimplementedObjectService => rocket::http::Status::NotImplemented,
        crate::proto::EPPResultCode::DataManagementPolicyViolation => rocket::http::Status::BadRequest,
        crate::proto::EPPResultCode::CommandFailed => rocket::http::Status::InternalServerError,
        crate::proto::EPPResultCode::CommandFailedServerClosingConnection => rocket::http::Status::InternalServerError,
        crate::proto::EPPResultCode::AuthenticationServerClosingConnection => rocket::http::Status::InternalServerError,
        crate::proto::EPPResultCode::SessionLimitExceededServerClosingConnection => rocket::http::Status::InternalServerError,
        crate::proto::EPPResultCode::Other(_) => rocket::http::Status::Ok,
    }
}

#[derive(Debug)]
pub(crate) struct ErrorResponse {
    http_status: rocket::http::Status,
    error_type: ErrorType,
    title: String,
    detail: String,
    server_transaction_id: Option<String>,
    client_transaction_id: Option<String>,
}

#[derive(Debug)]
enum ErrorType {
    EPP(crate::proto::EPPResultCode),
    Custom(String)
}

#[derive(Debug, serde::Serialize)]
struct ProblemDetail {
    #[serde(rename = "type")]
    error_type: String,
    title: String,
    status: u16,
    detail: String,
}

impl<'r> rocket::response::Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let epp_code = match &self.error_type {
            ErrorType::EPP(code) => Some(*code),
            _ => None
        };
        let problem_detail = ProblemDetail {
            error_type: match self.error_type {
                ErrorType::EPP(code) => format!("urn:ietf:params:rpp:code:{}", u16::from(code)),
                ErrorType::Custom(code) => code,
            },
            title: self.title,
            status: self.http_status.code,
            detail: self.detail,
        };
        let mut resp = rocket::Response::build_from(rocket::serde::json::Json(problem_detail).respond_to(req)?);
        resp.header(rocket::http::ContentType::new("application", "problem+json"));
        if let Some(epp_code) = epp_code {
            resp.raw_header("rpp-code", u16::from(epp_code).to_string());
        }
        if let Some(server_transaction_id) = self.server_transaction_id {
            resp.raw_header("rpp-svtrid", server_transaction_id);
        }
        if let Some(client_transaction_id) = self.client_transaction_id {
            resp.raw_header("rpp-cltrid", client_transaction_id);
        }
        resp.status(self.http_status);
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

pub(crate) fn get_client(client: &RPPProxy, registry_id: &str) -> Result<client::RequestSender, ErrorResponse> {
    match client.client_router.client_by_id(registry_id) {
        Some(client) => Ok(client),
        None => Err(ErrorResponse {
            http_status: rocket::http::Status::NotFound,
            error_type: ErrorType::Custom("https://as207960.ltd.uk/epp-proxy/error/unknown-registry".to_string()),
            title: "Unknown registry".to_string(),
            detail: "The requested EPP server isn't known".to_string(),
            server_transaction_id: None,
            client_transaction_id: None,
        }),
    }
}

pub(crate) fn map_error(err: client::Error) -> ErrorResponse {
    match err {
        client::Error::NotReady => ErrorResponse {
            http_status: rocket::http::Status::ServiceUnavailable,
            error_type: ErrorType::Custom("https://as207960.ltd.uk/epp-proxy/error/not-ready".to_string()),
            title: "Not yet ready".to_string(),
            detail: "The connection to the EPP server isn't up yet".to_string(),
            server_transaction_id: None,
            client_transaction_id: None,
        },
        client::Error::Unsupported => ErrorResponse {
            http_status: rocket::http::Status::NotImplemented,
            error_type: ErrorType::EPP(crate::proto::EPPResultCode::UnimplementedCommand),
            title: "Unsupported request".to_string(),
            detail: "The EPP server doesn't support the request".to_string(),
            server_transaction_id: None,
            client_transaction_id: None,
        },
        client::Error::ServerInternal => ErrorResponse {
            http_status: rocket::http::Status::InternalServerError,
            error_type: ErrorType::EPP(crate::proto::EPPResultCode::CommandFailed),
            title: "Internal Server Error".to_string(),
            detail: "Something went very wrong".to_string(),
            server_transaction_id: None,
            client_transaction_id: None,
        },
        client::Error::Timeout => ErrorResponse {
            http_status: rocket::http::Status::GatewayTimeout,
            error_type: ErrorType::Custom("https://as207960.ltd.uk/epp-proxy/error/timeout".to_string()),
            title: "Timeout".to_string(),
            detail: "The EPP server didn't respond in time to the command".to_string(),
            server_transaction_id: None,
            client_transaction_id: None,
        },
        client::Error::InvalidRequest(e) => ErrorResponse {
            http_status: rocket::http::Status::BadRequest,
            error_type: ErrorType::EPP(crate::proto::EPPResultCode::CommandUseError),
            title: "Invalid request".to_string(),
            detail: e,
            server_transaction_id: None,
            client_transaction_id: None,
        },
        client::Error::Err { result_code, text, message, transaction_id, .. } => ErrorResponse {
            http_status: map_status(result_code),
            error_type: ErrorType::EPP(result_code),
            title: message,
            detail: text,
            server_transaction_id: transaction_id.as_ref().map(|t| t.server.clone()),
            client_transaction_id: transaction_id.as_ref().map(|t| t.client.clone()),
        }
    }
}

pub(crate) fn map_json_error(err: rocket::serde::json::Error) -> ErrorResponse {
    ErrorResponse {
        http_status: rocket::http::Status::UnprocessableEntity,
        error_type: ErrorType::EPP(crate::proto::EPPResultCode::CommandSyntaxError),
        title: "Invalid command syntax".to_string(),
        detail: err.to_string(),
        server_transaction_id: None,
        client_transaction_id: None,
    }
}

#[rocket::catch(401)]
fn unauthorized_catcher() -> ErrorResponse {
    ErrorResponse {
        http_status: rocket::http::Status::Unauthorized,
        error_type: ErrorType::EPP(crate::proto::EPPResultCode::AuthenticationError),
        title: "Unauthorized".to_string(),
        detail: "This API requires authentication to access".to_string(),
        server_transaction_id: None,
        client_transaction_id: None,
    }
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
        ) -> Result<crate::rpp::Response<$ret>, crate::rpp::ErrorResponse> {
            let client = crate::rpp::get_client(client, registry_id)?;
            ($call)(client, header_info, $($param,)*).await.map_err(crate::rpp::map_error)
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
        #[rocket::route($method, uri = $url, data = "<data>")]
        pub async fn $name(
            client: &rocket::State<crate::rpp::RPPProxy>, _auth: crate::rpp::Authorized, header_info: crate::rpp::HeaderInfo, registry_id: &str,
            data: Result<rocket::serde::json::Json::<$body>, rocket::serde::json::Error<'_>>,
            $($param: $param_type,)*
        ) -> Result<crate::rpp::Response<$ret>, crate::rpp::ErrorResponse> {
            let data = match data {
                Ok(data) => data.into_inner(),
                Err(err) => return Err(crate::rpp::map_json_error(err))
            };
            let client = crate::rpp::get_client(client, registry_id)?;
            ($call)(client, header_info, data, $($param,)*).await.map_err(crate::rpp::map_error)
        }
    };
}



