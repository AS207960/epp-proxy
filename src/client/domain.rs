//! EPP commands relating to domain objects

use super::router::HandleReqReturn;
use super::{proto, EPPClientServerFeatures, Request, Response, Sender};
use chrono::prelude::*;

#[derive(Debug)]
pub struct CheckRequest {
    name: String,
    pub return_path: Sender<CheckResponse>,
}

/// Response to a domain check query
#[derive(Debug)]
pub struct CheckResponse {
    /// Is the domain available for registration
    pub avail: bool,
    /// An optional reason for the domain's status
    pub reason: Option<String>,
}

#[derive(Debug)]
pub struct InfoRequest {
    name: String,
    pub return_path: Sender<InfoResponse>,
}

/// Response to a domain info query
#[derive(Debug)]
pub struct InfoResponse {
    /// Domain name in question
    pub name: String,
    /// Internal registry ID
    pub registry_id: String,
    /// Statuses attached to the domain
    pub statuses: Vec<Status>,
    /// Contact ID of the registrant
    pub registrant: String,
    /// Additional contacts on the domain
    pub contacts: Vec<InfoContact>,
    /// Nameservers for the domain
    pub nameservers: Vec<InfoNameserver>,
    /// Host names attached to the domain
    pub hosts: Vec<String>,
    /// Sponsoring client ID
    pub client_id: String,
    /// ID of the client that originally registered the domain
    pub client_created_id: Option<String>,
    /// Date of initial registration
    pub creation_date: Option<DateTime<Utc>>,
    /// Date of registration expiration
    pub expiry_date: Option<DateTime<Utc>>,
    /// ID of the client that last updated the domain
    pub last_updated_client: Option<String>,
    /// Date of last update
    pub last_updated_date: Option<DateTime<Utc>>,
    /// Date of last transfer
    pub last_transfer_date: Option<DateTime<Utc>>,
    /// Redemption grace period state of the domain
    pub rgp_state: super::rgp::RGPState,
}

/// Additional contact associated with a domain
#[derive(Debug)]
pub struct InfoContact {
    /// Type of contact
    pub contact_type: String,
    /// Contact ID of the contact
    pub contact_id: String,
}

/// Nameserver associated with a domain
#[derive(Debug)]
pub enum InfoNameserver {
    /// Host only type
    HostOnly(String),
    /// Host name with glue records
    HostAndAddress {
        host: String,
        address: String,
        ip_version: InfoNameserverAddressVersion,
    },
}

#[derive(Debug)]
pub struct CreateRequest {
    name: String,
    period: Option<Period>,
    registrant: String,
    contacts: Vec<InfoContact>,
    nameservers: Vec<InfoNameserver>,
    auth_info: String,
    pub return_path: Sender<CreateResponse>,
}

/// Domain registration period
#[derive(Debug)]
pub struct Period {
    /// Unit of time
    pub unit: PeriodUnit,
    /// Number of units of time
    pub value: u32,
}

/// Domain registration period time unit
#[derive(Debug)]
pub enum PeriodUnit {
    Years,
    Months,
}

#[derive(Debug)]
pub struct CreateResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    /// What date did the server log as the date of creation
    pub creation_date: DateTime<Utc>,
    /// When will the domain expire
    pub expiration_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub enum InfoNameserverAddressVersion {
    IPv4,
    IPv6,
}

#[derive(Debug)]
pub struct DeleteRequest {
    name: String,
    pub return_path: Sender<DeleteResponse>,
}

#[derive(Debug)]
pub struct DeleteResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
}

#[derive(Debug)]
pub struct UpdateRequest {
    name: String,
    add: Vec<UpdateObject>,
    remove: Vec<UpdateObject>,
    new_registrant: Option<String>,
    new_auth_info: Option<String>,
    pub return_path: Sender<UpdateResponse>,
}

#[derive(Debug)]
pub enum UpdateObject {
    Status(Status),
    Contact(InfoContact),
    Nameserver(InfoNameserver),
}

#[derive(Debug)]
pub struct UpdateResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
}

#[derive(Debug)]
pub struct RenewRequest {
    name: String,
    add_period: Option<Period>,
    cur_expiry_date: DateTime<Utc>,
    pub return_path: Sender<RenewResponse>,
}

#[derive(Debug)]
pub struct RenewResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    pub new_expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct TransferQueryRequest {
    name: String,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferRequestRequest {
    name: String,
    auth_info: String,
    add_period: Option<Period>,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    pub status: super::TransferStatus,
    /// Which client requested the transfer
    pub requested_client_id: String,
    /// The date of the transfer request
    pub requested_date: DateTime<Utc>,
    /// Whcich client last acted / needs to act
    pub act_client_id: String,
    /// Date on which a client acted / must act by
    pub act_date: DateTime<Utc>,
    /// New domain expiry date if amended by the transfer
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub enum Status {
    ClientDeleteProhibited,
    ClientHold,
    ClientRenewProhibited,
    ClientTransferProhibited,
    ClientUpdateProhibited,
    Inactive,
    Ok,
    PendingCreate,
    PendingDelete,
    PendingRenew,
    PendingTransfer,
    PendingUpdate,
    ServerDeleteProhibited,
    ServerHold,
    ServerRenewProhibited,
    ServerTransferProhibited,
    ServerUpdateProhibited,
}

impl From<proto::domain::EPPDomainStatusType> for Status {
    fn from(from: proto::domain::EPPDomainStatusType) -> Self {
        use proto::domain::EPPDomainStatusType;
        match from {
            EPPDomainStatusType::ClientDeleteProhibited => Status::ClientDeleteProhibited,
            EPPDomainStatusType::ClientHold => Status::ClientHold,
            EPPDomainStatusType::ClientRenewProhibited => Status::ClientRenewProhibited,
            EPPDomainStatusType::ClientTransferProhibited => Status::ClientTransferProhibited,
            EPPDomainStatusType::ClientUpdateProhibited => Status::ClientUpdateProhibited,
            EPPDomainStatusType::Inactive => Status::Inactive,
            EPPDomainStatusType::Ok => Status::Ok,
            EPPDomainStatusType::PendingCreate => Status::PendingCreate,
            EPPDomainStatusType::PendingDelete => Status::PendingDelete,
            EPPDomainStatusType::PendingRenew => Status::PendingRenew,
            EPPDomainStatusType::PendingTransfer => Status::PendingTransfer,
            EPPDomainStatusType::PendingUpdate => Status::PendingUpdate,
            EPPDomainStatusType::ServerDeleteProhibited => Status::ServerDeleteProhibited,
            EPPDomainStatusType::ServerHold => Status::ServerHold,
            EPPDomainStatusType::ServerRenewProhibited => Status::ServerRenewProhibited,
            EPPDomainStatusType::ServerTransferProhibited => Status::ServerTransferProhibited,
            EPPDomainStatusType::ServerUpdateProhibited => Status::ServerUpdateProhibited,
        }
    }
}

impl From<&Status> for proto::domain::EPPDomainStatusType {
    fn from(from: &Status) -> Self {
        use proto::domain::EPPDomainStatusType;
        match from {
            Status::ClientDeleteProhibited => EPPDomainStatusType::ClientDeleteProhibited,
            Status::ClientHold => EPPDomainStatusType::ClientHold,
            Status::ClientRenewProhibited => EPPDomainStatusType::ClientRenewProhibited,
            Status::ClientTransferProhibited => EPPDomainStatusType::ClientTransferProhibited,
            Status::ClientUpdateProhibited => EPPDomainStatusType::ClientUpdateProhibited,
            Status::Inactive => EPPDomainStatusType::Inactive,
            Status::Ok => EPPDomainStatusType::Ok,
            Status::PendingCreate => EPPDomainStatusType::PendingCreate,
            Status::PendingDelete => EPPDomainStatusType::PendingDelete,
            Status::PendingRenew => EPPDomainStatusType::PendingRenew,
            Status::PendingTransfer => EPPDomainStatusType::PendingTransfer,
            Status::PendingUpdate => EPPDomainStatusType::PendingUpdate,
            Status::ServerDeleteProhibited => EPPDomainStatusType::ServerDeleteProhibited,
            Status::ServerHold => EPPDomainStatusType::ServerHold,
            Status::ServerRenewProhibited => EPPDomainStatusType::ServerRenewProhibited,
            Status::ServerTransferProhibited => EPPDomainStatusType::ServerTransferProhibited,
            Status::ServerUpdateProhibited => EPPDomainStatusType::ServerUpdateProhibited,
        }
    }
}

impl From<&InfoNameserver> for proto::domain::EPPDomainInfoNameserverSer {
    fn from(from: &InfoNameserver) -> Self {
        match from {
            InfoNameserver::HostOnly(h) => {
                proto::domain::EPPDomainInfoNameserverSer::HostOnly(h.to_string())
            }
            InfoNameserver::HostAndAddress {
                host,
                address,
                ip_version,
            } => proto::domain::EPPDomainInfoNameserverSer::HostAndAddress {
                host: host.to_string(),
                address: proto::domain::EPPDomainInfoNameserverAddressSer {
                    address: address.to_string(),
                    ip_version: match ip_version {
                        InfoNameserverAddressVersion::IPv4 => {
                            proto::domain::EPPDomainInfoNameserverAddressVersion::IPv4
                        }
                        InfoNameserverAddressVersion::IPv6 => {
                            proto::domain::EPPDomainInfoNameserverAddressVersion::IPv6
                        }
                    },
                },
            },
        }
    }
}

impl From<&Period> for proto::domain::EPPDomainPeriod {
    fn from(from: &Period) -> Self {
        proto::domain::EPPDomainPeriod {
            unit: match from.unit {
                PeriodUnit::Months => proto::domain::EPPDomainPeriodUnit::Months,
                PeriodUnit::Years => proto::domain::EPPDomainPeriodUnit::Years,
            },
            value: from.value.to_string(),
        }
    }
}

fn check_id<T>(id: &str) -> Result<(), Response<T>> {
    if let 3..=16 = id.len() {
        Ok(())
    } else {
        Err(Response::Err(
            "contact id has a min length of 3 and a max length of 16".to_string(),
        ))
    }
}

pub fn handle_check(
    client: &EPPClientServerFeatures,
    req: &CheckRequest,
) -> HandleReqReturn<CheckResponse> {
    if !client.domain_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err(
            "domain name has a min length of 1".to_string(),
        ));
    }
    let command = proto::EPPCheck::Domain(proto::domain::EPPDomainCheck {
        name: req.name.clone(),
    });
    Ok((proto::EPPCommandType::Check(command), None))
}

pub fn handle_check_response(response: proto::EPPResponse) -> Response<CheckResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPDomainCheckResult(domain_check) => {
                if let Some(domain_check) = domain_check.data.first() {
                    Response::Ok(CheckResponse {
                        avail: domain_check.name.available,
                        reason: domain_check.reason.to_owned(),
                    })
                } else {
                    Response::InternalServerError
                }
            }
            _ => Response::InternalServerError,
        },
        None => Response::InternalServerError,
    }
}

pub fn handle_info(
    client: &EPPClientServerFeatures,
    req: &InfoRequest,
) -> HandleReqReturn<InfoResponse> {
    if !client.domain_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err(
            "domain name has a min length of 1".to_string(),
        ));
    }
    let command = proto::EPPInfo::Domain(proto::domain::EPPDomainCheck {
        name: req.name.clone(),
    });
    Ok((proto::EPPCommandType::Info(command), None))
}

pub fn handle_info_response(response: proto::EPPResponse) -> Response<InfoResponse> {
    let rgp_state = match response.extension {
        Some(ext) => match ext.value.iter().find(|p| match p {
            proto::EPPResponseExtensionType::EPPRGPInfo(_) => true,
            _ => false,
        }) {
            Some(e) => match e {
                proto::EPPResponseExtensionType::EPPRGPInfo(info) => (&info.state.state).into(),
                _ => unreachable!(),
            },
            None => super::rgp::RGPState::Unknown,
        },
        None => super::rgp::RGPState::Unknown,
    };

    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPDomainInfoResult(domain_info) => {
                Response::Ok(InfoResponse {
                    name: domain_info.name,
                    registry_id: domain_info.registry_id,
                    statuses: domain_info
                        .statuses
                        .into_iter()
                        .map(|s| s.status.into())
                        .collect(),
                    registrant: domain_info.registrant,
                    contacts: domain_info
                        .contacts
                        .into_iter()
                        .map(|c| InfoContact {
                            contact_id: c.contact_id,
                            contact_type: c.contact_type,
                        })
                        .collect(),
                    nameservers: domain_info
                        .nameservers
                        .servers
                        .into_iter()
                        .map(|s| match s {
                            proto::domain::EPPDomainInfoNameserver::HostOnly(h) => {
                                InfoNameserver::HostOnly(h)
                            }
                            proto::domain::EPPDomainInfoNameserver::HostAndAddress {
                                host,
                                address,
                            } => InfoNameserver::HostAndAddress {
                                host,
                                address: address.address,
                                ip_version: match address.ip_version {
                                    proto::domain::EPPDomainInfoNameserverAddressVersion::IPv4 => {
                                        InfoNameserverAddressVersion::IPv4
                                    }
                                    proto::domain::EPPDomainInfoNameserverAddressVersion::IPv6 => {
                                        InfoNameserverAddressVersion::IPv6
                                    }
                                },
                            },
                        })
                        .collect(),
                    hosts: domain_info.hosts,
                    client_id: domain_info.client_id,
                    client_created_id: domain_info.client_created_id,
                    creation_date: domain_info.creation_date,
                    expiry_date: domain_info.expiry_date,
                    last_updated_client: domain_info.last_updated_client,
                    last_updated_date: domain_info.last_updated_date,
                    last_transfer_date: domain_info.last_transfer_date,
                    rgp_state,
                })
            }
            _ => Response::InternalServerError,
        },
        None => Response::InternalServerError,
    }
}

pub fn handle_create(
    client: &EPPClientServerFeatures,
    req: &CreateRequest,
) -> HandleReqReturn<CreateResponse> {
    if !client.domain_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err(
            "domain name has a min length of 1".to_string(),
        ));
    }
    check_id(&req.registrant)?;
    let command = proto::EPPCreate::Domain(proto::domain::EPPDomainCreate {
        name: req.name.clone(),
        period: req.period.as_ref().map(|p| p.into()),
        nameservers: proto::domain::EPPDomainInfoNameserversSer {
            servers: req.nameservers.iter().map(|n| n.into()).collect(),
        },
        registrant: req.registrant.to_string(),
        contacts: req
            .contacts
            .iter()
            .map(|c| {
                check_id(&c.contact_id)?;
                Ok(proto::domain::EPPDomainInfoContactSer {
                    contact_type: c.contact_type.to_string(),
                    contact_id: c.contact_id.to_string(),
                })
            })
            .collect::<Result<Vec<_>, super::router::Response<CreateResponse>>>()?,
        auth_info: proto::domain::EPPDomainAuthInfo {
            password: req.auth_info.to_string(),
        },
    });
    Ok((proto::EPPCommandType::Create(command), None))
}

pub fn handle_create_response(response: proto::EPPResponse) -> Response<CreateResponse> {
    match &response.data {
        Some(value) => match &value.value {
            proto::EPPResultDataValue::EPPDomainCreateResult(domain_create) => {
                Response::Ok(CreateResponse {
                    pending: response.is_pending(),
                    creation_date: domain_create.creation_date,
                    expiration_date: domain_create.expiry_date,
                })
            }
            _ => Response::InternalServerError,
        },
        None => Response::InternalServerError,
    }
}

pub fn handle_delete(
    client: &EPPClientServerFeatures,
    req: &DeleteRequest,
) -> HandleReqReturn<DeleteResponse> {
    if !client.domain_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err(
            "domain name has a min length of 1".to_string(),
        ));
    }
    let command = proto::EPPDelete::Domain(proto::domain::EPPDomainCheck {
        name: req.name.clone(),
    });
    Ok((proto::EPPCommandType::Delete(command), None))
}

pub fn handle_delete_response(response: proto::EPPResponse) -> Response<DeleteResponse> {
    Response::Ok(DeleteResponse {
        pending: response.is_pending(),
    })
}

pub fn handle_update(
    client: &EPPClientServerFeatures,
    req: &UpdateRequest,
) -> HandleReqReturn<UpdateResponse> {
    if !client.domain_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err(
            "domain name has a min length of 1".to_string(),
        ));
    }
    if let Some(new_registrant) = &req.new_registrant {
        check_id(&new_registrant)?;
    }
    let mut adds = vec![];
    let mut rems = vec![];
    let mut add_ns = vec![];
    let mut rem_ns = vec![];
    for add in &req.add {
        match add {
            UpdateObject::Status(s) => adds.push(proto::domain::EPPDomainUpdateParam::Status(
                proto::domain::EPPDomainStatusSer {
                    status: s.into(),
                    message: None,
                },
            )),
            UpdateObject::Contact(c) => adds.push(proto::domain::EPPDomainUpdateParam::Contact(
                proto::domain::EPPDomainInfoContactSer {
                    contact_type: c.contact_type.clone(),
                    contact_id: c.contact_id.clone(),
                },
            )),
            UpdateObject::Nameserver(n) => add_ns.push(n.into()),
        }
    }
    for rem in &req.remove {
        match rem {
            UpdateObject::Status(s) => rems.push(proto::domain::EPPDomainUpdateParam::Status(
                proto::domain::EPPDomainStatusSer {
                    status: s.into(),
                    message: None,
                },
            )),
            UpdateObject::Contact(c) => rems.push(proto::domain::EPPDomainUpdateParam::Contact(
                proto::domain::EPPDomainInfoContactSer {
                    contact_type: c.contact_type.clone(),
                    contact_id: c.contact_id.clone(),
                },
            )),
            UpdateObject::Nameserver(n) => rem_ns.push(n.into()),
        }
    }
    if !add_ns.is_empty() {
        adds.push(proto::domain::EPPDomainUpdateParam::Nameserver(
            proto::domain::EPPDomainInfoNameserversSer { servers: add_ns },
        ))
    }
    if !rem_ns.is_empty() {
        rems.push(proto::domain::EPPDomainUpdateParam::Nameserver(
            proto::domain::EPPDomainInfoNameserversSer { servers: rem_ns },
        ))
    }

    let update_as_i32 = |u: &proto::domain::EPPDomainUpdateParam| match u {
        proto::domain::EPPDomainUpdateParam::Nameserver(_) => 0,
        proto::domain::EPPDomainUpdateParam::Contact(_) => 1,
        proto::domain::EPPDomainUpdateParam::Status(_) => 2,
    };
    adds.sort_unstable_by(|a, b| (update_as_i32(a)).cmp(&update_as_i32(b)));
    rems.sort_unstable_by(|a, b| (update_as_i32(a)).cmp(&update_as_i32(b)));

    let is_not_change = req.new_registrant.is_none() && req.new_auth_info.is_none();
    if req.add.is_empty() && req.remove.is_empty() && is_not_change {
        return Err(Response::Err(
            "at least one operation must be specified".to_string(),
        ));
    }
    let command = proto::EPPUpdate::Domain(proto::domain::EPPDomainUpdate {
        name: req.name.clone(),
        add: if adds.is_empty() {
            None
        } else {
            Some(proto::domain::EPPDomainUpdateAdd { params: adds })
        },
        remove: if rems.is_empty() {
            None
        } else {
            Some(proto::domain::EPPDomainUpdateRemove { params: rems })
        },
        change: if is_not_change {
            None
        } else {
            Some(proto::domain::EPPDomainUpdateChange {
                registrant: req.new_registrant.clone(),
                auth_info: req
                    .new_auth_info
                    .as_ref()
                    .map(|a| proto::domain::EPPDomainAuthInfo {
                        password: a.clone(),
                    }),
            })
        },
    });
    Ok((proto::EPPCommandType::Update(Box::new(command)), None))
}

pub fn handle_update_response(response: proto::EPPResponse) -> Response<UpdateResponse> {
    Response::Ok(UpdateResponse {
        pending: response.is_pending(),
    })
}

pub fn handle_renew(
    client: &EPPClientServerFeatures,
    req: &RenewRequest,
) -> HandleReqReturn<RenewResponse> {
    if !client.domain_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err(
            "domain name has a min length of 1".to_string(),
        ));
    }
    let command = proto::EPPRenew::Domain(proto::domain::EPPDomainRenew {
        name: req.name.clone(),
        period: req.add_period.as_ref().map(|p| p.into()),
        current_expiry_date: req.cur_expiry_date.date(),
    });
    Ok((proto::EPPCommandType::Renew(command), None))
}

pub fn handle_renew_response(response: proto::EPPResponse) -> Response<RenewResponse> {
    match &response.data {
        Some(value) => match &value.value {
            proto::EPPResultDataValue::EPPDomainRenewResult(domain_renew) => {
                Response::Ok(RenewResponse {
                    pending: response.is_pending(),
                    new_expiry_date: domain_renew.expiry_date,
                })
            }
            _ => Response::InternalServerError,
        },
        None => Response::InternalServerError,
    }
}

pub fn handle_transfer_query(
    client: &EPPClientServerFeatures,
    req: &TransferQueryRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.domain_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err(
            "domain name has a min length of 1".to_string(),
        ));
    }
    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Query,
        command: proto::EPPTransferCommand::DomainQuery(proto::domain::EPPDomainCheck {
            name: req.name.clone(),
        }),
    };
    Ok((proto::EPPCommandType::Transfer(command), None))
}

pub fn handle_transfer_request(
    client: &EPPClientServerFeatures,
    req: &TransferRequestRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.domain_supported {
        return Err(Response::Unsupported);
    }
    if req.name.is_empty() {
        return Err(Response::Err(
            "domain name has a min length of 1".to_string(),
        ));
    }
    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Request,
        command: proto::EPPTransferCommand::DomainRequest(proto::domain::EPPDomainTransfer {
            name: req.name.clone(),
            period: req.add_period.as_ref().map(|p| p.into()),
            auth_info: proto::domain::EPPDomainAuthInfo {
                password: req.auth_info.clone(),
            },
        }),
    };
    Ok((proto::EPPCommandType::Transfer(command), None))
}

pub fn handle_transfer_response(response: proto::EPPResponse) -> Response<TransferResponse> {
    match &response.data {
        Some(value) => match &value.value {
            proto::EPPResultDataValue::EPPDomainTransferResult(domain_transfer) => {
                Response::Ok(TransferResponse {
                    pending: response.is_pending(),
                    status: (&domain_transfer.transfer_status).into(),
                    requested_client_id: domain_transfer.requested_client_id.clone(),
                    requested_date: domain_transfer.requested_date,
                    act_client_id: domain_transfer.act_client_id.clone(),
                    act_date: domain_transfer.act_date,
                    expiry_date: domain_transfer.expiry_date,
                })
            }
            _ => Response::InternalServerError,
        },
        None => Response::InternalServerError,
    }
}

/// Checks if a domain name
///
/// # Arguments
/// * `domain` - The domain in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn check(
    domain: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CheckResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainCheck(Box::new(CheckRequest {
            name: domain.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Fetches information about a domain name
///
/// # Arguments
/// * `domain` - The domain in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn info(
    domain: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<InfoResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainInfo(Box::new(InfoRequest {
            name: domain.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Registers a new domain
///
/// # Arguments
/// * `domain` - The domain to be registered
/// * `period` - How long to register for
/// * `registrant` - Registrant contact ID,
/// * `contacts` - Other contact types for the domain
/// * `nameservers` - Domain nameservers
/// * `auth_info` - Auth info password for future transfers
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn create(
    domain: &str,
    period: Option<Period>,
    registrant: &str,
    contacts: Vec<InfoContact>,
    nameservers: Vec<InfoNameserver>,
    auth_info: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CreateResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainCreate(Box::new(CreateRequest {
            name: domain.to_string(),
            period,
            registrant: registrant.to_string(),
            contacts,
            nameservers,
            auth_info: auth_info.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Deletes a domain name
///
/// # Arguments
/// * `domain` - The domain in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn delete(
    domain: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<DeleteResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainDelete(Box::new(DeleteRequest {
            name: domain.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Updates properties of a domain name
///
/// # Arguments
/// * `domain` - The domain to be updated
/// * `add` - Attributes to be added
/// * `remove` - Attributes to be removed
/// * `new_registrant` - New registrant ID
/// * `new_auth_info` - New auth info password for future transfers
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn update(
    domain: &str,
    add: Vec<UpdateObject>,
    remove: Vec<UpdateObject>,
    new_registrant: Option<&str>,
    new_auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<UpdateResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainUpdate(Box::new(UpdateRequest {
            name: domain.to_string(),
            add,
            remove,
            new_registrant: new_registrant.map(|s| s.into()),
            new_auth_info: new_auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Renews a domain name
///
/// # Arguments
/// * `domain` - The domain in question
/// * `add_period` - How much time to add to the domain
/// * `cur_expiry_date` - The current expiry date
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn renew(
    domain: &str,
    add_period: Option<Period>,
    cur_expiry_date: DateTime<Utc>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<RenewResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainRenew(Box::new(RenewRequest {
            name: domain.to_string(),
            add_period,
            cur_expiry_date,
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Queries the current transfer status of a domain name
///
/// # Arguments
/// * `domain` - The domain to be queried
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_query(
    domain: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<TransferResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTransferQuery(Box::new(TransferQueryRequest {
            name: domain.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}

/// Requests the transfer of a domain name
///
/// # Arguments
/// * `domain` - The domain to be queried
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_request(
    domain: &str,
    add_period: Option<Period>,
    auth_info: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<TransferResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTransferRequest(Box::new(TransferRequestRequest {
            name: domain.to_string(),
            add_period,
            auth_info: auth_info.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}
