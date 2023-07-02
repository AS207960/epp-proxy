use super::super::client;
use super::epp_proto;
use chrono::prelude::*;

pub fn entity_type_from_i32(from: i32) -> Option<client::contact::EntityType> {
    match epp_proto::contact::EntityType::from_i32(from) {
        Some(e) => match e {
            epp_proto::contact::EntityType::UkLimitedCompany => {
                Some(client::contact::EntityType::UkLimitedCompany)
            }
            epp_proto::contact::EntityType::UkPublicLimitedCompany => {
                Some(client::contact::EntityType::UkPublicLimitedCompany)
            }
            epp_proto::contact::EntityType::UkPartnership => {
                Some(client::contact::EntityType::UkPartnership)
            }
            epp_proto::contact::EntityType::UkSoleTrader => {
                Some(client::contact::EntityType::UkSoleTrader)
            }
            epp_proto::contact::EntityType::UkLimitedLiabilityPartnership => {
                Some(client::contact::EntityType::UkLimitedLiabilityPartnership)
            }
            epp_proto::contact::EntityType::UkIndustrialProvidentRegisteredCompany => {
                Some(client::contact::EntityType::UkIndustrialProvidentRegisteredCompany)
            }
            epp_proto::contact::EntityType::UkIndividual => {
                Some(client::contact::EntityType::UkIndividual)
            }
            epp_proto::contact::EntityType::UkSchool => Some(client::contact::EntityType::UkSchool),
            epp_proto::contact::EntityType::UkRegisteredCharity => {
                Some(client::contact::EntityType::UkRegisteredCharity)
            }
            epp_proto::contact::EntityType::UkGovernmentBody => {
                Some(client::contact::EntityType::UkGovernmentBody)
            }
            epp_proto::contact::EntityType::UkCorporationByRoyalCharter => {
                Some(client::contact::EntityType::UkCorporationByRoyalCharter)
            }
            epp_proto::contact::EntityType::UkStatutoryBody => {
                Some(client::contact::EntityType::UkStatutoryBody)
            }
            epp_proto::contact::EntityType::UkPoliticalParty => {
                Some(client::contact::EntityType::UkPoliticalParty)
            }
            epp_proto::contact::EntityType::OtherUkEntity => {
                Some(client::contact::EntityType::OtherUkEntity)
            }
            epp_proto::contact::EntityType::FinnishIndividual => {
                Some(client::contact::EntityType::FinnishIndividual)
            }
            epp_proto::contact::EntityType::FinnishCompany => {
                Some(client::contact::EntityType::FinnishCompany)
            }
            epp_proto::contact::EntityType::FinnishAssociation => {
                Some(client::contact::EntityType::FinnishAssociation)
            }
            epp_proto::contact::EntityType::FinnishInstitution => {
                Some(client::contact::EntityType::FinnishInstitution)
            }
            epp_proto::contact::EntityType::FinnishPoliticalParty => {
                Some(client::contact::EntityType::FinnishPoliticalParty)
            }
            epp_proto::contact::EntityType::FinnishMunicipality => {
                Some(client::contact::EntityType::FinnishMunicipality)
            }
            epp_proto::contact::EntityType::FinnishGovernment => {
                Some(client::contact::EntityType::FinnishGovernment)
            }
            epp_proto::contact::EntityType::FinnishPublicCommunity => {
                Some(client::contact::EntityType::FinnishPublicCommunity)
            }
            epp_proto::contact::EntityType::OtherIndividual => {
                Some(client::contact::EntityType::OtherIndividual)
            }
            epp_proto::contact::EntityType::OtherCompany => {
                Some(client::contact::EntityType::OtherCompany)
            }
            epp_proto::contact::EntityType::OtherAssociation => {
                Some(client::contact::EntityType::OtherAssociation)
            }
            epp_proto::contact::EntityType::OtherInstitution => {
                Some(client::contact::EntityType::OtherInstitution)
            }
            epp_proto::contact::EntityType::OtherPoliticalParty => {
                Some(client::contact::EntityType::OtherPoliticalParty)
            }
            epp_proto::contact::EntityType::OtherMunicipality => {
                Some(client::contact::EntityType::OtherMunicipality)
            }
            epp_proto::contact::EntityType::OtherGovernment => {
                Some(client::contact::EntityType::OtherGovernment)
            }
            epp_proto::contact::EntityType::OtherPublicCommunity => {
                Some(client::contact::EntityType::OtherPublicCommunity)
            }
            epp_proto::contact::EntityType::UnknownEntity => {
                Some(client::contact::EntityType::Unknown)
            }
            epp_proto::contact::EntityType::NotSet => None,
        },
        None => None,
    }
}

pub fn disclosure_type_from_i32(from: Vec<i32>) -> Vec<client::contact::DisclosureType> {
    let mut out = vec![];
    for i in from {
        if let Some(e) = epp_proto::contact::DisclosureType::from_i32(i) {
            out.push(match e {
                epp_proto::contact::DisclosureType::LocalName => {
                    client::contact::DisclosureType::LocalName
                }
                epp_proto::contact::DisclosureType::InternationalisedName => {
                    client::contact::DisclosureType::InternationalisedName
                }
                epp_proto::contact::DisclosureType::LocalOrganisation => {
                    client::contact::DisclosureType::LocalOrganisation
                }
                epp_proto::contact::DisclosureType::InternationalisedOrganisation => {
                    client::contact::DisclosureType::InternationalisedOrganisation
                }
                epp_proto::contact::DisclosureType::LocalAddress => {
                    client::contact::DisclosureType::LocalAddress
                }
                epp_proto::contact::DisclosureType::InternationalisedAddress => {
                    client::contact::DisclosureType::InternationalisedAddress
                }
                epp_proto::contact::DisclosureType::Voice => client::contact::DisclosureType::Voice,
                epp_proto::contact::DisclosureType::Fax => client::contact::DisclosureType::Fax,
                epp_proto::contact::DisclosureType::Email => client::contact::DisclosureType::Email,
            })
        }
    }
    out
}

pub fn contact_status_from_i32(from: Vec<i32>) -> Vec<client::contact::Status> {
    let mut out = vec![];
    for i in from {
        if let Some(e) = epp_proto::contact::ContactStatus::from_i32(i) {
            out.push(match e {
                epp_proto::contact::ContactStatus::ClientDeleteProhibited => {
                    client::contact::Status::ClientDeleteProhibited
                }
                epp_proto::contact::ContactStatus::ClientTransferProhibited => {
                    client::contact::Status::ClientTransferProhibited
                }
                epp_proto::contact::ContactStatus::ClientUpdateProhibited => {
                    client::contact::Status::ClientUpdateProhibited
                }
                epp_proto::contact::ContactStatus::Linked => client::contact::Status::Linked,
                epp_proto::contact::ContactStatus::Ok => client::contact::Status::Ok,
                epp_proto::contact::ContactStatus::PendingCreate => {
                    client::contact::Status::PendingCreate
                }
                epp_proto::contact::ContactStatus::PendingDelete => {
                    client::contact::Status::PendingDelete
                }
                epp_proto::contact::ContactStatus::PendingTransfer => {
                    client::contact::Status::PendingTransfer
                }
                epp_proto::contact::ContactStatus::PendingUpdate => {
                    client::contact::Status::PendingUpdate
                }
                epp_proto::contact::ContactStatus::ServerDeleteProhibited => {
                    client::contact::Status::ServerDeleteProhibited
                }
                epp_proto::contact::ContactStatus::ServerTransferProhibited => {
                    client::contact::Status::ServerTransferProhibited
                }
                epp_proto::contact::ContactStatus::ServerUpdateProhibited => {
                    client::contact::Status::ServerUpdateProhibited
                }
            })
        }
    }
    out
}

impl From<client::contact::QualifiedLawyerInfo>
    for epp_proto::contact::qualified_lawyer::QualifiedLawyer
{
    fn from(from: client::contact::QualifiedLawyerInfo) -> Self {
        epp_proto::contact::qualified_lawyer::QualifiedLawyer {
            accreditation_id: from.accreditation_id,
            accreditation_body: from.accreditation_body,
            accreditation_year: from.accreditation_year,
            jurisdiction_country: from.jurisdiction_country,
            jurisdiction_province: from.jurisdiction_province,
        }
    }
}

impl From<epp_proto::contact::qualified_lawyer::QualifiedLawyer>
    for client::contact::QualifiedLawyerInfo
{
    fn from(from: epp_proto::contact::qualified_lawyer::QualifiedLawyer) -> Self {
        client::contact::QualifiedLawyerInfo {
            accreditation_id: from.accreditation_id,
            accreditation_body: from.accreditation_body,
            accreditation_year: from.accreditation_year,
            jurisdiction_country: from.jurisdiction_country,
            jurisdiction_province: from.jurisdiction_province,
        }
    }
}

impl From<client::contact::TransferResponse> for epp_proto::contact::ContactTransferReply {
    fn from(res: client::contact::TransferResponse) -> Self {
        epp_proto::contact::ContactTransferReply {
            pending: res.pending,
            status: super::utils::i32_from_transfer_status(res.data.status),
            requested_client_id: res.data.requested_client_id,
            requested_date: super::utils::chrono_to_proto(Some(res.data.requested_date)),
            act_client_id: res.data.act_client_id,
            act_date: super::utils::chrono_to_proto(Some(res.data.act_date)),
            cmd_resp: None,
        }
    }
}

impl From<client::contact::PanData> for epp_proto::contact::ContactPanReply {
    fn from(res: client::contact::PanData) -> Self {
        epp_proto::contact::ContactPanReply {
            id: res.id,
            result: res.result,
            server_transaction_id: res.server_transaction_id,
            client_transaction_id: res.client_transaction_id,
            date: super::utils::chrono_to_proto(Some(res.date)),
        }
    }
}

impl From<client::contact::InfoResponse> for epp_proto::contact::ContactInfoReply {
    fn from(res: client::contact::InfoResponse) -> Self {
        let map_addr = |a: client::contact::Address| epp_proto::contact::PostalAddress {
            name: a.name,
            organisation: a.organisation,
            streets: a.streets,
            city: a.city,
            province: a.province,
            postal_code: a.postal_code,
            country_code: a.country_code,
            identity_number: a.identity_number,
            birth_date: super::utils::chrono_to_proto(
                a.birth_date
                    .map(|d| Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).unwrap())),
            ),
        };

        epp_proto::contact::ContactInfoReply {
            id: res.id,
            registry_id: res.registry_id,
            statuses: res
                .statuses
                .into_iter()
                .map(|s| match s {
                    client::contact::Status::ClientDeleteProhibited => {
                        epp_proto::contact::ContactStatus::ClientDeleteProhibited.into()
                    }
                    client::contact::Status::ClientTransferProhibited => {
                        epp_proto::contact::ContactStatus::ClientTransferProhibited.into()
                    }
                    client::contact::Status::ClientUpdateProhibited => {
                        epp_proto::contact::ContactStatus::ClientUpdateProhibited.into()
                    }
                    client::contact::Status::Linked => {
                        epp_proto::contact::ContactStatus::Linked.into()
                    }
                    client::contact::Status::Ok => epp_proto::contact::ContactStatus::Ok.into(),
                    client::contact::Status::PendingCreate => {
                        epp_proto::contact::ContactStatus::PendingCreate.into()
                    }
                    client::contact::Status::PendingDelete => {
                        epp_proto::contact::ContactStatus::PendingDelete.into()
                    }
                    client::contact::Status::PendingTransfer => {
                        epp_proto::contact::ContactStatus::PendingTransfer.into()
                    }
                    client::contact::Status::PendingUpdate => {
                        epp_proto::contact::ContactStatus::PendingUpdate.into()
                    }
                    client::contact::Status::ServerDeleteProhibited => {
                        epp_proto::contact::ContactStatus::ServerDeleteProhibited.into()
                    }
                    client::contact::Status::ServerTransferProhibited => {
                        epp_proto::contact::ContactStatus::ServerTransferProhibited.into()
                    }
                    client::contact::Status::ServerUpdateProhibited => {
                        epp_proto::contact::ContactStatus::ServerUpdateProhibited.into()
                    }
                })
                .collect(),
            local_address: res.local_address.map(map_addr),
            internationalised_address: res.internationalised_address.map(map_addr),
            phone: res.phone.map(Into::into),
            fax: res.fax.map(Into::into),
            email: res.email,
            client_id: res.client_id,
            client_created_id: res.client_created_id,
            creation_date: super::utils::chrono_to_proto(res.creation_date),
            last_updated_client: res.last_updated_client,
            last_updated_date: super::utils::chrono_to_proto(res.last_updated_date),
            last_transfer_date: super::utils::chrono_to_proto(res.last_transfer_date),
            entity_type: match res.entity_type {
                client::contact::EntityType::UkLimitedCompany => {
                    epp_proto::contact::EntityType::UkLimitedCompany.into()
                }
                client::contact::EntityType::UkPublicLimitedCompany => {
                    epp_proto::contact::EntityType::UkLimitedCompany.into()
                }
                client::contact::EntityType::UkPartnership => {
                    epp_proto::contact::EntityType::UkPartnership.into()
                }
                client::contact::EntityType::UkSoleTrader => {
                    epp_proto::contact::EntityType::UkSoleTrader.into()
                }
                client::contact::EntityType::UkLimitedLiabilityPartnership => {
                    epp_proto::contact::EntityType::UkLimitedLiabilityPartnership.into()
                }
                client::contact::EntityType::UkIndustrialProvidentRegisteredCompany => {
                    epp_proto::contact::EntityType::UkIndustrialProvidentRegisteredCompany.into()
                }
                client::contact::EntityType::UkIndividual => {
                    epp_proto::contact::EntityType::UkIndividual.into()
                }
                client::contact::EntityType::UkSchool => {
                    epp_proto::contact::EntityType::UkSchool.into()
                }
                client::contact::EntityType::UkRegisteredCharity => {
                    epp_proto::contact::EntityType::UkRegisteredCharity.into()
                }
                client::contact::EntityType::UkGovernmentBody => {
                    epp_proto::contact::EntityType::UkGovernmentBody.into()
                }
                client::contact::EntityType::UkCorporationByRoyalCharter => {
                    epp_proto::contact::EntityType::UkCorporationByRoyalCharter.into()
                }
                client::contact::EntityType::UkStatutoryBody => {
                    epp_proto::contact::EntityType::UkStatutoryBody.into()
                }
                client::contact::EntityType::UkPoliticalParty => {
                    epp_proto::contact::EntityType::UkPoliticalParty.into()
                }
                client::contact::EntityType::OtherUkEntity => {
                    epp_proto::contact::EntityType::OtherUkEntity.into()
                }
                client::contact::EntityType::FinnishIndividual => {
                    epp_proto::contact::EntityType::FinnishIndividual.into()
                }
                client::contact::EntityType::FinnishCompany => {
                    epp_proto::contact::EntityType::FinnishCompany.into()
                }
                client::contact::EntityType::FinnishAssociation => {
                    epp_proto::contact::EntityType::FinnishAssociation.into()
                }
                client::contact::EntityType::FinnishInstitution => {
                    epp_proto::contact::EntityType::FinnishInstitution.into()
                }
                client::contact::EntityType::FinnishPoliticalParty => {
                    epp_proto::contact::EntityType::FinnishPoliticalParty.into()
                }
                client::contact::EntityType::FinnishMunicipality => {
                    epp_proto::contact::EntityType::FinnishMunicipality.into()
                }
                client::contact::EntityType::FinnishGovernment => {
                    epp_proto::contact::EntityType::FinnishGovernment.into()
                }
                client::contact::EntityType::FinnishPublicCommunity => {
                    epp_proto::contact::EntityType::FinnishPublicCommunity.into()
                }
                client::contact::EntityType::OtherIndividual => {
                    epp_proto::contact::EntityType::OtherIndividual.into()
                }
                client::contact::EntityType::OtherCompany => {
                    epp_proto::contact::EntityType::OtherCompany.into()
                }
                client::contact::EntityType::OtherAssociation => {
                    epp_proto::contact::EntityType::OtherAssociation.into()
                }
                client::contact::EntityType::OtherInstitution => {
                    epp_proto::contact::EntityType::OtherInstitution.into()
                }
                client::contact::EntityType::OtherPoliticalParty => {
                    epp_proto::contact::EntityType::OtherPoliticalParty.into()
                }
                client::contact::EntityType::OtherMunicipality => {
                    epp_proto::contact::EntityType::OtherMunicipality.into()
                }
                client::contact::EntityType::OtherGovernment => {
                    epp_proto::contact::EntityType::OtherGovernment.into()
                }
                client::contact::EntityType::OtherPublicCommunity => {
                    epp_proto::contact::EntityType::OtherPublicCommunity.into()
                }
                client::contact::EntityType::Unknown => {
                    epp_proto::contact::EntityType::UnknownEntity.into()
                }
            },
            trading_name: res.trading_name,
            company_number: res.company_number,
            disclosure: res
                .disclosure
                .into_iter()
                .map(|d| match d {
                    client::contact::DisclosureType::LocalName => {
                        epp_proto::contact::DisclosureType::LocalName.into()
                    }
                    client::contact::DisclosureType::InternationalisedName => {
                        epp_proto::contact::DisclosureType::InternationalisedName.into()
                    }
                    client::contact::DisclosureType::LocalOrganisation => {
                        epp_proto::contact::DisclosureType::LocalOrganisation.into()
                    }
                    client::contact::DisclosureType::InternationalisedOrganisation => {
                        epp_proto::contact::DisclosureType::InternationalisedOrganisation.into()
                    }
                    client::contact::DisclosureType::LocalAddress => {
                        epp_proto::contact::DisclosureType::LocalAddress.into()
                    }
                    client::contact::DisclosureType::InternationalisedAddress => {
                        epp_proto::contact::DisclosureType::InternationalisedAddress.into()
                    }
                    client::contact::DisclosureType::Voice => {
                        epp_proto::contact::DisclosureType::Voice.into()
                    }
                    client::contact::DisclosureType::Fax => {
                        epp_proto::contact::DisclosureType::Fax.into()
                    }
                    client::contact::DisclosureType::Email => {
                        epp_proto::contact::DisclosureType::Email.into()
                    }
                })
                .collect(),
            auth_info: res.auth_info,
            nominet_data_quality: res.nominet_data_quality.map(|q| {
                epp_proto::nominet_ext::DataQuality {
                    status: match q.status {
                        client::nominet::DataQualityStatus::Invalid => {
                            epp_proto::nominet_ext::DataQualityStatus::Invalid.into()
                        }
                        client::nominet::DataQualityStatus::Valid => {
                            epp_proto::nominet_ext::DataQualityStatus::Valid.into()
                        }
                    },
                    reason: q.reason,
                    date_commenced: super::utils::chrono_to_proto(q.date_commenced),
                    date_to_suspend: super::utils::chrono_to_proto(q.date_to_suspend),
                    lock_applied: q.lock_applied,
                    domains: q.domains.unwrap_or_default(),
                }
            }),
            eurid_info: res.eurid_contact_extension.map(Into::into),
            qualified_lawyer: res.qualified_lawyer.map(Into::into),
            isnic_info: res.isnic_info.map(|c| epp_proto::isnic::ContactInfo {
                statuses: c
                    .statuses
                    .into_iter()
                    .map(|s| match s {
                        client::isnic::ContactStatus::Ok => {
                            epp_proto::isnic::ContactStatus::Ok.into()
                        }
                        client::isnic::ContactStatus::OkUnconfirmed => {
                            epp_proto::isnic::ContactStatus::OkUnconfirmed.into()
                        }
                        client::isnic::ContactStatus::PendingCreate => {
                            epp_proto::isnic::ContactStatus::PendingCreate.into()
                        }
                        client::isnic::ContactStatus::ServerExpired => {
                            epp_proto::isnic::ContactStatus::ServerExpired.into()
                        }
                        client::isnic::ContactStatus::ServerSuspended => {
                            epp_proto::isnic::ContactStatus::ServerSuspended.into()
                        }
                    })
                    .collect(),
                mobile: c.mobile.map(Into::into),
                sid: c.sid,
                auto_update_from_national_registry: c.auto_update_from_national_registry,
                paper_invoices: c.paper_invoices,
            }),
            cmd_resp: None,
            keysys: res.keysys.map(Into::into),
        }
    }
}
