//! EPP commands relating to EURid extensions

use chrono::prelude::*;
use super::router::HandleReqReturn;
use super::{proto, EPPClientServerFeatures, Error, Request, Response, CommandResponse, Sender};

#[derive(Debug)]
pub struct HitPointsRequest {
    pub return_path: Sender<HitPointsResponse>,
}

#[derive(Debug)]
pub struct HitPointsResponse {
    pub hit_points: u64,
    pub max_hit_points: u64,
    pub blocked_until: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct RegistrationLimitRequest {
    pub return_path: Sender<RegistrationLimitResponse>,
}

#[derive(Debug)]
pub struct RegistrationLimitResponse {
    pub monthly_registrations: u64,
    pub max_monthly_registrations: Option<u64>,
    pub limited_until: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct DNSSECEligibilityRequest {
    name: String,
    pub return_path: Sender<DNSSECEligibilityResponse>,
}

#[derive(Debug)]
pub struct DNSSECEligibilityResponse {
    pub eligible: bool,
    pub message: String,
    pub code: u32,
    pub idn: Option<IDN>,
}

#[derive(Debug)]
pub struct DNSQualityRequest {
    name: String,
    pub return_path: Sender<DNSQualityResponse>,
}

#[derive(Debug)]
pub struct DNSQualityResponse {
    pub check_time: Option<DateTime<Utc>>,
    pub score: String,
    pub idn: Option<IDN>,
}

#[derive(Debug)]
pub struct PollResponse {
    pub context: String,
    pub object_type: String,
    pub object: String,
    pub object_unicode: Option<String>,
    pub action: String,
    pub code: u32,
    pub detail: Option<String>,
    pub registrar: Option<String>,
}

#[derive(Debug)]
pub struct ContactExtension {
    pub contact_type: ContactType,
    pub whois_email: Option<String>,
    pub vat: Option<String>,
    pub citizenship_country: Option<String>,
    pub language: String,
}

#[derive(Debug)]
pub struct ContactExtensionUpdate {
    pub contact_type: Option<ContactType>,
    pub whois_email: Option<String>,
    pub vat: Option<String>,
    pub citizenship_country: Option<String>,
    pub language: Option<String>,
}

pub fn is_entity_natural_person(entity: Option<&super::contact::EntityType>) -> bool {
    match entity {
        Some(e) => match e {
            super::contact::EntityType::UkIndividual
            | super::contact::EntityType::FinnishIndividual
            | super::contact::EntityType::OtherIndividual => true,
            _ => false,
        },
        None => true,
    }
}

pub fn eurid_ext_to_entity(from: &proto::eurid::EURIDContactInfo) -> super::contact::EntityType {
    match from.natural_person {
        true => super::contact::EntityType::OtherIndividual,
        false => super::contact::EntityType::Unknown,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContactType {
    Billing,
    Tech,
    Registrant,
    OnSite,
    Reseller,
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
        }
    }
}

pub fn contact_info_from_extension(from: &ContactExtension, entity_type: &Option<super::contact::EntityType>) -> proto::eurid::EURIDContactInfo {
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
    }
}

pub fn contact_info_update_from_extension(from: &Option<ContactExtensionUpdate>, entity_type: &Option<super::contact::EntityType>) -> proto::eurid::EURIDContactUpdate {
    use proto::eurid::EURIDContactType;
    proto::eurid::EURIDContactUpdate {
        change: proto::eurid::EURIDContactUpdateInfo {
            contact_type: from.as_ref().map_or(None, |f| f.contact_type.as_ref().map(|c| match c {
                ContactType::Billing => EURIDContactType::Billing,
                ContactType::Tech => EURIDContactType::Tech,
                ContactType::Registrant => EURIDContactType::Registrant,
                ContactType::OnSite => EURIDContactType::OnSite,
                ContactType::Reseller => EURIDContactType::Reseller,
            })),
            whois_email: from.as_ref().map_or(None, |f| f.whois_email.as_deref().map(Into::into)),
            vat: from.as_ref().map_or(None, |f| f.vat.as_deref().map(Into::into)),
            language: from.as_ref().map_or(None, |f| f.language.as_deref().map(Into::into)),
            natural_person: entity_type.as_ref().map(|e| is_entity_natural_person(Some(e))),
            country_of_citizenship: from.as_ref().map_or(None, |f| f.citizenship_country.as_deref().map(Into::into)),
        }
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

#[derive(Debug)]
pub struct IDN {
    pub ace: String,
    pub unicode: String,
}

pub fn extract_eurid_idn(from: &Option<proto::EPPResponseExtension>) -> Option<Vec<IDN>> {
    let eurid_ext_idn = match from {
        Some(e) => match e.value.iter().find(|e| match e {
            proto::EPPResponseExtensionType::EURIDIDNMapping(_) => true,
            _ => false,
        }) {
            Some(e) => match e {
                proto::EPPResponseExtensionType::EURIDIDNMapping(e) => Some(e),
                _ => unreachable!(),
            },
            None => None,
        },
        None => None,
    };

    match eurid_ext_idn {
        Some(e) => Some(e.names.iter().map(|n| IDN {
            ace: (&n.ace).into(),
            unicode: (&n.unicode).into(),
        }).collect()),
        None => None,
    }
}

pub fn extract_eurid_idn_singular(from: &Option<proto::EPPResponseExtension>) -> Result<Option<IDN>, Error> {
    match extract_eurid_idn(from) {
        Some(mut i) => match i.len() {
            1 => Ok(Some(i.pop().unwrap())),
            _ => Err(Error::InternalServerError)
        },
        None => Ok(None)
    }
}


#[derive(Debug)]
pub struct DomainCheck {
    pub available_date: Option<DateTime<Utc>>,
    pub status: Vec<super::domain::Status>
}

pub fn extract_eurid_domain_check_singular(from: &Option<proto::EPPResponseExtension>) -> Result<Option<DomainCheck>, Error> {
    let eurid_ext_check = match from {
        Some(e) => match e.value.iter().find(|e| match e {
            proto::EPPResponseExtensionType::EURIDDomainCheckData(_) => true,
            _ => false,
        }) {
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
                         status: c.status.into_iter().map(|s| s.status.into()).collect()
                     }))
                 },
                 _ => Err(Error::InternalServerError)
             }
         },
        None => Ok(None)
    }
}


pub fn handle_hit_points(
    client: &EPPClientServerFeatures,
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

pub fn handle_hit_points_response(response: proto::EPPResponse) -> Response<HitPointsResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EURIDRegistrarHitPointsData(hit_points) => {
                Response::Ok(HitPointsResponse {
                    hit_points: hit_points.hit_points,
                    max_hit_points: hit_points.max_hit_points,
                    blocked_until: hit_points.blocked_until,
                })
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

pub fn handle_registration_limits(
    client: &EPPClientServerFeatures,
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

pub fn handle_registration_limits_response(response: proto::EPPResponse) -> Response<RegistrationLimitResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EURIDRegistrationLimitData(registration_limit) => {
                Response::Ok(RegistrationLimitResponse {
                    monthly_registrations: registration_limit.monthly_registrations,
                    max_monthly_registrations: registration_limit.max_monthly_registrations,
                    limited_until: registration_limit.limited_until,
                })
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

pub fn handle_dnssec_eligibility(
    client: &EPPClientServerFeatures,
    req: &DNSSECEligibilityRequest,
) -> HandleReqReturn<DNSSECEligibilityResponse> {
    if client.eurid_dnssec_eligibility_support {
        Ok((
            proto::EPPCommandType::Info(proto::EPPInfo::EURIDDNSSECEligibilityInfo(
                proto::eurid::EURIDDNSSECEligibilityInfo {
                    name: req.name.to_string(),
                }
            )),
            None,
        ))
    } else {
        Err(Err(Error::Unsupported))
    }
}

pub fn handle_dnssec_eligibility_response(response: proto::EPPResponse) -> Response<DNSSECEligibilityResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EURIDDNSSECEligibilityInfoData(dnssec_eligibility) => {
                Response::Ok(DNSSECEligibilityResponse {
                    eligible: dnssec_eligibility.eligible,
                    message: dnssec_eligibility.msg,
                    code: dnssec_eligibility.code,
                    idn: extract_eurid_idn_singular(&response.extension)?
                })
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

pub fn handle_dns_quality(
    client: &EPPClientServerFeatures,
    req: &DNSQualityRequest,
) -> HandleReqReturn<DNSQualityResponse> {
    if client.eurid_dns_quality_support {
        Ok((
            proto::EPPCommandType::Info(proto::EPPInfo::EURIDDNSQuality(
                proto::eurid::EURIDDNSQualityInfo {
                    name: req.name.to_string(),
                }
            )),
            None,
        ))
    } else {
        Err(Err(Error::Unsupported))
    }
}

pub fn handle_dns_quality_response(response: proto::EPPResponse) -> Response<DNSQualityResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::EURIDDNSQualityData(dns_quality) => {
                Response::Ok(DNSQualityResponse {
                    check_time: dns_quality.check_time,
                    score: dns_quality.score,
                    idn: extract_eurid_idn_singular(&response.extension)?
                })
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

/// Makes a hit points enquiry to the registry
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn hit_points_info(
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<HitPointsResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::EURIDHitPoints(Box::new(HitPointsRequest {
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Makes a registration limits enquiry to the registry
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn registration_limit_info(
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<RegistrationLimitResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::EURIDRegistrationLimit(Box::new(RegistrationLimitRequest {
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Makes a DNSSEC discount eligibility enquiry to the registry
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn dnssec_eligibility_info(
    name: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<DNSSECEligibilityResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::EURIDDNSSECEligibility(Box::new(DNSSECEligibilityRequest {
            name: name.to_string(),
            return_path: sender,
        })),
        receiver,
    )
        .await
}

/// Makes a DNS quality enquiry to the registry
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn dns_quality_info(
    name: &str,
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<CommandResponse<DNSQualityResponse>, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::EURIDDNSQuality(Box::new(DNSQualityRequest {
            name: name.to_string(),
            return_path: sender,
        })),
        receiver,
    )
        .await
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
        assert_eq!(data.blocked_until.is_none(), true);
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
        assert_eq!(data.blocked_until.is_some(), true);
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
        assert_eq!(data.limited_until.is_none(), true);
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
        assert_eq!(data.max_monthly_registrations.is_none(), true);
        assert_eq!(data.limited_until.is_none(), true);
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
        assert_eq!(data.limited_until.is_some(), true);
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
        assert_eq!(data.eligible, true);
        assert_eq!(data.message, "Eligible for DNSSEC discount");
        assert_eq!(data.code, 1001);
        assert_eq!(data.idn.is_none(), true);
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
        assert_eq!(data.eligible, false);
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
        assert_eq!(data.check_time.is_some(), true);
        assert_eq!(data.score, "10000");
        assert_eq!(data.idn.is_none(), true);
    }

    #[test]
    fn contact_info_0() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:contact-ext="http://www.eurid.eu/xml/epp/contact-ext-1.3" xmlns:contact="urn:ietf:params:xml:ns:contact-1.0">
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
        assert_eq!(data.entity_type, super::super::contact::EntityType::Unknown);
        assert_eq!(eurid_extension.contact_type, super::ContactType::Tech);
        assert_eq!(eurid_extension.language, "en");
        assert_eq!(eurid_extension.citizenship_country.is_none(), true);
        assert_eq!(eurid_extension.whois_email.is_none(), true);
        assert_eq!(eurid_extension.vat.is_none(), true);
    }

    #[test]
    fn contact_info_1() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:contact-ext="http://www.eurid.eu/xml/epp/contact-ext-1.3" xmlns:contact="urn:ietf:params:xml:ns:contact-1.0">
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
        assert_eq!(data.entity_type, super::super::contact::EntityType::OtherIndividual);
        assert_eq!(eurid_extension.contact_type, super::ContactType::Registrant);
        assert_eq!(eurid_extension.language, "en");
        assert_eq!(eurid_extension.citizenship_country.is_none(), true);
        assert_eq!(eurid_extension.whois_email.is_none(), true);
        assert_eq!(eurid_extension.vat.is_none(), true);
    }

    #[test]
    fn contact_info_2() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:contact-ext="http://www.eurid.eu/xml/epp/contact-ext-1.3" xmlns:contact="urn:ietf:params:xml:ns:contact-1.0">
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
        assert_eq!(data.entity_type, super::super::contact::EntityType::Unknown);
        assert_eq!(eurid_extension.contact_type, super::ContactType::Registrant);
        assert_eq!(eurid_extension.language, "en");
        assert_eq!(eurid_extension.citizenship_country.is_none(), true);
        assert_eq!(eurid_extension.whois_email.is_none(), true);
        assert_eq!(eurid_extension.vat.unwrap(), "VAT1234567890");
    }

    #[test]
    fn contact_info_3() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:contact-ext="http://www.eurid.eu/xml/epp/contact-ext-1.3" xmlns:contact="urn:ietf:params:xml:ns:contact-1.0">
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
        assert_eq!(data.entity_type, super::super::contact::EntityType::OtherIndividual);
        assert_eq!(eurid_extension.contact_type, super::ContactType::Reseller);
        assert_eq!(eurid_extension.language, "en");
        assert_eq!(eurid_extension.citizenship_country.is_none(), true);
        assert_eq!(eurid_extension.whois_email.is_none(), true);
        assert_eq!(eurid_extension.vat.is_none(), true);
    }

    #[test]
    fn domain_check_0() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:domain-ext-2.3="http://www.eurid.eu/xml/epp/domain-ext-2.4" xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
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
      <domain-ext-2.3:chkData>
        <domain-ext-2.3:domain>
          <domain-ext-2.3:name>europa.eu</domain-ext-2.3:name>
          <domain-ext-2.3:status s="serverTransferProhibited"/>
        </domain-ext-2.3:domain>
      </domain-ext-2.3:chkData>
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
        assert_eq!(data.avail, false);
        assert_eq!(data.reason.unwrap(), "registered");
        assert_eq!(eurid_extension.available_date.is_none(), true);
        assert_eq!(eurid_extension.status.len(), 1);
        assert_eq!(eurid_extension.status[0], super::super::domain::Status::ServerTransferProhibited);
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
        assert_eq!(data.avail, false);
        assert_eq!(data.reason.unwrap(), "registered");
        assert_eq!(data.eurid_check.is_none(), true);
        assert_eq!(idn.ace, "xn---1573042515287-f9jaaaqc.xn--qxa6a");
        assert_eq!(idn.unicode, "αβααβα-1573042515287.ευ");
    }
}
