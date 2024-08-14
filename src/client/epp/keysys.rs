impl From<&super::proto::keysys::ContactInfoData> for super::super::keysys::ContactInfo {
    fn from(from: &super::proto::keysys::ContactInfoData) -> Self {
        super::super::keysys::ContactInfo {
            validated: from.validated,
            verification_requested: from.verification_requested,
            verified: from.verified,
        }
    }
}

impl std::convert::TryFrom<&super::proto::keysys::DomainInfoData>
    for super::super::keysys::DomainInfo
{
    type Error = super::super::Error;

    fn try_from(from: &super::proto::keysys::DomainInfoData) -> Result<Self, Self::Error> {
        Ok(super::super::keysys::DomainInfo {
            renewal_date: from.renewal_date,
            paid_until_date: from.paid_until_date,
            roid: from.domain_roid.clone(),
            renewal_mode: match from.renewal_mode {
                super::proto::keysys::RenewalMode::Default => {
                    super::super::keysys::RenewalMode::Default
                }
                super::proto::keysys::RenewalMode::AutoRenew => {
                    super::super::keysys::RenewalMode::AutoRenew
                }
                super::proto::keysys::RenewalMode::AutoDelete => {
                    super::super::keysys::RenewalMode::AutoDelete
                }
                super::proto::keysys::RenewalMode::AutoExpire => {
                    super::super::keysys::RenewalMode::AutoExpire
                }
                super::proto::keysys::RenewalMode::AutoRenewQuarterly => {
                    super::super::keysys::RenewalMode::AutoRenewQuarterly
                }
                super::proto::keysys::RenewalMode::AutoRenewMonthly => {
                    super::super::keysys::RenewalMode::AutoRenewMonthly
                }
                super::proto::keysys::RenewalMode::ExpireAuction => {
                    super::super::keysys::RenewalMode::ExpireAuction
                }
                super::proto::keysys::RenewalMode::RenewOnce => {
                    super::super::keysys::RenewalMode::RenewOnce
                }
            },
            transfer_mode: match from.transfer_mode {
                super::proto::keysys::TransferMode::Default => {
                    super::super::keysys::TransferMode::Default
                }
                super::proto::keysys::TransferMode::AutoApprove => {
                    super::super::keysys::TransferMode::AutoApprove
                }
                super::proto::keysys::TransferMode::AutoDeny => {
                    super::super::keysys::TransferMode::AutoDeny
                }
            },
            whois_banner: match (&from.whois_banner_0, &from.whois_banner_1) {
                (Some(whois_banner_0), Some(whois_banner_1)) => {
                    vec![whois_banner_0.clone(), whois_banner_1.clone()]
                }
                (Some(whois_banner_0), None) => vec![whois_banner_0.clone()],
                (None, _) => vec![],
            },
            whois_rsp: from.whois_rsp.clone(),
            whois_url: from.whois_url.clone(),
            tld: if from.ca_legal_type.is_some() || from.ca_trademark {
                Some(super::super::keysys::DomainInfoTLD::CA(super::super::keysys::DomainCreateCA {
                    legal_type: match from.ca_legal_type {
                        Some(super::proto::keysys::CALegalType::Citizen) => super::super::keysys::CALegalType::Citizen,
                        Some(super::proto::keysys::CALegalType::Corporation) => super::super::keysys::CALegalType::Corporation,
                        Some(super::proto::keysys::CALegalType::Government) => super::super::keysys::CALegalType::Government,
                        Some(super::proto::keysys::CALegalType::CanadianUnincorporatedAssociation) => super::super::keysys::CALegalType::CanadianUnincorporatedAssociation,
                        Some(super::proto::keysys::CALegalType::CanadianEducationalInstitution) => super::super::keysys::CALegalType::CanadianEducationalInstitution,
                        Some(super::proto::keysys::CALegalType::PermanentResident) => super::super::keysys::CALegalType::PermanentResident,
                        Some(super::proto::keysys::CALegalType::CanadianLibraryArchiveMuseum) => super::super::keysys::CALegalType::CanadianLibraryArchiveMuseum,
                        Some(super::proto::keysys::CALegalType::AboriginalPeoples) => super::super::keysys::CALegalType::AboriginalPeoples,
                        Some(super::proto::keysys::CALegalType::CanadianHospital) => super::super::keysys::CALegalType::CanadianHospital,
                        Some(super::proto::keysys::CALegalType::IndianBand) => super::super::keysys::CALegalType::IndianBand,
                        Some(super::proto::keysys::CALegalType::LegalRepOfCanadianCitizenOrPermanentResident) => super::super::keysys::CALegalType::LegalRepOfCanadianCitizenOrPermanentResident,
                        Some(super::proto::keysys::CALegalType::CanadianPoliticalParty) => super::super::keysys::CALegalType::CanadianPoliticalParty,
                        Some(super::proto::keysys::CALegalType::OfficialMark) => super::super::keysys::CALegalType::OfficialMark,
                        Some(super::proto::keysys::CALegalType::Partnership) => super::super::keysys::CALegalType::Partnership,
                        Some(super::proto::keysys::CALegalType::TheQueen) => super::super::keysys::CALegalType::TheQueen,
                        Some(super::proto::keysys::CALegalType::TradeMark) => super::super::keysys::CALegalType::TradeMark,
                        Some(super::proto::keysys::CALegalType::TradeUnion) => super::super::keysys::CALegalType::TradeUnion,
                        Some(super::proto::keysys::CALegalType::Trust) => super::super::keysys::CALegalType::Trust,
                        None => return Err(super::super::Error::Err("CA legal type is not set".to_string())),
                    },
                    trademark: from.ca_trademark,
                }))
            } else if from.eu_accept_trustee_tac
                || from.eu_registrant_lang.is_some()
                || from.eu_registrant_citizenship.is_some()
            {
                Some(super::super::keysys::DomainInfoTLD::EU(
                    super::super::keysys::DomainCreateEU {
                        accept_trustee_tac: from.eu_accept_trustee_tac,
                        registrant_lang: from.eu_registrant_lang.as_ref().map(|t| match t {
                            super::proto::keysys::EULanguage::Bulgarian => {
                                super::super::keysys::EULanguage::Bulgarian
                            }
                            super::proto::keysys::EULanguage::Croatian => {
                                super::super::keysys::EULanguage::Croatian
                            }
                            super::proto::keysys::EULanguage::Czech => {
                                super::super::keysys::EULanguage::Czech
                            }
                            super::proto::keysys::EULanguage::Danish => {
                                super::super::keysys::EULanguage::Danish
                            }
                            super::proto::keysys::EULanguage::DutchFlemish => {
                                super::super::keysys::EULanguage::DutchFlemish
                            }
                            super::proto::keysys::EULanguage::English => {
                                super::super::keysys::EULanguage::English
                            }
                            super::proto::keysys::EULanguage::Estonian => {
                                super::super::keysys::EULanguage::Estonian
                            }
                            super::proto::keysys::EULanguage::Finnish => {
                                super::super::keysys::EULanguage::Finnish
                            }
                            super::proto::keysys::EULanguage::French => {
                                super::super::keysys::EULanguage::French
                            }
                            super::proto::keysys::EULanguage::German => {
                                super::super::keysys::EULanguage::German
                            }
                            super::proto::keysys::EULanguage::Hungarian => {
                                super::super::keysys::EULanguage::Hungarian
                            }
                            super::proto::keysys::EULanguage::Italian => {
                                super::super::keysys::EULanguage::Italian
                            }
                            super::proto::keysys::EULanguage::Latvian => {
                                super::super::keysys::EULanguage::Latvian
                            }
                            super::proto::keysys::EULanguage::Lithuanian => {
                                super::super::keysys::EULanguage::Lithuanian
                            }
                            super::proto::keysys::EULanguage::Polish => {
                                super::super::keysys::EULanguage::Polish
                            }
                            super::proto::keysys::EULanguage::Portuguese => {
                                super::super::keysys::EULanguage::Portuguese
                            }
                            super::proto::keysys::EULanguage::Romanian => {
                                super::super::keysys::EULanguage::Romanian
                            }
                            super::proto::keysys::EULanguage::Slovak => {
                                super::super::keysys::EULanguage::Slovak
                            }
                            super::proto::keysys::EULanguage::Spanish => {
                                super::super::keysys::EULanguage::Spanish
                            }
                            super::proto::keysys::EULanguage::Swedish => {
                                super::super::keysys::EULanguage::Swedish
                            }
                            super::proto::keysys::EULanguage::ModernGreek => {
                                super::super::keysys::EULanguage::ModernGreek
                            }
                            super::proto::keysys::EULanguage::Gaelic => {
                                super::super::keysys::EULanguage::Gaelic
                            }
                            super::proto::keysys::EULanguage::Maltese => {
                                super::super::keysys::EULanguage::Maltese
                            }
                            super::proto::keysys::EULanguage::Slovene => {
                                super::super::keysys::EULanguage::Slovene
                            }
                        }),
                        registrant_citizenship: from.eu_registrant_citizenship.as_ref().map(|t| {
                            match t {
                                super::proto::keysys::EUCountry::Austria => {
                                    super::super::keysys::EUCountry::Austria
                                }
                                super::proto::keysys::EUCountry::Belgium => {
                                    super::super::keysys::EUCountry::Belgium
                                }
                                super::proto::keysys::EUCountry::Bulgaria => {
                                    super::super::keysys::EUCountry::Bulgaria
                                }
                                super::proto::keysys::EUCountry::Croatia => {
                                    super::super::keysys::EUCountry::Croatia
                                }
                                super::proto::keysys::EUCountry::Cyprus => {
                                    super::super::keysys::EUCountry::Cyprus
                                }
                                super::proto::keysys::EUCountry::Czech => {
                                    super::super::keysys::EUCountry::Czech
                                }
                                super::proto::keysys::EUCountry::Denmark => {
                                    super::super::keysys::EUCountry::Denmark
                                }
                                super::proto::keysys::EUCountry::Estonia => {
                                    super::super::keysys::EUCountry::Estonia
                                }
                                super::proto::keysys::EUCountry::Finland => {
                                    super::super::keysys::EUCountry::Finland
                                }
                                super::proto::keysys::EUCountry::France => {
                                    super::super::keysys::EUCountry::France
                                }
                                super::proto::keysys::EUCountry::Germany => {
                                    super::super::keysys::EUCountry::Germany
                                }
                                super::proto::keysys::EUCountry::Greece => {
                                    super::super::keysys::EUCountry::Greece
                                }
                                super::proto::keysys::EUCountry::Hungary => {
                                    super::super::keysys::EUCountry::Hungary
                                }
                                super::proto::keysys::EUCountry::Ireland => {
                                    super::super::keysys::EUCountry::Ireland
                                }
                                super::proto::keysys::EUCountry::Italy => {
                                    super::super::keysys::EUCountry::Italy
                                }
                                super::proto::keysys::EUCountry::Latvia => {
                                    super::super::keysys::EUCountry::Latvia
                                }
                                super::proto::keysys::EUCountry::Lithuania => {
                                    super::super::keysys::EUCountry::Lithuania
                                }
                                super::proto::keysys::EUCountry::Luxembourg => {
                                    super::super::keysys::EUCountry::Luxembourg
                                }
                                super::proto::keysys::EUCountry::Malta => {
                                    super::super::keysys::EUCountry::Malta
                                }
                                super::proto::keysys::EUCountry::Netherlands => {
                                    super::super::keysys::EUCountry::Netherlands
                                }
                                super::proto::keysys::EUCountry::Poland => {
                                    super::super::keysys::EUCountry::Poland
                                }
                                super::proto::keysys::EUCountry::Portugal => {
                                    super::super::keysys::EUCountry::Portugal
                                }
                                super::proto::keysys::EUCountry::Romania => {
                                    super::super::keysys::EUCountry::Romania
                                }
                                super::proto::keysys::EUCountry::Slovakia => {
                                    super::super::keysys::EUCountry::Slovakia
                                }
                                super::proto::keysys::EUCountry::Spain => {
                                    super::super::keysys::EUCountry::Spain
                                }
                                super::proto::keysys::EUCountry::Sweden => {
                                    super::super::keysys::EUCountry::Sweden
                                }
                                super::proto::keysys::EUCountry::Liechtenstein => {
                                    super::super::keysys::EUCountry::Liechtenstein
                                }
                                super::proto::keysys::EUCountry::Slovenia => {
                                    super::super::keysys::EUCountry::Slovenia
                                }
                            }
                        }),
                    },
                ))
            } else if from.de_abuse_contact.is_some()
                || from.de_general_request.is_some()
                || from.de_accept_trustee_tac.is_some()
                || from.de_holder_person
            {
                Some(super::super::keysys::DomainInfoTLD::DE(
                    super::super::keysys::DomainCreateDE {
                        abuse_contact: from.de_abuse_contact.clone(),
                        general_request: from.de_general_request.clone(),
                        accept_trustee_tac: match from.de_accept_trustee_tac {
                            Some(super::proto::keysys::DETrustee::None) => {
                                super::super::keysys::DETrustee::None
                            }
                            Some(super::proto::keysys::DETrustee::Monthly) => {
                                super::super::keysys::DETrustee::Monthly
                            }
                            Some(super::proto::keysys::DETrustee::Annual) => {
                                super::super::keysys::DETrustee::Annually
                            }
                            None => super::super::keysys::DETrustee::None,
                        },
                        holder_person: from.de_holder_person,
                    },
                ))
            } else if from.fr_accept_trustee_tac {
                Some(super::super::keysys::DomainInfoTLD::FR(
                    super::super::keysys::DomainCreateFR {
                        accept_trustee_tac: from.fr_accept_trustee_tac,
                    },
                ))
            } else if from.name_emailforward.is_some() {
                Some(super::super::keysys::DomainInfoTLD::Name(
                    super::super::keysys::DomainName {
                        email_forward: from.name_emailforward.clone(),
                    },
                ))
            } else if from.rs_owner_idcard.is_some()
                || from.rs_owner_company_number.is_some()
                || from.rs_admin_idcard.is_some()
                || from.rs_admin_company_number.is_some()
                || from.rs_tech_idcard.is_some()
                || from.rs_tech_company_number.is_some()
            {
                Some(super::super::keysys::DomainInfoTLD::RS(
                    super::super::keysys::DomainUpdateRS {
                        owner: match (&from.rs_owner_idcard, &from.rs_owner_company_number) {
                            (Some(idcard), _) => {
                                Some(super::super::keysys::RsId::IDCard(idcard.clone()))
                            }
                            (_, Some(company_number)) => Some(
                                super::super::keysys::RsId::CompanyNumber(company_number.clone()),
                            ),
                            _ => None,
                        },
                        admin: match (&from.rs_admin_idcard, &from.rs_admin_company_number) {
                            (Some(idcard), _) => {
                                Some(super::super::keysys::RsId::IDCard(idcard.clone()))
                            }
                            (_, Some(company_number)) => Some(
                                super::super::keysys::RsId::CompanyNumber(company_number.clone()),
                            ),
                            _ => None,
                        },
                        tech: match (&from.rs_tech_idcard, &from.rs_tech_company_number) {
                            (Some(idcard), _) => {
                                Some(super::super::keysys::RsId::IDCard(idcard.clone()))
                            }
                            (_, Some(company_number)) => Some(
                                super::super::keysys::RsId::CompanyNumber(company_number.clone()),
                            ),
                            _ => None,
                        },
                    },
                ))
            } else if from.us_purpose.is_some()
                || from.us_category.is_some()
                || from.us_validator.is_some()
            {
                Some(super::super::keysys::DomainInfoTLD::US(
                    super::super::keysys::DomainCreateUS {
                        purpose: match from.us_purpose {
                            Some(super::proto::keysys::USPurpose::Business) => {
                                super::super::keysys::USPurpose::Business
                            }
                            Some(super::proto::keysys::USPurpose::Educational) => {
                                super::super::keysys::USPurpose::Educational
                            }
                            Some(super::proto::keysys::USPurpose::Government) => {
                                super::super::keysys::USPurpose::Government
                            }
                            Some(super::proto::keysys::USPurpose::NonProfit) => {
                                super::super::keysys::USPurpose::NonProfit
                            }
                            Some(super::proto::keysys::USPurpose::Personal) => {
                                super::super::keysys::USPurpose::Personal
                            }
                            None => {
                                return Err(super::super::Error::Err(
                                    "US purpose is missing".to_string(),
                                ))
                            }
                        },
                        category: match from.us_category {
                            Some(super::proto::keysys::USCategory::Citizen) => {
                                super::super::keysys::USCategory::Citizen
                            }
                            Some(super::proto::keysys::USCategory::PermanentResident) => {
                                super::super::keysys::USCategory::PermanentResident
                            }
                            Some(super::proto::keysys::USCategory::OfficeOrFacility) => {
                                super::super::keysys::USCategory::OfficeOrFacility
                            }
                            Some(super::proto::keysys::USCategory::RegularActivity) => {
                                super::super::keysys::USCategory::RegularActivity
                            }
                            Some(super::proto::keysys::USCategory::USOrganisation) => {
                                super::super::keysys::USCategory::USOrganisation
                            }
                            None => {
                                return Err(super::super::Error::Err(
                                    "US category is missing".to_string(),
                                ))
                            }
                        },
                        validator: from.us_validator.clone(),
                    },
                ))
            } else if from.tel_whois_type.is_some()
                || from.tel_publish_whois.is_some()
            {
                Some(super::super::keysys::DomainInfoTLD::Tel(
                    super::super::keysys::DomainCreateTel {
                        publish_whois: from.tel_publish_whois.unwrap_or_default(),
                        whois_type: match from.tel_whois_type {
                            Some(super::proto::keysys::TelWhoisType::NaturalPerson) => {
                                super::super::keysys::TelWhoisType::NaturalPerson
                            }
                            Some(super::proto::keysys::TelWhoisType::LegalPerson) => {
                                super::super::keysys::TelWhoisType::LegalPerson
                            }
                            None => {
                                return Err(super::super::Error::Err(
                                    "Tel WHOIS type is missing".to_string(),
                                ))
                            }
                        }
                    }
                ))
            } else {
                None
            },
        })
    }
}

#[cfg(test)]
mod domain_tests {
    #[test_log::test]
    fn info_check() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <domain:infData xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
        <domain:name>fedi.monster</domain:name>
        <domain:roid>19787813119534_DOMAIN-KEYSYS</domain:roid>
        <domain:status s="inactive"/>
        <domain:status s="ok"/>
        <domain:status s="serverTransferProhibited"/>
        <domain:registrant>P-AYI1850</domain:registrant>
        <domain:contact type="admin">P-AYI1850</domain:contact>
        <domain:contact type="tech">P-AYI1850</domain:contact>
        <domain:contact type="billing">P-AYI1850</domain:contact>
        <domain:clID>as207960</domain:clID>
        <domain:crID>as207960</domain:crID>
        <domain:crDate>2022-08-29T23:33:07.0Z</domain:crDate>
        <domain:upID>as207960</domain:upID>
        <domain:upDate>2022-08-29T23:33:07.0Z</domain:upDate>
        <domain:exDate>2023-08-29T23:59:59.0Z</domain:exDate>
        <domain:authInfo>
          <domain:pw>test</domain:pw>
        </domain:authInfo>
      </domain:infData>
    </resData>
    <extension>
      <keysys:resData xmlns:keysys="http://www.key-systems.net/epp/keysys-1.0">
        <keysys:infData>
          <keysys:renDate>2023-10-03T23:59:59.0Z</keysys:renDate>
          <keysys:punDate>2023-08-29T23:59:59.0Z</keysys:punDate>
          <keysys:domain-roid>D320175808-CNIC</keysys:domain-roid>
          <keysys:renewalmode>DEFAULT</keysys:renewalmode>
          <keysys:transferlock>0</keysys:transferlock>
          <keysys:transfermode>DEFAULT</keysys:transfermode>
          <keysys:whois-privacy>0</keysys:whois-privacy>
        </keysys:infData>
      </keysys:resData>
      <rgp:infData xmlns:rgp="urn:ietf:params:xml:ns:rgp-1.0">
        <rgp:rgpStatus s="addPeriod"/>
      </rgp:infData>
    </extension>
    <trID>
      <clTRID>0acb6c43-f631-4586-a012-523cd266ef91</clTRID>
      <svTRID>8103732a-d866-4db2-b2ba-47bb86f93d5b</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::super::domain::handle_info_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap();
        assert_eq!(data.name, "fedi.monster");
        assert_eq!(data.registry_id, "19787813119534_DOMAIN-KEYSYS");
        assert_eq!(data.registrant, "P-AYI1850");
        assert_eq!(data.client_id, "as207960");
        assert_eq!(data.client_created_id.unwrap(), "as207960");
        assert_eq!(data.last_updated_client.unwrap(), "as207960");
        assert_eq!(
            data.creation_date.unwrap(),
            "2022-08-29T23:33:07.0Z"
                .parse::<chrono::DateTime<chrono::Utc>>()
                .unwrap()
        );
        assert_eq!(
            data.last_updated_date.unwrap(),
            "2022-08-29T23:33:07.0Z"
                .parse::<chrono::DateTime<chrono::Utc>>()
                .unwrap()
        );
        assert_eq!(
            data.expiry_date.unwrap(),
            "2023-08-29T23:59:59.0Z"
                .parse::<chrono::DateTime<chrono::Utc>>()
                .unwrap()
        );

        let keysys = data.keysys.unwrap();
        assert_eq!(
            keysys.transfer_mode,
            super::super::super::keysys::TransferMode::Default
        );
        assert_eq!(
            keysys.renewal_mode,
            super::super::super::keysys::RenewalMode::Default
        );
        assert_eq!(
            keysys.renewal_date,
            "2023-10-03T23:59:59.0Z"
                .parse::<chrono::DateTime<chrono::Utc>>()
                .unwrap()
        );
        assert_eq!(
            keysys.paid_until_date,
            "2023-08-29T23:59:59.0Z"
                .parse::<chrono::DateTime<chrono::Utc>>()
                .unwrap()
        );
        assert_eq!(keysys.roid.unwrap(), "D320175808-CNIC");

        assert_eq!(data.rgp_state.len(), 1);
        assert_eq!(
            *data.rgp_state.get(0).unwrap(),
            super::super::super::rgp::RGPState::AddPeriod
        );
    }
}
