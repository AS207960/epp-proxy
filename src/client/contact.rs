//! Commands relating to contact objects

use chrono::prelude::*;

use super::{CommandResponse, RequestMessage, Sender};

#[derive(Debug)]
pub struct CheckRequest {
    pub(super) id: String,
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
    pub(super) id: String,
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
    pub nominet_data_quality: Option<super::nominet::DataQualityData>,
    pub eurid_contact_extension: Option<super::eurid::ContactExtension>,
    pub isnic_info: Option<super::isnic::ContactInfo>,
    pub qualified_lawyer: Option<QualifiedLawyerInfo>,
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct QualifiedLawyerInfo {
    pub accreditation_id: String,
    pub accreditation_body: String,
    pub accreditation_year: i32,
    pub jurisdiction_country: String,
    pub jurisdiction_province: Option<String>,
}

#[derive(Debug)]
pub struct CreateRequest {
    pub(super) id: String,
    pub(super) local_address: Option<Address>,
    pub(super) internationalised_address: Option<Address>,
    pub(super) phone: Option<Phone>,
    pub(super) fax: Option<Phone>,
    pub(super) email: String,
    pub(super) entity_type: Option<EntityType>,
    pub(super) trading_name: Option<String>,
    pub(super) company_number: Option<String>,
    pub(super) disclosure: Option<Vec<DisclosureType>>,
    pub(super) auth_info: String,
    pub(super) eurid_contact_extension: Option<super::eurid::ContactExtension>,
    pub(super) qualified_lawyer: Option<QualifiedLawyerInfo>,
    pub(super) isnic_info: Option<super::isnic::ContactCreate>,
    pub return_path: Sender<CreateResponse>,
}

#[derive(Debug)]
pub struct CreateResponse {
    /// The actual contact ID created
    pub id: String,
    /// Was the request completed instantly or not
    pub pending: bool,
    /// What date did the server log as the date of creation
    pub creation_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct DeleteRequest {
    pub(super) id: String,
    pub return_path: Sender<DeleteResponse>,
}

#[derive(Debug)]
pub struct DeleteResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
}

#[derive(Debug)]
pub struct UpdateRequest {
    pub(super) id: String,
    pub(super) add_statuses: Vec<Status>,
    pub(super) remove_statuses: Vec<Status>,
    pub(super) new_local_address: Option<Address>,
    pub(super) new_internationalised_address: Option<Address>,
    pub(super) new_phone: Option<Phone>,
    pub(super) new_fax: Option<Phone>,
    pub(super) new_email: Option<String>,
    pub(super) new_entity_type: Option<EntityType>,
    pub(super) new_trading_name: Option<String>,
    pub(super) new_company_number: Option<String>,
    pub(super) new_disclosure: Option<Vec<DisclosureType>>,
    pub(super) new_auth_info: Option<String>,
    pub(super) new_eurid_contact_extension: Option<super::eurid::ContactExtensionUpdate>,
    pub(super) qualified_lawyer: Option<QualifiedLawyerInfo>,
    pub(super) isnic_info: Option<super::isnic::ContactUpdate>,
    pub return_path: Sender<UpdateResponse>,
}

#[derive(Debug)]
pub struct UpdateResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
}

#[derive(Debug)]
pub struct TransferQueryRequest {
    pub(super) id: String,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferRequestRequest {
    pub(super) id: String,
    pub(super) auth_info: String,
    pub return_path: Sender<TransferResponse>,
}

#[derive(Debug)]
pub struct TransferResponse {
    /// Was the request completed instantly or not
    pub pending: bool,
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

/// Checks if a contact ID exists
///
/// # Arguments
/// * `id` - The ID in question
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn check(
    id: &str,
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<CheckResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::ContactCheck(Box::new(CheckRequest {
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
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<InfoResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::ContactInfo(Box::new(InfoRequest {
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
    pub eurid_info: Option<super::eurid::ContactExtension>,
    pub isnic_info: Option<super::isnic::ContactCreate>,
    pub qualified_lawyer: Option<QualifiedLawyerInfo>,
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
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<CreateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::ContactCreate(Box::new(CreateRequest {
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
            eurid_contact_extension: data.eurid_info,
            isnic_info: data.isnic_info,
            qualified_lawyer: data.qualified_lawyer,
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
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<DeleteResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::ContactDelete(Box::new(DeleteRequest {
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
    pub eurid_info: Option<super::eurid::ContactExtensionUpdate>,
    pub isnic_info: Option<super::isnic::ContactUpdate>,
    pub qualified_lawyer: Option<QualifiedLawyerInfo>,
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
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<UpdateResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::ContactUpdate(Box::new(UpdateRequest {
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
            new_eurid_contact_extension: new_data.eurid_info,
            isnic_info: new_data.isnic_info,
            qualified_lawyer: new_data.qualified_lawyer,
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
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::ContactTransferQuery(Box::new(TransferQueryRequest {
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
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::ContactTransferRequest(Box::new(TransferRequestRequest {
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
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::ContactTransferAccept(Box::new(TransferRequestRequest {
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
    client_sender: &mut futures::channel::mpsc::Sender<RequestMessage>,
) -> Result<CommandResponse<TransferResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        RequestMessage::ContactTransferReject(Box::new(TransferRequestRequest {
            id: id.to_string(),
            auth_info: auth_info.to_string(),
            return_path: sender,
        })),
        receiver,
    )
    .await
}
