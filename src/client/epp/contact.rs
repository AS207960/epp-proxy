//! EPP commands relating to contact objects

use std::convert::{TryFrom, TryInto};

use regex::Regex;

use super::super::contact::{
    Address, CheckRequest, CheckResponse, CreateRequest, CreateResponse, DeleteRequest,
    DeleteResponse, DisclosureType, EntityType, InfoRequest, InfoResponse, PanData,
    QualifiedLawyerInfo, Status, TransferData, TransferQueryRequest, TransferRequestRequest,
    TransferResponse, UpdateRequest, UpdateResponse,
};
use super::super::{proto, Error, Phone, Response};
use super::router::HandleReqReturn;
use super::ServerFeatures;

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
        Some(e) => matches!(
            e,
            EntityType::FinnishIndividual
                | EntityType::FinnishCompany
                | EntityType::FinnishAssociation
                | EntityType::FinnishInstitution
                | EntityType::FinnishGovernment
                | EntityType::FinnishMunicipality
                | EntityType::FinnishPoliticalParty
                | EntityType::FinnishPublicCommunity
        ),
        None => false,
    }
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

impl From<&proto::contact::EPPContactPhone> for Phone {
    fn from(from: &proto::contact::EPPContactPhone) -> Self {
        Phone {
            number: from.number.clone(),
            extension: from.extension.clone(),
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

impl From<&proto::qualified_lawyer::QualifiedLawyerInfoData> for QualifiedLawyerInfo {
    fn from(from: &proto::qualified_lawyer::QualifiedLawyerInfoData) -> QualifiedLawyerInfo {
        QualifiedLawyerInfo {
            accreditation_id: from.accreditation_id.to_string(),
            accreditation_body: from.accreditation_body.to_string(),
            accreditation_year: from.accreditation_year,
            jurisdiction_country: from.jurisdiction_country.to_string(),
            jurisdiction_province: from.jurisdiction_province.as_deref().map(Into::into),
        }
    }
}

impl From<&QualifiedLawyerInfo> for proto::qualified_lawyer::QualifiedLawyerInfoData {
    fn from(from: &QualifiedLawyerInfo) -> proto::qualified_lawyer::QualifiedLawyerInfoData {
        proto::qualified_lawyer::QualifiedLawyerInfoData {
            accreditation_id: from.accreditation_id.to_string(),
            accreditation_body: from.accreditation_body.to_string(),
            accreditation_year: from.accreditation_year,
            jurisdiction_country: from.jurisdiction_country.to_string(),
            jurisdiction_province: from.jurisdiction_province.as_deref().map(Into::into),
        }
    }
}

pub(crate) fn check_id<T>(id: &str) -> Result<(), Response<T>> {
    if let 3..=32 = id.len() {
        Ok(())
    } else {
        Err(Err(Error::Err(
            "contact id has a min length of 3 and a max length of 32".to_string(),
        )))
    }
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
        let map_addr = |a: Option<&proto::contact::EPPContactPostalInfo>| {
            a.map(|p| Address {
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
            })
        };
        let contact_ext_info =
            match extension {
                Some(e) => match e.value.iter().find(|e| {
                    matches!(e, proto::EPPResponseExtensionType::NominetContactExtInfo(_))
                }) {
                    Some(e) => match e {
                        proto::EPPResponseExtensionType::NominetContactExtInfo(e) => Some(e),
                        _ => unreachable!(),
                    },
                    None => None,
                },
                None => None,
            };
        let data_quality_ext_info = match extension {
            Some(e) => match e
                .value
                .iter()
                .find(|e| matches!(e, proto::EPPResponseExtensionType::NominetDataQuality(_)))
            {
                Some(e) => match e {
                    proto::EPPResponseExtensionType::NominetDataQuality(e) => Some(e),
                    _ => unreachable!(),
                },
                None => None,
            },
            None => None,
        };
        let eurid_ext_info = match extension {
            Some(e) => match e
                .value
                .iter()
                .find(|e| matches!(e, proto::EPPResponseExtensionType::EURIDContactInfoData(_)))
            {
                Some(e) => match e {
                    proto::EPPResponseExtensionType::EURIDContactInfoData(e) => Some(e),
                    _ => unreachable!(),
                },
                None => None,
            },
            None => None,
        };
        let qualified_lawyer_ext_info = match extension {
            Some(e) => match e
                .value
                .iter()
                .find(|e| matches!(e, proto::EPPResponseExtensionType::QualifiedLawyerInfo(_)))
            {
                Some(e) => match e {
                    proto::EPPResponseExtensionType::QualifiedLawyerInfo(e) => Some(e),
                    _ => unreachable!(),
                },
                None => None,
            },
            None => None,
        };
        let isnic_ext_info = match extension {
            Some(e) => match e
                .value
                .iter()
                .find(|e| matches!(e, proto::EPPResponseExtensionType::ISNICContactInfo(_)))
            {
                Some(e) => match e {
                    proto::EPPResponseExtensionType::ISNICContactInfo(e) => Some(e),
                    _ => unreachable!(),
                },
                None => None,
            },
            None => None,
        };

        let keysys = match extension {
            Some(ext) => {
                let i = ext.value.iter().find_map(|p| match p {
                    proto::EPPResponseExtensionType::KeysysResultData(
                        proto::keysys::ResultData::Contact(i),
                    ) => Some(i),
                    _ => None,
                });
                i.map(Into::into)
            }
            None => None,
        };

        let local_address = contact_info
            .postal_info
            .iter()
            .find(|p| p.addr_type == proto::contact::EPPContactPostalInfoType::Local);
        let any_address = contact_info.postal_info.get(0);
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
            phone: contact_info.phone.as_ref().map(|p| p.into()),
            fax: contact_info.fax.as_ref().map(|p| p.into()),
            email: match contact_info.email {
                Some(e) => e,
                None => contact_info.traficom_legal_email.unwrap_or_default(),
            },
            client_id: contact_info.client_id,
            client_created_id: contact_info.client_created_id,
            creation_date: contact_info.creation_date,
            last_updated_client: contact_info.last_updated_client,
            last_updated_date: contact_info.last_updated_date,
            last_transfer_date: contact_info.last_transfer_date,
            trading_name: match &contact_ext_info {
                Some(e) => e.trading_name.clone(),
                None => None,
            },
            company_number: match &contact_ext_info {
                Some(e) => e.company_number.clone(),
                None => match local_address {
                    Some(a) => a.traficom_register_number.clone(),
                    None => None,
                },
            },
            entity_type: if let Some(e) = &contact_ext_info {
                match &e.contact_type {
                    Some(i) => i.into(),
                    None => EntityType::Unknown,
                }
            } else if let Some(t) = contact_info.traficom_type {
                traficom_type_to_entity_type(
                    &t,
                    match local_address {
                        Some(a) => a.traficom_is_finnish.unwrap_or(false),
                        None => false,
                    },
                )
            } else if let Some(e) = &eurid_ext_info {
                super::eurid::eurid_ext_to_entity(e)
            } else if let Some(i) = &isnic_ext_info {
                super::isnic::isnic_ext_to_entity(i, any_address)
            } else {
                EntityType::Unknown
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
            nominet_data_quality: data_quality_ext_info.map(Into::into),
            eurid_contact_extension: eurid_ext_info.map(Into::into),
            qualified_lawyer: qualified_lawyer_ext_info.map(Into::into),
            isnic_info: isnic_ext_info.map(Into::into),
            keysys,
        })
    }
}

impl From<&proto::contact::EPPContactTransferData> for TransferResponse {
    fn from(contact_transfer: &proto::contact::EPPContactTransferData) -> Self {
        TransferResponse {
            pending: false,
            data: TransferData {
                status: (&contact_transfer.transfer_status).into(),
                requested_client_id: contact_transfer.requested_client_id.clone(),
                requested_date: contact_transfer.requested_date,
                act_client_id: contact_transfer.act_client_id.clone(),
                act_date: contact_transfer.act_date,
            },
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

pub fn handle_check(client: &ServerFeatures, req: &CheckRequest) -> HandleReqReturn<CheckResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;
    let command = proto::EPPCheck::Contact(proto::contact::EPPContactCheck { id: req.id.clone() });
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((
        proto::EPPCommandType::Check(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_check_response(
    response: proto::EPPResponse, _metrics: &crate::metrics::ScopedMetrics
) -> Response<CheckResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPContactCheckResult(contact_check) => {
                if let Some(contact_check) = contact_check.data.first() {
                    Ok(CheckResponse {
                        avail: contact_check.id.available,
                        reason: contact_check.reason.to_owned(),
                    })
                } else {
                    Err(Error::ServerInternal)
                }
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_info(client: &ServerFeatures, req: &InfoRequest) -> HandleReqReturn<InfoResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;
    let command = proto::EPPInfo::Contact(proto::contact::EPPContactCheck { id: req.id.clone() });
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((
        proto::EPPCommandType::Info(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_info_response(
    response: proto::EPPResponse, _metrics: &crate::metrics::ScopedMetrics
) -> Response<InfoResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EPPContactInfoResult(contact_info) => {
                (*contact_info, &response.extension).try_into()
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_create(
    client: &ServerFeatures,
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
            organisation: if client.eurid_contact_support {
                if super::super::eurid::is_entity_natural_person(req.entity_type.as_ref()) {
                    None
                } else {
                    Some(a.organisation.clone().unwrap_or_else(|| a.name.clone()))
                }
            } else {
                a.organisation.clone()
            },
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
                    a.postal_code.as_ref().map(|s| s.replace(' ', ""))
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
    match &req.eurid_contact_extension {
        Some(e) => {
            if client.eurid_contact_support {
                ext.push(proto::EPPCommandExtensionType::EURIDContactCreate(
                    super::eurid::contact_info_from_extension(e, &req.entity_type),
                ))
            } else {
                return Err(Err(Error::Unsupported));
            }
        }
        None => {
            if client.eurid_contact_support {
                return Err(Err(Error::Err(
                    "contact extension required for EURid".to_string(),
                )));
            }
        }
    }
    match &req.isnic_info {
        Some(e) => {
            if client.isnic_contact_supported {
                ext.push(proto::EPPCommandExtensionType::ISNICContactCreate(
                    (Some(e), &req.entity_type).into(),
                ))
            } else {
                return Err(Err(Error::Unsupported));
            }
        }
        None => {
            if client.isnic_contact_supported {
                ext.push(proto::EPPCommandExtensionType::ISNICContactCreate(
                    (None, &req.entity_type).into(),
                ))
            }
        }
    }
    if let Some(qualified_lawyer) = &req.qualified_lawyer {
        if client.qualified_lawyer_supported {
            ext.push(proto::EPPCommandExtensionType::QualifiedLawyerCreate(
                qualified_lawyer.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    if let Some(keysys) = &req.keysys {
        if client.keysys_supported {
            ext.push(proto::EPPCommandExtensionType::KeysysCreate(
                proto::keysys::Create::Contact(proto::keysys::ContactCreate {
                    checkonly: Some(keysys.check_only),
                    force_duplication: Some(keysys.force_duplication),
                    pre_verify: Some(keysys.pre_verify),
                }),
            ));
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);

    if !req.auth_info.is_empty() {
        super::domain::check_pass(&req.auth_info)?;
    }

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
            false => req.disclosure.clone().and_then(|mut d| {
                let contains_local_name = d.contains(&DisclosureType::LocalName);
                let contains_int_name = d.contains(&DisclosureType::InternationalisedName);
                let contains_local_addr = d.contains(&DisclosureType::LocalAddress);
                let contains_int_addr = d.contains(&DisclosureType::InternationalisedAddress);
                let contains_local_org = d.contains(&DisclosureType::LocalOrganisation);
                let contains_int_org = d.contains(&DisclosureType::InternationalisedOrganisation);

                let suppress_int_name = client.nominet_contact_ext;
                let suppress_local_name = client.nominet_contact_ext;

                let suppress_int_addr = client.nominet_contact_ext
                    && contains_local_addr
                    && contains_int_addr
                    && req.local_address.is_some();
                let suppress_local_addr = client.nominet_contact_ext
                    && contains_local_addr
                    && contains_int_addr
                    && !suppress_int_addr;

                if client.nominet_contact_ext && contains_local_name && !contains_local_org {
                    d.push(DisclosureType::LocalOrganisation);
                } else if client.nominet_contact_ext && contains_int_name && !contains_int_org {
                    d.push(DisclosureType::InternationalisedOrganisation);
                }

                let suppress_int_org = client.nominet_contact_ext
                    && contains_local_org
                    && contains_int_org
                    && req.local_address.is_some();
                let suppress_local_org = client.nominet_contact_ext
                    && contains_local_org
                    && contains_int_org
                    && !suppress_int_org;

                d.sort_unstable_by_key(|a| (*a as i32));
                let elements: Vec<_> = d
                    .iter()
                    .filter(|d| match d {
                        DisclosureType::LocalName => !suppress_local_name,
                        DisclosureType::InternationalisedName => !suppress_int_name,
                        DisclosureType::LocalAddress => !suppress_local_addr,
                        DisclosureType::InternationalisedAddress => !suppress_int_addr,
                        DisclosureType::LocalOrganisation => !suppress_local_org,
                        DisclosureType::InternationalisedOrganisation => !suppress_int_org,
                        _ => true,
                    })
                    .map(|e| e.into())
                    .collect();

                if elements.is_empty() {
                    None
                } else {
                    Some(proto::contact::EPPContactDisclosure {
                        flag: true,
                        elements,
                    })
                }
            }),
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
    Ok((
        proto::EPPCommandType::Create(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_create_response(
    response: proto::EPPResponse, _metrics: &crate::metrics::ScopedMetrics
) -> Response<CreateResponse> {
    match response.data {
        Some(ref value) => match &value.value {
            proto::EPPResultDataValue::EPPContactCreateResult(contact_create) => {
                Ok(CreateResponse {
                    id: contact_create.id.clone(),
                    pending: response.is_pending(),
                    creation_date: contact_create.creation_date,
                })
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_delete(
    client: &ServerFeatures,
    req: &DeleteRequest,
) -> HandleReqReturn<DeleteResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.isnic_contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;
    let command = proto::EPPDelete::Contact(proto::contact::EPPContactCheck { id: req.id.clone() });
    let mut ext = vec![];
    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);
    Ok((
        proto::EPPCommandType::Delete(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_delete_response(
    response: proto::EPPResponse, _metrics: &crate::metrics::ScopedMetrics
) -> Response<DeleteResponse> {
    Response::Ok(DeleteResponse {
        pending: response.is_pending(),
    })
}

pub fn handle_update(
    client: &ServerFeatures,
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
    let is_not_eurid_change = match &req.new_eurid_contact_extension {
        Some(e) => {
            e.language.is_none()
                && e.citizenship_country.is_none()
                && e.whois_email.is_none()
                && e.vat.is_none()
        }
        None => true,
    };
    let is_not_qualified_lawyer_change = req.qualified_lawyer.is_none();
    let is_not_isnic_change = match &req.isnic_info {
        Some(e) => {
            e.lang.is_none()
                && e.mobile.is_none()
                && e.auto_update_from_national_registry.is_none()
                && e.paper_invoices.is_none()
        }
        None => true,
    };
    if req.add_statuses.is_empty() && req.remove_statuses.is_empty() && is_not_change {
        if is_not_nom_change
            && is_not_eurid_change
            && is_not_qualified_lawyer_change
            && is_not_isnic_change
        {
            return Err(Err(Error::Err(
                "at least one operation must be specified".to_string(),
            )));
        } else {
            if (!is_not_nom_change) && (!client.nominet_contact_ext) {
                return Err(Ok(UpdateResponse { pending: false }));
            }
            if (!is_not_eurid_change) && (!client.eurid_contact_support) {
                return Err(Ok(UpdateResponse { pending: false }));
            }
            if (!is_not_qualified_lawyer_change) && (!client.qualified_lawyer_supported) {
                return Err(Ok(UpdateResponse { pending: false }));
            }
            if (!is_not_isnic_change) && (!client.isnic_contact_supported) {
                return Err(Ok(UpdateResponse { pending: false }));
            }
        }
    }
    let phone_re = Regex::new(r"^\+\d+\.\d+$").unwrap();
    if let Some(phone) = &req.new_phone {
        if !phone_re.is_match(&phone.number) && !phone.number.is_empty() {
            return Err(Err(Error::Err("invalid phone number format".to_string())));
        }
    }
    if let Some(fax) = &req.new_fax {
        if !phone_re.is_match(&fax.number) && !fax.number.is_empty() {
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
                    false => req.new_disclosure.clone().and_then(|mut d| {
                        d.sort_unstable_by_key(|a| (*a as i32));
                        let elements: Vec<_> = d.iter().map(|e| e.into()).collect();
                        if elements.is_empty() {
                            None
                        } else {
                            Some(proto::contact::EPPContactDisclosure {
                                flag: true,
                                elements,
                            })
                        }
                    }),
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
    if client.eurid_contact_support {
        ext.push(proto::EPPCommandExtensionType::EURIDContactUpdate(
            super::eurid::contact_info_update_from_extension(
                &req.new_eurid_contact_extension,
                &req.new_entity_type,
            ),
        ));
    } else if req.new_eurid_contact_extension.is_some() {
        return Err(Err(Error::Unsupported));
    }

    if let Some(isnic_info) = &req.isnic_info {
        if client.isnic_contact_supported {
            super::domain::check_pass(&isnic_info.password)?;
            ext.push(proto::EPPCommandExtensionType::ISNICContactUpdate(
                isnic_info.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    } else if client.isnic_domain_supported {
        return Err(Err(Error::Err(
            "contact extension required for ISNIC".to_string(),
        )));
    }

    if let Some(qualified_lawyer) = &req.qualified_lawyer {
        if client.qualified_lawyer_supported {
            ext.push(proto::EPPCommandExtensionType::QualifiedLawyerUpdate(
                qualified_lawyer.into(),
            ))
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    if let Some(keysys) = &req.keysys {
        if client.keysys_supported {
            ext.push(proto::EPPCommandExtensionType::KeysysUpdate(
                proto::keysys::Update::Contact(proto::keysys::ContactUpdate {
                    checkonly: Some(keysys.check_only),
                    pre_verify: Some(keysys.pre_verify),
                    trigger_foa: Some(keysys.trigger_foa),
                }),
            ));
        } else {
            return Err(Err(Error::Unsupported));
        }
    }

    super::verisign::handle_verisign_namestore_erratum(client, &mut ext);

    Ok((
        proto::EPPCommandType::Update(Box::new(command)),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_update_response(
    response: proto::EPPResponse, _metrics: &crate::metrics::ScopedMetrics
) -> Response<UpdateResponse> {
    Response::Ok(UpdateResponse {
        pending: response.is_pending(),
    })
}

pub fn handle_transfer_query(
    client: &ServerFeatures,
    req: &TransferQueryRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.isnic_contact_supported {
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
    Ok((
        proto::EPPCommandType::Transfer(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_transfer_request(
    client: &ServerFeatures,
    req: &TransferRequestRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.isnic_contact_supported {
        return Err(Err(Error::Unsupported));
    }

    check_id(&req.id)?;

    if !req.auth_info.is_empty() {
        super::domain::check_pass(&req.auth_info)?;
    }

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
    Ok((
        proto::EPPCommandType::Transfer(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_transfer_accept(
    client: &ServerFeatures,
    req: &TransferRequestRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.isnic_contact_supported {
        return Err(Err(Error::Unsupported));
    }

    check_id(&req.id)?;

    if !req.auth_info.is_empty() {
        super::domain::check_pass(&req.auth_info)?;
    }

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
    Ok((
        proto::EPPCommandType::Transfer(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_transfer_reject(
    client: &ServerFeatures,
    req: &TransferRequestRequest,
) -> HandleReqReturn<TransferResponse> {
    if !client.contact_supported {
        return Err(Err(Error::Unsupported));
    }
    if client.isnic_contact_supported {
        return Err(Err(Error::Unsupported));
    }
    check_id(&req.id)?;

    if !req.auth_info.is_empty() {
        super::domain::check_pass(&req.auth_info)?;
    }

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
    Ok((
        proto::EPPCommandType::Transfer(command),
        match ext.is_empty() {
            true => None,
            false => Some(ext),
        },
    ))
}

pub fn handle_transfer_response(
    response: proto::EPPResponse, _metrics: &crate::metrics::ScopedMetrics
) -> Response<TransferResponse> {
    let pending = response.is_pending();
    match &response.data {
        Some(value) => match &value.value {
            proto::EPPResultDataValue::EPPContactTransferResult(contact_transfer) => {
                let mut res: TransferResponse = contact_transfer.into();
                res.pending = pending;
                Ok(res)
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

mod test {
    #[test]
    fn qualified_lawyer_info() {
        const XML_DATA: &str = r#"
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemalocation="urn:ietf:params:xml:ns:epp-1.0 epp-1.0.xsd">
    <response>
        <result code="1000">
            <msg>Command completed successfully</msg>
        </result>
        <resData>
            <contact:infData xmlns:contact="urn:ietf:params:xml:ns:contact-1.0" xsi:schemalocation="urn:ietf:params:xml:ns:contact-1.0 contact-1.0.xsd">
                <contact:id>aw2015</contact:id>
                <contact:roid>51-Minds</contact:roid>
                <contact:status s="ok">No changes pending</contact:status>
                <contact:postalinfo type="int">
                    <contact:name>Andy Wiens</contact:name>
                    <contact:org>Minds + Machines</contact:org>
                    <contact:addr>
                        <contact:street>32 Nassau St</contact:street>
                        <contact:city>Dublin</contact:city>
                        <contact:sp>Leinster</contact:sp>
                        <contact:pc>Dublin 2</contact:pc>
                        <contact:cc>IE</contact:cc>
                    </contact:addr>
                </contact:postalinfo>
                <contact:voice>+353.16778933</contact:voice>
                <contact:email>andy@mindsandmachines.com</contact:email>
                <contact:clID>basic</contact:clID   >
                <contact:crID>basic</contact:crID>
                <contact:crdate>2015-09-28T18:18:51.0156Z</contact:crdate>
                <contact:authinfo>
                    <contact:pw>takeAw@y</contact:pw>
                </contact:authinfo>
                <contact:disclose flag="0">
                    <contact:name type="loc">
                    </contact:name>
                </contact:disclose>
            </contact:infData>
        </resData>
        <extension>
            <qualifiedLawyer:info xmlns:qualifiedLawyer="urn:ietf:params:xml:ns:qualifiedLawyer-1.0" xsi:schemalocation="urn:ietf:params:xml:ns:qualifiedLawyer-1.0.xsd">
                <qualifiedLawyer:accreditationId>KS-123456</qualifiedLawyer:accreditationId>
                <qualifiedLawyer:accreditationBody>Kansas Bar Association</qualifiedLawyer:accreditationBody>
                <qualifiedLawyer:accreditationYear>2003Z</qualifiedLawyer:accreditationYear>
                <qualifiedLawyer:jurisdictionCC>US</qualifiedLawyer:jurisdictionCC>
                <qualifiedLawyer:jurisdictionSP>Kansas</qualifiedLawyer:jurisdictionSP>
            </qualifiedLawyer:info>
        </extension>
        <trID>
            <cltrID>ABC-12345</cltrID>
            <svtrID>14435333324890</svtrID>
        </trID>
    </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_info_response(*res).unwrap();
        let qualified_lawyer = data.qualified_lawyer.unwrap();
        assert_eq!(data.id, "aw2015");
        assert_eq!(qualified_lawyer.accreditation_id, "KS-123456");
        assert_eq!(
            qualified_lawyer.accreditation_body,
            "Kansas Bar Association"
        );
        assert_eq!(qualified_lawyer.accreditation_year, 2003);
        assert_eq!(qualified_lawyer.jurisdiction_country, "US");
        assert_eq!(qualified_lawyer.jurisdiction_province.unwrap(), "Kansas");
    }
}
