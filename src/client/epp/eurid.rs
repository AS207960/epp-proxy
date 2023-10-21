//! EPP commands relating to EURid extensions

use super::super::eurid::{
    is_entity_natural_person, ContactExtension, ContactExtensionUpdate, ContactType,
    DNSQualityRequest, DNSQualityResponse, DNSSECEligibilityRequest, DNSSECEligibilityResponse,
    DomainAuthInfo, DomainCheck, DomainCreate, DomainDelete, DomainInfo, DomainInfoRequest,
    DomainRenewInfo, DomainTransfer, DomainTransferInfo, DomainUpdate, HitPointsRequest,
    HitPointsResponse, Idn, PollResponse, RegistrationLimitRequest, RegistrationLimitResponse,
};
use super::super::{proto, Error, Response};
use super::router::HandleReqReturn;
use super::ServerFeatures;

pub fn eurid_ext_to_entity(
    from: &proto::eurid::EURIDContactInfo,
) -> super::super::contact::EntityType {
    match from.natural_person {
        true => super::super::contact::EntityType::OtherIndividual,
        false => super::super::contact::EntityType::Unknown,
    }
}

impl From<&proto::eurid::EURIDContactType> for ContactType {
    fn from(from: &proto::eurid::EURIDContactType) -> Self {
        use proto::eurid::EURIDContactType;
        match from {
            EURIDContactType::Billing => ContactType::Billing,
            EURIDContactType::Tech => ContactType::Tech,
            EURIDContactType::Registrant => ContactType::Registrant,
            EURIDContactType::OnSite => ContactType::OnSite,
            EURIDContactType::Reseller => ContactType::Reseller,
        }
    }
}

impl From<&proto::eurid::EURIDContactInfo> for ContactExtension {
    fn from(from: &proto::eurid::EURIDContactInfo) -> Self {
        ContactExtension {
            contact_type: (&from.contact_type).into(),
            whois_email: from.whois_email.as_deref().map(Into::into),
            vat: from.vat.as_deref().map(Into::into),
            citizenship_country: from.country_of_citizenship.as_deref().map(Into::into),
            language: (&from.language).into(),
            has_reserved_domain: from.has_reserved_domain,
            has_domain_on_hold: from.has_domain_on_hold,
            has_domain_registrar_lock: from.has_reserved_domain,
            has_domain_registry_lock: from.has_domain_registry_lock,
        }
    }
}

pub fn contact_info_from_extension(
    from: &ContactExtension,
    entity_type: &Option<super::super::contact::EntityType>,
) -> proto::eurid::EURIDContactInfo {
    use proto::eurid::EURIDContactType;
    proto::eurid::EURIDContactInfo {
        contact_type: match from.contact_type {
            ContactType::Billing => EURIDContactType::Billing,
            ContactType::Tech => EURIDContactType::Tech,
            ContactType::Registrant => EURIDContactType::Registrant,
            ContactType::OnSite => EURIDContactType::OnSite,
            ContactType::Reseller => EURIDContactType::Reseller,
        },
        whois_email: from.whois_email.as_deref().map(Into::into),
        vat: from.vat.as_deref().map(Into::into),
        language: (&from.language).into(),
        natural_person: is_entity_natural_person(entity_type.as_ref()),
        country_of_citizenship: from.citizenship_country.as_deref().map(Into::into),
        has_domain_registrar_lock: false,
        has_domain_registry_lock: false,
        has_domain_on_hold: false,
        has_reserved_domain: false,
    }
}

pub fn contact_info_update_from_extension(
    from: &Option<ContactExtensionUpdate>,
    entity_type: &Option<super::super::contact::EntityType>,
) -> proto::eurid::EURIDContactUpdate {
    proto::eurid::EURIDContactUpdate {
        change: proto::eurid::EURIDContactUpdateInfo {
            whois_email: from
                .as_ref()
                .and_then(|f| f.whois_email.as_deref().map(Into::into)),
            vat: from.as_ref().and_then(|f| f.vat.as_deref().map(Into::into)),
            language: from
                .as_ref()
                .and_then(|f| f.language.as_deref().map(Into::into)),
            natural_person: entity_type
                .as_ref()
                .map(|e| is_entity_natural_person(Some(e))),
            country_of_citizenship: from
                .as_ref()
                .and_then(|f| f.citizenship_country.as_deref().map(Into::into)),
        },
    }
}

impl From<proto::eurid::EURIDPollData> for PollResponse {
    fn from(from: proto::eurid::EURIDPollData) -> Self {
        PollResponse {
            context: from.context,
            object_type: from.object_type,
            object: from.object,
            object_unicode: from.object_unicode,
            action: from.action,
            code: from.code,
            detail: from.detail,
            registrar: from.registrar,
        }
    }
}

pub fn extract_eurid_idn(from: &Option<proto::EPPResponseExtension>) -> Option<Vec<Idn>> {
    let eurid_ext_idn = match from {
        Some(e) => match e
            .value
            .iter()
            .find(|e| matches!(e, proto::EPPResponseExtensionType::EURIDIDNMapping(_)))
        {
            Some(e) => match e {
                proto::EPPResponseExtensionType::EURIDIDNMapping(e) => Some(e),
                _ => unreachable!(),
            },
            None => None,
        },
        None => None,
    };

    eurid_ext_idn.map(|e| {
        e.names
            .iter()
            .map(|n| Idn {
                ace: (&n.ace).into(),
                unicode: (&n.unicode).into(),
            })
            .collect()
    })
}

pub fn extract_eurid_idn_singular<'o, O: Into<Option<&'o str>>>(
    from: &Option<proto::EPPResponseExtension>,
    orig_name: O,
) -> Result<Option<Idn>, Error> {
    match extract_eurid_idn(from) {
        Some(mut i) => match (i.len(), orig_name.into()) {
            (1, None) => Ok(Some(i.pop().unwrap())),
            (_, None) => Err(Error::ServerInternal),
            (_, Some(o)) => Ok(i.into_iter().find(|i| i.ace == o || i.unicode == o)),
        },
        None => Ok(None),
    }
}

pub fn extract_eurid_domain_check_singular(
    from: &Option<proto::EPPResponseExtension>,
) -> Result<Option<DomainCheck>, Error> {
    let eurid_ext_check = match from {
        Some(e) => match e
            .value
            .iter()
            .find(|e| matches!(e, proto::EPPResponseExtensionType::EURIDDomainCheckData(_)))
        {
            Some(e) => match e {
                proto::EPPResponseExtensionType::EURIDDomainCheckData(e) => Some(e),
                _ => unreachable!(),
            },
            None => None,
        },
        None => None,
    };

    match eurid_ext_check {
        Some(e) => {
            let mut d = e.domains.clone();
            match d.len() {
                1 => {
                    let c = d.pop().unwrap();
                    Ok(Some(DomainCheck {
                        available_date: c.available_date,
                        status: c.status.into_iter().map(|s| s.status.into()).collect(),
                    }))
                }
                _ => Err(Error::ServerInternal),
            }
        }
        None => Ok(None),
    }
}

impl From<&DomainCreate> for proto::eurid::EURIDDomainCreate {
    fn from(from: &DomainCreate) -> Self {
        let mut contacts = vec![];

        if let Some(on_site) = &from.on_site {
            contacts.push(proto::eurid::EURIDDomainContact {
                contact_type: proto::eurid::EURIDContactType::OnSite,
                contact_id: on_site.to_string(),
            });
        }

        if let Some(reseller) = &from.reseller {
            contacts.push(proto::eurid::EURIDDomainContact {
                contact_type: proto::eurid::EURIDContactType::Reseller,
                contact_id: reseller.to_string(),
            });
        }

        proto::eurid::EURIDDomainCreate {
            contacts,
            nsgroups: vec![],
            keygroup: None,
            registrar_reference: from.registrar_reference.as_ref().map(Into::into),
        }
    }
}

impl From<&DomainUpdate> for proto::eurid::EURIDDomainUpdate {
    fn from(from: &DomainUpdate) -> Self {
        let mut add_contacts = vec![];
        let mut rem_contacts = vec![];

        if let Some(on_site) = &from.add_on_site {
            add_contacts.push(proto::eurid::EURIDDomainContact {
                contact_type: proto::eurid::EURIDContactType::OnSite,
                contact_id: on_site.to_string(),
            });
        }
        if let Some(on_site) = &from.remove_on_site {
            rem_contacts.push(proto::eurid::EURIDDomainContact {
                contact_type: proto::eurid::EURIDContactType::OnSite,
                contact_id: on_site.to_string(),
            });
        }

        if let Some(reseller) = &from.add_reseller {
            add_contacts.push(proto::eurid::EURIDDomainContact {
                contact_type: proto::eurid::EURIDContactType::Reseller,
                contact_id: reseller.to_string(),
            });
        }
        if let Some(reseller) = &from.remove_reseller {
            rem_contacts.push(proto::eurid::EURIDDomainContact {
                contact_type: proto::eurid::EURIDContactType::Reseller,
                contact_id: reseller.to_string(),
            });
        }

        proto::eurid::EURIDDomainUpdate {
            add: match add_contacts.is_empty() {
                true => None,
                false => Some(proto::eurid::EURIDDomainUpdateAddRemove {
                    contacts: add_contacts,
                    nsgroups: vec![],
                    keygroup: None,
                }),
            },
            remove: match rem_contacts.is_empty() {
                true => None,
                false => Some(proto::eurid::EURIDDomainUpdateAddRemove {
                    contacts: rem_contacts,
                    nsgroups: vec![],
                    keygroup: None,
                }),
            },
            change: if from.registrar_reference.is_some() {
                Some(proto::eurid::EURIDDomainUpdateChange {
                    registrar_reference: from.registrar_reference.as_ref().map(Into::into),
                })
            } else {
                None
            },
        }
    }
}

impl From<&DomainDelete> for proto::eurid::EURIDDomainDelete {
    fn from(from: &DomainDelete) -> Self {
        match from {
            DomainDelete::Schedule(t) => {
                proto::eurid::EURIDDomainDelete::Schedule(proto::eurid::EURIDDomainDeleteSchedule {
                    delete_date: t.to_owned(),
                })
            }
            DomainDelete::Cancel => proto::eurid::EURIDDomainDelete::Cancel {},
        }
    }
}

impl From<&DomainTransfer> for proto::eurid::EURIDDomainTransfer {
    fn from(from: &DomainTransfer) -> Self {
        let mut contacts = vec![proto::eurid::EURIDDomainContact {
            contact_type: proto::eurid::EURIDContactType::Registrant,
            contact_id: from.registrant.clone(),
        }];

        if let Some(billing) = &from.billing {
            contacts.push(proto::eurid::EURIDDomainContact {
                contact_type: proto::eurid::EURIDContactType::Billing,
                contact_id: billing.clone(),
            });
        }

        if let Some(on_site) = &from.on_site {
            contacts.push(proto::eurid::EURIDDomainContact {
                contact_type: proto::eurid::EURIDContactType::OnSite,
                contact_id: on_site.clone(),
            });
        }

        if let Some(technical) = &from.technical {
            contacts.push(proto::eurid::EURIDDomainContact {
                contact_type: proto::eurid::EURIDContactType::Tech,
                contact_id: technical.clone(),
            });
        }

        if let Some(reseller) = &from.reseller {
            contacts.push(proto::eurid::EURIDDomainContact {
                contact_type: proto::eurid::EURIDContactType::Reseller,
                contact_id: reseller.clone(),
            });
        }

        proto::eurid::EURIDDomainTransfer {
            transfer_request: Some(proto::eurid::EURIDDomainTransferRequest {
                registrant: from.registrant.clone(),
                contacts,
                nameservers: None,
                nsgroups: vec![],
                keygroup: None,
                registrar_reference: from.registrar_reference.as_ref().map(Into::into),
            }),
        }
    }
}

impl From<&DomainInfoRequest> for Option<proto::eurid::EURIDAuthInfo> {
    fn from(from: &DomainInfoRequest) -> Option<proto::eurid::EURIDAuthInfo> {
        from.auth_info.as_ref().map(|a| match a {
            DomainAuthInfo::Request => proto::eurid::EURIDAuthInfo::Request {},
            DomainAuthInfo::Cancel => proto::eurid::EURIDAuthInfo::Cancel {},
        })
    }
}

pub fn extract_eurid_domain_info(from: &Option<proto::EPPResponseExtension>) -> Option<DomainInfo> {
    let eurid_ext_info = match from {
        Some(e) => e.value.iter().find_map(|e| match e {
            proto::EPPResponseExtensionType::EURIDDomainInfoData(e) => Some(e),
            _ => None,
        }),
        None => None,
    };

    let eurid_auth_info = match from {
        Some(e) => e.value.iter().find_map(|e| match e {
            proto::EPPResponseExtensionType::EURIDAuthInfoData(e) => Some(e),
            _ => None,
        }),
        None => None,
    };

    eurid_ext_info.map(|e| DomainInfo {
        on_hold: e.on_hold,
        reserved: e.reserved,
        quarantined: e.quarantined,
        suspended: e.suspended,
        delayed: e.delayed,
        seized: e.seized,
        deletion_date: e.deletion_date,
        max_extension_period: e.max_extension_period,
        registrant_country: e.registrant_country.to_string(),
        registrant_country_of_citizenship: e
            .registrant_country_of_citizenship
            .as_deref()
            .map(Into::into),
        on_site: e.contacts.iter().find_map(|c| match c.contact_type {
            proto::eurid::EURIDContactType::OnSite => Some(c.contact_id.to_owned()),
            _ => None,
        }),
        reseller: e.contacts.iter().find_map(|c| match c.contact_type {
            proto::eurid::EURIDContactType::Reseller => Some(c.contact_id.to_string()),
            _ => None,
        }),
        auth_info_valid_until: eurid_auth_info.map(|a| a.valid_until),
        registrar_reference: e.registrar_reference.as_ref().map(Into::into),
    })
}

pub fn extract_eurid_domain_transfer_info(
    from: &Option<proto::EPPResponseExtension>,
) -> Option<DomainTransferInfo> {
    let eurid_ext_info = match from {
        Some(e) => e.value.iter().find_map(|e| match e {
            proto::EPPResponseExtensionType::EURIDDomainTransferData(e) => Some(e),
            _ => None,
        }),
        None => None,
    };

    eurid_ext_info.map(|e| DomainTransferInfo {
        on_hold: e.on_hold,
        reserved: e.reserved,
        quarantined: e.quarantined,
        delayed: e.delayed,
        on_site: e.contacts.iter().find_map(|c| match c.contact_type {
            proto::eurid::EURIDContactType::OnSite => Some(c.contact_id.to_owned()),
            _ => None,
        }),
        reseller: e.contacts.iter().find_map(|c| match c.contact_type {
            proto::eurid::EURIDContactType::Reseller => Some(c.contact_id.to_string()),
            _ => None,
        }),
        billing: e
            .contacts
            .iter()
            .find_map(|c| match c.contact_type {
                proto::eurid::EURIDContactType::Billing => Some(c.contact_id.to_string()),
                _ => None,
            })
            .unwrap_or_default(),
        technical: e.contacts.iter().find_map(|c| match c.contact_type {
            proto::eurid::EURIDContactType::Tech => Some(c.contact_id.to_string()),
            _ => None,
        }),
        registrant: e.registrant.to_string(),
        reason: e.reason.to_string(),
        registrar_reference: e.registrar_reference.as_ref().map(Into::into),
    })
}

pub fn extract_eurid_domain_renew_info(
    from: &Option<proto::EPPResponseExtension>,
) -> Option<DomainRenewInfo> {
    let eurid_ext_info = match from {
        Some(e) => e.value.iter().find_map(|e| match e {
            proto::EPPResponseExtensionType::EURIDDomainRenewData(e) => Some(e),
            _ => None,
        }),
        None => None,
    };

    eurid_ext_info.map(|e| DomainRenewInfo {
        removed_deletion: e
            .data
            .contains(&proto::eurid::EURIDDomainRenewDataType::RemovedDeletionDate),
    })
}

pub fn handle_hit_points(
    client: &ServerFeatures,
    _req: &HitPointsRequest,
) -> HandleReqReturn<HitPointsResponse> {
    if client.eurid_hit_points_supported {
        Ok((
            proto::EPPCommandType::Info(proto::EPPInfo::EURIDRegistrarHitPoints {}),
            None,
        ))
    } else {
        Err(Err(Error::Unsupported))
    }
}

pub fn handle_hit_points_response(
    response: proto::EPPResponse, _metrics: &crate::metrics::ScopedMetrics
) -> Response<HitPointsResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EURIDRegistrarHitPointsData(hit_points) => {
                Response::Ok(HitPointsResponse {
                    hit_points: hit_points.hit_points,
                    max_hit_points: hit_points.max_hit_points,
                    blocked_until: hit_points.blocked_until,
                })
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_registration_limits(
    client: &ServerFeatures,
    _req: &RegistrationLimitRequest,
) -> HandleReqReturn<RegistrationLimitResponse> {
    if client.eurid_registration_limit_supported {
        Ok((
            proto::EPPCommandType::Info(proto::EPPInfo::EURIDRegistrationLimit {}),
            None,
        ))
    } else {
        Err(Err(Error::Unsupported))
    }
}

pub fn handle_registration_limits_response(
    response: proto::EPPResponse, _metrics: &crate::metrics::ScopedMetrics
) -> Response<RegistrationLimitResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EURIDRegistrationLimitData(registration_limit) => {
                Response::Ok(RegistrationLimitResponse {
                    monthly_registrations: registration_limit.monthly_registrations,
                    max_monthly_registrations: registration_limit.max_monthly_registrations,
                    limited_until: registration_limit.limited_until,
                })
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_dnssec_eligibility(
    client: &ServerFeatures,
    req: &DNSSECEligibilityRequest,
) -> HandleReqReturn<DNSSECEligibilityResponse> {
    if client.eurid_dnssec_eligibility_support {
        Ok((
            proto::EPPCommandType::Info(proto::EPPInfo::EURIDDNSSECEligibilityInfo(
                proto::eurid::EURIDDNSSECEligibilityInfo {
                    name: req.name.to_string(),
                },
            )),
            None,
        ))
    } else {
        Err(Err(Error::Unsupported))
    }
}

pub fn handle_dnssec_eligibility_response(
    response: proto::EPPResponse, _metrics: &crate::metrics::ScopedMetrics
) -> Response<DNSSECEligibilityResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EURIDDNSSECEligibilityInfoData(dnssec_eligibility) => {
                Response::Ok(DNSSECEligibilityResponse {
                    eligible: dnssec_eligibility.eligible,
                    message: dnssec_eligibility.msg,
                    code: dnssec_eligibility.code,
                    idn: extract_eurid_idn_singular(
                        &response.extension,
                        dnssec_eligibility.name.as_str(),
                    )?,
                })
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_dns_quality(
    client: &ServerFeatures,
    req: &DNSQualityRequest,
) -> HandleReqReturn<DNSQualityResponse> {
    if client.eurid_dns_quality_support {
        Ok((
            proto::EPPCommandType::Info(proto::EPPInfo::EURIDDNSQuality(
                proto::eurid::EURIDDNSQualityInfo {
                    name: req.name.to_string(),
                },
            )),
            None,
        ))
    } else {
        Err(Err(Error::Unsupported))
    }
}

pub fn handle_dns_quality_response(
    response: proto::EPPResponse, _metrics: &crate::metrics::ScopedMetrics
) -> Response<DNSQualityResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EURIDDNSQualityData(dns_quality) => {
                Response::Ok(DNSQualityResponse {
                    check_time: dns_quality.check_time,
                    score: dns_quality.score,
                    idn: extract_eurid_idn_singular(
                        &response.extension,
                        dns_quality.name.as_str(),
                    )?,
                })
            }
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

#[cfg(test)]
mod eurid_tests {
    #[test]
    fn hit_points_0() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:registrarHitPoints="http://www.eurid.eu/xml/epp/registrarHitPoints-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <registrarHitPoints:infData>
        <registrarHitPoints:nbrHitPoints>0</registrarHitPoints:nbrHitPoints>
        <registrarHitPoints:maxNbrHitPoints>2000</registrarHitPoints:maxNbrHitPoints>
      </registrarHitPoints:infData>
    </resData>
    <trID>
      <clTRID>registrar-info-hitpoints-01</clTRID>
      <svTRID>e8b374106-8458-4909-8fc0-d9c698837595</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_hit_points_response(*res).unwrap();
        assert_eq!(data.hit_points, 0);
        assert_eq!(data.max_hit_points, 2000);
        assert!(data.blocked_until.is_none());
    }

    #[test]
    fn hit_points_1() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:registrarHitPoints="http://www.eurid.eu/xml/epp/registrarHitPoints-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <registrarHitPoints:infData>
        <registrarHitPoints:nbrHitPoints>6</registrarHitPoints:nbrHitPoints>
        <registrarHitPoints:maxNbrHitPoints>5</registrarHitPoints:maxNbrHitPoints>
        <registrarHitPoints:blockedUntil>2019-11-30T22:59:59.999Z</registrarHitPoints:blockedUntil>
      </registrarHitPoints:infData>
    </resData>
    <trID>
      <clTRID>registrar-info-hitpoints-02</clTRID>
      <svTRID>eeac2d5bb-caf0-4e50-9c60-3cce0cd134d0</svTRID>
    </trID>
  </response>
</epp>
"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_hit_points_response(*res).unwrap();
        assert_eq!(data.hit_points, 6);
        assert_eq!(data.max_hit_points, 5);
        assert!(data.blocked_until.is_some());
    }

    #[test]
    fn registration_limit_0() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:registrationLimit="http://www.eurid.eu/xml/epp/registrationLimit-1.1">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <registrationLimit:infData>
        <registrationLimit:monthlyRegistrations>0</registrationLimit:monthlyRegistrations>
        <registrationLimit:maxMonthlyRegistrations>1000</registrationLimit:maxMonthlyRegistrations>
      </registrationLimit:infData>
    </resData>
    <trID>
      <clTRID>registrationLimits-info03</clTRID>
      <svTRID>e87cf3433-f98b-43f8-8385-8e34ffabd091</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_registration_limits_response(*res).unwrap();
        assert_eq!(data.monthly_registrations, 0);
        assert_eq!(data.max_monthly_registrations.unwrap(), 1000);
        assert!(data.limited_until.is_none());
    }

    #[test]
    fn registration_limit_1() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:registrationLimit="http://www.eurid.eu/xml/epp/registrationLimit-1.1">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <registrationLimit:infData>
        <registrationLimit:monthlyRegistrations>0</registrationLimit:monthlyRegistrations>
      </registrationLimit:infData>
    </resData>
    <trID>
      <clTRID>registrationLimits-info03</clTRID>
      <svTRID>e037713b8-8e41-4507-ae46-7d5881da3e0c</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_registration_limits_response(*res).unwrap();
        assert_eq!(data.monthly_registrations, 0);
        assert!(data.max_monthly_registrations.is_none());
        assert!(data.limited_until.is_none());
    }

    #[test]
    fn registration_limit_2() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:registrationLimit="http://www.eurid.eu/xml/epp/registrationLimit-1.1">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <registrationLimit:infData>
        <registrationLimit:monthlyRegistrations>1</registrationLimit:monthlyRegistrations>
        <registrationLimit:maxMonthlyRegistrations>1</registrationLimit:maxMonthlyRegistrations>
        <registrationLimit:limitedUntil>2019-11-30T22:59:59.999Z</registrationLimit:limitedUntil>
      </registrationLimit:infData>
    </resData>
    <trID>
      <clTRID>registrationLimits-info03</clTRID>
      <svTRID>e88c70c35-226e-4f42-8c9e-56a8f4f725f5</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_registration_limits_response(*res).unwrap();
        assert_eq!(data.monthly_registrations, 1);
        assert_eq!(data.max_monthly_registrations.unwrap(), 1);
        assert!(data.limited_until.is_some());
    }

    #[test]
    fn dnssec_eligibility_0() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:dnssecEligibility="http://www.eurid.eu/xml/epp/dnssecEligibility-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <dnssecEligibility:infData>
        <dnssecEligibility:name>somedomain.eu</dnssecEligibility:name>
        <dnssecEligibility:eligible>true</dnssecEligibility:eligible>
        <dnssecEligibility:msg>Eligible for DNSSEC discount</dnssecEligibility:msg>
        <dnssecEligibility:code>1001</dnssecEligibility:code>
      </dnssecEligibility:infData>
    </resData>
    <trID>
      <clTRID>dnssecEligibility11-info</clTRID>
      <svTRID>e212b738c-f55d-40ec-a736-d49aca0898a9</svTRID>
    </trID>
  </response>
</epp>
"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_dnssec_eligibility_response(*res).unwrap();
        assert!(data.eligible);
        assert_eq!(data.message, "Eligible for DNSSEC discount");
        assert_eq!(data.code, 1001);
        assert!(data.idn.is_none());
    }

    #[test]
    fn dnssec_eligibility_1() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:dnssecEligibility="http://www.eurid.eu/xml/epp/dnssecEligibility-1.0" xmlns:idn="http://www.eurid.eu/xml/epp/idn-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <dnssecEligibility:infData>
        <dnssecEligibility:name>αβααβα-1522153897567.eu</dnssecEligibility:name>
        <dnssecEligibility:eligible>false</dnssecEligibility:eligible>
        <dnssecEligibility:msg>Not eligible for DNSSEC discount</dnssecEligibility:msg>
        <dnssecEligibility:code>2000</dnssecEligibility:code>
      </dnssecEligibility:infData>
    </resData>
    <extension>
      <idn:mapping>
        <idn:name>
          <idn:ace>xn---1522153897567-f9jaaaqc.eu</idn:ace>
          <idn:unicode>αβααβα-1522153897567.eu</idn:unicode>
        </idn:name>
      </idn:mapping>
    </extension>
    <trID>
      <clTRID>dnssecEligibility11-info</clTRID>
      <svTRID>e93bbf49c-4ec2-4dfc-bafd-abc9a5897ee5</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_dnssec_eligibility_response(*res).unwrap();
        assert!(!data.eligible);
        assert_eq!(data.message, "Not eligible for DNSSEC discount");
        assert_eq!(data.code, 2000);
        let idn = data.idn.unwrap();
        assert_eq!(idn.ace, "xn---1522153897567-f9jaaaqc.eu");
        assert_eq!(idn.unicode, "αβααβα-1522153897567.eu");
    }

    #[test]
    fn dns_quality() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:dnsQuality="http://www.eurid.eu/xml/epp/dnsQuality-2.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <dnsQuality:infData>
        <dnsQuality:name>somedomain.eu</dnsQuality:name>
        <dnsQuality:checkTime>2017-08-17T11:23:44.312+02:00</dnsQuality:checkTime>
        <dnsQuality:score>10000</dnsQuality:score>
      </dnsQuality:infData>
    </resData>
    <trID>
      <clTRID>dnsQuality-info01</clTRID>
      <svTRID>e92ec1b29-58f3-4e24-9b0f-6a3f0027ef07</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_dns_quality_response(*res).unwrap();
        assert!(data.check_time.is_some());
        assert_eq!(data.score, "10000");
        assert!(data.idn.is_none());
    }

    #[test]
    fn contact_info_0() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:contact-ext="http://www.eurid.eu/xml/epp/contact-ext-1.4" xmlns:contact="urn:ietf:params:xml:ns:contact-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <contact:infData>
        <contact:id>c57</contact:id>
        <contact:roid>c57-EURID</contact:roid>
        <contact:status s="ok"/>
        <contact:postalInfo type="loc">
          <contact:name>Teki-Sue Porter</contact:name>
          <contact:org>Tech Support Unlimited</contact:org>
          <contact:addr>
            <contact:street>Main Street 122</contact:street>
            <contact:city>Nowhere City</contact:city>
            <contact:pc>1234</contact:pc>
            <contact:cc>BE</contact:cc>
          </contact:addr>
        </contact:postalInfo>
        <contact:voice>+32.123456789</contact:voice>
        <contact:fax>+32.123456790</contact:fax>
        <contact:email>nobody@example.eu</contact:email>
        <contact:clID>t000001</contact:clID>
        <contact:crID>t000001</contact:crID>
        <contact:crDate>2019-11-06T12:14:18.156Z</contact:crDate>
        <contact:upDate>2019-11-06T12:14:18.000Z</contact:upDate>
      </contact:infData>
    </resData>
    <extension>
      <contact-ext:infData>
        <contact-ext:type>tech</contact-ext:type>
        <contact-ext:lang>en</contact-ext:lang>
        <contact-ext:naturalPerson>false</contact-ext:naturalPerson>
      </contact-ext:infData>
    </extension>
    <trID>
      <clTRID>contact-info01</clTRID>
      <svTRID>e544e5970-fefc-436e-ae3e-99b7d17c717d</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::super::contact::handle_info_response(*res).unwrap();
        let eurid_extension = data.eurid_contact_extension.unwrap();
        assert_eq!(
            data.entity_type,
            super::super::super::contact::EntityType::Unknown
        );
        assert_eq!(eurid_extension.contact_type, super::ContactType::Tech);
        assert_eq!(eurid_extension.language, "en");
        assert!(eurid_extension.citizenship_country.is_none());
        assert!(eurid_extension.whois_email.is_none());
        assert!(eurid_extension.vat.is_none());
    }

    #[test]
    fn contact_info_1() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:contact-ext="http://www.eurid.eu/xml/epp/contact-ext-1.4" xmlns:contact="urn:ietf:params:xml:ns:contact-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <contact:infData>
        <contact:id>c59</contact:id>
        <contact:roid>c59-EURID</contact:roid>
        <contact:status s="ok"/>
        <contact:postalInfo type="loc">
          <contact:name>Ann Onimous</contact:name>
          <contact:addr>
            <contact:street>Main Street 122</contact:street>
            <contact:city>Spiff City</contact:city>
            <contact:sp>Far Faraway County</contact:sp>
            <contact:pc>1234</contact:pc>
            <contact:cc>BE</contact:cc>
          </contact:addr>
        </contact:postalInfo>
        <contact:voice>+32.12345678911</contact:voice>
        <contact:email>nobody@example.com</contact:email>
        <contact:clID>t000001</contact:clID>
        <contact:crID>t000001</contact:crID>
        <contact:crDate>2019-11-06T12:14:22.299Z</contact:crDate>
        <contact:upDate>2019-11-06T12:14:22.000Z</contact:upDate>
      </contact:infData>
    </resData>
    <extension>
      <contact-ext:infData>
        <contact-ext:type>registrant</contact-ext:type>
        <contact-ext:lang>en</contact-ext:lang>
        <contact-ext:naturalPerson>true</contact-ext:naturalPerson>
      </contact-ext:infData>
    </extension>
    <trID>
      <clTRID>contact-info02</clTRID>
      <svTRID>e5ebf459d-f74d-4a56-a30b-1ff62d4d4040</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::super::contact::handle_info_response(*res).unwrap();
        let eurid_extension = data.eurid_contact_extension.unwrap();
        assert_eq!(
            data.entity_type,
            super::super::super::contact::EntityType::OtherIndividual
        );
        assert_eq!(eurid_extension.contact_type, super::ContactType::Registrant);
        assert_eq!(eurid_extension.language, "en");
        assert!(eurid_extension.citizenship_country.is_none());
        assert!(eurid_extension.whois_email.is_none());
        assert!(eurid_extension.vat.is_none());
    }

    #[test]
    fn contact_info_2() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:contact-ext="http://www.eurid.eu/xml/epp/contact-ext-1.4" xmlns:contact="urn:ietf:params:xml:ns:contact-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <contact:infData>
        <contact:id>c61</contact:id>
        <contact:roid>c61-EURID</contact:roid>
        <contact:status s="ok"/>
        <contact:postalInfo type="loc">
          <contact:name>Ann Ployee</contact:name>
          <contact:org>ACME Intercontinental</contact:org>
          <contact:addr>
            <contact:street>Main Street 122</contact:street>
            <contact:street>Building 5</contact:street>
            <contact:street>P.O. Box 123</contact:street>
            <contact:city>Nowhere City</contact:city>
            <contact:pc>1234</contact:pc>
            <contact:cc>BE</contact:cc>
          </contact:addr>
        </contact:postalInfo>
        <contact:voice>+32.123456789</contact:voice>
        <contact:fax>+32.123456790</contact:fax>
        <contact:email>nobody@example.com</contact:email>
        <contact:clID>t000001</contact:clID>
        <contact:crID>t000001</contact:crID>
        <contact:crDate>2019-11-06T12:14:26.200Z</contact:crDate>
        <contact:upDate>2019-11-06T12:14:26.000Z</contact:upDate>
      </contact:infData>
    </resData>
    <extension>
      <contact-ext:infData>
        <contact-ext:type>registrant</contact-ext:type>
        <contact-ext:vat>VAT1234567890</contact-ext:vat>
        <contact-ext:lang>en</contact-ext:lang>
        <contact-ext:naturalPerson>false</contact-ext:naturalPerson>
      </contact-ext:infData>
    </extension>
    <trID>
      <clTRID>contact-info03</clTRID>
      <svTRID>e25c814ba-7992-4eac-aaa7-b3dd89487ed6</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::super::contact::handle_info_response(*res).unwrap();
        let eurid_extension = data.eurid_contact_extension.unwrap();
        assert_eq!(
            data.entity_type,
            super::super::super::contact::EntityType::Unknown
        );
        assert_eq!(eurid_extension.contact_type, super::ContactType::Registrant);
        assert_eq!(eurid_extension.language, "en");
        assert!(eurid_extension.citizenship_country.is_none());
        assert!(eurid_extension.whois_email.is_none());
        assert_eq!(eurid_extension.vat.unwrap(), "VAT1234567890");
    }

    #[test]
    fn contact_info_3() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:contact-ext="http://www.eurid.eu/xml/epp/contact-ext-1.4" xmlns:contact="urn:ietf:params:xml:ns:contact-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <contact:infData>
        <contact:id>c63</contact:id>
        <contact:roid>c63-EURID</contact:roid>
        <contact:status s="ok"/>
        <contact:postalInfo type="loc">
          <contact:name>Ree Seller</contact:name>
          <contact:addr>
            <contact:street>Some Street 123</contact:street>
            <contact:city>Nowhere City</contact:city>
            <contact:sp>Some State 1372146059</contact:sp>
            <contact:pc>1234</contact:pc>
            <contact:cc>BE</contact:cc>
          </contact:addr>
        </contact:postalInfo>
        <contact:voice>+32.123456789</contact:voice>
        <contact:email>reseller@some-domain.eu</contact:email>
        <contact:clID>t000001</contact:clID>
        <contact:crID>t000001</contact:crID>
        <contact:crDate>2019-11-06T12:14:30.109Z</contact:crDate>
        <contact:upDate>2019-11-06T12:14:30.000Z</contact:upDate>
      </contact:infData>
    </resData>
    <extension>
      <contact-ext:infData>
        <contact-ext:type>reseller</contact-ext:type>
        <contact-ext:lang>en</contact-ext:lang>
        <contact-ext:naturalPerson>true</contact-ext:naturalPerson>
      </contact-ext:infData>
    </extension>
    <trID>
      <clTRID>contact-info04</clTRID>
      <svTRID>e77f03111-3ca1-4fab-ad7d-98de29f4d1b7</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::super::contact::handle_info_response(*res).unwrap();
        let eurid_extension = data.eurid_contact_extension.unwrap();
        assert_eq!(
            data.entity_type,
            super::super::super::contact::EntityType::OtherIndividual
        );
        assert_eq!(eurid_extension.contact_type, super::ContactType::Reseller);
        assert_eq!(eurid_extension.language, "en");
        assert!(eurid_extension.citizenship_country.is_none());
        assert!(eurid_extension.whois_email.is_none());
        assert!(eurid_extension.vat.is_none());
    }

    #[test]
    fn domain_check_0() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:domain-ext="http://www.eurid.eu/xml/epp/domain-ext-2.5" xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <domain:chkData>
        <domain:cd>
          <domain:name avail="false">europa.eu</domain:name>
          <domain:reason lang="en">registered</domain:reason>
        </domain:cd>
      </domain:chkData>
    </resData>
    <extension>
      <domain-ext:chkData>
        <domain-ext:domain>
          <domain-ext:name>europa.eu</domain-ext:name>
          <domain-ext:status s="serverTransferProhibited"/>
        </domain-ext:domain>
      </domain-ext:chkData>
    </extension>
    <trID>
      <clTRID>domain-check02</clTRID>
      <svTRID>ed74a5e3b-4dec-4831-ae37-c74187429d27</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::super::domain::handle_check_response(*res).unwrap();
        let eurid_extension = data.eurid_check.unwrap();
        assert!(!data.avail);
        assert_eq!(data.reason.unwrap(), "registered");
        assert!(eurid_extension.available_date.is_none());
        assert_eq!(eurid_extension.status.len(), 1);
        assert_eq!(
            eurid_extension.status[0],
            super::super::super::domain::Status::ServerTransferProhibited
        );
    }

    #[test]
    fn domain_check_1() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:idn="http://www.eurid.eu/xml/epp/idn-1.0" xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <domain:chkData>
        <domain:cd>
          <domain:name avail="false">αβααβα-1573042515287.ευ</domain:name>
          <domain:reason lang="en">registered</domain:reason>
        </domain:cd>
      </domain:chkData>
    </resData>
    <extension>
      <idn:mapping>
        <idn:name>
          <idn:ace>xn---1573042515287-f9jaaaqc.xn--qxa6a</idn:ace>
          <idn:unicode>αβααβα-1573042515287.ευ</idn:unicode>
        </idn:name>
      </idn:mapping>
    </extension>
    <trID>
      <clTRID>domain-check04</clTRID>
      <svTRID>e9024936b-bbae-48a4-b599-43f025b27cf8</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::super::domain::handle_check_response(*res).unwrap();
        let idn = data.eurid_idn.unwrap();
        assert!(!data.avail);
        assert_eq!(data.reason.unwrap(), "registered");
        assert!(data.eurid_check.is_none());
        assert_eq!(idn.ace, "xn---1573042515287-f9jaaaqc.xn--qxa6a");
        assert_eq!(idn.unicode, "αβααβα-1573042515287.ευ");
    }

    #[test]
    fn domain_info_0() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:domain-ext="http://www.eurid.eu/xml/epp/domain-ext-2.5" xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <domain:infData>
        <domain:name>somedomain.eu</domain:name>
        <domain:roid>somedomain_eu-EURID</domain:roid>
        <domain:status s="ok"/>
        <domain:registrant>c167</domain:registrant>
        <domain:contact type="billing">c166</domain:contact>
        <domain:contact type="tech">c168</domain:contact>
        <domain:ns>
          <domain:hostAttr>
            <domain:hostName>b.somedomain.eu</domain:hostName>
            <domain:hostAddr ip="v6">2001:db8:85a3:0:0:8a2e:371:7333</domain:hostAddr>
          </domain:hostAttr>
          <domain:hostAttr>
            <domain:hostName>a.somedomain.eu</domain:hostName>
            <domain:hostAddr ip="v4">203.0.113.0</domain:hostAddr>
          </domain:hostAttr>
        </domain:ns>
        <domain:clID>t000001</domain:clID>
        <domain:crID>t000001</domain:crID>
        <domain:crDate>2019-11-06T12:16:28.629Z</domain:crDate>
        <domain:upID>t000001</domain:upID>
        <domain:upDate>2019-11-06T12:16:28.000Z</domain:upDate>
        <domain:exDate>2022-11-06T22:59:59.999Z</domain:exDate>
      </domain:infData>
    </resData>
    <extension>
      <domain-ext:infData>
        <domain-ext:onHold>false</domain-ext:onHold>
        <domain-ext:reserved>false</domain-ext:reserved>
        <domain-ext:quarantined>false</domain-ext:quarantined>
        <domain-ext:suspended>false</domain-ext:suspended>
        <domain-ext:seized>false</domain-ext:seized>
        <domain-ext:contact type="onsite">c169</domain-ext:contact>
        <domain-ext:nsgroup>nsgroup-1573042588055</domain-ext:nsgroup>
        <domain-ext:nsgroup>nsgroup-1573042587789</domain-ext:nsgroup>
        <domain-ext:delayed>false</domain-ext:delayed>
        <domain-ext:maxExtensionPeriod>7</domain-ext:maxExtensionPeriod>
        <domain-ext:registrantCountry>BE</domain-ext:registrantCountry>
      </domain-ext:infData>
    </extension>
    <trID>
      <clTRID>domain-info01</clTRID>
      <svTRID>eaeddd5eb-534b-4602-95e1-bf5fd4328912</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::super::domain::handle_info_response(*res).unwrap();
        let eurid_data = data.eurid_data.unwrap();
        assert_eq!(data.name, "somedomain.eu");
        assert!(!eurid_data.on_hold);
        assert!(!eurid_data.quarantined);
        assert!(!eurid_data.suspended);
        assert!(!eurid_data.seized);
        assert_eq!(eurid_data.on_site.unwrap(), "c169");
        assert!(!eurid_data.delayed);
        assert_eq!(eurid_data.max_extension_period, 7);
        assert_eq!(eurid_data.registrant_country, "BE");
        assert!(eurid_data.registrant_country_of_citizenship.is_none());
        assert!(eurid_data.reseller.is_none());
        assert!(eurid_data.deletion_date.is_none());
    }

    #[test]
    fn domain_info_1() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:domain-ext="http://www.eurid.eu/xml/epp/domain-ext-2.5" xmlns:idn="http://www.eurid.eu/xml/epp/idn-1.0" xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <domain:infData>
        <domain:name>вмкйршаудхыийведйкгг.ею</domain:name>
        <domain:roid>xn__80adbeadbhzhddejt0e9bxb3cwd_xn__e1a4c-EURID</domain:roid>
        <domain:status s="ok"/>
        <domain:registrant>c193</domain:registrant>
        <domain:contact type="billing">c192</domain:contact>
        <domain:contact type="tech">c194</domain:contact>
        <domain:ns>
          <domain:hostAttr>
            <domain:hostName>b.вмкйршаудхыийведйкгг.ею</domain:hostName>
            <domain:hostAddr ip="v6">2001:db8:85a3:0:0:8a2e:371:7333</domain:hostAddr>
          </domain:hostAttr>
          <domain:hostAttr>
            <domain:hostName>a.вмкйршаудхыийведйкгг.ею</domain:hostName>
            <domain:hostAddr ip="v4">203.0.113.0</domain:hostAddr>
          </domain:hostAttr>
        </domain:ns>
        <domain:clID>t000001</domain:clID>
        <domain:crID>t000001</domain:crID>
        <domain:crDate>2019-11-06T12:16:56.905Z</domain:crDate>
        <domain:upID>t000001</domain:upID>
        <domain:upDate>2019-11-06T12:16:56.000Z</domain:upDate>
        <domain:exDate>2022-11-06T22:59:59.999Z</domain:exDate>
      </domain:infData>
    </resData>
    <extension>
      <domain-ext:infData>
        <domain-ext:onHold>false</domain-ext:onHold>
        <domain-ext:reserved>false</domain-ext:reserved>
        <domain-ext:quarantined>false</domain-ext:quarantined>
        <domain-ext:suspended>false</domain-ext:suspended>
        <domain-ext:seized>false</domain-ext:seized>
        <domain-ext:contact type="onsite">c195</domain-ext:contact>
        <domain-ext:nsgroup>nsgroup-1573042616260</domain-ext:nsgroup>
        <domain-ext:nsgroup>nsgroup-1573042615978</domain-ext:nsgroup>
        <domain-ext:delayed>false</domain-ext:delayed>
        <domain-ext:maxExtensionPeriod>7</domain-ext:maxExtensionPeriod>
        <domain-ext:registrantCountry>BE</domain-ext:registrantCountry>
      </domain-ext:infData>
      <idn:mapping>
        <idn:name>
          <idn:ace>a.xn--80adbeadbhzhddejt0e9bxb3cwd.xn--e1a4c</idn:ace>
          <idn:unicode>a.вмкйршаудхыийведйкгг.ею</idn:unicode>
        </idn:name>
        <idn:name>
          <idn:ace>b.xn--80adbeadbhzhddejt0e9bxb3cwd.xn--e1a4c</idn:ace>
          <idn:unicode>b.вмкйршаудхыийведйкгг.ею</idn:unicode>
        </idn:name>
        <idn:name>
          <idn:ace>xn--80adbeadbhzhddejt0e9bxb3cwd.xn--e1a4c</idn:ace>
          <idn:unicode>вмкйршаудхыийведйкгг.ею</idn:unicode>
        </idn:name>
      </idn:mapping>
    </extension>
    <trID>
      <clTRID>domain-info06</clTRID>
      <svTRID>eea5c141b-870b-47f0-bea5-641a269cc7bc</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::super::domain::handle_info_response(*res).unwrap();
        let eurid_data = data.eurid_data.unwrap();
        let eurid_idn = data.eurid_idn.unwrap();
        for ns in &data.nameservers {
            match ns {
                super::super::super::domain::InfoNameserver::HostAndAddress {
                    addresses: _,
                    host: _,
                    eurid_idn,
                } => {
                    assert!(eurid_idn.is_some());
                }
                _ => unreachable!(),
            }
        }
        assert_eq!(data.name, "вмкйршаудхыийведйкгг.ею");
        assert!(!eurid_data.on_hold);
        assert!(!eurid_data.quarantined);
        assert!(!eurid_data.suspended);
        assert!(!eurid_data.seized);
        assert_eq!(eurid_data.on_site.unwrap(), "c195");
        assert!(!eurid_data.delayed);
        assert_eq!(eurid_data.max_extension_period, 7);
        assert_eq!(eurid_data.registrant_country, "BE");
        assert!(eurid_data.registrant_country_of_citizenship.is_none());
        assert!(eurid_data.reseller.is_none());
        assert!(eurid_data.deletion_date.is_none());
        assert_eq!(eurid_idn.unicode, "вмкйршаудхыийведйкгг.ею");
        assert_eq!(eurid_idn.ace, "xn--80adbeadbhzhddejt0e9bxb3cwd.xn--e1a4c");
    }

    #[test]
    fn domain_renew_0() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:domain-ext="http://www.eurid.eu/xml/epp/domain-ext-2.5" xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <domain:renData>
        <domain:name>somedomain.eu</domain:name>
        <domain:exDate>2028-11-06T22:59:59.999Z</domain:exDate>
      </domain:renData>
    </resData>
    <extension>
      <domain-ext:renData>
        <domain-ext:removedDeletionDate/>
      </domain-ext:renData>
    </extension>
    <trID>
      <clTRID>Extend domain for 8y,deletion date is removed</clTRID>
      <svTRID>e2d4cddb6-a01b-4a6b-ae28-ff213fa9be8a</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::super::domain::handle_renew_response(*res).unwrap();
        let eurid_data = data.data.eurid_data.unwrap();
        assert!(data.data.eurid_idn.is_none());
        assert_eq!(data.data.name, "somedomain.eu");
        assert!(eurid_data.removed_deletion);
    }

    #[test]
    fn domain_renew_1() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:domain-ext="http://www.eurid.eu/xml/epp/domain-ext-2.5" xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <domain:renData>
        <domain:name>somedomain.eu</domain:name>
        <domain:exDate>2028-11-06T22:59:59.999Z</domain:exDate>
      </domain:renData>
    </resData>
    <extension>
      <domain-ext:renData>
      </domain-ext:renData>
    </extension>
    <trID>
      <clTRID>Extend domain for 8y,deletion date is removed</clTRID>
      <svTRID>e2d4cddb6-a01b-4a6b-ae28-ff213fa9be8a</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::super::domain::handle_renew_response(*res).unwrap();
        let eurid_data = data.data.eurid_data.unwrap();
        assert!(data.data.eurid_idn.is_none());
        assert_eq!(data.data.name, "somedomain.eu");
        assert!(!eurid_data.removed_deletion);
    }

    #[test]
    fn domain_transfer_0() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:domain-ext="http://www.eurid.eu/xml/epp/domain-ext-2.5" xmlns:idn="http://www.eurid.eu/xml/epp/idn-1.0" xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <domain:trnData>
        <domain:name>вмкйршаудхыийведйкгг.ею</domain:name>
        <domain:trStatus>pending</domain:trStatus>
        <domain:reID>t000002</domain:reID>
        <domain:reDate>2019-11-06T12:18:29.541Z</domain:reDate>
        <domain:acID>eurid.eu</domain:acID>
        <domain:acDate>2019-11-11T12:18:29.541Z</domain:acDate>
        <domain:exDate>2021-11-06T22:59:59.999Z</domain:exDate>
      </domain:trnData>
    </resData>
    <extension>
      <domain-ext:trnData>
        <domain-ext:onHold>false</domain-ext:onHold>
        <domain-ext:reserved>false</domain-ext:reserved>
        <domain-ext:quarantined>false</domain-ext:quarantined>
        <domain-ext:registrant>c293</domain-ext:registrant>
        <domain-ext:contact type="billing">c292</domain-ext:contact>
        <domain-ext:contact type="tech">c294</domain-ext:contact>
        <domain-ext:delayed>false</domain-ext:delayed>
        <domain-ext:reason>RANDOM CHECK</domain-ext:reason>
      </domain-ext:trnData>
      <idn:mapping>
        <idn:name>
          <idn:ace>xn--80adbeadbhzhddejt0e9bxb3cwd.xn--e1a4c</idn:ace>
          <idn:unicode>вмкйршаудхыийведйкгг.ею</idn:unicode>
        </idn:name>
      </idn:mapping>
    </extension>
    <trID>
      <clTRID>domain-transfer05</clTRID>
      <svTRID>ed9518b51-41aa-4d5f-bbe8-08ab182c7fee</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::super::domain::handle_transfer_response(*res).unwrap();
        let eurid_data = data.data.eurid_data.unwrap();
        let eurid_idn = data.data.eurid_idn.unwrap();
        assert_eq!(data.data.name, "вмкйршаудхыийведйкгг.ею");
        assert_eq!(eurid_idn.ace, "xn--80adbeadbhzhddejt0e9bxb3cwd.xn--e1a4c");
        assert!(!eurid_data.on_hold);
        assert!(!eurid_data.quarantined);
        assert!(!eurid_data.delayed);
        assert_eq!(eurid_data.reason, "RANDOM CHECK");
        assert_eq!(eurid_data.registrant, "c293");
        assert_eq!(eurid_data.billing, "c292");
        assert_eq!(eurid_data.technical.unwrap(), "c294");
        assert!(eurid_data.reseller.is_none());
        assert!(eurid_data.on_site.is_none());
    }
}
