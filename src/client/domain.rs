//! EPP commands relating to domain objects

use std::convert::{TryFrom, TryInto};

use chrono::prelude::*;

use super::{EPPClientServerFeatures, Error, fee, launch, proto, Request, Response, CommandResponse, Sender};
use super::router::HandleReqReturn;

#[derive(Debug)]
pub struct CheckRequest {
    name: String,
    fee_check: Option<fee::FeeCheck>,
    launch_check: Option<launch::LaunchAvailabilityCheck>,
    pub return_path: Sender<CheckResponse>,
}

#[derive(Debug)]
pub struct ClaimsCheckRequest {
    name: String,
    launch_check: launch::LaunchClaimsCheck,
    pub return_path: Sender<ClaimsCheckResponse>,
}

#[derive(Debug)]
pub struct TrademarkCheckRequest {
    name: String,
    pub return_path: Sender<ClaimsCheckResponse>,
}

/// Response to a domain check query
#[derive(Debug)]
pub struct CheckResponse {
    /// Is the domain available for registration
    pub avail: bool,
    /// An optional reason for the domain's status
    pub reason: Option<String>,
    /// Fee information (if supplied by the registry)
    pub fee_check: Option<fee::FeeCheckData>,
    pub donuts_fee_check: Option<fee::DonutsFeeData>,
    pub eurid_check: Option<super::eurid::DomainCheck>,
    pub eurid_idn: Option<super::eurid::IDN>,
}

/// Response to a domain claims check query
#[derive(Debug)]
pub struct ClaimsCheckResponse {
    /// Does a trademark claim exist
    pub exists: bool,
    /// Claims key for this domain
    pub claims_key: Vec<launch::LaunchClaimKey>,
}

#[derive(Debug)]
pub enum InfoHost {
    All,
    Delegated,
    Subordinate,
    None,
}

#[derive(Debug)]
pub struct InfoRequest {
    name: String,
    auth_info: Option<String>,
    launch_info: Option<launch::LaunchInfo>,
    hosts: Option<InfoHost>,
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
    pub rgp_state: Vec<super::rgp::RGPState>,
    pub auth_info: Option<String>,
    /// DNSSEC data
    pub sec_dns: Option<SecDNSData>,
    pub launch_info: Option<launch::LaunchInfoData>,
    pub donuts_fee_data: Option<fee::DonutsFeeData>,
    pub whois_info: Option<super::verisign::InfoWhois>,
    pub eurid_data: Option<super::eurid::DomainInfo>,
    pub eurid_idn: Option<super::eurid::IDN>,
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
        addresses: Vec<super::host::Address>,
    },
}

/// DNSSEC key data
#[derive(Debug)]
pub struct SecDNSData {
    pub max_sig_life: Option<i64>,
    pub data: SecDNSDataType,
}

#[derive(Debug)]
pub enum SecDNSDataType {
    DSData(Vec<SecDNSDSData>),
    KeyData(Vec<SecDNSKeyData>),
}

#[derive(Debug)]
pub struct SecDNSDSData {
    pub key_tag: u16,
    pub algorithm: u8,
    pub digest_type: u8,
    pub digest: String,
    pub key_data: Option<SecDNSKeyData>,
}

#[derive(Debug)]
pub struct SecDNSKeyData {
    pub flags: u16,
    pub protocol: u8,
    pub algorithm: u8,
    pub public_key: String,
}

#[derive(Debug)]
pub struct CreateRequest {
    name: String,
    period: Option<Period>,
    registrant: String,
    contacts: Vec<InfoContact>,
    nameservers: Vec<InfoNameserver>,
    auth_info: String,
    sec_dns: Option<SecDNSData>,
    launch_create: Option<launch::LaunchCreate>,
    fee_agreement: Option<fee::FeeAgreement>,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    eurid_data: Option<super::eurid::DomainCreate>,
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
    pub data: CreateData,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
    pub donuts_fee_data: Option<fee::DonutsFeeData>,
    pub launch_create: Option<launch::LaunchCreateData>,
    pub eurid_idn: Option<super::eurid::IDN>,
}

#[derive(Debug)]
pub struct CreateData {
    /// The actual domain name created
    pub name: String,
    /// What date did the server log as the date of creation
    pub creation_date: Option<DateTime<Utc>>,
    /// When will the domain expire
    pub expiration_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct DeleteRequest {
    name: String,
    launch_info: Option<launch::LaunchUpdate>,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    eurid_data: Option<super::eurid::DomainDelete>,
    pub return_path: Sender<DeleteResponse>,
}

#[derive(Debug)]
pub struct DeleteResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
    pub eurid_idn: Option<super::eurid::IDN>,
}

#[derive(Debug)]
pub struct UpdateRequest {
    name: String,
    add: Vec<UpdateObject>,
    remove: Vec<UpdateObject>,
    new_registrant: Option<String>,
    new_auth_info: Option<String>,
    sec_dns: Option<UpdateSecDNS>,
    launch_info: Option<launch::LaunchUpdate>,
    fee_agreement: Option<fee::FeeAgreement>,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub return_path: Sender<UpdateResponse>,
}

#[derive(Debug)]
pub enum UpdateObject {
    Status(Status),
    Contact(InfoContact),
    Nameserver(InfoNameserver),
}

#[derive(Debug)]
pub struct UpdateSecDNS {
    pub urgent: Option<bool>,
    pub remove: Option<UpdateSecDNSRemove>,
    pub add: Option<SecDNSDataType>,
    pub new_max_sig_life: Option<i64>,
}

#[derive(Debug)]
pub enum UpdateSecDNSRemove {
    All(bool),
    Data(SecDNSDataType),
}

#[derive(Debug)]
pub struct UpdateResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
    pub donuts_fee_data: Option<fee::DonutsFeeData>,
}

#[derive(Debug)]
pub struct RenewRequest {
    name: String,
    add_period: Option<Period>,
    cur_expiry_date: DateTime<Utc>,
    fee_agreement: Option<fee::FeeAgreement>,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub return_path: Sender<RenewResponse>,
}

#[derive(Debug)]
pub struct RenewResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    pub data: RenewData,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
    pub donuts_fee_data: Option<fee::DonutsFeeData>,
    pub eurid_idn: Option<super::eurid::IDN>,
}

#[derive(Debug)]
pub struct RenewData {
    pub name: String,
    pub new_expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct TransferQueryRequest {
    name: String,
    auth_info: Option<String>,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferRequestRequest {
    name: String,
    auth_info: String,
    add_period: Option<Period>,
    fee_agreement: Option<fee::FeeAgreement>,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferAcceptRejectRequest {
    name: String,
    auth_info: Option<String>,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    pub data: TransferData,
    /// Fee information (if supplied by the registry)
    pub fee_data: Option<fee::FeeData>,
    pub donuts_fee_data: Option<fee::DonutsFeeData>,
}

#[derive(Debug)]
pub struct TransferData {
    pub name: String,
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct PanData {
    pub name: String,
    pub result: bool,
    pub server_transaction_id: Option<String>,
    pub client_transaction_id: Option<String>,
    pub date: DateTime<Utc>,
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
            EPPDomainStatusType::Granted => Status::Ok,
            EPPDomainStatusType::PendingCreate => Status::PendingCreate,
            EPPDomainStatusType::PendingDelete => Status::PendingDelete,
            EPPDomainStatusType::Terminated => Status::PendingDelete,
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

impl From<&InfoNameserver> for proto::domain::EPPDomainInfoNameserver {
    fn from(from: &InfoNameserver) -> Self {
        match from {
            InfoNameserver::HostOnly(h) => {
                proto::domain::EPPDomainInfoNameserver::HostOnly(h.to_string())
            }
            InfoNameserver::HostAndAddress { host, addresses } => {
                proto::domain::EPPDomainInfoNameserver::HostAndAddress {
                    host: host.to_string(),
                    addresses: addresses
                        .iter()
                        .map(|addr| proto::domain::EPPDomainInfoNameserverAddress {
                            address: addr.address.to_string(),
                            ip_version: match addr.ip_version {
                                super::host::AddressVersion::IPv4 => {
                                    proto::domain::EPPDomainInfoNameserverAddressVersion::IPv4
                                }
                                super::host::AddressVersion::IPv6 => {
                                    proto::domain::EPPDomainInfoNameserverAddressVersion::IPv6
                                }
                            },
                        })
                        .collect(),
                }
            }
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
            value: std::cmp::min(99, std::cmp::max(from.value, 1)),
        }
    }
}

impl From<&proto::domain::EPPDomainPeriod> for Period {
    fn from(from: &proto::domain::EPPDomainPeriod) -> Self {
        Period {
            unit: match from.unit {
                proto::domain::EPPDomainPeriodUnit::Years => PeriodUnit::Years,
                proto::domain::EPPDomainPeriodUnit::Months => PeriodUnit::Months,
            },
            value: from.value,
        }
    }
}

impl
TryFrom<(
    proto::domain::EPPDomainInfoData,
    &Option<proto::EPPResponseExtension>,
)> for InfoResponse
{
    type Error = Error;

    fn try_from(
        from: (
            proto::domain::EPPDomainInfoData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (domain_info, extension) = from;
        let rgp_state = match extension {
            Some(ext) => match ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPRGPInfo(i) => Some(i),
                _ => None,
            }) {
                Some(e) => e.state.iter().map(|s| (&s.state).into()).collect(),
                None => vec![],
            },
            None => vec![],
        };

        let sec_dns = match extension {
            Some(ext) => match ext
                .value
                .iter()
                .find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPSecDNSInfo(i) => Some(i),
                    _ => None,
                })
                .map(|i| {
                    Ok(SecDNSData {
                        max_sig_life: i.max_signature_life,
                        data: if !i.ds_data.is_empty() {
                            SecDNSDataType::DSData(
                                i.ds_data
                                    .iter()
                                    .map(|d| SecDNSDSData {
                                        key_tag: d.key_tag,
                                        algorithm: d.algorithm,
                                        digest_type: d.digest_type,
                                        digest: d.digest.clone(),
                                        key_data: d.key_data.as_ref().map(|k| SecDNSKeyData {
                                            flags: k.flags,
                                            protocol: k.protocol,
                                            algorithm: k.algorithm,
                                            public_key: k.public_key.clone(),
                                        }),
                                    })
                                    .collect(),
                            )
                        } else if !i.key_data.is_empty() {
                            SecDNSDataType::KeyData(
                                i.key_data
                                    .iter()
                                    .map(|k| SecDNSKeyData {
                                        flags: k.flags,
                                        protocol: k.protocol,
                                        algorithm: k.algorithm,
                                        public_key: k.public_key.clone(),
                                    })
                                    .collect(),
                            )
                        } else {
                            return Err(Error::InternalServerError);
                        },
                    })
                }) {
                Some(i) => match i {
                    Ok(i) => Some(i),
                    Err(e) => return Err(e),
                },
                None => None,
            },
            None => None,
        };

        let launch_info = match extension {
            Some(ext) => (ext
                .value
                .iter()
                .find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPLaunchInfoData(i) => Some(i),
                    _ => None,
                })
                .map(|i| {
                    launch::LaunchInfoData::from(i)
                })),
            None => None,
        };

        let donuts_fee_data = match extension {
            Some(ext) => {
                let charge = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPDonutsChargeInfoData(i) => Some(i),
                    _ => None,
                });

                charge.map(Into::into)
            }
            None => None,
        };

        let whois_info = match extension {
            Some(ext) => {
                let charge = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::VerisignWhoisInfo(i) => Some(i),
                    _ => None,
                });
                charge.map(Into::into)
            }
            None => None,
        };

        Ok(InfoResponse {
            eurid_idn: super::eurid::extract_eurid_idn_singular(extension, domain_info.name.as_str())?,
            name: domain_info.name,
            registry_id: domain_info.registry_id.unwrap_or_default(),
            statuses: domain_info
                .statuses
                .into_iter()
                .map(|s| s.status.into())
                .collect(),
            registrant: domain_info.registrant.unwrap_or_default(),
            contacts: domain_info
                .contacts
                .into_iter()
                .map(|c| InfoContact {
                    contact_id: c.contact_id,
                    contact_type: c.contact_type,
                })
                .collect(),
            nameservers: match domain_info.nameservers {
                None => vec![],
                Some(n) => n
                    .servers
                    .into_iter()
                    .map(|s| match s {
                        proto::domain::EPPDomainInfoNameserver::HostOnly(h) => {
                            InfoNameserver::HostOnly(h)
                        }
                        proto::domain::EPPDomainInfoNameserver::HostAndAddress {
                            host,
                            addresses,
                        } => InfoNameserver::HostAndAddress {
                            host,
                            addresses: addresses
                                .into_iter()
                                .map(|addr| {
                                    super::host::Address {
                                        address: addr.address,
                                        ip_version: match addr.ip_version {
                                            proto::domain::EPPDomainInfoNameserverAddressVersion::IPv4 => {
                                                super::host::AddressVersion::IPv4
                                            }
                                            proto::domain::EPPDomainInfoNameserverAddressVersion::IPv6 => {
                                                super::host::AddressVersion::IPv6
                                            }
                                        },
                                    }
                                })
                                .collect(),
                        },
                    })
                    .collect(),
            },
            hosts: domain_info.hosts,
            client_id: domain_info.client_id,
            client_created_id: domain_info.client_created_id,
            creation_date: domain_info.creation_date,
            expiry_date: domain_info.expiry_date,
            last_updated_client: domain_info.last_updated_client,
            last_updated_date: domain_info.last_updated_date,
            last_transfer_date: domain_info.last_transfer_date,
            rgp_state,
            auth_info: match domain_info.auth_info {
                Some(a) => a.password,
                None => None,
            },
            sec_dns,
            launch_info,
            donuts_fee_data,
            whois_info,
            eurid_data: super::eurid::extract_eurid_domain_info(extension),
        })
    }
}

impl
TryFrom<(
    proto::domain::EPPDomainTransferData,
    &Option<proto::EPPResponseExtension>,
)> for TransferResponse
{
    type Error = Error;

    fn try_from(
        from: (
            proto::domain::EPPDomainTransferData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (domain_transfer, extension) = from;

        let fee_data = match extension {
            Some(ext) => {
                let fee10 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee10TransferData(i) => Some(i),
                    _ => None,
                });
                let fee011 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee011TransferData(i) => Some(i),
                    _ => None,
                });
                let fee09 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee09TransferData(i) => Some(i),
                    _ => None,
                });
                let fee08 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee08TransferData(i) => Some(i),
                    _ => None,
                });
                let fee07 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee07TransferData(i) => Some(i),
                    _ => None,
                });
                let fee05 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee05TransferData(i) => Some(i),
                    _ => None,
                });

                if let Some(f) = fee10 {
                    Some(f.into())
                } else if let Some(f) = fee011 {
                    Some(f.into())
                } else if let Some(f) = fee09 {
                    Some(f.into())
                } else if let Some(f) = fee08 {
                    Some(f.into())
                } else if let Some(f) = fee07 {
                    Some(f.into())
                } else if let Some(f) = fee05 {
                    Some(f.into())
                } else {
                    None
                }
            }
            None => None,
        };

        let donuts_fee_data = match extension {
            Some(ext) => {
                let charge = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPDonutsChargeTransferData(i) => Some(i),
                    _ => None,
                });

                charge.map(Into::into)
            }
            None => None,
        };

        Ok(TransferResponse {
            pending: false,
            data: TransferData {
                name: domain_transfer.name.clone(),
                status: (&domain_transfer.transfer_status).into(),
                requested_client_id: domain_transfer.requested_client_id.clone(),
                requested_date: domain_transfer.requested_date,
                act_client_id: domain_transfer.act_client_id.clone(),
                act_date: domain_transfer.act_date,
                expiry_date: domain_transfer.expiry_date,
            },
            fee_data,
            donuts_fee_data,
        })
    }
}

impl
TryFrom<(
    Option<proto::domain::EPPDomainCreateData>,
    &Option<proto::EPPResponseExtension>,
)> for CreateResponse
{
    type Error = Error;

    fn try_from(
        from: (
            Option<proto::domain::EPPDomainCreateData>,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (domain_create, extension) = from;

        let fee_data = match extension {
            Some(ext) => {
                let fee10 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee10CreateData(i) => Some(i),
                    _ => None,
                });
                let fee011 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee011CreateData(i) => Some(i),
                    _ => None,
                });
                let fee09 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee09CreateData(i) => Some(i),
                    _ => None,
                });
                let fee08 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee08CreateData(i) => Some(i),
                    _ => None,
                });
                let fee07 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee07CreateData(i) => Some(i),
                    _ => None,
                });
                let fee05 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee05CreateData(i) => Some(i),
                    _ => None,
                });

                if let Some(f) = fee10 {
                    Some(f.into())
                } else if let Some(f) = fee011 {
                    Some(f.into())
                } else if let Some(f) = fee09 {
                    Some(f.into())
                } else if let Some(f) = fee08 {
                    Some(f.into())
                } else if let Some(f) = fee07 {
                    Some(f.into())
                } else if let Some(f) = fee05 {
                    Some(f.into())
                } else {
                    None
                }
            }
            None => None,
        };


        let launch_create = match extension {
            Some(ext) => (ext
                .value
                .iter()
                .find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPLaunchCreateData(i) => Some(i),
                    _ => None,
                })
                .map(|i| {
                    launch::LaunchCreateData::from(i)
                })),
            None => None,
        };

        let donuts_fee_data = match extension {
            Some(ext) => {
                let charge = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPDonutsChargeCreateData(i) => Some(i),
                    _ => None,
                });

                charge.map(Into::into)
            }
            None => None,
        };

        match domain_create {
            Some(domain_create) => {
                Ok(CreateResponse {
                    pending: false,
                    eurid_idn: super::eurid::extract_eurid_idn_singular(extension, domain_create.name.as_str())?,
                    data: CreateData {
                        name: domain_create.name.clone(),
                        creation_date: Some(domain_create.creation_date),
                        expiration_date: domain_create.expiry_date,
                    },
                    fee_data,
                    donuts_fee_data,
                    launch_create,
                })
            }
            None => {
                Ok(CreateResponse {
                    pending: false,
                    eurid_idn: super::eurid::extract_eurid_idn_singular(extension, None)?,
                    data: CreateData {
                        name: "".to_string(),
                        creation_date: None,
                        expiration_date: None,
                    },
                    fee_data,
                    donuts_fee_data,
                    launch_create,
                })
            }
        }
    }
}

impl
TryFrom<(
    proto::domain::EPPDomainRenewData,
    &Option<proto::EPPResponseExtension>,
)> for RenewResponse
{
    type Error = Error;

    fn try_from(
        from: (
            proto::domain::EPPDomainRenewData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (domain_renew, extension) = from;

        let fee_data = match extension {
            Some(ext) => {
                let fee10 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee10RenewData(i) => Some(i),
                    _ => None,
                });
                let fee011 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee011RenewData(i) => Some(i),
                    _ => None,
                });
                let fee09 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee09RenewData(i) => Some(i),
                    _ => None,
                });
                let fee08 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee08RenewData(i) => Some(i),
                    _ => None,
                });
                let fee07 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee07RenewData(i) => Some(i),
                    _ => None,
                });
                let fee05 = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPFee05RenewData(i) => Some(i),
                    _ => None,
                });

                if let Some(f) = fee10 {
                    Some(f.into())
                } else if let Some(f) = fee011 {
                    Some(f.into())
                } else if let Some(f) = fee09 {
                    Some(f.into())
                } else if let Some(f) = fee08 {
                    Some(f.into())
                } else if let Some(f) = fee07 {
                    Some(f.into())
                } else if let Some(f) = fee05 {
                    Some(f.into())
                } else {
                    None
                }
            }
            None => None,
        };

        let donuts_fee_data = match extension {
            Some(ext) => {
                let charge = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::EPPDonutsChargeRenewData(i) => Some(i),
                    _ => None,
                });

                charge.map(Into::into)
            }
            None => None,
        };

        Ok(RenewResponse {
            pending: false,
            eurid_idn: super::eurid::extract_eurid_idn_singular(extension, domain_renew.name.as_str())?,
            data: RenewData {
                name: domain_renew.name.to_owned(),
                new_expiry_date: domain_renew.expiry_date,
            },
            fee_data,
            donuts_fee_data,
        })
    }
}

impl From<&proto::domain::EPPDomainPanData> for PanData {
    fn from(from: &proto::domain::EPPDomainPanData) -> Self {
        PanData {
            name: from.name.domain.clone(),
            result: from.name.result,
            server_transaction_id: from.transaction_id.server_transaction_id.clone(),
            client_transaction_id: from.transaction_id.client_transaction_id.clone(),
            date: from.action_date,
        }
    }
}

pub(crate) fn check_domain<T>(id: &str) -> Result<(), Response<T>> {
    if !id.is_empty() {
        Ok(())
    } else {
        Err(Err(Error::Err(
            "domain name has a min length of 1".to_string(),
        )))
    }
}

pub fn handle_check(
    client: &EPPClientServerFeatures,
    req: &CheckRequest,
) -> HandleReqReturn<CheckResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPCheck::Domain(proto::domain::EPPDomainCheck {
        name: req.name.clone(),
        auth_info: None,
    });
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    if let Some(fee_check) = &req.fee_check {
        if client.fee_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee10Check(proto::fee::EPPFee10Check {
                currency: fee_check.currency.to_owned(),
                commands: fee_check.commands.iter().map(|c| Ok(proto::fee::EPPFee10CheckCommand {
                    name: match (&c.command).into() {
                        Some(n) => n,
                        None => return Err(Err(Error::Unsupported))
                    },
                    period: c.period.as_ref().map(Into::into),
                })).collect::<Result<Vec<_>, _>>()?,
            }))
        } else if client.fee_011_supported {
            fee_check.commands.iter().map(|c| {
                ext.push(proto::EPPCommandExtensionType::EPPFee011Check(proto::fee::EPPFee011Check {
                    currency: fee_check.currency.to_owned(),
                    command: match (&c.command).into() {
                        Some(n) => n,
                        None => return Err(Err(Error::Unsupported))
                    },
                    period: c.period.as_ref().map(Into::into),
                }));
                Ok(())
            }).collect::<Result<Vec<_>, _>>()?;
        } else if client.fee_09_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee09Check(proto::fee::EPPFee09Check {
                objects: fee_check.commands.iter().map(|c| Ok(proto::fee::EPPFee09CheckObject {
                    object_uri: Some("urn:ietf:params:xml:ns:domain-1.0".to_string()),
                    object_id: proto::fee::EPPFee10ObjectID {
                        element: "name".to_string(),
                        id: req.name.to_owned(),
                    },
                    currency: fee_check.currency.to_owned(),
                    command: match (&c.command).into() {
                        Some(n) => n,
                        None => return Err(Err(Error::Unsupported))
                    },
                    period: c.period.as_ref().map(Into::into),
                })).collect::<Result<Vec<_>, _>>()?,
            }))
        } else if client.fee_08_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee08Check(proto::fee::EPPFee08Check {
                domains: fee_check.commands.iter().map(|c| Ok(proto::fee::EPPFee08CheckDomain {
                    name: req.name.to_owned(),
                    currency: fee_check.currency.to_owned(),
                    command: match (&c.command).into() {
                        Some(n) => n,
                        None => return Err(Err(Error::Unsupported))
                    },
                    period: c.period.as_ref().map(Into::into),
                })).collect::<Result<Vec<_>, _>>()?,
            }))
        } else if client.fee_07_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee07Check(proto::fee::EPPFee07Check {
                domains: fee_check.commands.iter().map(|c| Ok(proto::fee::EPPFee07CheckDomain {
                    name: req.name.to_owned(),
                    currency: fee_check.currency.to_owned(),
                    command: match (&c.command).into() {
                        Some(n) => n,
                        None => return Err(Err(Error::Unsupported))
                    },
                    period: c.period.as_ref().map(Into::into),
                })).collect::<Result<Vec<_>, _>>()?,
            }))
        } else if client.fee_05_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee05Check(proto::fee::EPPFee05Check {
                domains: fee_check.commands.iter().map(|c| Ok(proto::fee::EPPFee05CheckDomain {
                    name: req.name.to_owned(),
                    currency: fee_check.currency.to_owned(),
                    command: match (&c.command).into() {
                        Some(n) => n,
                        None => return Err(Err(Error::Unsupported))
                    },
                    period: c.period.as_ref().map(Into::into),
                })).collect::<Result<Vec<_>, _>>()?,
            }))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }
    if let Some(launch_check) = &req.launch_check {
        if client.launch_supported {
            ext.push(proto::EPPCommandExtensionType::EPPLaunchCheck(
                launch_check.into()
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }
    Ok((proto::EPPCommandType::Check(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_check_response(response: proto::EPPResponse) -> Response<CheckResponse> {
    let fee_check = match &response.extension {
        Some(ext) => {
            let fee10 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee10CheckData(i) => Some(i),
                _ => None,
            });
            let fee011 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee011CheckData(i) => Some(i),
                _ => None,
            });
            let fee09 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee09CheckData(i) => Some(i),
                _ => None,
            });
            let fee08 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee08CheckData(i) => Some(i),
                _ => None,
            });
            let fee07 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee07CheckData(i) => Some(i),
                _ => None,
            });
            let fee05 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee05CheckData(i) => Some(i),
                _ => None,
            });

            if let Some(f) = fee10 {
                let d = match f.objects.iter().next() {
                    Some(o) => o,
                    None => return Err(Error::InternalServerError)
                };
                Some(fee::FeeCheckData {
                    available: d.available,
                    commands: d.commands.iter().map(|c| fee::FeeCommand {
                        command: (&c.name).into(),
                        period: c.period.as_ref().map(Into::into),
                        standard: Some(c.standard),
                        currency: f.currency.to_owned(),
                        fees: c.fee.iter().map(Into::into).collect(),
                        credits: c.credit.iter().map(Into::into).collect(),
                        reason: c.reason.to_owned(),
                        class: d.class.to_owned(),
                    }).collect(),
                    reason: d.reason.to_owned(),
                })
            } else if let Some(f) = fee011 {
                let d = match f.objects.iter().next() {
                    Some(o) => o,
                    None => return Err(Error::InternalServerError)
                };
                Some(fee::FeeCheckData {
                    available: d.available,
                    commands: f.objects.iter().map(|c| fee::FeeCommand {
                        command: (&c.command.name).into(),
                        period: c.period.as_ref().map(Into::into),
                        standard: Some(c.command.standard),
                        currency: c.currency.to_owned(),
                        fees: c.fee.iter().map(Into::into).collect(),
                        credits: c.credit.iter().map(Into::into).collect(),
                        reason: c.reason.to_owned(),
                        class: c.class.to_owned(),
                    }).collect(),
                    reason: d.reason.to_owned(),
                })
            } else if let Some(f) = fee09 {
                Some(fee::FeeCheckData {
                    available: true,
                    commands: f.objects.iter().map(|d| fee::FeeCommand {
                        command: (&d.command).into(),
                        period: d.period.as_ref().map(Into::into),
                        standard: None,
                        currency: d.currency.to_owned(),
                        fees: d.fee.iter().map(Into::into).collect(),
                        credits: d.credit.iter().map(Into::into).collect(),
                        class: d.class.to_owned(),
                        reason: None,
                    }).collect(),
                    reason: None,
                })
            } else if let Some(f) = fee08 {
                Some(fee::FeeCheckData {
                    available: true,
                    commands: f.domains.iter().map(|d| fee::FeeCommand {
                        command: (&d.command).into(),
                        period: d.period.as_ref().map(Into::into),
                        standard: None,
                        currency: d.currency.to_owned(),
                        fees: d.fee.iter().map(Into::into).collect(),
                        credits: d.credit.iter().map(Into::into).collect(),
                        class: d.class.to_owned(),
                        reason: None,
                    }).collect(),
                    reason: None,
                })
            } else if let Some(f) = fee07 {
                Some(fee::FeeCheckData {
                    available: true,
                    commands: f.domains.iter().map(|d| fee::FeeCommand {
                        command: (&d.command).into(),
                        period: d.period.as_ref().map(Into::into),
                        standard: None,
                        currency: d.currency.to_owned().unwrap_or_default(),
                        fees: d.fee.iter().map(Into::into).collect(),
                        credits: d.credit.iter().map(Into::into).collect(),
                        class: d.class.to_owned(),
                        reason: None,
                    }).collect(),
                    reason: None,
                })
            } else if let Some(f) = fee05 {
                Some(fee::FeeCheckData {
                    available: true,
                    commands: f.domains.iter().map(|d| fee::FeeCommand {
                        command: (&d.command).into(),
                        period: Some((&d.period).into()),
                        standard: None,
                        currency: d.currency.to_owned(),
                        fees: d.fee.iter().map(Into::into).collect(),
                        class: d.class.to_owned(),
                        credits: vec![],
                        reason: None,
                    }).collect(),
                    reason: None,
                })
            } else {
                None
            }
        }
        None => None,
    };

    let donuts_fee_check = match &response.extension {
        Some(ext) => {
            let charge = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPDonutsChargeCheckData(i) => Some(i),
                _ => None,
            });

            if let Some(c) = charge {
                let d = match c.domains.iter().next() {
                    Some(o) => o,
                    None => return Err(Error::InternalServerError)
                };
                Some(d.into())
            } else {
                None
            }
        }
        None => None,
    };

    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPDomainCheckResult(domain_check) => {
                if let Some(domain_check) = domain_check.data.first() {
                    Response::Ok(CheckResponse {
                        eurid_idn: super::eurid::extract_eurid_idn_singular(&response.extension, domain_check.name.name.as_str())?,
                        avail: domain_check.name.available,
                        reason: domain_check.reason.to_owned(),
                        fee_check,
                        donuts_fee_check,
                        eurid_check: super::eurid::extract_eurid_domain_check_singular(&response.extension)?,
                    })
                } else {
                    Err(Error::InternalServerError)
                }
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

pub fn handle_claims_check(
    client: &EPPClientServerFeatures,
    req: &ClaimsCheckRequest,
) -> HandleReqReturn<ClaimsCheckResponse> {
    if !client.launch_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPCheck::Domain(proto::domain::EPPDomainCheck {
        name: req.name.clone(),
        auth_info: None,
    });
    let mut ext = vec![proto::EPPCommandExtensionType::EPPLaunchCheck(
        (&req.launch_check).into()
    )];

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((proto::EPPCommandType::Check(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_trademark_check(
    client: &EPPClientServerFeatures,
    req: &TrademarkCheckRequest,
) -> HandleReqReturn<ClaimsCheckResponse> {
    if !client.launch_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPCheck::Domain(proto::domain::EPPDomainCheck {
        name: req.name.clone(),
        auth_info: None,
    });
    let mut ext = vec![proto::EPPCommandExtensionType::EPPLaunchCheck(
        (&launch::LaunchTrademarkCheck {}).into()
    )];

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((proto::EPPCommandType::Check(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_claims_check_response(response: proto::EPPResponse) -> Response<ClaimsCheckResponse> {
    let claims_check = match response.extension {
        Some(ext) => {
            let claims = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPLaunchCheckData(i) => Some(i),
                _ => None,
            });

            if let Some(c) = claims {
                if let Some(domain_check) = c.data.first() {
                    Some(ClaimsCheckResponse {
                        exists: domain_check.name.exists,
                        claims_key: domain_check.claim_key.iter().map(Into::into).collect(),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        }
        None => None,
    };

    match response.data {
        Some(_) => Err(Error::InternalServerError),
        None => match claims_check {
            Some(c) => Response::Ok(c),
            None => Err(Error::InternalServerError),
        }
    }
}

pub fn handle_info(
    client: &EPPClientServerFeatures,
    req: &InfoRequest,
) -> HandleReqReturn<InfoResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPInfo::Domain(proto::domain::EPPDomainInfo {
        name: proto::domain::EPPDomainInfoName {
            name: req.name.clone(),
            hosts: req.hosts.as_ref().map(|h| match h {
                InfoHost::All => proto::domain::EPPDomainInfoHosts::All,
                InfoHost::Delegated => proto::domain::EPPDomainInfoHosts::Delegated,
                InfoHost::Subordinate => proto::domain::EPPDomainInfoHosts::Subordinate,
                InfoHost::None => proto::domain::EPPDomainInfoHosts::None,
            }),
        },
        auth_info: req.auth_info.as_ref().map(|a| proto::domain::EPPDomainAuthInfo {
            password: Some(a.clone())
        }),
    });
    let mut exts = vec![];
    if let Some(launch_info) = &req.launch_info {
        if client.launch_supported {
            exts.push(proto::EPPCommandExtensionType::EPPLaunchInfo(launch_info.into()))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }
    if client.verisign_whois_info {
        exts.push(proto::EPPCommandExtensionType::VerisignWhoisInfExt(proto::verisign::EPPWhoisInfoExt {
            flag: true
        }))
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut exts);
    Ok((proto::EPPCommandType::Info(command), match exts.is_empty() {
        true => None,
        false => Some(exts)
    }))
}

pub fn handle_info_response(response: proto::EPPResponse) -> Response<InfoResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPDomainInfoResult(domain_info) => {
                (*domain_info, &response.extension).try_into()
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

pub fn handle_create(
    client: &EPPClientServerFeatures,
    req: &CreateRequest,
) -> HandleReqReturn<CreateResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let no_registrant = client.has_erratum("verisign-com") || client.has_erratum("verisign-net")
        || client.has_erratum("verisign-cc") || client.has_erratum("verisign-tv");
    if !no_registrant {
        super::contact::check_id(&req.registrant)?;
    }

    let mut exts = vec![];
    match &req.sec_dns {
        Some(sec_dns) => {
            if client.secdns_supported || client.has_erratum("pir") {
                exts.push(proto::EPPCommandExtensionType::EPPSecDNSCreate(
                    match &sec_dns.data {
                        SecDNSDataType::DSData(ds_data) => proto::secdns::EPPSecDNSData {
                            max_signature_life: sec_dns.max_sig_life,
                            key_data: vec![],
                            ds_data: ds_data
                                .iter()
                                .map(|d| proto::secdns::EPPSecDNSDSData {
                                    key_tag: d.key_tag,
                                    algorithm: d.algorithm,
                                    digest_type: d.digest_type,
                                    digest: d.digest.clone(),
                                    key_data: d.key_data.as_ref().map(|k| {
                                        proto::secdns::EPPSecDNSKeyData {
                                            flags: k.flags,
                                            protocol: k.protocol,
                                            algorithm: k.algorithm,
                                            public_key: k.public_key.clone(),
                                        }
                                    }),
                                })
                                .collect(),
                        },
                        SecDNSDataType::KeyData(key_data) => proto::secdns::EPPSecDNSData {
                            max_signature_life: sec_dns.max_sig_life,
                            ds_data: vec![],
                            key_data: key_data
                                .iter()
                                .map(|k| proto::secdns::EPPSecDNSKeyData {
                                    flags: k.flags,
                                    protocol: k.protocol,
                                    algorithm: k.algorithm,
                                    public_key: k.public_key.clone(),
                                })
                                .collect(),
                        },
                    },
                ))
            } else {
                return Err(Err(Error::Unsupported));
            }
        }
        None => {}
    }

    if let Some(launch_create) = &req.launch_create {
        if client.launch_supported {
            if !launch_create.core_nic.is_empty() {
                if !(client.corenic_mark || client.has_erratum("corenic")) {
                    return Err(Err(Error::Unsupported));
                }

                for info in launch_create.core_nic.iter() {
                    if let Some(info_type) = &info.info_type {
                        if info_type.len() < 1 || info_type.len() > 64 {
                            return Err(Err(Error::Err(
                                "application info type has a min length of 1 and a max length of 64".to_string(),
                            )));
                        }
                    }
                    if info.info.len() < 1 || info.info.len() > 2048 {
                        return Err(Err(Error::Err(
                            "application info has a min length of 1 and a max length of 2048".to_string(),
                        )));
                    }
                }
            }

            exts.push(proto::EPPCommandExtensionType::EPPLaunchCreate(launch_create.into()))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    if let Some(fee_agreement) = &req.fee_agreement {
        if client.fee_supported {
            exts.push(proto::EPPCommandExtensionType::EPPFee10Create(fee_agreement.into()));
        } else if client.fee_011_supported {
            exts.push(proto::EPPCommandExtensionType::EPPFee011Create(fee_agreement.into()));
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    if let Some(eurid_data) = &req.eurid_data {
        if client.eurid_domain_support {
            exts.push(proto::EPPCommandExtensionType::EURIDDomainCreate(eurid_data.into()))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut exts);
    super::fee::handle_donuts_fee_agreement(client, &req.donuts_fee_agreement, &mut exts)?;

    let command = proto::EPPCreate::Domain(proto::domain::EPPDomainCreate {
        name: req.name.clone(),
        period: req.period.as_ref().map(|p| p.into()),
        nameservers: match req.nameservers.len() {
            0 => None,
            _ => Some(proto::domain::EPPDomainInfoNameservers {
                servers: req.nameservers.iter().map(|n| n.into()).collect(),
            }),
        },
        registrant: if no_registrant {
            None
        } else {
            Some(req.registrant.to_string())
        },
        contacts: req
            .contacts
            .iter()
            .map(|c| {
                super::contact::check_id(&c.contact_id)?;
                Ok(proto::domain::EPPDomainInfoContact {
                    contact_type: c.contact_type.to_string(),
                    contact_id: c.contact_id.to_string(),
                })
            })
            .collect::<Result<Vec<_>, super::router::Response<CreateResponse>>>()?,
        auth_info: proto::domain::EPPDomainAuthInfo {
            password: Some(req.auth_info.to_string()),
        },
    });
    Ok((
        proto::EPPCommandType::Create(command),
        match exts.len() {
            0 => None,
            _ => Some(exts),
        },
    ))
}

pub fn handle_create_response(response: proto::EPPResponse) -> Response<CreateResponse> {
    let pending = response.is_pending();
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPDomainCreateResult(domain_create) => {
                let mut res: CreateResponse = (Some(domain_create), &response.extension).try_into()?;
                res.pending = pending;
                Ok(res)
            }
            _ => Err(Error::InternalServerError),
        },
        None => {
            if response.is_pending() {
                let mut res: CreateResponse = (None, &response.extension).try_into()?;
                res.pending = response.is_pending();
                Ok(res)
            } else {
                Err(Error::InternalServerError)
            }
        }
    }
}

pub fn handle_delete(
    client: &EPPClientServerFeatures,
    req: &DeleteRequest,
) -> HandleReqReturn<DeleteResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPDelete::Domain(proto::domain::EPPDomainCheck {
        name: req.name.clone(),
        auth_info: None,
    });
    let mut ext = vec![];

    if let Some(eurid_data) = &req.eurid_data {
        if client.eurid_domain_support {
            ext.push(proto::EPPCommandExtensionType::EURIDDomainDelete(eurid_data.into()))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    super::fee::handle_donuts_fee_agreement(client, &req.donuts_fee_agreement, &mut ext)?;

    if let Some(launch_info) = &req.launch_info {
        if client.launch_supported {
            ext.push(proto::EPPCommandExtensionType::EPPLaunchDelete(launch_info.into()))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }
    Ok((proto::EPPCommandType::Delete(command), match ext.len() {
        0 => None,
        _ => Some(ext),
    }))
}

pub fn handle_delete_response(response: proto::EPPResponse) -> Response<DeleteResponse> {
    let fee_data = match &response.extension {
        Some(ext) => {
            let fee10 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee10DeleteData(i) => Some(i),
                _ => None,
            });
            let fee09 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee09DeleteData(i) => Some(i),
                _ => None,
            });
            let fee08 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee08DeleteData(i) => Some(i),
                _ => None,
            });
            let fee07 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee07DeleteData(i) => Some(i),
                _ => None,
            });
            let fee05 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee05DeleteData(i) => Some(i),
                _ => None,
            });

            if let Some(f) = fee10 {
                Some(f.into())
            } else if let Some(f) = fee09 {
                Some(f.into())
            } else if let Some(f) = fee08 {
                Some(f.into())
            } else if let Some(f) = fee07 {
                Some(f.into())
            } else if let Some(f) = fee05 {
                Some(f.into())
            } else {
                None
            }
        }
        None => None,
    };

    Response::Ok(DeleteResponse {
        pending: response.is_pending(),
        fee_data,
        eurid_idn: super::eurid::extract_eurid_idn_singular(&response.extension, None)?,
    })
}

pub fn handle_update(
    client: &EPPClientServerFeatures,
    req: &UpdateRequest,
) -> HandleReqReturn<UpdateResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let no_registrant = client.has_erratum("verisign-com") || client.has_erratum("verisign-net")
        || client.has_erratum("verisign-cc") || client.has_erratum("verisign-tv");
    if !no_registrant {
        if let Some(new_registrant) = &req.new_registrant {
            super::contact::check_id(&new_registrant)?;
        }
    }
    let mut adds = vec![];
    let mut rems = vec![];
    let mut add_ns = vec![];
    let mut rem_ns = vec![];
    for add in &req.add {
        match add {
            UpdateObject::Status(s) => adds.push(proto::domain::EPPDomainUpdateParam::Status(
                proto::domain::EPPDomainStatus {
                    status: s.into(),
                    message: None,
                },
            )),
            UpdateObject::Contact(c) => {
                super::contact::check_id(&c.contact_id)?;
                adds.push(proto::domain::EPPDomainUpdateParam::Contact(
                    proto::domain::EPPDomainInfoContact {
                        contact_type: c.contact_type.clone(),
                        contact_id: c.contact_id.clone(),
                    },
                ))
            }
            UpdateObject::Nameserver(n) => add_ns.push(n.into()),
        }
    }
    for rem in &req.remove {
        match rem {
            UpdateObject::Status(s) => rems.push(proto::domain::EPPDomainUpdateParam::Status(
                proto::domain::EPPDomainStatus {
                    status: s.into(),
                    message: None,
                },
            )),
            UpdateObject::Contact(c) => {
                super::contact::check_id(&c.contact_id)?;
                rems.push(proto::domain::EPPDomainUpdateParam::Contact(
                    proto::domain::EPPDomainInfoContact {
                        contact_type: c.contact_type.clone(),
                        contact_id: c.contact_id.clone(),
                    },
                ))
            }
            UpdateObject::Nameserver(n) => rem_ns.push(n.into()),
        }
    }
    if !add_ns.is_empty() {
        adds.push(proto::domain::EPPDomainUpdateParam::Nameserver(
            proto::domain::EPPDomainInfoNameservers { servers: add_ns },
        ))
    }
    if !rem_ns.is_empty() {
        rems.push(proto::domain::EPPDomainUpdateParam::Nameserver(
            proto::domain::EPPDomainInfoNameservers { servers: rem_ns },
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
    if req.add.is_empty()
        && req.remove.is_empty()
        && is_not_change
        && (req.sec_dns.is_none() || !client.secdns_supported)
    {
        return Err(Err(Error::Err(
            "at least one operation must be specified".to_string(),
        )));
    }

    let mut exts = vec![];
    match &req.sec_dns {
        Some(sec_dns) => {
            if client.secdns_supported || client.has_erratum("pir") {
                exts.push(proto::EPPCommandExtensionType::EPPSecDNSUpdate(
                    proto::secdns::EPPSecDNSUpdate {
                        urgent: sec_dns.urgent,
                        add: sec_dns.add.as_ref().map(|a| match a {
                            SecDNSDataType::DSData(ds_data) => proto::secdns::EPPSecDNSUpdateAdd {
                                key_data: vec![],
                                ds_data: ds_data
                                    .iter()
                                    .map(|d| proto::secdns::EPPSecDNSDSData {
                                        key_tag: d.key_tag,
                                        algorithm: d.algorithm,
                                        digest_type: d.digest_type,
                                        digest: d.digest.clone(),
                                        key_data: d.key_data.as_ref().map(|k| {
                                            proto::secdns::EPPSecDNSKeyData {
                                                flags: k.flags,
                                                protocol: k.protocol,
                                                algorithm: k.algorithm,
                                                public_key: k.public_key.clone(),
                                            }
                                        }),
                                    })
                                    .collect(),
                            },
                            SecDNSDataType::KeyData(key_data) => proto::secdns::EPPSecDNSUpdateAdd {
                                ds_data: vec![],
                                key_data: key_data
                                    .iter()
                                    .map(|k| proto::secdns::EPPSecDNSKeyData {
                                        flags: k.flags,
                                        protocol: k.protocol,
                                        algorithm: k.algorithm,
                                        public_key: k.public_key.clone(),
                                    })
                                    .collect(),
                            },
                        }),
                        remove: sec_dns.remove.as_ref().map(|r| match r {
                            UpdateSecDNSRemove::All(a) => proto::secdns::EPPSecDNSUpdateRemove {
                                all: Some(*a),
                                ds_data: vec![],
                                key_data: vec![],
                            },
                            UpdateSecDNSRemove::Data(d) => match d {
                                SecDNSDataType::DSData(ds_data) => {
                                    proto::secdns::EPPSecDNSUpdateRemove {
                                        all: None,
                                        key_data: vec![],
                                        ds_data: ds_data
                                            .iter()
                                            .map(|d| proto::secdns::EPPSecDNSDSData {
                                                key_tag: d.key_tag,
                                                algorithm: d.algorithm,
                                                digest_type: d.digest_type,
                                                digest: d.digest.clone(),
                                                key_data: None,
                                            })
                                            .collect(),
                                    }
                                }
                                SecDNSDataType::KeyData(key_data) => {
                                    proto::secdns::EPPSecDNSUpdateRemove {
                                        all: None,
                                        ds_data: vec![],
                                        key_data: key_data
                                            .iter()
                                            .map(|k| proto::secdns::EPPSecDNSKeyData {
                                                flags: k.flags,
                                                protocol: k.protocol,
                                                algorithm: k.algorithm,
                                                public_key: k.public_key.clone(),
                                            })
                                            .collect(),
                                    }
                                }
                            },
                        }),
                        change: sec_dns.new_max_sig_life.map(|s| {
                            proto::secdns::EPPSecDNSUpdateChange {
                                max_signature_life: Some(s),
                            }
                        }),
                    },
                ))
            } else {
                return Err(Err(Error::Unsupported));
            }
        },
        None => {}
    }

    if let Some(fee_agreement) = &req.fee_agreement {
        if client.fee_supported {
            exts.push(proto::EPPCommandExtensionType::EPPFee10Update(fee_agreement.into()));
        } else if client.fee_011_supported {
            exts.push(proto::EPPCommandExtensionType::EPPFee011Update(fee_agreement.into()));
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut exts);
    super::fee::handle_donuts_fee_agreement(client, &req.donuts_fee_agreement, &mut exts)?;

    if let Some(launch_info) = &req.launch_info {
        if client.launch_supported {
            exts.push(proto::EPPCommandExtensionType::EPPLaunchUpdate(launch_info.into()))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    let command = proto::EPPUpdate::Domain(proto::domain::EPPDomainUpdate {
        name: req.name.clone(),
        add: if adds.is_empty() {
            None
        } else {
            Some(proto::domain::EPPDomainUpdateAddRemove { params: adds })
        },
        remove: if rems.is_empty() {
            None
        } else {
            Some(proto::domain::EPPDomainUpdateAddRemove { params: rems })
        },
        change: if is_not_change {
            None
        } else {
            Some(proto::domain::EPPDomainUpdateChange {
                registrant: if no_registrant {
                    None
                } else {
                    req.new_registrant.clone()
                },
                auth_info: req
                    .new_auth_info
                    .as_ref()
                    .map(|a| proto::domain::EPPDomainAuthInfo {
                        password: Some(a.clone()),
                    }),
            })
        },
    });
    Ok((
        proto::EPPCommandType::Update(Box::new(command)),
        match exts.len() {
            0 => None,
            _ => Some(exts),
        },
    ))
}

pub fn handle_update_response(response: proto::EPPResponse) -> Response<UpdateResponse> {
    let fee_data = match &response.extension {
        Some(ext) => {
            let fee10 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee10UpdateData(i) => Some(i),
                _ => None,
            });
            let fee09 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee09UpdateData(i) => Some(i),
                _ => None,
            });
            let fee08 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee08UpdateData(i) => Some(i),
                _ => None,
            });
            let fee07 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee07UpdateData(i) => Some(i),
                _ => None,
            });
            let fee05 = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPFee05UpdateData(i) => Some(i),
                _ => None,
            });

            if let Some(f) = fee10 {
                Some(f.into())
            } else if let Some(f) = fee09 {
                Some(f.into())
            } else if let Some(f) = fee08 {
                Some(f.into())
            } else if let Some(f) = fee07 {
                Some(f.into())
            } else if let Some(f) = fee05 {
                Some(f.into())
            } else {
                None
            }
        }
        None => None,
    };

    let donuts_fee_data = match &response.extension {
        Some(ext) => {
            let charge = ext.value.iter().find_map(|p| match p {
                proto::EPPResponseExtensionType::EPPDonutsChargeUpdateData(i) => Some(i),
                _ => None,
            });

            charge.map(Into::into)
        }
        None => None,
    };

    Response::Ok(UpdateResponse {
        pending: response.is_pending(),
        fee_data,
        donuts_fee_data,
    })
}

pub fn handle_renew(
    client: &EPPClientServerFeatures,
    req: &RenewRequest,
) -> HandleReqReturn<RenewResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPRenew::Domain(proto::domain::EPPDomainRenew {
        name: req.name.clone(),
        period: req.add_period.as_ref().map(|p| p.into()),
        current_expiry_date: req.cur_expiry_date.date(),
    });
    let mut ext = vec![];

    if let Some(fee_agreement) = &req.fee_agreement {
        if client.fee_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee10Renew(fee_agreement.into()));
        } else if client.fee_011_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee011Renew(fee_agreement.into()));
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    super::fee::handle_donuts_fee_agreement(client, &req.donuts_fee_agreement, &mut ext)?;

    Ok((proto::EPPCommandType::Renew(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_renew_response(response: proto::EPPResponse) -> Response<RenewResponse> {
    let pending = response.is_pending();
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPDomainRenewResult(domain_renew) => {
                let mut res: RenewResponse = (domain_renew, &response.extension).try_into()?;
                res.pending = pending;
                Ok(res)
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

pub fn handle_transfer_query(
    client: &EPPClientServerFeatures,
    req: &TransferQueryRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Query,
        command: proto::EPPTransferCommand::DomainQuery(proto::domain::EPPDomainCheck {
            name: req.name.clone(),
            auth_info: req.auth_info.as_ref().map(|a| proto::domain::EPPDomainAuthInfo {
                password: Some(a.clone())
            }),
        }),
    };
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((proto::EPPCommandType::Transfer(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_transfer_request(
    client: &EPPClientServerFeatures,
    req: &TransferRequestRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Request,
        command: proto::EPPTransferCommand::DomainRequest(proto::domain::EPPDomainTransfer {
            name: req.name.clone(),
            period: req.add_period.as_ref().map(|p| p.into()),
            auth_info: Some(proto::domain::EPPDomainAuthInfo {
                password: Some(req.auth_info.clone())
            }),
        }),
    };
    let mut ext = vec![];

    if let Some(fee_agreement) = &req.fee_agreement {
        if client.fee_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee10Transfer(fee_agreement.into()));
        } else if client.fee_011_supported {
            ext.push(proto::EPPCommandExtensionType::EPPFee011Transfer(fee_agreement.into()));
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    super::fee::handle_donuts_fee_agreement(client, &req.donuts_fee_agreement, &mut ext)?;

    Ok((proto::EPPCommandType::Transfer(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_transfer_cancel(
    client: &EPPClientServerFeatures,
    req: &TransferAcceptRejectRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Cancel,
        command: proto::EPPTransferCommand::DomainRequest(proto::domain::EPPDomainTransfer {
            name: req.name.clone(),
            period: None,
            auth_info: req.auth_info.as_ref().map(|a| proto::domain::EPPDomainAuthInfo {
                password: Some(a.clone())
            }),
        }),
    };
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((proto::EPPCommandType::Transfer(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_transfer_accept(
    client: &EPPClientServerFeatures,
    req: &TransferAcceptRejectRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Accept,
        command: proto::EPPTransferCommand::DomainRequest(proto::domain::EPPDomainTransfer {
            name: req.name.clone(),
            period: None,
            auth_info: req.auth_info.as_ref().map(|a| proto::domain::EPPDomainAuthInfo {
                password: Some(a.clone())
            }),
        }),
    };
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((proto::EPPCommandType::Transfer(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_transfer_reject(
    client: &EPPClientServerFeatures,
    req: &TransferAcceptRejectRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.domain_supported {
        return Err(Err(Error::Unsupported));
    }
    check_domain(&req.name)?;
    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Reject,
        command: proto::EPPTransferCommand::DomainRequest(proto::domain::EPPDomainTransfer {
            name: req.name.clone(),
            period: None,
            auth_info: req.auth_info.as_ref().map(|a| proto::domain::EPPDomainAuthInfo {
                password: Some(a.clone())
            }),
        }),
    };
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);

    Ok((proto::EPPCommandType::Transfer(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_transfer_response(response: proto::EPPResponse) -> Response<TransferResponse> {
    let pending = response.is_pending();
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPDomainTransferResult(domain_transfer) => {
                let mut res: TransferResponse = (domain_transfer, &response.extension).try_into()?;
                res.pending = pending;
                Ok(res)
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

/// Checks if a domain name is available
///
/// # Arguments
/// * `domain` - The domain in question
/// * `launch_check` - Launch availability info
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn check(
    domain: &str,
    fee_check: Option<fee::FeeCheck>,
    launch_check: Option<launch::LaunchAvailabilityCheck>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<CheckResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainCheck(Box::new(CheckRequest {
            name: domain.to_string(),
            fee_check,
            launch_check,
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Checks if a domain name has claims registered for a launch phase
///
/// # Arguments
/// * `domain` - The domain in question
/// * `launch_check` - Launch claims info
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn launch_claims_check(
    domain: &str,
    launch_check: launch::LaunchClaimsCheck,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<ClaimsCheckResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainClaimsCheck(Box::new(ClaimsCheckRequest {
            name: domain.to_string(),
            launch_check,
            return_path: sender,
        })),
        receiver,
    )
        .await
}


/// Checks if a domain name has trademarks registered for it
///
/// # Arguments
/// * `domain` - The domain in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn launch_trademark_check(
    domain: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<ClaimsCheckResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTrademarkCheck(Box::new(TrademarkCheckRequest {
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
    auth_info: Option<&str>,
    hosts: Option<InfoHost>,
    launch_info: Option<launch::LaunchInfo>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<InfoResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainInfo(Box::new(InfoRequest {
            name: domain.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            hosts,
            launch_info,
            return_path: sender,
        })),
        receiver,
    )
        .await
}

pub struct CreateInfo<'a> {
    pub domain: &'a str,
    pub period: Option<Period>,
    pub registrant: &'a str,
    pub contacts: Vec<InfoContact>,
    pub nameservers: Vec<InfoNameserver>,
    pub auth_info: &'a str,
    pub sec_dns: Option<SecDNSData>,
    pub launch_create: Option<launch::LaunchCreate>,
    pub fee_agreement: Option<fee::FeeAgreement>,
    pub donuts_fee_agreement: Option<fee::DonutsFeeData>,
    pub eurid_data: Option<super::eurid::DomainCreate>,
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
    info: CreateInfo<'_>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<CreateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainCreate(Box::new(CreateRequest {
            name: info.domain.to_string(),
            period: info.period,
            registrant: info.registrant.to_string(),
            contacts: info.contacts,
            nameservers: info.nameservers,
            auth_info: info.auth_info.to_string(),
            sec_dns: info.sec_dns,
            launch_create: info.launch_create,
            fee_agreement: info.fee_agreement,
            donuts_fee_agreement: info.donuts_fee_agreement,
            eurid_data: info.eurid_data,
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
    launch_info: Option<launch::LaunchUpdate>,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    eurid_data: Option<super::eurid::DomainDelete>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<DeleteResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainDelete(Box::new(DeleteRequest {
            name: domain.to_string(),
            launch_info,
            donuts_fee_agreement,
            eurid_data,
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
    sec_dns: Option<UpdateSecDNS>,
    launch_info: Option<launch::LaunchUpdate>,
    fee_agreement: Option<fee::FeeAgreement>,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<UpdateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainUpdate(Box::new(UpdateRequest {
            name: domain.to_string(),
            add,
            remove,
            new_registrant: new_registrant.map(|s| s.into()),
            new_auth_info: new_auth_info.map(|s| s.into()),
            sec_dns,
            launch_info,
            fee_agreement,
            donuts_fee_agreement,
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
    fee_agreement: Option<fee::FeeAgreement>,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<RenewResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainRenew(Box::new(RenewRequest {
            name: domain.to_string(),
            add_period,
            cur_expiry_date,
            fee_agreement,
            donuts_fee_agreement,
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
/// * `auth_info` - Auth info for the domain (not always required)
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_query(
    domain: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTransferQuery(Box::new(TransferQueryRequest {
            name: domain.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Requests the transfer of a domain name
///
/// # Arguments
/// * `domain` - The domain to be transferred
/// * `add_period` - How much time to add to the domain's expiry on transfer
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_request(
    domain: &str,
    add_period: Option<Period>,
    auth_info: &str,
    fee_agreement: Option<fee::FeeAgreement>,
    donuts_fee_agreement: Option<fee::DonutsFeeData>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTransferRequest(Box::new(TransferRequestRequest {
            name: domain.to_string(),
            add_period,
            auth_info: auth_info.to_string(),
            fee_agreement,
            donuts_fee_agreement,
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Cancels the pending transfer of a domain name
///
/// # Arguments
/// * `domain` - The domain to be cancelled
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_cancel(
    domain: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTransferCancel(Box::new(TransferAcceptRejectRequest {
            name: domain.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Accepts the transfer of a domain name
///
/// # Arguments
/// * `domain` - The domain to be approved
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_accept(
    domain: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTransferAccept(Box::new(TransferAcceptRejectRequest {
            name: domain.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Rejects the transfer of a domain name
///
/// # Arguments
/// * `domain` - The domain to be rejected
/// * `auth_info` - Auth info for the domain
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_reject(
    domain: &str,
    auth_info: Option<&str>,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::DomainTransferReject(Box::new(TransferAcceptRejectRequest {
            name: domain.to_string(),
            auth_info: auth_info.map(|s| s.into()),
            return_path: sender,
        })),
        receiver,
    )
        .await
}

#[cfg(test)]
mod domain_tests {
    #[test]
    fn claims_check() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1000">
     <msg>Command completed successfully</msg>
    </result>
    <extension>
     <launch:chkData
      xmlns:launch="urn:ietf:params:xml:ns:launch-1.0">
      <launch:phase>claims</launch:phase>
      <launch:cd>
        <launch:name exists="1">domain3.example</launch:name>
        <launch:claimKey validatorID="tmch">
        2013041500/2/6/9/rJ1NrDO92vDsAzf7EQzgjX4R0000000001
        </launch:claimKey>
        <launch:claimKey validatorID="custom-tmch">
        20140423200/1/2/3/rJ1Nr2vDsAzasdff7EasdfgjX4R000000002
        </launch:claimKey>
      </launch:cd>
     </launch:chkData>
    </extension>
    <trID>
     <clTRID>ABC-12345</clTRID>
     <svTRID>54321-XYZ</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_claims_check_response(*res).unwrap();
        assert_eq!(data.exists, true);
        assert_eq!(data.claims_key.len(), 2);
        let claims_key_1 = data.claims_key.get(0).unwrap();
        let claims_key_2 = data.claims_key.get(1).unwrap();
        assert_eq!(claims_key_1.validator_id.as_ref().unwrap(), "tmch");
        assert_eq!(claims_key_1.key, "2013041500/2/6/9/rJ1NrDO92vDsAzf7EQzgjX4R0000000001");
        assert_eq!(claims_key_2.validator_id.as_ref().unwrap(), "custom-tmch");
        assert_eq!(claims_key_2.key, "20140423200/1/2/3/rJ1Nr2vDsAzasdff7EasdfgjX4R000000002");
    }

    #[test]
    fn launch_info() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <domain:infData
       xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
        <domain:name>domain.example</domain:name>
        <domain:roid>EXAMPLE1-REP</domain:roid>
        <domain:status s="pendingCreate"/>
        <domain:registrant>jd1234</domain:registrant>
        <domain:contact type="admin">sh8013</domain:contact>
        <domain:contact type="tech">sh8013</domain:contact>
        <domain:clID>ClientX</domain:clID>
        <domain:crID>ClientY</domain:crID>
        <domain:crDate>2012-04-03T22:00:00.0Z</domain:crDate>
        <domain:authInfo>
          <domain:pw>2fooBAR</domain:pw>
        </domain:authInfo>
      </domain:infData>
    </resData>
    <extension>
      <launch:infData
       xmlns:launch="urn:ietf:params:xml:ns:launch-1.0">
        <launch:phase>sunrise</launch:phase>
          <launch:applicationID>abc123</launch:applicationID>
          <launch:status s="pendingValidation"/>
          <mark:mark
            xmlns:mark="urn:ietf:params:xml:ns:mark-1.0">
             Test
         </mark:mark>
      </launch:infData>
    </extension>
    <trID>
      <clTRID>ABC-12345</clTRID>
      <svTRID>54321-XYZ</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_info_response(*res).unwrap();
        assert_eq!(data.name, "domain.example");
        let launch_info = data.launch_info.unwrap();
        assert_eq!(launch_info.phase.phase_type, super::launch::PhaseType::Sunrise);
        assert_eq!(launch_info.application_id.unwrap(), "abc123");
        assert_eq!(launch_info.status.unwrap().status_type, super::launch::LaunchStatusType::PendingValidation);
        assert_eq!(launch_info.mark.unwrap(), r#"<mark:mark xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:launch="urn:ietf:params:xml:ns:launch-1.0" xmlns:mark="urn:ietf:params:xml:ns:mark-1.0">Test</mark:mark>"#);
    }

    #[test]
    fn create_info() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1001">
      <msg>Command completed successfully; action pending</msg>
    </result>
    <resData>
      <domain:creData
         xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
       <domain:name>domain.example</domain:name>
       <domain:crDate>2010-08-10T15:38:26.623854Z</domain:crDate>
      </domain:creData>
    </resData>
    <extension>
      <launch:creData
        xmlns:launch="urn:ietf:params:xml:ns:launch-1.0">
        <launch:phase>sunrise</launch:phase>
        <launch:applicationID>2393-9323-E08C-03B1
        </launch:applicationID>
      </launch:creData>
    </extension>
    <trID>
      <clTRID>ABC-12345</clTRID>
      <svTRID>54321-XYZ</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_create_response(*res).unwrap();
        assert_eq!(data.pending, true);
        let launch_create = data.launch_create.unwrap();
        assert_eq!(launch_create.phase.phase_type, super::launch::PhaseType::Sunrise);
        assert_eq!(launch_create.application_id.unwrap(), "2393-9323-E08C-03B1");
    }
}
