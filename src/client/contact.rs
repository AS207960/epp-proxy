//! EPP commands relating to contact objects

use std::convert::{TryFrom, TryInto};

use chrono::prelude::*;
use regex::Regex;

use super::{EPPClientServerFeatures, Error, proto, Request, Response, Sender};
use super::router::HandleReqReturn;

#[derive(Debug)]
pub struct CheckRequest {
    id: String,
    pub return_path: Sender<CheckResponse>,
}

/// Response to a contact check query
#[derive(Debug)]
pub struct CheckResponse {
    /// Is the contact available for creation commands
    pub avail: bool,
    /// An optional reason for the ID's status
    pub reason: Option<String>,
}

#[derive(Debug)]
pub struct InfoRequest {
    id: String,
    pub return_path: Sender<InfoResponse>,
}

/// Response to a contact info query
#[derive(Debug)]
pub struct InfoResponse {
    /// The contact's ID
    pub id: String,
    /// The contact's internal registry ID
    pub registry_id: String,
    /// Statuses currently set on the contact
    pub statuses: Vec<Status>,
    /// The localised address of the contact
    pub local_address: Option<Address>,
    /// The internationalised address of the contact
    pub internationalised_address: Option<Address>,
    /// Voice phone number of the contact
    pub phone: Option<Phone>,
    /// Fax number of the contact
    pub fax: Option<Phone>,
    /// Email address of the contact
    pub email: String,
    /// Sponsoring client ID
    pub client_id: String,
    /// ID of the client that created the contact
    pub client_created_id: Option<String>,
    /// Date of creation
    pub creation_date: Option<DateTime<Utc>>,
    /// ID of the client that last updated the contact
    pub last_updated_client: Option<String>,
    /// Date of last update
    pub last_updated_date: Option<DateTime<Utc>>,
    /// Date of last transfer
    pub last_transfer_date: Option<DateTime<Utc>>,
    pub entity_type: EntityType,
    pub trading_name: Option<String>,
    pub company_number: Option<String>,
    pub disclosure: Vec<DisclosureType>,
    pub auth_info: Option<String>,
}

#[derive(Debug)]
pub struct Phone {
    /// Initial dialable part of the number
    pub number: String,
    /// Optional internal extension
    pub extension: Option<String>,
}

#[derive(Debug)]
pub struct Address {
    /// Name of the contact
    pub name: String,
    /// Organisation of the contact
    pub organisation: Option<String>,
    /// 1-3 street address lines
    pub streets: Vec<String>,
    pub city: String,
    /// Province or state
    pub province: Option<String>,
    pub postal_code: Option<String>,
    /// ISO 2 letter country code
    pub country_code: String,
    /// National ID number for individuals
    pub identity_number: Option<String>,
    /// Individuals birth date
    pub birth_date: Option<Date<Utc>>,
}

#[derive(Debug)]
pub enum EntityType {
    UkLimitedCompany,
    UkPublicLimitedCompany,
    UkPartnership,
    UkSoleTrader,
    UkLimitedLiabilityPartnership,
    UkIndustrialProvidentRegisteredCompany,
    UkIndividual,
    UkSchool,
    UkRegisteredCharity,
    UkGovernmentBody,
    UkCorporationByRoyalCharter,
    UkStatutoryBody,
    UkPoliticalParty,
    OtherUkEntity,
    FinnishIndividual,
    FinnishCompany,
    FinnishAssociation,
    FinnishInstitution,
    FinnishPoliticalParty,
    FinnishMunicipality,
    FinnishGovernment,
    FinnishPublicCommunity,
    OtherIndividual,
    OtherCompany,
    OtherAssociation,
    OtherInstitution,
    OtherPoliticalParty,
    OtherMunicipality,
    OtherGovernment,
    OtherPublicCommunity,
    Unknown,
}

impl From<&proto::nominet::EPPContactType> for EntityType {
    fn from(from: &proto::nominet::EPPContactType) -> Self {
        use proto::nominet::EPPContactType;
        match from {
            EPPContactType::UkLimitedCompany => EntityType::UkLimitedCompany,
            EPPContactType::UkPublicLimitedCompany => EntityType::UkLimitedCompany,
            EPPContactType::UkPartnership => EntityType::UkPartnership,
            EPPContactType::UkSoleTrader => EntityType::UkSoleTrader,
            EPPContactType::UkLimitedLiabilityPartnership => {
                EntityType::UkLimitedLiabilityPartnership
            }
            EPPContactType::UkIndustrialProvidentRegisteredCompany => {
                EntityType::UkIndustrialProvidentRegisteredCompany
            }
            EPPContactType::UkIndividual => EntityType::UkIndividual,
            EPPContactType::UkSchool => EntityType::UkSchool,
            EPPContactType::UkRegisteredCharity => EntityType::UkRegisteredCharity,
            EPPContactType::UkGovernmentBody => EntityType::UkGovernmentBody,
            EPPContactType::UkCorporationByRoyalCharter => EntityType::UkCorporationByRoyalCharter,
            EPPContactType::UkStatutoryBody => EntityType::UkStatutoryBody,
            EPPContactType::NonUkIndividual => EntityType::OtherIndividual,
            EPPContactType::NonUkCompany => EntityType::OtherCompany,
            EPPContactType::OtherUkEntity => EntityType::OtherUkEntity,
            EPPContactType::OtherNonUkEntity => EntityType::OtherCompany,
            EPPContactType::Unknown => EntityType::Unknown,
        }
    }
}

impl From<&EntityType> for proto::nominet::EPPContactType {
    fn from(from: &EntityType) -> Self {
        use proto::nominet::EPPContactType;
        match from {
            EntityType::UkLimitedCompany => EPPContactType::UkLimitedCompany,
            EntityType::UkPublicLimitedCompany => EPPContactType::UkPublicLimitedCompany,
            EntityType::UkPartnership => EPPContactType::UkPartnership,
            EntityType::UkSoleTrader => EPPContactType::UkSoleTrader,
            EntityType::UkLimitedLiabilityPartnership => {
                EPPContactType::UkLimitedLiabilityPartnership
            }
            EntityType::UkIndustrialProvidentRegisteredCompany => {
                EPPContactType::UkIndustrialProvidentRegisteredCompany
            }
            EntityType::UkIndividual => EPPContactType::UkIndividual,
            EntityType::UkSchool => EPPContactType::UkSchool,
            EntityType::UkRegisteredCharity => EPPContactType::UkRegisteredCharity,
            EntityType::UkGovernmentBody => EPPContactType::UkGovernmentBody,
            EntityType::UkCorporationByRoyalCharter => EPPContactType::UkCorporationByRoyalCharter,
            EntityType::UkStatutoryBody => EPPContactType::UkStatutoryBody,
            EntityType::OtherUkEntity => EPPContactType::OtherUkEntity,
            EntityType::FinnishIndividual | EntityType::OtherIndividual => {
                EPPContactType::NonUkIndividual
            }
            EntityType::FinnishCompany | EntityType::OtherCompany => EPPContactType::NonUkCompany,
            _ => EPPContactType::OtherNonUkEntity,
        }
    }
}

impl From<&EntityType> for proto::traficom::EPPContactTraficomType {
    fn from(from: &EntityType) -> Self {
        use proto::traficom::EPPContactTraficomType;
        match from {
            EntityType::FinnishIndividual
            | EntityType::OtherIndividual
            | EntityType::UkIndividual
            | EntityType::UkSoleTrader
            | EntityType::OtherUkEntity
            | EntityType::Unknown => EPPContactTraficomType::PrivatePerson,
            EntityType::FinnishCompany
            | EntityType::OtherCompany
            | EntityType::UkLimitedCompany
            | EntityType::UkPublicLimitedCompany
            | EntityType::UkCorporationByRoyalCharter
            | EntityType::UkRegisteredCharity
            | EntityType::UkIndustrialProvidentRegisteredCompany => EPPContactTraficomType::Company,
            EntityType::UkPartnership
            | EntityType::UkLimitedLiabilityPartnership
            | EntityType::FinnishAssociation
            | EntityType::OtherAssociation => EPPContactTraficomType::Association,
            EntityType::UkSchool
            | EntityType::UkStatutoryBody
            | EntityType::FinnishInstitution
            | EntityType::OtherInstitution => EPPContactTraficomType::Institution,
            EntityType::UkPoliticalParty
            | EntityType::FinnishPoliticalParty
            | EntityType::OtherPoliticalParty => EPPContactTraficomType::PoliticalParty,
            EntityType::UkGovernmentBody
            | EntityType::FinnishGovernment
            | EntityType::OtherGovernment => EPPContactTraficomType::Government,
            EntityType::FinnishMunicipality | EntityType::OtherMunicipality => {
                EPPContactTraficomType::Municipality
            }
            EntityType::FinnishPublicCommunity | EntityType::OtherPublicCommunity => {
                EPPContactTraficomType::PublicCommunity
            }
        }
    }
}

fn traficom_type_to_entity_type(
    from: &proto::traficom::EPPContactTraficomType,
    is_finnish: bool,
) -> EntityType {
    use proto::traficom::EPPContactTraficomType;
    match (from, is_finnish) {
        (EPPContactTraficomType::PrivatePerson, true) => EntityType::FinnishIndividual,
        (EPPContactTraficomType::PrivatePerson, false) => EntityType::OtherIndividual,
        (EPPContactTraficomType::Company, true) => EntityType::FinnishCompany,
        (EPPContactTraficomType::Company, false) => EntityType::OtherCompany,
        (EPPContactTraficomType::Association, true) => EntityType::FinnishAssociation,
        (EPPContactTraficomType::Association, false) => EntityType::OtherAssociation,
        (EPPContactTraficomType::Institution, true) => EntityType::FinnishInstitution,
        (EPPContactTraficomType::Institution, false) => EntityType::OtherInstitution,
        (EPPContactTraficomType::PoliticalParty, true) => EntityType::FinnishPoliticalParty,
        (EPPContactTraficomType::PoliticalParty, false) => EntityType::OtherPoliticalParty,
        (EPPContactTraficomType::Municipality, true) => EntityType::FinnishMunicipality,
        (EPPContactTraficomType::Municipality, false) => EntityType::OtherMunicipality,
        (EPPContactTraficomType::Government, true) => EntityType::FinnishGovernment,
        (EPPContactTraficomType::Government, false) => EntityType::OtherGovernment,
        (EPPContactTraficomType::PublicCommunity, true) => EntityType::FinnishPublicCommunity,
        (EPPContactTraficomType::PublicCommunity, false) => EntityType::OtherPublicCommunity,
    }
}

fn is_entity_finnish(entity: &Option<EntityType>) -> bool {
    match entity {
        Some(e) => match e {
            EntityType::FinnishIndividual
            | EntityType::FinnishCompany
            | EntityType::FinnishAssociation
            | EntityType::FinnishInstitution
            | EntityType::FinnishGovernment
            | EntityType::FinnishMunicipality
            | EntityType::FinnishPoliticalParty
            | EntityType::FinnishPublicCommunity => true,
            _ => false,
        },
        None => false,
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DisclosureType {
    LocalName = 1,
    InternationalisedName = 2,
    LocalOrganisation = 3,
    InternationalisedOrganisation = 4,
    LocalAddress = 5,
    InternationalisedAddress = 6,
    Voice = 7,
    Fax = 8,
    Email = 9,
}

impl From<&DisclosureType> for proto::contact::EPPContactDisclosureItem {
    fn from(from: &DisclosureType) -> Self {
        use proto::contact::EPPContactDisclosureItem;
        use proto::contact::EPPContactPostalInfoType;
        match from {
            DisclosureType::LocalName => EPPContactDisclosureItem::Name {
                addr_type: EPPContactPostalInfoType::Local,
            },
            DisclosureType::InternationalisedName => EPPContactDisclosureItem::Name {
                addr_type: EPPContactPostalInfoType::Internationalised,
            },
            DisclosureType::LocalOrganisation => EPPContactDisclosureItem::Organisation {
                addr_type: EPPContactPostalInfoType::Local,
            },
            DisclosureType::InternationalisedOrganisation => {
                EPPContactDisclosureItem::Organisation {
                    addr_type: EPPContactPostalInfoType::Internationalised,
                }
            }
            DisclosureType::LocalAddress => EPPContactDisclosureItem::Address {
                addr_type: EPPContactPostalInfoType::Local,
            },
            DisclosureType::InternationalisedAddress => EPPContactDisclosureItem::Address {
                addr_type: EPPContactPostalInfoType::Internationalised,
            },
            DisclosureType::Voice => EPPContactDisclosureItem::Voice {},
            DisclosureType::Fax => EPPContactDisclosureItem::Fax {},
            DisclosureType::Email => EPPContactDisclosureItem::Email {},
        }
    }
}

impl From<&proto::contact::EPPContactDisclosureItem> for Option<DisclosureType> {
    fn from(from: &proto::contact::EPPContactDisclosureItem) -> Self {
        use proto::contact::EPPContactDisclosureItem;
        use proto::contact::EPPContactPostalInfoType;
        match from {
            EPPContactDisclosureItem::DisclosureType { .. } => None,
            EPPContactDisclosureItem::Name {
                addr_type: EPPContactPostalInfoType::Local,
            } => Some(DisclosureType::LocalName),
            EPPContactDisclosureItem::Name {
                addr_type: EPPContactPostalInfoType::Internationalised,
            } => Some(DisclosureType::InternationalisedName),
            EPPContactDisclosureItem::Organisation {
                addr_type: EPPContactPostalInfoType::Local,
            } => Some(DisclosureType::LocalOrganisation),
            EPPContactDisclosureItem::Organisation {
                addr_type: EPPContactPostalInfoType::Internationalised,
            } => Some(DisclosureType::InternationalisedOrganisation),
            EPPContactDisclosureItem::Address {
                addr_type: EPPContactPostalInfoType::Local,
            } => Some(DisclosureType::LocalAddress),
            EPPContactDisclosureItem::Address {
                addr_type: EPPContactPostalInfoType::Internationalised,
            } => Some(DisclosureType::InternationalisedAddress),
            EPPContactDisclosureItem::Voice {} => Some(DisclosureType::Voice),
            EPPContactDisclosureItem::Fax {} => Some(DisclosureType::Fax),
            EPPContactDisclosureItem::Email {} => Some(DisclosureType::Email),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Status {
    ClientDeleteProhibited,
    ClientTransferProhibited,
    ClientUpdateProhibited,
    Linked,
    Ok,
    PendingCreate,
    PendingDelete,
    PendingTransfer,
    PendingUpdate,
    ServerDeleteProhibited,
    ServerTransferProhibited,
    ServerUpdateProhibited,
}

impl From<proto::contact::EPPContactStatusType> for Status {
    fn from(from: proto::contact::EPPContactStatusType) -> Self {
        use proto::contact::EPPContactStatusType;
        match from {
            EPPContactStatusType::ClientDeleteProhibited => Status::ClientDeleteProhibited,
            EPPContactStatusType::ClientTransferProhibited => Status::ClientTransferProhibited,
            EPPContactStatusType::ClientUpdateProhibited => Status::ClientUpdateProhibited,
            EPPContactStatusType::Linked => Status::Linked,
            EPPContactStatusType::Ok => Status::Ok,
            EPPContactStatusType::PendingCreate => Status::PendingCreate,
            EPPContactStatusType::PendingDelete => Status::PendingDelete,
            EPPContactStatusType::PendingTransfer => Status::PendingTransfer,
            EPPContactStatusType::PendingUpdate => Status::PendingUpdate,
            EPPContactStatusType::ServerDeleteProhibited => Status::ServerDeleteProhibited,
            EPPContactStatusType::ServerTransferProhibited => Status::ServerTransferProhibited,
            EPPContactStatusType::ServerUpdateProhibited => Status::ServerUpdateProhibited,
        }
    }
}

impl From<&Status> for proto::contact::EPPContactStatusType {
    fn from(from: &Status) -> Self {
        use proto::contact::EPPContactStatusType;
        match from {
            Status::ClientDeleteProhibited => EPPContactStatusType::ClientDeleteProhibited,
            Status::ClientTransferProhibited => EPPContactStatusType::ClientTransferProhibited,
            Status::ClientUpdateProhibited => EPPContactStatusType::ClientUpdateProhibited,
            Status::Linked => EPPContactStatusType::Linked,
            Status::Ok => EPPContactStatusType::Ok,
            Status::PendingCreate => EPPContactStatusType::PendingCreate,
            Status::PendingDelete => EPPContactStatusType::PendingDelete,
            Status::PendingTransfer => EPPContactStatusType::PendingTransfer,
            Status::PendingUpdate => EPPContactStatusType::PendingUpdate,
            Status::ServerDeleteProhibited => EPPContactStatusType::ServerDeleteProhibited,
            Status::ServerTransferProhibited => EPPContactStatusType::ServerTransferProhibited,
            Status::ServerUpdateProhibited => EPPContactStatusType::ServerUpdateProhibited,
        }
    }
}

impl From<proto::contact::EPPContactPhone> for Phone {
    fn from(from: proto::contact::EPPContactPhone) -> Self {
        Phone {
            number: from.number,
            extension: from.extension,
        }
    }
}

impl From<&Phone> for proto::contact::EPPContactPhone {
    fn from(from: &Phone) -> Self {
        proto::contact::EPPContactPhone {
            number: from.number.clone(),
            extension: from.extension.clone(),
        }
    }
}

#[derive(Debug)]
pub struct CreateRequest {
    id: String,
    local_address: Option<Address>,
    internationalised_address: Option<Address>,
    phone: Option<Phone>,
    fax: Option<Phone>,
    email: String,
    entity_type: Option<EntityType>,
    trading_name: Option<String>,
    company_number: Option<String>,
    disclosure: Option<Vec<DisclosureType>>,
    auth_info: String,
    pub return_path: Sender<CreateResponse>,
}

#[derive(Debug)]
pub struct CreateResponse {
    /// The actual contact ID created
    pub id: String,
    /// Was the request completed instantly or not
    pub pending: bool,
    pub transaction_id: String,
    /// What date did the server log as the date of creation
    pub creation_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct DeleteRequest {
    id: String,
    pub return_path: Sender<DeleteResponse>,
}

#[derive(Debug)]
pub struct DeleteResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    pub transaction_id: String,
}

#[derive(Debug)]
pub struct UpdateRequest {
    id: String,
    add_statuses: Vec<Status>,
    remove_statuses: Vec<Status>,
    new_local_address: Option<Address>,
    new_internationalised_address: Option<Address>,
    new_phone: Option<Phone>,
    new_fax: Option<Phone>,
    new_email: Option<String>,
    new_entity_type: Option<EntityType>,
    new_trading_name: Option<String>,
    new_company_number: Option<String>,
    new_disclosure: Option<Vec<DisclosureType>>,
    new_auth_info: Option<String>,
    pub return_path: Sender<UpdateResponse>,
}

#[derive(Debug)]
pub struct UpdateResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    pub transaction_id: String,
}

pub(crate) fn check_id<T>(id: &str) -> Result<(), Response<T>> {
    if let 3..=16 = id.len() {
        Ok(())
    } else {
        Err(Err(Error::Err(
            "contact id has a min length of 3 and a max length of 16".to_string(),
        )))
    }
}

#[derive(Debug)]
pub struct TransferQueryRequest {
    id: String,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferRequestRequest {
    id: String,
    auth_info: String,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
    pub transaction_id: String,
    pub data: TransferData,

}

#[derive(Debug)]
pub struct TransferData {
    pub status: super::TransferStatus,
    /// Which client requested the transfer
    pub requested_client_id: String,
    /// The date of the transfer request
    pub requested_date: DateTime<Utc>,
    /// Whcich client last acted / needs to act
    pub act_client_id: String,
    /// Date on which a client acted / must act by
    pub act_date: DateTime<Utc>,
}

#[derive(Debug)]
pub struct PanData {
    pub id: String,
    pub result: bool,
    pub server_transaction_id: Option<String>,
    pub client_transaction_id: Option<String>,
    pub date: DateTime<Utc>,
}

impl
TryFrom<(
    proto::contact::EPPContactInfoData,
    &Option<proto::EPPResponseExtension>,
)> for InfoResponse
{
    type Error = Error;

    fn try_from(
        from: (
            proto::contact::EPPContactInfoData,
            &Option<proto::EPPResponseExtension>,
        ),
    ) -> Result<Self, Self::Error> {
        let (contact_info, extension) = from;
        let map_addr = |a: Option<&proto::contact::EPPContactPostalInfo>| match a {
            Some(p) => Some(Address {
                name: match &p.name {
                    Some(n) => n.clone(),
                    None => format!(
                        "{} {}",
                        p.traficom_first_name.as_deref().unwrap_or_default(),
                        p.traficom_last_name.as_deref().unwrap_or_default()
                    ),
                },
                organisation: p.organisation.clone(),
                streets: p.address.streets.clone(),
                city: p.address.city.clone(),
                province: p.address.province.clone(),
                postal_code: p.address.postal_code.clone(),
                country_code: p.address.country_code.clone(),
                identity_number: p.traficom_identity.clone(),
                birth_date: p.traficom_birth_date,
            }),
            None => None,
        };
        let ext_info = match extension {
            Some(e) => match e.value.iter().find(|e| match e {
                proto::EPPResponseExtensionType::NominetContactExtInfo(_) => true,
                _ => false,
            }) {
                Some(e) => match e {
                    proto::EPPResponseExtensionType::NominetContactExtInfo(e) => Some(e),
                    _ => unreachable!(),
                },
                None => None,
            },
            None => None,
        };
        let local_address = contact_info
            .postal_info
            .iter()
            .find(|p| p.addr_type == proto::contact::EPPContactPostalInfoType::Local);
        Ok(InfoResponse {
            id: contact_info.id,
            statuses: contact_info
                .statuses
                .into_iter()
                .map(|s| s.status.into())
                .collect(),
            registry_id: contact_info.registry_id,
            local_address: map_addr(local_address),
            internationalised_address: map_addr(contact_info.postal_info.iter().find(|p| {
                p.addr_type == proto::contact::EPPContactPostalInfoType::Internationalised
            })),
            phone: contact_info.phone.map(|p| p.into()),
            fax: contact_info.fax.map(|p| p.into()),
            email: match contact_info.email {
                Some(e) => e,
                None => contact_info.traficom_legal_email.unwrap_or_default()
            },
            client_id: contact_info.client_id,
            client_created_id: contact_info.client_created_id,
            creation_date: contact_info.creation_date,
            last_updated_client: contact_info.last_updated_client,
            last_updated_date: contact_info.last_updated_date,
            last_transfer_date: contact_info.last_transfer_date,
            trading_name: match &ext_info {
                Some(e) => e.trading_name.clone(),
                None => None,
            },
            company_number: match &ext_info {
                Some(e) => e.company_number.clone(),
                None => match local_address {
                    Some(a) => a.traficom_register_number.clone(),
                    None => None,
                },
            },
            entity_type: match &ext_info {
                Some(e) => match &e.contact_type {
                    Some(i) => i.into(),
                    None => EntityType::Unknown,
                },
                None => match contact_info.traficom_type {
                    Some(t) => traficom_type_to_entity_type(
                        &t,
                        match local_address {
                            Some(a) => a.traficom_is_finnish.unwrap_or(false),
                            None => false,
                        },
                    ),
                    None => EntityType::Unknown,
                },
            },
            disclosure: match contact_info.disclose {
                Some(d) => {
                    if d.flag {
                        d.elements.iter().filter_map(|e| e.into()).collect()
                    } else {
                        vec![]
                    }
                }
                None => vec![],
            },
            auth_info: match contact_info.auth_info {
                Some(a) => a.password,
                None => None,
            },
        })
    }
}

impl From<&proto::contact::EPPContactTransferData> for TransferResponse {
    fn from(contact_transfer: &proto::contact::EPPContactTransferData) -> Self {
        TransferResponse {
            pending: false,
            transaction_id: String::new(),
            data: TransferData {
                status: (&contact_transfer.transfer_status).into(),
                requested_client_id: contact_transfer.requested_client_id.clone(),
                requested_date: contact_transfer.requested_date,
                act_client_id: contact_transfer.act_client_id.clone(),
                act_date: contact_transfer.act_date,
            }
        }
    }
}

impl From<&proto::contact::EPPContactPanData> for PanData {
    fn from(from: &proto::contact::EPPContactPanData) -> Self {
        PanData {
            id: from.contact.contact.clone(),
            result: from.contact.result,
            server_transaction_id: from.transaction_id.server_transaction_id.clone(),
            client_transaction_id: from.transaction_id.client_transaction_id.clone(),
            date: from.action_date,
        }
    }
}

pub fn handle_check(
    client: &EPPClientServerFeatures,
    req: &CheckRequest,
) -> HandleReqReturn<CheckResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;
    let command = proto::EPPCheck::Contact(proto::contact::EPPContactCheck { id: req.id.clone() });
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((proto::EPPCommandType::Check(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_check_response(response: proto::EPPResponse) -> Response<CheckResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPContactCheckResult(contact_check) => {
                if let Some(contact_check) = contact_check.data.first() {
                    Ok(CheckResponse {
                        avail: contact_check.id.available,
                        reason: contact_check.reason.to_owned(),
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

pub fn handle_info(
    client: &EPPClientServerFeatures,
    req: &InfoRequest,
) -> HandleReqReturn<InfoResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;
    let command = proto::EPPInfo::Contact(proto::contact::EPPContactCheck { id: req.id.clone() });
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((proto::EPPCommandType::Info(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_info_response(response: proto::EPPResponse) -> Response<InfoResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPContactInfoResult(contact_info) => {
                (*contact_info, &response.extension).try_into()
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
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;
    if req.email.is_empty() {
        return Err(Err(Error::Err("contact email cannot be empty".to_string())));
    }
    let phone_re = Regex::new(r"^\+\d+\.\d+$").unwrap();
    if let Some(phone) = &req.phone {
        if !phone_re.is_match(&phone.number) {
            return Err(Err(Error::Err("invalid phone number format".to_string())));
        }
    }
    if let Some(fax) = &req.fax {
        if !phone_re.is_match(&fax.number) {
            return Err(Err(Error::Err("invalid fax number format".to_string())));
        }
    }
    if req.local_address.is_none() && req.internationalised_address.is_none() {
        return Err(Err(Error::Err(
            "either a local or internationalised address must be specified format".to_string(),
        )));
    }

    let map_addr = |a: &Address, t: proto::contact::EPPContactPostalInfoType| {
        if a.name.is_empty() {
            return Err(Err(Error::Err("contact name cannot be empty".to_string())));
        }
        if a.city.is_empty() {
            return Err(Err(Error::Err("contact city cannot be empty".to_string())));
        }
        if a.country_code.len() != 2 {
            return Err(Err(Error::Err(
                "contact country code must be of length 2".to_string(),
            )));
        }
        if let Some(pc) = &a.postal_code {
            if pc.len() > 16 {
                return Err(Err(Error::Err(
                    "contact postal code has a max length of 16".to_string(),
                )));
            }
        }
        if a.streets.is_empty() {
            return Err(Err(Error::Err(
                "contact streets cannot be empty".to_string(),
            )));
        }
        let mut name_parts: Vec<&str> = a.name.rsplitn(2, ' ').collect();
        Ok(proto::contact::EPPContactPostalInfo {
            addr_type: t,
            name: Some(a.name.clone()),
            organisation: a.organisation.clone(),
            traficom_last_name: if client.has_erratum("traficom") {
                Some(format!("{:.<2}", name_parts.pop().unwrap_or_default()))
            } else {
                None
            },
            traficom_first_name: if client.has_erratum("traficom") {
                Some(format!("{:.<2}", name_parts.pop().unwrap_or_default()))
            } else {
                None
            },
            traficom_register_number: if client.has_erratum("traficom") {
                req.company_number.clone()
            } else {
                None
            },
            traficom_is_finnish: if client.has_erratum("traficom") {
                Some(is_entity_finnish(&req.entity_type))
            } else {
                None
            },
            traficom_birth_date: if client.has_erratum("traficom") {
                match &req.entity_type {
                    Some(i) => match (
                        proto::traficom::EPPContactTraficomType::from(i),
                        is_entity_finnish(&req.entity_type),
                    ) {
                        (proto::traficom::EPPContactTraficomType::PrivatePerson, false) => {
                            a.birth_date
                        }
                        _ => None,
                    },
                    None => None,
                }
            } else {
                None
            },
            traficom_identity: if client.has_erratum("traficom") {
                match &req.entity_type {
                    Some(i) => match (
                        proto::traficom::EPPContactTraficomType::from(i),
                        is_entity_finnish(&req.entity_type),
                    ) {
                        (proto::traficom::EPPContactTraficomType::PrivatePerson, true) => {
                            a.identity_number.clone()
                        }
                        _ => None,
                    },
                    None => None,
                }
            } else {
                None
            },
            address: proto::contact::EPPContactAddress {
                streets: a.streets.clone(),
                city: a.city.clone(),
                province: a.province.clone(),
                postal_code: if client.has_erratum("traficom") {
                    a.postal_code.as_ref().map(|s| s.replace(" ", ""))
                } else {
                    a.postal_code.clone()
                },
                country_code: a.country_code.clone(),
            },
        })
    };

    let mut postal_info = vec![];
    if let Some(local_address) = &req.local_address {
        postal_info.push(map_addr(
            local_address,
            proto::contact::EPPContactPostalInfoType::Local,
        )?)
    }
    if let Some(internationalised_address) = &req.internationalised_address {
        postal_info.push(map_addr(
            internationalised_address,
            proto::contact::EPPContactPostalInfoType::Internationalised,
        )?)
    }

    let mut ext = vec![];
    if client.nominet_contact_ext {
        ext.push(proto::EPPCommandExtensionType::NominetContactExtCreate(
            proto::nominet::EPPContactInfo {
                contact_type: match &req.entity_type {
                    Some(i) => match i {
                        EntityType::Unknown => None,
                        i => Some(i.into()),
                    },
                    None => None,
                },
                trading_name: req.trading_name.clone(),
                company_number: req.company_number.clone(),
            },
        ));
    }
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);

    let command = proto::EPPCreate::Contact(Box::new(proto::contact::EPPContactCreate {
        id: req.id.clone(),
        postal_info,
        phone: req.phone.as_ref().map(|p| p.into()),
        fax: req.fax.as_ref().map(|p| p.into()),
        email: req.email.clone(),
        auth_info: proto::contact::EPPContactAuthInfo {
            password: Some(req.auth_info.clone()),
        },
        disclose: match client.switch_balance {
            true => None,
            false => req.disclosure.clone().map(|mut d| {
                d.sort_unstable_by(|a, b| (*a as i32).cmp(&(*b as i32)));
                proto::contact::EPPContactDisclosure {
                    flag: true,
                    elements: d.iter().map(|e| e.into()).collect(),
                }
            })
        },
        traficom_role: if client.has_erratum("traficom") {
            Some(proto::traficom::EPPContactTraficomRole::Registrant)
        } else {
            None
        },
        traficom_type: if client.has_erratum("traficom") {
            match &req.entity_type {
                Some(i) => match i {
                    EntityType::Unknown => None,
                    i => Some(i.into()),
                },
                None => None,
            }
        } else {
            None
        },
        traficom_legal_email: if client.has_erratum("traficom") {
            Some(req.email.clone())
        } else {
            None
        },
    }));
    Ok((proto::EPPCommandType::Create(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_create_response(response: proto::EPPResponse) -> Response<CreateResponse> {
    match response.data {
        Some(ref value) => match &value.value {
            proto::EPPResultDataValue::EPPContactCreateResult(contact_create) => {
                Ok(CreateResponse {
                    id: contact_create.id.clone(),
                    pending: response.is_pending(),
                    transaction_id: response.transaction_id.server_transaction_id.unwrap_or_default(),
                    creation_date: contact_create.creation_date,
                })
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

pub fn handle_delete(
    client: &EPPClientServerFeatures,
    req: &DeleteRequest,
) -> HandleReqReturn<DeleteResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;
    let command = proto::EPPDelete::Contact(proto::contact::EPPContactCheck { id: req.id.clone() });
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((proto::EPPCommandType::Delete(command), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_delete_response(response: proto::EPPResponse) -> Response<DeleteResponse> {
    Response::Ok(DeleteResponse {
        pending: response.is_pending(),
        transaction_id: response.transaction_id.server_transaction_id.unwrap_or_default(),
    })
}

pub fn handle_update(
    client: &EPPClientServerFeatures,
    req: &UpdateRequest,
) -> HandleReqReturn<UpdateResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;
    let is_not_change = req.new_email.is_none()
        && req.new_phone.is_none()
        && req.new_fax.is_none()
        && req.new_local_address.is_none()
        && req.new_internationalised_address.is_none()
        && req.new_disclosure.is_none();
    let is_not_nom_change = req.new_entity_type.is_none()
        && req.new_trading_name.is_none()
        && req.new_company_number.is_none();
    if req.add_statuses.is_empty() && req.remove_statuses.is_empty() && is_not_change {
        if is_not_nom_change {
            return Err(Err(Error::Err(
                "at least one operation must be specified".to_string(),
            )));
        } else if !client.nominet_contact_ext {
            return Err(Ok(UpdateResponse { pending: false, transaction_id: "".to_string() }));
        }
    }
    let phone_re = Regex::new(r"^\+\d+\.\d+$").unwrap();
    if let Some(phone) = &req.new_phone {
        if !phone_re.is_match(&phone.number) && &phone.number != "" {
            return Err(Err(Error::Err("invalid phone number format".to_string())));
        }
    }
    if let Some(fax) = &req.new_fax {
        if !phone_re.is_match(&fax.number) && &fax.number != "" {
            return Err(Err(Error::Err("invalid fax number format".to_string())));
        }
    }
    let mut postal_info = vec![];
    let map_addr = |a: &Address, t: proto::contact::EPPContactPostalInfoType| {
        if a.name.is_empty() {
            return Err(Err(Error::Err("contact name cannot be empty".to_string())));
        }
        if a.city.is_empty() {
            return Err(Err(Error::Err("contact city cannot be empty".to_string())));
        }
        if a.country_code.len() != 2 {
            return Err(Err(Error::Err(
                "contact country code must be of length 2".to_string(),
            )));
        }
        if a.streets.is_empty() {
            return Err(Err(Error::Err(
                "contact streets cannot be empty".to_string(),
            )));
        }
        Ok(proto::contact::EPPContactUpdatePostalInfo {
            addr_type: t,
            name: Some(a.name.clone()),
            organisation: a.organisation.clone(),
            address: Some(proto::contact::EPPContactAddress {
                streets: a.streets.clone(),
                city: a.city.clone(),
                province: a.province.clone(),
                postal_code: a.postal_code.clone(),
                country_code: a.country_code.clone(),
            }),
        })
    };
    if let Some(new_local_address) = &req.new_local_address {
        postal_info.push(map_addr(
            new_local_address,
            proto::contact::EPPContactPostalInfoType::Local,
        )?)
    }
    if let Some(new_internationalised_address) = &req.new_internationalised_address {
        postal_info.push(map_addr(
            new_internationalised_address,
            proto::contact::EPPContactPostalInfoType::Internationalised,
        )?)
    }
    let command = proto::EPPUpdate::Contact(proto::contact::EPPContactUpdate {
        id: req.id.clone(),
        traficom_role: if client.has_erratum("traficom") {
            Some(proto::traficom::EPPContactTraficomRole::Registrant)
        } else {
            None
        },
        add: if req.add_statuses.is_empty() {
            None
        } else {
            Some(proto::contact::EPPContactUpdateAdd {
                statuses: req
                    .add_statuses
                    .iter()
                    .map(|s| proto::contact::EPPContactStatus { status: s.into() })
                    .collect(),
            })
        },
        remove: if req.remove_statuses.is_empty() {
            None
        } else {
            Some(proto::contact::EPPContactUpdateRemove {
                statuses: req
                    .remove_statuses
                    .iter()
                    .map(|s| proto::contact::EPPContactStatus { status: s.into() })
                    .collect(),
            })
        },
        change: if is_not_change {
            None
        } else {
            Some(proto::contact::EPPContactUpdateChange {
                email: req.new_email.clone(),
                phone: req.new_phone.as_ref().map(|p| p.into()),
                fax: req.new_fax.as_ref().map(|p| p.into()),
                postal_info,
                disclose: match client.switch_balance {
                    true => None,
                    false => req.new_disclosure.clone().map(|mut d| {
                        d.sort_unstable_by(|a, b| (*a as i32).cmp(&(*b as i32)));
                        proto::contact::EPPContactDisclosure {
                            flag: true,
                            elements: d.iter().map(|e| e.into()).collect(),
                        }
                    })
                },
                auth_info: req
                    .new_auth_info
                    .as_ref()
                    .map(|p| proto::contact::EPPContactAuthInfo {
                        password: Some(p.clone()),
                    }),
            })
        },
    });

    let mut ext = vec![];
    if client.nominet_contact_ext {
        ext.push(proto::EPPCommandExtensionType::NominetContactExtUpdate(
            proto::nominet::EPPContactInfo {
                contact_type: match &req.new_entity_type {
                    Some(i) => match i {
                        EntityType::Unknown => None,
                        i => Some(i.into()),
                    },
                    None => None,
                },
                trading_name: req.new_trading_name.clone(),
                company_number: req.new_company_number.clone(),
            },
        ));
    }
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);

    Ok((proto::EPPCommandType::Update(Box::new(command)), match ext.is_empty() {
        true => None,
        false => Some(ext)
    }))
}

pub fn handle_update_response(response: proto::EPPResponse) -> Response<UpdateResponse> {
    Response::Ok(UpdateResponse {
        pending: response.is_pending(),
        transaction_id: response.transaction_id.server_transaction_id.unwrap_or_default(),
    })
}

pub fn handle_transfer_query(
    client: &EPPClientServerFeatures,
    req: &TransferQueryRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;
    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Query,
        command: proto::EPPTransferCommand::ContactQuery(proto::contact::EPPContactCheck {
            id: req.id.clone(),
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
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;
    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Request,
        command: proto::EPPTransferCommand::ContactRequest(proto::contact::EPPContactTransfer {
            id: req.id.clone(),
            auth_info: proto::contact::EPPContactAuthInfo {
                password: Some(req.auth_info.clone()),
            },
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
    req: &TransferRequestRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;
    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Accept,
        command: proto::EPPTransferCommand::ContactRequest(proto::contact::EPPContactTransfer {
            id: req.id.clone(),
            auth_info: proto::contact::EPPContactAuthInfo {
                password: Some(req.auth_info.clone()),
            },
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
    req: &TransferRequestRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;
    let command = proto::EPPTransfer {
        operation: proto::EPPTransferOperation::Reject,
        command: proto::EPPTransferCommand::ContactRequest(proto::contact::EPPContactTransfer {
            id: req.id.clone(),
            auth_info: proto::contact::EPPContactAuthInfo {
                password: Some(req.auth_info.clone()),
            },
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
    match &response.data {
        Some(value) => match &value.value {
            proto::EPPResultDataValue::EPPContactTransferResult(contact_transfer) => {
                let mut res: TransferResponse = contact_transfer.into();
                res.pending = pending;
                res.transaction_id = response.transaction_id.server_transaction_id.unwrap_or_default();
                Ok(res)
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

/// Checks if a contact ID exists
///
/// # Arguments
/// * `id` - The ID in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn check(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CheckResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::ContactCheck(Box::new(CheckRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Fetches information about a specific contact
///
/// # Arguments
/// * `id` - The ID of the contact
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn info(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<InfoResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::ContactInfo(Box::new(InfoRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
        .await
}

pub struct NewContactData {
    /// Localised address of the contact
    pub local_address: Option<Address>,
    /// Internationalised address of the contact
    pub internationalised_address: Option<Address>,
    /// Voice phone number of the contact
    pub phone: Option<Phone>,
    /// Fax number of the contact
    pub fax: Option<Phone>,
    /// Email address of the contact
    pub email: String,
    /// New entity type of the contact
    pub entity_type: Option<EntityType>,
    /// New trading of the contact
    pub trading_name: Option<String>,
    /// New company number of the contact
    pub company_number: Option<String>,
    /// Elements the contact has consented to disclosure of
    pub disclosure: Option<Vec<DisclosureType>>,
    pub auth_info: String,
}

/// Creates a new contact
///
/// At least one of `local_address` or `internationalised_address` must be set. Contact numbers must
/// be in `+cc.xxxxxxxxxx` format where `c` is the country dialing code and `x` is the country local
/// number
///
/// # Arguments
/// * `id` - The desired contact ID
/// * `data` - Data for the new contact
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn create(
    id: &str,
    data: NewContactData,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CreateResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::ContactCreate(Box::new(CreateRequest {
            id: id.to_string(),
            local_address: data.local_address,
            internationalised_address: data.internationalised_address,
            phone: data.phone,
            fax: data.fax,
            email: data.email,
            entity_type: data.entity_type,
            trading_name: data.trading_name,
            company_number: data.company_number,
            disclosure: data.disclosure,
            auth_info: data.auth_info,
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Deletes a contact contact
///
/// # Arguments
/// * `id` - The ID of the contact
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn delete(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<DeleteResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::ContactDelete(Box::new(DeleteRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
        .await
}

pub struct UpdateContactData {
    ///  New localised address of the contact
    pub local_address: Option<Address>,
    /// New internationalised address of the contact
    pub internationalised_address: Option<Address>,
    /// New voice phone number of the contact
    pub phone: Option<Phone>,
    /// New fax number of the contact
    pub fax: Option<Phone>,
    /// New email address of the contact
    pub email: Option<String>,
    /// New entity type of the contact
    pub entity_type: Option<EntityType>,
    /// New trading of the contact
    pub trading_name: Option<String>,
    /// New company number of the contact
    pub company_number: Option<String>,
    /// Elements the contact has consented to disclosure of
    pub disclosure: Option<Vec<DisclosureType>>,
    pub auth_info: Option<String>,
}

/// Updates an existing contact
///
/// Contact numbers must be in `+cc.xxxxxxxxxx` format where `c` is the country dialing code and
/// `x` is the country local number
///
/// # Arguments
/// * `id` - The ID of said contact
/// * `add_statuses` - Statuses to be set on the contact
/// * `remove_statuses` - Statuses to be removed from the contact
/// * `new_contact_data` - New data to be set on the contact
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn update(
    id: &str,
    add_statuses: Vec<Status>,
    remove_statuses: Vec<Status>,
    new_data: UpdateContactData,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<UpdateResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::ContactUpdate(Box::new(UpdateRequest {
            id: id.to_string(),
            add_statuses,
            remove_statuses,
            new_local_address: new_data.local_address,
            new_internationalised_address: new_data.internationalised_address,
            new_phone: new_data.phone,
            new_fax: new_data.fax,
            new_email: new_data.email,
            new_entity_type: new_data.entity_type,
            new_trading_name: new_data.trading_name,
            new_company_number: new_data.company_number,
            new_disclosure: new_data.disclosure,
            new_auth_info: new_data.auth_info,
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Queries the current transfer status of a contact
///
/// # Arguments
/// * `id` - The contact ID to be queried
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_query(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<TransferResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::ContactTransferQuery(Box::new(TransferQueryRequest {
            id: id.to_string(),
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Requests the transfer of a contact
///
/// # Arguments
/// * `id` - The contact ID to be transferred
/// * `auth_info` - Auth info for the contact
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_request(
    id: &str,
    auth_info: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<TransferResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::ContactTransferRequest(Box::new(TransferRequestRequest {
            id: id.to_string(),
            auth_info: auth_info.to_string(),
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Accepts the transfer of a contact
///
/// # Arguments
/// * `id` - The contact ID to be approved
/// * `auth_info` - Auth info for the contact
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_accept(
    id: &str,
    auth_info: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<TransferResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::ContactTransferAccept(Box::new(TransferRequestRequest {
            id: id.to_string(),
            auth_info: auth_info.to_string(),
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Rejects the transfer of a contact
///
/// # Arguments
/// * `id` - The contact ID to be rejected
/// * `auth_info` - Auth info for the contact
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn transfer_reject(
    id: &str,
    auth_info: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<TransferResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::ContactTransferReject(Box::new(TransferRequestRequest {
            id: id.to_string(),
            auth_info: auth_info.to_string(),
            return_path: sender,
        })),
        receiver,
    )
        .await
}
