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
mod balance_tests {
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
    }

    #[test]
    fn dnssec_quality() {
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
    }
}
