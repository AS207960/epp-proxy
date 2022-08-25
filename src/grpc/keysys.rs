use crate::grpc::utils::chrono_to_proto;
use std::convert::TryFrom;
use super::super::client;
use super::epp_proto;

impl From<client::keysys::ContactInfo> for epp_proto::keysys::ContactInfo {
    fn from(res: client::keysys::ContactInfo) -> Self {
        epp_proto::keysys::ContactInfo {
            validated: res.validated,
            verification_requested: res.verification_requested,
            verified: res.verified,
        }
    }
}

impl From<epp_proto::keysys::ContactCreate> for client::keysys::ContactCreate {
    fn from(res: epp_proto::keysys::ContactCreate) -> Self {
        client::keysys::ContactCreate {
            check_only: res.check_only,
            force_duplication: res.force_duplication,
            pre_verify: res.pre_verify,
        }
    }
}

impl From<epp_proto::keysys::ContactUpdate> for client::keysys::ContactUpdate {
    fn from(res: epp_proto::keysys::ContactUpdate) -> Self {
        client::keysys::ContactUpdate {
            check_only: res.check_only,
            pre_verify: res.pre_verify,
            trigger_foa: res.trigger_foa,
        }
    }
}

impl From<epp_proto::keysys::DomainCheck> for client::keysys::DomainCheck {
    fn from(res: epp_proto::keysys::DomainCheck) -> Self {
        client::keysys::DomainCheck {
            allocation_token: if res.allocation_token.is_empty() {
                None
            } else {
                Some(res.allocation_token)
            },
        }
    }
}

impl From<epp_proto::keysys::DomainRenew> for client::keysys::DomainRenew {
    fn from(res: epp_proto::keysys::DomainRenew) -> Self {
        client::keysys::DomainRenew {
            accept_premium_price: res.accept_premium_price,
            promotion_code: if res.promotion_code.is_empty() {
                None
            } else {
                Some(res.promotion_code)
            },
        }
    }
}

impl From<epp_proto::keysys::DomainDelete> for client::keysys::DomainDelete {
    fn from(res: epp_proto::keysys::DomainDelete) -> Self {
        client::keysys::DomainDelete {
            action: match epp_proto::keysys::DomainDeleteAction::from_i32(res.action) {
                Some(epp_proto::keysys::DomainDeleteAction::DefaultDelete) => client::keysys::DomainDeleteAction::Default,
                None => client::keysys::DomainDeleteAction::Default,
                Some(epp_proto::keysys::DomainDeleteAction::SetAutoExpire) => client::keysys::DomainDeleteAction::AutoExpire,
                Some(epp_proto::keysys::DomainDeleteAction::SetAutoDelete) => client::keysys::DomainDeleteAction::AutoDelete,
                Some(epp_proto::keysys::DomainDeleteAction::Instant) => client::keysys::DomainDeleteAction::Instant,
                Some(epp_proto::keysys::DomainDeleteAction::Push) => client::keysys::DomainDeleteAction::Push,
            },
            target: if res.target.is_empty() {
                None
            } else {
                Some(res.target)
            },
        }
    }
}

impl From<epp_proto::keysys::DomainTransfer> for client::keysys::DomainTransfer {
    fn from(res: epp_proto::keysys::DomainTransfer) -> Self {
        client::keysys::DomainTransfer {
            accept_premium_price: res.accept_premium_price,
            accept_quarantine: res.accept_quarantine,
            accept_trade: res.accept_trade,
            allocation_token: if res.allocation_token.is_empty() {
                None
            } else {
                Some(res.allocation_token)
            },
            at_request_authcode: res.at_request_authcode,
            be_request_authcode: res.be_request_authcode,
            promotion_code: if res.promotion_code.is_empty() {
                None
            } else {
                Some(res.promotion_code)
            },
        }
    }
}

fn map_renewal_mode(renewal_mode: i32) -> Option<client::keysys::RenewalMode> {
    match epp_proto::keysys::RenewalMode::from_i32(renewal_mode) {
        None => None,
        Some(epp_proto::keysys::RenewalMode::UnknownRenew) => None,
        Some(epp_proto::keysys::RenewalMode::DefaultRenew) => Some(client::keysys::RenewalMode::Default),
        Some(epp_proto::keysys::RenewalMode::AutoRenew) => Some(client::keysys::RenewalMode::AutoRenew),
        Some(epp_proto::keysys::RenewalMode::AutoExpire) => Some(client::keysys::RenewalMode::AutoExpire),
        Some(epp_proto::keysys::RenewalMode::AutoDelete) => Some(client::keysys::RenewalMode::AutoDelete),
        Some(epp_proto::keysys::RenewalMode::AutoRenewMonthly) => Some(client::keysys::RenewalMode::AutoRenewMonthly),
        Some(epp_proto::keysys::RenewalMode::AutoRenewQuarterly) => Some(client::keysys::RenewalMode::AutoRenewQuarterly),
        Some(epp_proto::keysys::RenewalMode::ExpireAuction) => Some(client::keysys::RenewalMode::ExpireAuction),
        Some(epp_proto::keysys::RenewalMode::RenewOnce) => Some(client::keysys::RenewalMode::RenewOnce),
    }
}

fn map_transfer_mode(transfer_mode: i32) -> Option<client::keysys::TransferMode> {
    match epp_proto::keysys::TransferMode::from_i32(transfer_mode) {
        None => None,
        Some(epp_proto::keysys::TransferMode::UnknownTransfer) => None,
        Some(epp_proto::keysys::TransferMode::DefaultTransfer) => Some(client::keysys::TransferMode::Default),
        Some(epp_proto::keysys::TransferMode::AutoApprove) => Some(client::keysys::TransferMode::AutoApprove),
        Some(epp_proto::keysys::TransferMode::AutoDeny) => Some(client::keysys::TransferMode::AutoDeny),
    }
}

fn map_eu_language(eu_language: i32) -> Option<client::keysys::EULanguage> {
    match epp_proto::keysys::EuLanguage::from_i32(eu_language) {
        None => None,
        Some(epp_proto::keysys::EuLanguage::UnknownLanguage) => None,
        Some(epp_proto::keysys::EuLanguage::Bulgarian) => Some(client::keysys::EULanguage::Bulgarian),
        Some(epp_proto::keysys::EuLanguage::Czech) => Some(client::keysys::EULanguage::Czech),
        Some(epp_proto::keysys::EuLanguage::Danish) => Some(client::keysys::EULanguage::Danish),
        Some(epp_proto::keysys::EuLanguage::German) => Some(client::keysys::EULanguage::German),
        Some(epp_proto::keysys::EuLanguage::ModernGreek) => Some(client::keysys::EULanguage::ModernGreek),
        Some(epp_proto::keysys::EuLanguage::English) => Some(client::keysys::EULanguage::English),
        Some(epp_proto::keysys::EuLanguage::Spanish) => Some(client::keysys::EULanguage::Spanish),
        Some(epp_proto::keysys::EuLanguage::Estonian) => Some(client::keysys::EULanguage::Estonian),
        Some(epp_proto::keysys::EuLanguage::Finnish) => Some(client::keysys::EULanguage::Finnish),
        Some(epp_proto::keysys::EuLanguage::French) => Some(client::keysys::EULanguage::French),
        Some(epp_proto::keysys::EuLanguage::Gaelic) => Some(client::keysys::EULanguage::Gaelic),
        Some(epp_proto::keysys::EuLanguage::Croatian) => Some(client::keysys::EULanguage::Croatian),
        Some(epp_proto::keysys::EuLanguage::Hungarian) => Some(client::keysys::EULanguage::Hungarian),
        Some(epp_proto::keysys::EuLanguage::Italian) => Some(client::keysys::EULanguage::Italian),
        Some(epp_proto::keysys::EuLanguage::Lithuanian) => Some(client::keysys::EULanguage::Lithuanian),
        Some(epp_proto::keysys::EuLanguage::Latvian) => Some(client::keysys::EULanguage::Latvian),
        Some(epp_proto::keysys::EuLanguage::Maltese) => Some(client::keysys::EULanguage::Maltese),
        Some(epp_proto::keysys::EuLanguage::DutchFlemish) => Some(client::keysys::EULanguage::DutchFlemish),
        Some(epp_proto::keysys::EuLanguage::Polish) => Some(client::keysys::EULanguage::Polish),
        Some(epp_proto::keysys::EuLanguage::Portuguese) => Some(client::keysys::EULanguage::Portuguese),
        Some(epp_proto::keysys::EuLanguage::Romanian) => Some(client::keysys::EULanguage::Romanian),
        Some(epp_proto::keysys::EuLanguage::Slovak) => Some(client::keysys::EULanguage::Slovak),
        Some(epp_proto::keysys::EuLanguage::Slovene) => Some(client::keysys::EULanguage::Slovene),
        Some(epp_proto::keysys::EuLanguage::Swedish) => Some(client::keysys::EULanguage::Swedish),
    }
}

fn map_eu_country(eu_country: i32) -> Option<client::keysys::EUCountry> {
    match epp_proto::keysys::EuCountry::from_i32(eu_country) {
        None => None,
        Some(epp_proto::keysys::EuCountry::UnknownCountry) => None,
        Some(epp_proto::keysys::EuCountry::Austria) => Some(client::keysys::EUCountry::Austria),
        Some(epp_proto::keysys::EuCountry::Belgium) => Some(client::keysys::EUCountry::Belgium),
        Some(epp_proto::keysys::EuCountry::Bulgaria) => Some(client::keysys::EUCountry::Bulgaria),
        Some(epp_proto::keysys::EuCountry::CzechRepublic) => Some(client::keysys::EUCountry::Czech),
        Some(epp_proto::keysys::EuCountry::Cyprus) => Some(client::keysys::EUCountry::Cyprus),
        Some(epp_proto::keysys::EuCountry::Germany) => Some(client::keysys::EUCountry::Germany),
        Some(epp_proto::keysys::EuCountry::Denmark) => Some(client::keysys::EUCountry::Denmark),
        Some(epp_proto::keysys::EuCountry::Spain) => Some(client::keysys::EUCountry::Spain),
        Some(epp_proto::keysys::EuCountry::Estonia) => Some(client::keysys::EUCountry::Estonia),
        Some(epp_proto::keysys::EuCountry::Finland) => Some(client::keysys::EUCountry::Finland),
        Some(epp_proto::keysys::EuCountry::France) => Some(client::keysys::EUCountry::France),
        Some(epp_proto::keysys::EuCountry::Greece) => Some(client::keysys::EUCountry::Greece),
        Some(epp_proto::keysys::EuCountry::Hungary) => Some(client::keysys::EUCountry::Hungary),
        Some(epp_proto::keysys::EuCountry::Ireland) => Some(client::keysys::EUCountry::Ireland),
        Some(epp_proto::keysys::EuCountry::Italy) => Some(client::keysys::EUCountry::Italy),
        Some(epp_proto::keysys::EuCountry::Liechtenstein) => Some(client::keysys::EUCountry::Liechtenstein),
        Some(epp_proto::keysys::EuCountry::Lithuania) => Some(client::keysys::EUCountry::Lithuania),
        Some(epp_proto::keysys::EuCountry::Luxembourg) => Some(client::keysys::EUCountry::Luxembourg),
        Some(epp_proto::keysys::EuCountry::Latvia) => Some(client::keysys::EUCountry::Latvia),
        Some(epp_proto::keysys::EuCountry::Malta) => Some(client::keysys::EUCountry::Malta),
        Some(epp_proto::keysys::EuCountry::Netherlands) => Some(client::keysys::EUCountry::Netherlands),
        Some(epp_proto::keysys::EuCountry::Poland) => Some(client::keysys::EUCountry::Poland),
        Some(epp_proto::keysys::EuCountry::Portugal) => Some(client::keysys::EUCountry::Portugal),
        Some(epp_proto::keysys::EuCountry::Romania) => Some(client::keysys::EUCountry::Romania),
        Some(epp_proto::keysys::EuCountry::Sweden) => Some(client::keysys::EUCountry::Sweden),
        Some(epp_proto::keysys::EuCountry::Slovakia) => Some(client::keysys::EUCountry::Slovakia),
        Some(epp_proto::keysys::EuCountry::Slovenia) => Some(client::keysys::EUCountry::Slovenia),
        Some(epp_proto::keysys::EuCountry::Croatia) => Some(client::keysys::EUCountry::Croatia),
    }
}

fn map_us_purpose(us_purpose: i32) -> Option<client::keysys::USPurpose> {
    match epp_proto::keysys::UsPurpose::from_i32(us_purpose) {
        None => None,
        Some(epp_proto::keysys::UsPurpose::UnknownPurpose) => None,
        Some(epp_proto::keysys::UsPurpose::Business) => Some(client::keysys::USPurpose::Business),
        Some(epp_proto::keysys::UsPurpose::Personal) => Some(client::keysys::USPurpose::Personal),
        Some(epp_proto::keysys::UsPurpose::NonProfit) => Some(client::keysys::USPurpose::NonProfit),
        Some(epp_proto::keysys::UsPurpose::Educational) => Some(client::keysys::USPurpose::Educational),
        Some(epp_proto::keysys::UsPurpose::UsGovernment) => Some(client::keysys::USPurpose::Government),
    }
}

fn map_us_category(us_category: i32) -> Option<client::keysys::USCategory> {
    match epp_proto::keysys::UsCategory::from_i32(us_category) {
        None => None,
        Some(epp_proto::keysys::UsCategory::UnknownCategory) => None,
        Some(epp_proto::keysys::UsCategory::UsCitizen) => Some(client::keysys::USCategory::Citizen),
        Some(epp_proto::keysys::UsCategory::UsPermanentResident) => Some(client::keysys::USCategory::PermanentResident),
        Some(epp_proto::keysys::UsCategory::UsOrganisation) => Some(client::keysys::USCategory::USOrganisation),
        Some(epp_proto::keysys::UsCategory::OfficeOrFacility) => Some(client::keysys::USCategory::OfficeOrFacility),
        Some(epp_proto::keysys::UsCategory::RegularActivity) => Some(client::keysys::USCategory::RegularActivity),
    }
}

fn map_ca_legal_type(legal_type: i32) -> Option<client::keysys::CALegalType> {
    match epp_proto::keysys::CaLegalType::from_i32(legal_type) {
        None => None,
        Some(epp_proto::keysys::CaLegalType::UnknownCaLegalType) => None,
        Some(epp_proto::keysys::CaLegalType::AboriginalPeoples) => Some(client::keysys::CALegalType::AboriginalPeoples),
        Some(epp_proto::keysys::CaLegalType::CanadianUnincorporatedAssociation) => Some(client::keysys::CALegalType::CanadianUnincorporatedAssociation),
        Some(epp_proto::keysys::CaLegalType::CanadianCorporation) => Some(client::keysys::CALegalType::Corporation),
        Some(epp_proto::keysys::CaLegalType::CanadianCitizen) => Some(client::keysys::CALegalType::Citizen),
        Some(epp_proto::keysys::CaLegalType::CanadianEducationalInstitution) => Some(client::keysys::CALegalType::CanadianEducationalInstitution),
        Some(epp_proto::keysys::CaLegalType::CanadianGovernment) => Some(client::keysys::CALegalType::Government),
        Some(epp_proto::keysys::CaLegalType::CanadianHospital) => Some(client::keysys::CALegalType::CanadianHospital),
        Some(epp_proto::keysys::CaLegalType::IndianBand) => Some(client::keysys::CALegalType::IndianBand),
        Some(epp_proto::keysys::CaLegalType::CanadianLibraryArchiveMuseum) => Some(client::keysys::CALegalType::CanadianLibraryArchiveMuseum),
        Some(epp_proto::keysys::CaLegalType::LegalRepOfCanadianCitizenOrPermanentResident) => Some(client::keysys::CALegalType::LegalRepOfCanadianCitizenOrPermanentResident),
        Some(epp_proto::keysys::CaLegalType::TheQueen) => Some(client::keysys::CALegalType::TheQueen),
        Some(epp_proto::keysys::CaLegalType::OfficialMark) => Some(client::keysys::CALegalType::OfficialMark),
        Some(epp_proto::keysys::CaLegalType::CanadianPoliticalParty) => Some(client::keysys::CALegalType::CanadianPoliticalParty),
        Some(epp_proto::keysys::CaLegalType::Partnership) => Some(client::keysys::CALegalType::Partnership),
        Some(epp_proto::keysys::CaLegalType::CanadianPermanentResident) => Some(client::keysys::CALegalType::PermanentResident),
        Some(epp_proto::keysys::CaLegalType::TradeMark) => Some(client::keysys::CALegalType::TradeMark),
        Some(epp_proto::keysys::CaLegalType::TradeUnion) => Some(client::keysys::CALegalType::TradeUnion),
        Some(epp_proto::keysys::CaLegalType::Trust) => Some(client::keysys::CALegalType::Trust),
    }
}

impl TryFrom<epp_proto::keysys::DomainCreate> for client::keysys::DomainCreate {
    type Error = client::Error;

    fn try_from(res: epp_proto::keysys::DomainCreate) -> Result<Self, Self::Error> {
        Ok(client::keysys::DomainCreate {
            accept_premium_price: res.accept_premium_price,
            accept_ssl_requirements: res.accept_ssl_requirements,
            allocation_token: if res.allocation_token.is_empty() {
                None
            } else {
                Some(res.allocation_token)
            },
            renewal_mode: map_renewal_mode(res.renewal_mode).unwrap_or(client::keysys::RenewalMode::Default),
            transfer_mode: map_transfer_mode(res.transfer_mode).unwrap_or(client::keysys::TransferMode::Default),
            whois_banner: res.whois_banner,
            whois_rsp: if res.whois_rsp.is_empty() {
                None
            } else {
                Some(res.whois_rsp)
            },
            whois_url: if res.whois_url.is_empty() {
                None
            } else {
                Some(res.whois_url)
            },
            tld: match res.tld.map(|t| Ok(match t {
                epp_proto::keysys::domain_create::Tld::Ca(t) => client::keysys::DomainCreateTLD::CA(client::keysys::DomainCreateCA {
                    legal_type: match map_ca_legal_type(t.legal_type) {
                        Some(legal_type) => legal_type,
                        None => return Err(client::Error::Err("CA legal type required".to_string())),
                    },
                    trademark: t.trademark,
                }),
                epp_proto::keysys::domain_create::Tld::De(t) => client::keysys::DomainCreateTLD::DE(client::keysys::DomainCreateDE {
                    abuse_contact: t.abuse_contact,
                    general_request: t.general_contact,
                    holder_person: t.holder_person,
                    accept_trustee_tac: match epp_proto::keysys::DeTrustee::from_i32(t.trustee) {
                        None => client::keysys::DETrustee::None,
                        Some(epp_proto::keysys::DeTrustee::None) => client::keysys::DETrustee::None,
                        Some(epp_proto::keysys::DeTrustee::Disable) => client::keysys::DETrustee::None,
                        Some(epp_proto::keysys::DeTrustee::Monthly) => client::keysys::DETrustee::Monthly,
                        Some(epp_proto::keysys::DeTrustee::Annually) => client::keysys::DETrustee::Annually
                    },
                }),
                epp_proto::keysys::domain_create::Tld::Eu(t) => client::keysys::DomainCreateTLD::EU(client::keysys::DomainCreateEU {
                    accept_trustee_tac: t.trustee,
                    registrant_lang: map_eu_language(t.registrant_language),
                    registrant_citizenship: map_eu_country(t.registrant_citizenship),
                }),
                epp_proto::keysys::domain_create::Tld::Fr(t) => client::keysys::DomainCreateTLD::FR(client::keysys::DomainCreateFR {
                    accept_trustee_tac: t.trustee,
                }),
                epp_proto::keysys::domain_create::Tld::Gay(t) => client::keysys::DomainCreateTLD::Gay(client::keysys::DomainCreateGay {
                    accept_requirements: t.accept_requirements,
                }),
                epp_proto::keysys::domain_create::Tld::Name(t) => client::keysys::DomainCreateTLD::Name(client::keysys::DomainName {
                    email_forward: t.email_forward,
                }),
                epp_proto::keysys::domain_create::Tld::Rs(t) => client::keysys::DomainCreateTLD::RS(client::keysys::DomainCreateRS {
                    admin: match t.admin {
                        None => return Err(client::Error::Err("RS admin required".to_string())),
                        Some(epp_proto::keysys::domain_info_rs::Admin::AdminIdCard(n)) => client::keysys::RsId::IDCard(n),
                        Some(epp_proto::keysys::domain_info_rs::Admin::AdminCompanyNumber(n)) => client::keysys::RsId::CompanyNumber(n),
                    },
                    tech: match t.tech {
                        None => return Err(client::Error::Err("RS tech required".to_string())),
                        Some(epp_proto::keysys::domain_info_rs::Tech::TechIdCard(n)) => client::keysys::RsId::IDCard(n),
                        Some(epp_proto::keysys::domain_info_rs::Tech::TechCompanyNumber(n)) => client::keysys::RsId::CompanyNumber(n),
                    },
                    owner: match t.owner {
                        None => return Err(client::Error::Err("RS owner required".to_string())),
                        Some(epp_proto::keysys::domain_info_rs::Owner::OwnerIdCard(n)) => client::keysys::RsId::IDCard(n),
                        Some(epp_proto::keysys::domain_info_rs::Owner::OwnerCompanyNumber(n)) => client::keysys::RsId::CompanyNumber(n),
                    }
                }),
                epp_proto::keysys::domain_create::Tld::Us(t) => client::keysys::DomainCreateTLD::US(client::keysys::DomainCreateUS {
                    category: match map_us_category(t.category) {
                        Some(c) => c,
                        None => return Err(client::Error::Err("US category required".to_string())),
                    },
                    purpose: match map_us_purpose(t.purpose) {
                        Some(p) => p,
                        None => return Err(client::Error::Err("US purpose required".to_string())),
                    },
                    validator: t.validator,
                })
            })) {
                None => None,
                Some(Err(e)) => return Err(e),
                Some(Ok(v)) => Some(v),
            }
        })
    }
}

impl From<epp_proto::keysys::DomainUpdate> for client::keysys::DomainUpdate {
    fn from(res: epp_proto::keysys::DomainUpdate) -> Self {
        client::keysys::DomainUpdate {
            renewal_mode: map_renewal_mode(res.renewal_mode),
            transfer_mode: map_transfer_mode(res.transfer_mode),
            whois_banner: res.whois_banner,
            whois_rsp: res.whois_rsp,
            whois_url: res.whois_url,
            tld: res.tld.map(|t| match t {
                epp_proto::keysys::domain_update::Tld::Ca(t) => client::keysys::DomainUpdateTLD::CA(client::keysys::DomainUpdateCA {
                    trademark: t.trademark,
                    legal_type: map_ca_legal_type(t.legal_type)
                }),
                epp_proto::keysys::domain_update::Tld::De(t) => client::keysys::DomainUpdateTLD::DE(client::keysys::DomainUpdateDE {
                    abuse_contact: t.abuse_contact,
                    general_request: t.general_contact,
                    holder_person: t.holder_person,
                    accept_trustee_tac: match epp_proto::keysys::DeTrustee::from_i32(t.trustee) {
                        None => None,
                        Some(epp_proto::keysys::DeTrustee::None) => None,
                        Some(epp_proto::keysys::DeTrustee::Monthly) => Some(client::keysys::DETrustee::Monthly),
                        Some(epp_proto::keysys::DeTrustee::Annually) => Some(client::keysys::DETrustee::Annually),
                        Some(epp_proto::keysys::DeTrustee::Disable) => Some(client::keysys::DETrustee::None)
                    },
                }),
                epp_proto::keysys::domain_update::Tld::Eu(t) => client::keysys::DomainUpdateTLD::EU(client::keysys::DomainUpdateEU {
                    registrant_lang: map_eu_language(t.registrant_language),
                    registrant_citizenship: map_eu_country(t.registrant_citizenship),
                    accept_trustee_tac: t.trustee,
                }),
                epp_proto::keysys::domain_update::Tld::Fr(t) => client::keysys::DomainUpdateTLD::FR(client::keysys::DomainUpdateFR {
                    accept_trustee_tac: t.trustee,
                }),
                epp_proto::keysys::domain_update::Tld::Name(t) => client::keysys::DomainUpdateTLD::Name(client::keysys::DomainName {
                    email_forward: t.email_forward,
                }),
                epp_proto::keysys::domain_update::Tld::Rs(t) => client::keysys::DomainUpdateTLD::RS(client::keysys::DomainUpdateRS {
                    owner: t.owner.map(|o| match o {
                        epp_proto::keysys::domain_info_rs::Owner::OwnerIdCard(n) => client::keysys::RsId::IDCard(n),
                        epp_proto::keysys::domain_info_rs::Owner::OwnerCompanyNumber(n) => client::keysys::RsId::CompanyNumber(n),
                    }),
                    tech: t.tech.map(|o| match o {
                        epp_proto::keysys::domain_info_rs::Tech::TechIdCard(n) => client::keysys::RsId::IDCard(n),
                        epp_proto::keysys::domain_info_rs::Tech::TechCompanyNumber(n) => client::keysys::RsId::CompanyNumber(n),
                    }),
                    admin: t.admin.map(|o| match o {
                        epp_proto::keysys::domain_info_rs::Admin::AdminIdCard(n) => client::keysys::RsId::IDCard(n),
                        epp_proto::keysys::domain_info_rs::Admin::AdminCompanyNumber(n) => client::keysys::RsId::CompanyNumber(n),
                    }),
                }),
                epp_proto::keysys::domain_update::Tld::Us(t) => client::keysys::DomainUpdateTLD::US(client::keysys::DomainUpdateUS {
                    purpose: map_us_purpose(t.purpose),
                    category: map_us_category(t.category),
                    validator: t.validator,
                })
            })
        }
    }
}

impl From<client::keysys::DomainInfo> for epp_proto::keysys::DomainInfo {
    fn from(res: client::keysys::DomainInfo) -> Self {
        epp_proto::keysys::DomainInfo {
            renewal_date: chrono_to_proto(Some(res.renewal_date)),
            paid_until_date: chrono_to_proto(Some(res.paid_until_date)),
            roid: res.roid,
            renewal_mode: match res.renewal_mode {
                client::keysys::RenewalMode::Default => epp_proto::keysys::RenewalMode::DefaultRenew,
                client::keysys::RenewalMode::AutoRenew => epp_proto::keysys::RenewalMode::AutoRenew,
                client::keysys::RenewalMode::AutoExpire => epp_proto::keysys::RenewalMode::AutoExpire,
                client::keysys::RenewalMode::AutoDelete => epp_proto::keysys::RenewalMode::AutoDelete,
                client::keysys::RenewalMode::AutoRenewMonthly => epp_proto::keysys::RenewalMode::AutoRenewMonthly,
                client::keysys::RenewalMode::AutoRenewQuarterly => epp_proto::keysys::RenewalMode::AutoRenewQuarterly,
                client::keysys::RenewalMode::ExpireAuction => epp_proto::keysys::RenewalMode::ExpireAuction,
                client::keysys::RenewalMode::RenewOnce => epp_proto::keysys::RenewalMode::RenewOnce,
            }.into(),
            transfer_mode: match res.transfer_mode {
                client::keysys::TransferMode::Default => epp_proto::keysys::TransferMode::DefaultTransfer,
                client::keysys::TransferMode::AutoApprove => epp_proto::keysys::TransferMode::AutoApprove,
                client::keysys::TransferMode::AutoDeny => epp_proto::keysys::TransferMode::AutoDeny,
            }.into(),
            whois_banner: res.whois_banner,
            whois_rsp: res.whois_rsp,
            whois_url: res.whois_url,
            tld: res.tld.map(|t| match t {
                client::keysys::DomainInfoTLD::CA(t) => epp_proto::keysys::domain_info::Tld::Ca(epp_proto::keysys::DomainInfoCa {
                    trademark: t.trademark,
                    legal_type: match t.legal_type {
                        client::keysys::CALegalType::AboriginalPeoples => epp_proto::keysys::CaLegalType::AboriginalPeoples,
                        client::keysys::CALegalType::CanadianLibraryArchiveMuseum => epp_proto::keysys::CaLegalType::CanadianLibraryArchiveMuseum,
                        client::keysys::CALegalType::CanadianEducationalInstitution => epp_proto::keysys::CaLegalType::CanadianEducationalInstitution,
                        client::keysys::CALegalType::CanadianPoliticalParty => epp_proto::keysys::CaLegalType::CanadianPoliticalParty,
                        client::keysys::CALegalType::CanadianUnincorporatedAssociation => epp_proto::keysys::CaLegalType::CanadianUnincorporatedAssociation,
                        client::keysys::CALegalType::CanadianHospital => epp_proto::keysys::CaLegalType::CanadianHospital,
                        client::keysys::CALegalType::Corporation => epp_proto::keysys::CaLegalType::CanadianCorporation,
                        client::keysys::CALegalType::Government => epp_proto::keysys::CaLegalType::CanadianGovernment,
                        client::keysys::CALegalType::TradeUnion => epp_proto::keysys::CaLegalType::TradeUnion,
                        client::keysys::CALegalType::Trust => epp_proto::keysys::CaLegalType::Trust,
                        client::keysys::CALegalType::TradeMark => epp_proto::keysys::CaLegalType::TradeMark,
                        client::keysys::CALegalType::TheQueen => epp_proto::keysys::CaLegalType::TheQueen,
                        client::keysys::CALegalType::Partnership => epp_proto::keysys::CaLegalType::Partnership,
                        client::keysys::CALegalType::PermanentResident => epp_proto::keysys::CaLegalType::CanadianPermanentResident,
                        client::keysys::CALegalType::OfficialMark => epp_proto::keysys::CaLegalType::OfficialMark,
                        client::keysys::CALegalType::Citizen => epp_proto::keysys::CaLegalType::CanadianCitizen,
                        client::keysys::CALegalType::LegalRepOfCanadianCitizenOrPermanentResident => epp_proto::keysys::CaLegalType::LegalRepOfCanadianCitizenOrPermanentResident,
                        client::keysys::CALegalType::IndianBand => epp_proto::keysys::CaLegalType::IndianBand,
                    }.into()
                }),
                client::keysys::DomainInfoTLD::EU(t) => epp_proto::keysys::domain_info::Tld::Eu(epp_proto::keysys::DomainInfoEu {
                    trustee: t.accept_trustee_tac,
                    registrant_language: match t.registrant_lang {
                        None => epp_proto::keysys::EuLanguage::UnknownLanguage,
                        Some(client::keysys::EULanguage::English) => epp_proto::keysys::EuLanguage::English,
                        Some(client::keysys::EULanguage::Bulgarian) => epp_proto::keysys::EuLanguage::Bulgarian,
                        Some(client::keysys::EULanguage::Czech) => epp_proto::keysys::EuLanguage::Czech,
                        Some(client::keysys::EULanguage::Danish) => epp_proto::keysys::EuLanguage::Danish,
                        Some(client::keysys::EULanguage::German) => epp_proto::keysys::EuLanguage::German,
                        Some(client::keysys::EULanguage::ModernGreek) => epp_proto::keysys::EuLanguage::ModernGreek,
                        Some(client::keysys::EULanguage::Spanish) => epp_proto::keysys::EuLanguage::Spanish,
                        Some(client::keysys::EULanguage::Estonian) => epp_proto::keysys::EuLanguage::Estonian,
                        Some(client::keysys::EULanguage::Finnish) => epp_proto::keysys::EuLanguage::Finnish,
                        Some(client::keysys::EULanguage::French) => epp_proto::keysys::EuLanguage::French,
                        Some(client::keysys::EULanguage::Gaelic) => epp_proto::keysys::EuLanguage::Gaelic,
                        Some(client::keysys::EULanguage::Croatian) => epp_proto::keysys::EuLanguage::Croatian,
                        Some(client::keysys::EULanguage::Hungarian) => epp_proto::keysys::EuLanguage::Hungarian,
                        Some(client::keysys::EULanguage::Italian) => epp_proto::keysys::EuLanguage::Italian,
                        Some(client::keysys::EULanguage::Lithuanian) => epp_proto::keysys::EuLanguage::Lithuanian,
                        Some(client::keysys::EULanguage::Latvian) => epp_proto::keysys::EuLanguage::Latvian,
                        Some(client::keysys::EULanguage::Maltese) => epp_proto::keysys::EuLanguage::Maltese,
                        Some(client::keysys::EULanguage::DutchFlemish) => epp_proto::keysys::EuLanguage::DutchFlemish,
                        Some(client::keysys::EULanguage::Polish) => epp_proto::keysys::EuLanguage::Polish,
                        Some(client::keysys::EULanguage::Portuguese) => epp_proto::keysys::EuLanguage::Portuguese,
                        Some(client::keysys::EULanguage::Romanian) => epp_proto::keysys::EuLanguage::Romanian,
                        Some(client::keysys::EULanguage::Slovak) => epp_proto::keysys::EuLanguage::Slovak,
                        Some(client::keysys::EULanguage::Slovene) => epp_proto::keysys::EuLanguage::Slovene,
                        Some(client::keysys::EULanguage::Swedish) => epp_proto::keysys::EuLanguage::Swedish,
                    }.into(),
                    registrant_citizenship: match t.registrant_citizenship {
                        None => epp_proto::keysys::EuCountry::UnknownCountry,
                        Some(client::keysys::EUCountry::Austria) => epp_proto::keysys::EuCountry::Austria,
                        Some(client::keysys::EUCountry::Belgium) => epp_proto::keysys::EuCountry::Belgium,
                        Some(client::keysys::EUCountry::Bulgaria) => epp_proto::keysys::EuCountry::Bulgaria,
                        Some(client::keysys::EUCountry::Czech) => epp_proto::keysys::EuCountry::CzechRepublic,
                        Some(client::keysys::EUCountry::Cyprus) => epp_proto::keysys::EuCountry::Cyprus,
                        Some(client::keysys::EUCountry::Germany) => epp_proto::keysys::EuCountry::Germany,
                        Some(client::keysys::EUCountry::Denmark) => epp_proto::keysys::EuCountry::Denmark,
                        Some(client::keysys::EUCountry::Spain) => epp_proto::keysys::EuCountry::Spain,
                        Some(client::keysys::EUCountry::Estonia) => epp_proto::keysys::EuCountry::Estonia,
                        Some(client::keysys::EUCountry::Finland) => epp_proto::keysys::EuCountry::Finland,
                        Some(client::keysys::EUCountry::France) => epp_proto::keysys::EuCountry::France,
                        Some(client::keysys::EUCountry::Greece) => epp_proto::keysys::EuCountry::Greece,
                        Some(client::keysys::EUCountry::Hungary) => epp_proto::keysys::EuCountry::Hungary,
                        Some(client::keysys::EUCountry::Ireland) => epp_proto::keysys::EuCountry::Ireland,
                        Some(client::keysys::EUCountry::Italy) => epp_proto::keysys::EuCountry::Italy,
                        Some(client::keysys::EUCountry::Liechtenstein) => epp_proto::keysys::EuCountry::Liechtenstein,
                        Some(client::keysys::EUCountry::Lithuania) => epp_proto::keysys::EuCountry::Lithuania,
                        Some(client::keysys::EUCountry::Luxembourg) => epp_proto::keysys::EuCountry::Luxembourg,
                        Some(client::keysys::EUCountry::Latvia) => epp_proto::keysys::EuCountry::Latvia,
                        Some(client::keysys::EUCountry::Malta) => epp_proto::keysys::EuCountry::Malta,
                        Some(client::keysys::EUCountry::Netherlands) => epp_proto::keysys::EuCountry::Netherlands,
                        Some(client::keysys::EUCountry::Poland) => epp_proto::keysys::EuCountry::Poland,
                        Some(client::keysys::EUCountry::Portugal) => epp_proto::keysys::EuCountry::Portugal,
                        Some(client::keysys::EUCountry::Romania) => epp_proto::keysys::EuCountry::Romania,
                        Some(client::keysys::EUCountry::Sweden) => epp_proto::keysys::EuCountry::Sweden,
                        Some(client::keysys::EUCountry::Slovakia) => epp_proto::keysys::EuCountry::Slovakia,
                        Some(client::keysys::EUCountry::Slovenia) => epp_proto::keysys::EuCountry::Slovenia,
                        Some(client::keysys::EUCountry::Croatia) => epp_proto::keysys::EuCountry::Croatia,
                    }.into()
                }),
                client::keysys::DomainInfoTLD::DE(t) => epp_proto::keysys::domain_info::Tld::De(epp_proto::keysys::DomainInfoDe {
                    abuse_contact: t.abuse_contact,
                    general_contact: t.general_request,
                    trustee: match t.accept_trustee_tac {
                        client::keysys::DETrustee::None => epp_proto::keysys::DeTrustee::None,
                        client::keysys::DETrustee::Monthly => epp_proto::keysys::DeTrustee::Monthly,
                        client::keysys::DETrustee::Annually => epp_proto::keysys::DeTrustee::Annually,
                    }.into(),
                    holder_person: t.holder_person
                }),
                client::keysys::DomainInfoTLD::US(t) => epp_proto::keysys::domain_info::Tld::Us(epp_proto::keysys::DomainInfoUs {
                    category: match t.category {
                        client::keysys::USCategory::Citizen => epp_proto::keysys::UsCategory::UsCitizen,
                        client::keysys::USCategory::PermanentResident => epp_proto::keysys::UsCategory::UsPermanentResident,
                        client::keysys::USCategory::RegularActivity => epp_proto::keysys::UsCategory::RegularActivity,
                        client::keysys::USCategory::USOrganisation => epp_proto::keysys::UsCategory::UsOrganisation,
                        client::keysys::USCategory::OfficeOrFacility => epp_proto::keysys::UsCategory::OfficeOrFacility,
                    }.into(),
                    purpose: match t.purpose {
                        client::keysys::USPurpose::Personal => epp_proto::keysys::UsPurpose::Personal,
                        client::keysys::USPurpose::Business => epp_proto::keysys::UsPurpose::Business,
                        client::keysys::USPurpose::NonProfit => epp_proto::keysys::UsPurpose::NonProfit,
                        client::keysys::USPurpose::Educational => epp_proto::keysys::UsPurpose::Educational,
                        client::keysys::USPurpose::Government => epp_proto::keysys::UsPurpose::UsGovernment,
                    }.into(),
                    validator: t.validator
                }),
                client::keysys::DomainInfoTLD::RS(t) => epp_proto::keysys::domain_info::Tld::Rs(epp_proto::keysys::DomainInfoRs {
                    owner: t.owner.map(|o| match o {
                        client::keysys::RsId::IDCard(n) => epp_proto::keysys::domain_info_rs::Owner::OwnerIdCard(n),
                        client::keysys::RsId::CompanyNumber(n) => epp_proto::keysys::domain_info_rs::Owner::OwnerCompanyNumber(n)
                    }),
                    tech: t.tech.map(|o| match o {
                        client::keysys::RsId::IDCard(n) => epp_proto::keysys::domain_info_rs::Tech::TechIdCard(n),
                        client::keysys::RsId::CompanyNumber(n) => epp_proto::keysys::domain_info_rs::Tech::TechCompanyNumber(n)
                    }),
                    admin: t.admin.map(|o| match o {
                        client::keysys::RsId::IDCard(n) => epp_proto::keysys::domain_info_rs::Admin::AdminIdCard(n),
                        client::keysys::RsId::CompanyNumber(n) => epp_proto::keysys::domain_info_rs::Admin::AdminCompanyNumber(n)
                    }),
                }),
                client::keysys::DomainInfoTLD::FR(t) => epp_proto::keysys::domain_info::Tld::Fr(epp_proto::keysys::DomainInfoFr {
                    trustee: t.accept_trustee_tac
                }),
                client::keysys::DomainInfoTLD::Name(t) => epp_proto::keysys::domain_info::Tld::Name(epp_proto::keysys::DomainInfoName {
                    email_forward: t.email_forward
                })
            })
        }
    }
}
