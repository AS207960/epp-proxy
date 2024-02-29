//! EPP commands relating to domain objects

use std::convert::TryInto;

use chrono::prelude::*;

use super::super::poll::{
    ChangeCaseId, ChangeCaseIdType, ChangeData, ChangeOperation, ChangeOperationType, ChangeState,
    PollAckRequest, PollAckResponse, PollData, PollRequest, PollResponse,
};
use super::super::{proto, Error, Response};
use super::router::HandleReqReturn;
use super::ServerFeatures;

fn change_data_from_response(
    from: &Option<proto::EPPResponseExtension>,
) -> Result<Option<ChangeData>, Error> {
    match from {
        Some(ext) => match ext.value.iter().find_map(|p| match p {
            proto::EPPResponseExtensionType::EPPChangePoll(i) => Some(i),
            _ => None,
        }) {
            Some(e) => Ok(Some(ChangeData {
                state: match e.state {
                    proto::change_poll::EPPChangeState::After => ChangeState::After,
                    proto::change_poll::EPPChangeState::Before => ChangeState::Before,
                },
                operation: ChangeOperation {
                    operation: e.operation.operation.clone(),
                    op_type: match e.operation.op_type {
                        proto::change_poll::EPPChangeOperationType::Create => {
                            ChangeOperationType::Create
                        }
                        proto::change_poll::EPPChangeOperationType::Delete => {
                            ChangeOperationType::Delete
                        }
                        proto::change_poll::EPPChangeOperationType::Renew => {
                            ChangeOperationType::Renew
                        }
                        proto::change_poll::EPPChangeOperationType::Transfer => {
                            ChangeOperationType::Transfer
                        }
                        proto::change_poll::EPPChangeOperationType::Update => {
                            ChangeOperationType::Update
                        }
                        proto::change_poll::EPPChangeOperationType::Restore => {
                            ChangeOperationType::Restore
                        }
                        proto::change_poll::EPPChangeOperationType::AutoRenew => {
                            ChangeOperationType::AutoRenew
                        }
                        proto::change_poll::EPPChangeOperationType::AutoDelete => {
                            ChangeOperationType::AutoDelete
                        }
                        proto::change_poll::EPPChangeOperationType::AutoPurge => {
                            ChangeOperationType::AutoPurge
                        }
                        proto::change_poll::EPPChangeOperationType::Custom => {
                            ChangeOperationType::Custom
                        }
                    },
                },
                date: e.date,
                server_transaction_id: e.server_transaction_id.clone(),
                who: e.who.clone(),
                case_id: e.case_id.as_ref().map(|c| ChangeCaseId {
                    name: c.name.clone(),
                    case_id: c.case_id.clone(),
                    case_type: match c.case_type {
                        proto::change_poll::EPPChangeCaseIdType::Udrp => ChangeCaseIdType::Udrp,
                        proto::change_poll::EPPChangeCaseIdType::Urs => ChangeCaseIdType::Urs,
                        proto::change_poll::EPPChangeCaseIdType::Custom => ChangeCaseIdType::Custom,
                    },
                }),
                reason: e.reason.clone(),
            })),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

pub fn handle_poll(
    _client: &ServerFeatures,
    _req: &PollRequest,
) -> HandleReqReturn<Option<PollResponse>> {
    let command = proto::EPPPoll {
        operation: proto::EPPPollOperation::Request,
        message_id: None,
    };
    Ok((proto::EPPCommandType::Poll(command), None))
}

pub fn handle_poll_response<M: crate::metrics::Metrics>(
    response: proto::EPPResponse, metrics: &M
) -> Response<Option<PollResponse>> {
    match response.results.first() {
        Some(result) => match result.code {
            proto::EPPResultCode::SuccessNoMessages => Response::Ok(None),
            proto::EPPResultCode::SuccessAckToDequeue => match response.message_queue {
                Some(value) => Response::Ok(Some(PollResponse {
                    count: value.count,
                    id: value.id.unwrap_or_default(),
                    enqueue_time: value.enqueue_date.unwrap_or_else(Utc::now),
                    message: value.message.unwrap_or_default(),
                    data: match response.data {
                        Some(value) => {
                            metrics.poll_received(value.value.name());
                            match value.value {
                                proto::EPPResultDataValue::EPPDomainInfoResult(domain_info) => {
                                    PollData::DomainInfoData {
                                        data: Box::new((*domain_info, &response.extension).try_into()?),
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::EPPContactInfoResult(contact_info) => {
                                    PollData::ContactInfoData {
                                        data: Box::new(
                                            (*contact_info, &response.extension).try_into()?,
                                        ),
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::EPPHostInfoResult(host_info) => {
                                    PollData::HostInfoData {
                                        data: Box::new((*host_info).try_into()?),
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::EPPDomainTransferResult(domain_transfer) => {
                                    PollData::DomainTransferData {
                                        data: (domain_transfer, &response.extension).try_into()?,
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::EPPContactTransferResult(
                                    contact_transfer,
                                ) => PollData::ContactTransferData {
                                    data: (&contact_transfer).into(),
                                    change_data: change_data_from_response(&response.extension)?,
                                },
                                proto::EPPResultDataValue::EPPDomainCreateResult(domain_create) => {
                                    PollData::DomainCreateData {
                                        data: (Some(domain_create), &response.extension).try_into()?,
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::EPPDomainRenewResult(domain_renew) => {
                                    PollData::DomainRenewData {
                                        data: (domain_renew, &response.extension).try_into()?,
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::EPPDomainPendingActionNotification(
                                    domain_data,
                                ) => PollData::DomainPanData {
                                    data: (&domain_data).into(),
                                    change_data: change_data_from_response(&response.extension)?,
                                },
                                proto::EPPResultDataValue::EPPContactPendingActionNotification(
                                    contact_data,
                                ) => PollData::ContactPanData {
                                    data: (&contact_data).into(),
                                    change_data: change_data_from_response(&response.extension)?,
                                },
                                proto::EPPResultDataValue::NominetCancelData(canc_data) => {
                                    PollData::NominetDomainCancelData {
                                        data: canc_data.into(),
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::NominetReleaseData(rel_data) => {
                                    PollData::NominetDomainReleaseData {
                                        data: rel_data.into(),
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::NominetRegistrarChangeData(rc_data) => {
                                    PollData::NominetDomainRegistrarChangeData {
                                        data: (rc_data, &response.extension).try_into()?,
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::NominetHostCancelData(canc_data) => {
                                    PollData::NominetHostCancelData {
                                        data: canc_data.into(),
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::NominetProcessData(p_data) => {
                                    PollData::NominetProcessData {
                                        data: (p_data, &response.extension).try_into()?,
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::NominetSuspendData(sus_data) => {
                                    PollData::NominetSuspendData {
                                        data: sus_data.into(),
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::NominetDomainFailData(fail_data) => {
                                    PollData::NominetDomainFailData {
                                        data: fail_data.into(),
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::NominetTransferData(trn_data) => {
                                    PollData::NominetRegistrantTransferData {
                                        data: (trn_data, &response.extension).try_into()?,
                                        change_data: change_data_from_response(&response.extension)?,
                                    }
                                }
                                proto::EPPResultDataValue::VerisignLowBalanceData(bal_data) => {
                                    PollData::VerisignLowBalanceData(bal_data.try_into()?)
                                }
                                proto::EPPResultDataValue::TraficomTrnData(trn_data) => {
                                    PollData::TraficomTrnData(trn_data.into())
                                }
                                proto::EPPResultDataValue::EURIDPollData(poll_data) => {
                                    PollData::EURIDPoll(poll_data.into())
                                }
                                proto::EPPResultDataValue::EPPMaintenanceInfo(
                                    proto::maintenance::EPPMaintenanceInfoData::Maintenance(item),
                                ) => PollData::MaintenanceData(item.into()),
                                proto::EPPResultDataValue::EPPMaintenanceInfo02(
                                    proto::maintenance::EPPMaintenanceInfoData02::Maintenance(item),
                                ) => PollData::MaintenanceData(item.into()),
                                _ => return Err(Error::ServerInternal),
                            }
                        },
                        None => PollData::None,
                    },
                })),
                None => Err(Error::ServerInternal),
            },
            _ => Err(Error::ServerInternal),
        },
        None => Err(Error::ServerInternal),
    }
}

pub fn handle_poll_ack(
    _client: &ServerFeatures,
    req: &PollAckRequest,
) -> HandleReqReturn<PollAckResponse> {
    let command = proto::EPPPoll {
        operation: proto::EPPPollOperation::Acknowledge,
        message_id: Some(req.id.clone()),
    };
    Ok((proto::EPPCommandType::Poll(command), None))
}

pub fn handle_poll_ack_response<M: crate::metrics::Metrics>(
    response: proto::EPPResponse, _metrics: &M
) -> Response<PollAckResponse> {
    match response.message_queue {
        Some(value) => Response::Ok(PollAckResponse {
            count: Some(value.count),
            next_id: value.id,
        }),
        None => Response::Ok(PollAckResponse {
            count: None,
            next_id: None,
        }),
    }
}

#[cfg(test)]
mod poll_tests {
    #[test]
    fn switch_transfer_complete() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1301">
      <msg lang="en">Command completed successfully; ack to dequeue</msg>
    </result>
    <msgQ count="1" id="1139047">
      <qDate>2007-09-26T00:00:00+02:00</qDate>
      <msg>Domain transfer completed successfully</msg>
    </msgQ>
    <epp:resData xmlns:epp="urn:ietf:params:xml:ns:epp-1.0">
      <domain:trnData xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
        <domain:name>yourname.ch</domain:name>
        <domain:trStatus>serverApproved</domain:trStatus>
        <domain:reID>RegistrarB</domain:reID>
        <domain:reDate>2007-09-18T22:43:00+02:00</domain:reDate>
        <domain:acID>NULL</domain:acID>
        <domain:acDate>2007-09-18T22:43:00+02:00</domain:acDate>
      </domain:trnData>
    </epp:resData>
    <trID>
      <clTRID>Registrar_00_2</clTRID>
      <svTRID>20071008.13688.27039</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "Domain transfer completed successfully");
        match data.data {
            super::PollData::DomainTransferData {
                data: _,
                change_data: None,
            } => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn switch_domain_delete() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
  <epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
    <response>
      <result code="1301">
        <msg lang="en">Command completed successfully; ack to dequeue</msg>
      </result>
      <msgQ count="1" id="46535949">
        <qDate>2019-01-28T16:14:47+01:00</qDate>
      </msgQ>
      <resData>
      <domain:infData xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
        <domain:name>delete-me11.ch</domain:name>
        <domain:roid>D6006352-SWITCH</domain:roid>
        <domain:status s="serverHold" lang="en" />
        <domain:status s="serverRenewProhibited" lang="en" />
        <domain:status s="serverTransferProhibited" lang="en" />
        <domain:status s="inactive" lang="en" />
        <domain:registrant>CH-MYTECH</domain:registrant>
        <domain:clID>Test-Registrar-X</domain:clID>
        <domain:upDate>2019-01-28T16:14:47+01:00</domain:upDate>
      </domain:infData>
    </resData>
    <extension>
      <changePoll:changeData xmlns:changePoll="urn:ietf:params:xml:ns:changePoll-1.0" state="after">
        <changePoll:operation>delete</changePoll:operation>
        <changePoll:date>2019-01-28T16:14:47+01:00</changePoll:date>
        <changePoll:svTRID>20190128.34733373</changePoll:svTRID>
        <changePoll:who>SWITCH manual delete</changePoll:who>
        <changePoll:reason>domain name abuse</changePoll:reason>
      </changePoll:changeData>
    </extension>
    <trID>
      <clTRID>ABC-12345</clTRID>
      <svTRID>20190128.75290441.758467418</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "");
        match data.data {
            super::PollData::DomainInfoData { data, change_data } => {
                let change_data = change_data.unwrap();
                assert_eq!(change_data.state, super::ChangeState::After);
                assert_eq!(
                    change_data.operation.op_type,
                    super::ChangeOperationType::Delete
                );
                assert_eq!(change_data.who, "SWITCH manual delete");
                assert_eq!(change_data.reason.unwrap(), "domain name abuse");
                assert_eq!(data.name, "delete-me11.ch");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn switch_dnssec_initialized() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
    <response>
        <result code="1301">
            <msg lang="en">Command completed successfully; ack to dequeue</msg>
        </result>
        <msgQ count="1" id="46533741">
            <qDate>2018-11-20T15:01:01+01:00</qDate>
        </msgQ>
        <resData>
            <domain:infData xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
                <domain:name>polltest-cds-bootstrap.ch</domain:name>
                <domain:roid>D123456-SWITCH</domain:roid>
                <domain:status s="inactive" lang="en" />
                <domain:registrant>D1234567-SWITCH</domain:registrant>
                <domain:clID>D1234568-SWITCH</domain:clID>
                <domain:upDate>2018-11-20T15:01:01+01:00</domain:upDate>
            </domain:infData>
        </resData>
        <extension>
            <changePoll:changeData xmlns:changePoll="urn:ietf:params:xml:ns:changePoll-1.0" state="after">
                <changePoll:operation>update</changePoll:operation>
                <changePoll:date>2018-11-20T15:01:01+01:00</changePoll:date>
                <changePoll:svTRID>20181120.123456</changePoll:svTRID>
                <changePoll:who>SWITCH CDS: see https://www.nic.ch/cds/</changePoll:who>
                <changePoll:reason>DNSSEC initialized</changePoll:reason>
            </changePoll:changeData>
            <secDNS:infData xmlns:secDNS="urn:ietf:params:xml:ns:secDNS-1.1">
                <secDNS:dsData>
                    <secDNS:keyTag>1337</secDNS:keyTag>
                    <secDNS:alg>13</secDNS:alg>
                    <secDNS:digestType>4</secDNS:digestType>
                    <secDNS:digest>AAAA54840FBBB6F4270F8B6D8C06C6A2B3152E55D2E9F81132130E507829B6D24FA56A4E074B4692DDC46F512B048AAC</secDNS:digest>
                </secDNS:dsData>
                <secDNS:dsData>
                    <secDNS:keyTag>1337</secDNS:keyTag>
                    <secDNS:alg>13</secDNS:alg>
                    <secDNS:digestType>2</secDNS:digestType>
                    <secDNS:digest>AAAA9AB3E7D203FF7923B8773599E248717F1DC79A9BEF09D8981B13AB7A049E</secDNS:digest>
                </secDNS:dsData>
            </secDNS:infData>
        </extension>
        <trID>
            <clTRID>ABC-12345</clTRID>
            <svTRID>20181120.75241918.758340721</svTRID>
        </trID>
    </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "");
        match data.data {
            super::PollData::DomainInfoData { data, change_data } => {
                let change_data = change_data.unwrap();
                assert_eq!(change_data.state, super::ChangeState::After);
                assert_eq!(
                    change_data.operation.op_type,
                    super::ChangeOperationType::Update
                );
                assert_eq!(change_data.who, "SWITCH CDS: see https://www.nic.ch/cds/");
                assert_eq!(change_data.reason.unwrap(), "DNSSEC initialized");
                assert!(data.sec_dns.is_some());
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn switch_dnssec_deactivated() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
    <response>
        <result code="1301">
            <msg lang="en">Command completed successfully; ack to dequeue</msg>
        </result>
        <msgQ count="1" id="46533742">
            <qDate>2018-11-20T15:12:41+01:00</qDate>
        </msgQ>
        <resData>
            <domain:infData xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
                <domain:name>polltest-cds-delete.ch</domain:name>
                <domain:roid>D123456-SWITCH</domain:roid>
                <domain:status s="inactive" lang="en" />
                <domain:registrant>D1234567-SWITCH</domain:registrant>
                <domain:clID>D1234568-SWITCH</domain:clID>
                <domain:upDate>2018-11-20T15:12:41+01:00</domain:upDate>
            </domain:infData>
        </resData>
        <extension>
            <changePoll:changeData xmlns:changePoll="urn:ietf:params:xml:ns:changePoll-1.0" state="after">
                <changePoll:operation>update</changePoll:operation>
                <changePoll:date>2018-11-20T15:12:41+01:00</changePoll:date>
                <changePoll:svTRID>20181120.123456</changePoll:svTRID>
                <changePoll:who>SWITCH CDS: see https://www.nic.ch/cds/</changePoll:who>
                <changePoll:reason>DNSSEC deactivated</changePoll:reason>
            </changePoll:changeData>
        </extension>
        <trID>
            <clTRID>ABC-12345</clTRID>
            <svTRID>20181120.75241923.758340738</svTRID>
        </trID>
    </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "");
        match data.data {
            super::PollData::DomainInfoData { data, change_data } => {
                let change_data = change_data.unwrap();
                assert_eq!(change_data.state, super::ChangeState::After);
                assert_eq!(
                    change_data.operation.op_type,
                    super::ChangeOperationType::Update
                );
                assert_eq!(change_data.who, "SWITCH CDS: see https://www.nic.ch/cds/");
                assert_eq!(change_data.reason.unwrap(), "DNSSEC deactivated");
                assert!(data.sec_dns.is_none());
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn nominet_amend_account() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0"
  xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
  xsi:schemaLocation="urn:ietf:params:xml:ns:epp-1.0
  epp-1.0.xsd">
  <response>
    <result code="1301">
      <msg>Command completed successfully; ack to dequeue</msg>
    </result>
    <msgQ count="4" id="123456">
      <qDate>2005-10-06T10:29:30Z</qDate>
      <msg>Account Details Change Notification</msg>
    </msgQ>
    <resData>
      <contact:infData xmlns:contact="urn:ietf:params:xml:ns:contact-1.0" xsi:schemaLocation="urn:ietf:params:xml:ns:contact-1.0 contact-1.0.xsd">
        <contact:id>CMyContactID</contact:id>
        <contact:roid>548965487-UK</contact:roid>
        <contact:status s="ok"/>
        <contact:postalInfo type="loc">
          <contact:name>Mr Jones</contact:name>
          <contact:org>Company.</contact:org>
          <contact:addr>
            <contact:street>High Street</contact:street>
            <contact:city>Oxford</contact:city>
            <contact:pc>OX1 1AH</contact:pc>
            <contact:cc>GB</contact:cc>
          </contact:addr>
        </contact:postalInfo>
        <contact:voice>+44.1865658754</contact:voice>
        <contact:email>example@epp-example.org.uk</contact:email>
        <contact:clID>EXAMPLE-TAG</contact:clID>
        <contact:crID>n/a</contact:crID>
        <contact:crDate>2007-05-12T12:44:00Z</contact:crDate>
        <contact:upDate>2008-06-12T06:46:00Z</contact:upDate>
        <contact:disclose flag="1">
          <contact:org type="loc"/>
          <contact:addr type="loc"/>
        </contact:disclose>
    </contact:infData>
    </resData>
    <trID>
      <clTRID>ABC-12345</clTRID>
      <svTRID>123456</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "Account Details Change Notification");
        match data.data {
            super::PollData::ContactInfoData { data, change_data } => {
                assert!(change_data.is_none());
                assert_eq!(data.id, "CMyContactID");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn nominet_domain_cancelled() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="urn:ietf:params:xml:ns:epp-1.0 epp-1.0.xsd">
   <response>
     <result code="1301">
       <msg>Command completed successfully; ack to dequeue</msg>
     </result>
     <msgQ count="10" id="12345">
       <qDate>2007-09-26T07:31:30</qDate>
       <msg>Domain name Cancellation Notification</msg>
     </msgQ>
     <resData>
       <n:cancData xmlns:n="http://www.nominet.org.uk/epp/xml/std-notifications-1.2">
         <n:domainName>epp-example1.co.uk</n:domainName>
         <n:orig>example@nominet</n:orig>
       </n:cancData>
     </resData>
     <trID>
       <clTRID>ABC-12345</clTRID>
       <svTRID>123456</svTRID>
     </trID>
   </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "Domain name Cancellation Notification");
        match data.data {
            super::PollData::NominetDomainCancelData {
                data: canc_data,
                change_data: None,
            } => {
                assert_eq!(canc_data.domain_name, "epp-example1.co.uk");
                assert_eq!(canc_data.originator, "example@nominet");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn nominet_domain_released() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1301">
      <msg>Command completed successfully; ack to dequeue</msg>
    </result>
    <msgQ count="10" id="12345">
      <qDate>2007-09-26T07:31:30</qDate>
      <msg>Domains Released Notification</msg>
    </msgQ>
    <resData>
      <n:relData xmlns:n="http://www.nominet.org.uk/epp/xml/std-notifications-1.2">
        <n:accountId moved="Y">12345</n:accountId>
        <n:from>EXAMPLE1-TAG</n:from>
        <n:registrarTag>EXAMPLE2-TAG</n:registrarTag>
        <n:domainListData noDomains="6">
          <n:domainName>epp-example1.co.uk</n:domainName>
          <n:domainName>epp-example2.co.uk</n:domainName>
          <n:domainName>epp-example3.co.uk</n:domainName>
          <n:domainName>epp-example4.co.uk</n:domainName>
          <n:domainName>epp-example5.co.uk</n:domainName>
          <n:domainName>epp-example6.co.uk</n:domainName>
        </n:domainListData>
      </n:relData>
    </resData>
    <trID>
      <clTRID>ABC-12345</clTRID>
      <svTRID>123456</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "Domains Released Notification");
        match data.data {
            super::PollData::NominetDomainReleaseData {
                data: reslease_data,
                change_data: None,
            } => {
                assert_eq!(reslease_data.account_id, "12345");
                assert!(reslease_data.account_moved);
                assert_eq!(reslease_data.from, "EXAMPLE1-TAG");
                assert_eq!(reslease_data.registrar_tag, "EXAMPLE2-TAG");
                assert_eq!(reslease_data.domains.len(), 6);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn nominet_handshake_request() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
   <response>
     <result code="1301">
       <msg>Command completed successfully; ack to dequeue</msg>
     </result>
     <msgQ count="10" id="12345">
       <qDate>2007-09-26T07:31:30</qDate>
       <msg>Registrar Change Authorisation Request</msg>
     </msgQ>
     <resData>
       <n:rcData xmlns:n="http://www.nominet.org.uk/epp/xml/std-notifications-1.2">
         <n:orig>p@epp-example.org.uk</n:orig>
         <n:registrarTag>EXAMPLE</n:registrarTag>
         <n:caseId>3560</n:caseId>
         <n:domainListData noDomains="2" xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
           <domain:infData>
             <domain:name>epp-example1.co.uk</domain:name>
             <domain:roid>57486578-UK</domain:roid>
             <domain:registrant>1245435</domain:registrant>
             <domain:ns>
               <domain:hostObj>ns0.epp-example.co.uk</domain:hostObj>
             </domain:ns>
             <domain:host>ns0.epp-example1.co.uk</domain:host>
             <domain:clID>EPP-EXAMPLE2</domain:clID>
           </domain:infData>
           <domain:infData>
             <domain:name>epp-example2.co.uk</domain:name>
             <domain:roid>57486578-UK</domain:roid>
             <domain:registrant>1245435</domain:registrant>
             <domain:ns>
               <domain:hostObj>ns0.epp-example.co.uk</domain:hostObj>
             </domain:ns>
             <domain:clID>EPP-EXAMPLE2</domain:clID>
           </domain:infData>
         </n:domainListData>
         <contact:infData xmlns:contact="urn:ietf:params:xml:ns:contact-1.0">
          <contact:id>CMyContactID</contact:id>
          <contact:roid>548965487-UK</contact:roid>
          <contact:status s="ok"/>
          <contact:postalInfo type="loc">
           <contact:name>Mr Jones</contact:name>
           <contact:org>Company.</contact:org>
           <contact:addr>
             <contact:street>High Street</contact:street>
             <contact:city>Oxford</contact:city>
             <contact:pc>OX1 1AH</contact:pc>
             <contact:cc>GB</contact:cc>
           </contact:addr>
          </contact:postalInfo>
          <contact:voice>+44.1865658754</contact:voice>
          <contact:email>example@epp-example.org.uk</contact:email>
          <contact:clID>EXAMPLE-TAG</contact:clID>
          <contact:crID>n/a</contact:crID>
          <contact:crDate>2007-05-12T12:44:00Z</contact:crDate>
          <contact:upDate>2008-06-12T06:46:00Z</contact:upDate>
         </contact:infData>
       </n:rcData>
     </resData>
     <trID>
       <clTRID>ABC-12345</clTRID>
       <svTRID>123456</svTRID>
     </trID>
   </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "Registrar Change Authorisation Request");
        match data.data {
            super::PollData::NominetDomainRegistrarChangeData {
                data: rc_data,
                change_data: None,
            } => {
                assert_eq!(rc_data.originator, "p@epp-example.org.uk");
                assert_eq!(rc_data.registrar_tag, "EXAMPLE");
                assert_eq!(rc_data.case_id.unwrap(), "3560");
                assert_eq!(rc_data.contact.id, "CMyContactID");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn nominet_host_cancel() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0"
  xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
  xsi:schemaLocation="http://www.nominet.org.uk/epp/xml/epp-1.0
  epp-1.0.xsd">
  <response>
    <result code="1301">
      <msg>Command completed successfully; ack to dequeue</msg>
    </result>
    <msgQ count="2" id="1">
      <qDate>2008-04-30T13:39:13Z</qDate>
      <msg>Host cancellation notification</msg>
    </msgQ>
    <resData>
      <n:hostCancData xmlns:n="http://www.nominet.org.uk/epp/xml/std-notifications-1.2">
        <n:hostListData noHosts="2">
          <n:hostObj>ns0.example.co.uk.</n:hostObj>
          <n:hostObj>ns1.example.co.uk.</n:hostObj>
        </n:hostListData>
        <n:domainListData noDomains="2">
          <n:domainName>example-a.co.uk</n:domainName>
          <n:domainName>example-b.co.uk</n:domainName>
        </n:domainListData>
      </n:hostCancData>
    </resData>
    <trID>
      <clTRID>EPP-ABC-12345</clTRID>
      <svTRID>203355</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "Host cancellation notification");
        match data.data {
            super::PollData::NominetHostCancelData {
                data: hc_data,
                change_data: None,
            } => {
                assert_eq!(hc_data.host_objects.len(), 2);
                assert_eq!(hc_data.domain_names.len(), 2);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn nominet_data_quality() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0"
  xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
  xsi:schemaLocation="http://www.nominet.org.uk/epp/xml/epp-1.0
  epp-1.0.xsd">
  <response>
    <result code="1301">
      <msg>Command completed successfully; ack to dequeue</msg>
    </result>
    <msgQ count="4" id="123456">
       <qDate>2007-10-06T10:29:30Z</qDate>
       <msg>Data Quality - {{Workflow type}} process commenced notification</msg>
    </msgQ>
    <resData>
       <n:processData stage="initial" xmlns:n="http://www.nominet.org.uk/epp/xml/std-notifications-1.2">
         <contact:infData xmlns:contact="urn:ietf:params:xml:ns:contact-1.0">
           <contact:id>E2CD4B4D83DB0857</contact:id>
           <contact:roid>589695</contact:roid>
           <contact:status s="ok"/>
           <contact:postalInfo type="loc">
             <contact:name>E. Example</contact:name>
             <contact:org>Example Org</contact:org>
             <contact:addr>
               <contact:street>n/a</contact:street>
               <contact:city>n/a</contact:city>
               <contact:sp>n/a</contact:sp>
               <contact:pc>N1 1NA</contact:pc>
               <contact:cc>GB</contact:cc>
             </contact:addr>
           </contact:postalInfo>
           <contact:email>email@epp-example.co.uk</contact:email>
           <contact:clID>TEST</contact:clID>
           <contact:crID>test@epp-example.co.uk</contact:crID>
           <contact:crDate>2009-04-16T11:02:49</contact:crDate>
         </contact:infData>
         <n:processType>{{Workflow type}}</n:processType>
         <n:suspendDate>2010-10-26T00:00:00</n:suspendDate>
         <n:domainListData noDomains="2">
           <n:domainName>epp-example1.co.uk</n:domainName>
           <n:domainName>epp-example2.co.uk</n:domainName>
         </n:domainListData>
       </n:processData>
    </resData>
    <trID>
      <clTRID>EPP-ABC-12345</clTRID>
      <svTRID>203355</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(
            data.message,
            "Data Quality - {{Workflow type}} process commenced notification"
        );
        match data.data {
            super::PollData::NominetProcessData {
                data: p_data,
                change_data: None,
            } => {
                assert_eq!(p_data.contact.id, "E2CD4B4D83DB0857");
                assert_eq!(p_data.process_type, "{{Workflow type}}");
                assert_eq!(p_data.domain_names.len(), 2);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn nominet_data_quality_lifted() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0"
  xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
  xsi:schemaLocation="http://www.nominet.org.uk/epp/xml/epp-1.0
  epp-1.0.xsd">
  <response>
    <result code="1301">
      <msg>Command completed successfully; ack to dequeue</msg>
    </result>
    <msgQ count="4" id="123456">
       <qDate>2007-10-06T10:29:30Z</qDate>
       <msg>DQ Workflow process lifted notification</msg>
     </msgQ>
     <resData>
       <n:processData stage="updated" xmlns:n="http://www.nominet.org.uk/epp/xml/std-notifications-1.2">
         <contact:infData xmlns:contact="urn:ietf:params:xml:ns:contact-1.0">
           <contact:id>E2CD4B4D83DB0857</contact:id>
           <contact:roid>589695</contact:roid>
           <contact:status s="ok"/>
           <contact:postalInfo type="loc">
             <contact:name>E. Example</contact:name>
             <contact:org>Example Org</contact:org>
             <contact:addr>
               <contact:street>n/a</contact:street>
               <contact:city>n/a</contact:city>
               <contact:sp>n/a</contact:sp>
               <contact:pc>N1 1NA</contact:pc>
               <contact:cc>GB</contact:cc>
             </contact:addr>
           </contact:postalInfo>
           <contact:email>email@epp-example.co.uk</contact:email>
           <contact:clID>TEST</contact:clID>
           <contact:crID>test@epp-example.co.uk</contact:crID>
           <contact:crDate>2009-04-16T11:02:49</contact:crDate>
         </contact:infData>
         <n:processType>DQ Workflow</n:processType>
         <n:domainListData noDomains="2">
           <n:domainName>epp-example1.co.uk</n:domainName>
           <n:domainName>epp-example2.co.uk</n:domainName>
         </n:domainListData>
       </n:processData>
     </resData>
    <trID>
      <clTRID>EPP-ABC-12345</clTRID>
      <svTRID>203355</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "DQ Workflow process lifted notification");
        match data.data {
            super::PollData::NominetProcessData {
                data: p_data,
                change_data: None,
            } => {
                assert_eq!(p_data.contact.id, "E2CD4B4D83DB0857");
                assert_eq!(p_data.process_type, "DQ Workflow");
                assert_eq!(p_data.domain_names.len(), 2);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn nominet_domain_suspend() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
    <response>
        <result code="1301">
            <msg>Command completed successfully; ack to dequeue</msg>
        </result>
        <msgQ count="2" id="1">
            <qDate>2008-04-30T13:39:13Z</qDate>
            <msg>Domains Suspended Notification</msg>
        </msgQ>
        <resData>
            <n:suspData xmlns:n="http://www.nominet.org.uk/epp/xml/std-notifications-1.2">
                <n:reason>Data Quality</n:reason>
                <n:cancelDate>2009-12-12T00:00:13Z</n:cancelDate>
                <n:domainListData noDomains="2">
                    <n:domainName>epp-example1.co.uk</n:domainName>
                    <n:domainName>epp-example2.co.uk</n:domainName>
                </n:domainListData>
            </n:suspData>
        </resData>
        <trID>
            <clTRID>EPP-ABC-12345</clTRID>
            <svTRID>203355</svTRID>
        </trID>
    </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "Domains Suspended Notification");
        match data.data {
            super::PollData::NominetSuspendData {
                data: sus_data,
                change_data: None,
            } => {
                assert_eq!(sus_data.reason, "Data Quality");
                assert_eq!(sus_data.domain_names.len(), 2);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn nominet_referral_accept() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
 <epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
   <response>
     <result code="1301">
       <msg>Command completed successfully; ack to dequeue</msg>
     </result>
     <msgQ count="10" id="12345">
       <qDate>2007-09-26T07:31:30</qDate>
       <msg>Referral Accepted Notification</msg>
     </msgQ>
     <resData>
       <domain:creData xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
         <domain:name>epp-example1.ltd.uk</domain:name>
         <domain:crDate>2007-09-25T11:30:45</domain:crDate>
         <domain:exDate>2009-09-25T11:30:45</domain:exDate>
       </domain:creData>
     </resData>
     <trID>
       <clTRID>ABC-12345</clTRID>
       <svTRID>123456</svTRID>
     </trID>
   </response>
 </epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "Referral Accepted Notification");
        match data.data {
            super::PollData::DomainCreateData {
                data: create_data,
                change_data: None,
            } => {
                assert_eq!(create_data.data.name, "epp-example1.ltd.uk");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn nominet_referral_reject() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
 <epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
   <response>
     <result code="1301">
       <msg>Command completed successfully; ack to dequeue</msg>
     </result>
     <msgQ count="10" id="12345">
       <qDate>2007-09-26T07:31:30</qDate>
       <msg>Referral Rejected Notification</msg>
     </msgQ>
     <resData>
       <n:domainFailData xmlns:n="http://www.nominet.org.uk/epp/xml/std-notifications-1.2">
         <n:domainName>epp-example2.ltd.uk</n:domainName>
         <n:reason>V205 Registrant does not match domain name</n:reason>
       </n:domainFailData>
     </resData>
     <trID>
       <clTRID>ABC-12345</clTRID>
       <svTRID>123456</svTRID>
     </trID>
   </response>
 </epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "Referral Rejected Notification");
        match data.data {
            super::PollData::NominetDomainFailData {
                data: fail_data,
                change_data: None,
            } => {
                assert_eq!(fail_data.domain_name, "epp-example2.ltd.uk");
                assert_eq!(
                    fail_data.reason,
                    "V205 Registrant does not match domain name"
                );
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn nominet_registrant_transfer() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1301">
      <msg>Command completed successfully; ack to dequeue</msg>
    </result>
    <msgQ count="4" id="123456">
      <qDate>2007-10-06T10:29:30Z</qDate>
      <msg>Registrant Transfer Notification</msg>
    </msgQ>
    <resData>
      <n:trnData xmlns:n="http://www.nominet.org.uk/epp/xml/std-notifications-1.2">
        <n:orig>p@automaton-example.org.uk</n:orig>
        <n:accountId>58658458</n:accountId>
        <n:oldAccountId>596859</n:oldAccountId>
        <n:domainListData noDomains="2" xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
          <n:domainName>epp-example1.co.uk</n:domainName>
          <n:domainName>epp-example2.co.uk</n:domainName>
        </n:domainListData>
        <contact:infData xmlns:contact="urn:ietf:params:xml:ns:contact-1.0">
          <contact:id>ST68956589R4</contact:id>
          <contact:roid>123456-UK</contact:roid>
          <contact:status s="ok"/>
          <contact:postalInfo type="loc">
           <contact:name>Mr R. Strant</contact:name>
           <contact:addr>
            <contact:street>2102 High Street</contact:street>
            <contact:city>Oxford</contact:city>
            <contact:sp>Oxon</contact:sp>
            <contact:pc>OX1 1QQ</contact:pc>
            <contact:cc>GB</contact:cc>
           </contact:addr>
          </contact:postalInfo>
          <contact:email>example@epp-example1.co.uk</contact:email>
          <contact:clID>TEST</contact:clID>
          <contact:crID>TEST</contact:crID>
          <contact:crDate>1999-04-03T22:00:00.0Z</contact:crDate>
          <contact:upID>domains@isp.com</contact:upID>
          <contact:upDate>1999-12-03T09:00:00.0Z</contact:upDate>
        </contact:infData>
      </n:trnData>
    </resData>
    <trID>
      <clTRID>ABC-12345</clTRID>
      <svTRID>123456</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "Registrant Transfer Notification");
        match data.data {
            super::PollData::NominetRegistrantTransferData {
                data: trn_data,
                change_data: None,
            } => {
                assert_eq!(trn_data.originator, "p@automaton-example.org.uk");
                assert_eq!(trn_data.account_id, "58658458");
                assert_eq!(trn_data.old_account_id, "596859");
                assert_eq!(trn_data.domain_names.len(), 2);
                assert_eq!(trn_data.contact.id, "ST68956589R4");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn centralnic_transfer_accept() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="utf-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
    <response>
        <result code="1301">
            <msg>Command completed successfully; ack to dequeue.</msg>
        </result>
        <msgQ count="1" id="12345"/>
        <resData>
            <domain:trnData xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
                <domain:name>example.uk.com</domain:name>
                <domain:trStatus>clientApproved</domain:trStatus>
                <domain:reID>H12345</domain:reID>
                <domain:reDate>2011-01-27T23:50:00.0Z</domain:reDate>
                <domain:acID>H54321</domain:acID>
                <domain:acDate>2011-02-01T23:50:00.0Z</domain:acDate>
            </domain:trnData>
        </resData>
        <trID>
            <clTRID>abc123</clTRID>
            <svTRID>321cba</svTRID>
        </trID>
    </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "");
        match data.data {
            super::PollData::DomainTransferData {
                data: trn_data,
                change_data: None,
            } => {
                assert_eq!(trn_data.data.name, "example.uk.com");
                assert_eq!(
                    trn_data.data.status,
                    super::super::super::TransferStatus::ClientApproved
                );
                assert_eq!(trn_data.data.requested_client_id, "H12345");
                assert_eq!(trn_data.data.act_client_id, "H54321");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn centralnic_transfer_pending() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="utf-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
    <response>
        <result code="1301">
            <msg>Command completed successfully; ack to dequeue.</msg>
        </result>
        <msgQ count="1" id="12345"/>
        <resData>
            <domain:trnData xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
                <domain:name>example.uk.com</domain:name>
                <domain:trStatus>pending</domain:trStatus>
                <domain:reID>H12345</domain:reID>
                <domain:reDate>2011-01-27T23:50:00.0Z</domain:reDate>
                <domain:acID>H54321</domain:acID>
                <domain:acDate>2011-02-01T23:50:00.0Z</domain:acDate>
            </domain:trnData>
        </resData>
        <trID>
            <clTRID>abc123</clTRID>
            <svTRID>321cba</svTRID>
        </trID>
    </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "");
        match data.data {
            super::PollData::DomainTransferData {
                data: trn_data,
                change_data: None,
            } => {
                assert_eq!(trn_data.data.name, "example.uk.com");
                assert_eq!(
                    trn_data.data.status,
                    super::super::super::TransferStatus::Pending
                );
                assert_eq!(trn_data.data.requested_client_id, "H12345");
                assert_eq!(trn_data.data.act_client_id, "H54321");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn verisign_low_balance() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1301">
      <msg>Command completed successfully; ack to dequeue</msg>
    </result>
  <msgQ count="1" id="12345">
    <qDate>2013-03-25T18:20:07.0078Z</qDate>
    <msg>Low Account Balance</msg>
  </msgQ>
  <resData>
    <lowbalance-poll:pollData
      xmlns:lowbalance-poll=
      "http://www.verisign.com/epp/lowbalance-poll-1.0">
      <lowbalance-poll:registrarName>Test Registar</lowbalance-poll:registrarName>
      <lowbalance-poll:creditLimit>1000</lowbalance-poll:creditLimit>
      <lowbalance-poll:creditThreshold type="PERCENT">10</lowbalance-poll:creditThreshold>
      <lowbalance-poll:availableCredit>80</lowbalance-poll:availableCredit>
      </lowbalance-poll:pollData>
    </resData>
    <trID>
      <clTRID>ABC-12345</clTRID>
      <svTRID>54322-XYZ</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "Low Account Balance");
        match data.data {
            super::PollData::VerisignLowBalanceData(bal_data) => {
                assert_eq!(bal_data.registrar_name, "Test Registar");
                assert_eq!(bal_data.credit_limit, "1000");
                assert_eq!(
                    bal_data.credit_threshold,
                    super::super::super::verisign::CreditThreshold::Percentage(10)
                );
                assert_eq!(bal_data.available_credit, "80");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn rrpproxy_renew() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
 <response>
   <result code="1301">
     <msg>Command completed successfully; ack to dequeue</msg>
   </result>
   <msgQ count="1" id="28">
     <qDate>2009-04-14T13:23:50.0Z</qDate>
     <msg>DOMAIN_RENEWAL_SUCCESSFUL</msg>
   </msgQ>
   <resData>
     <domain:renData xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
       <domain:name>siatki.eu</domain:name>
     </domain:renData>
   </resData>
   <trID>
     <clTRID>AE7F32C2-28F7-11DE-A163-8000000099E9</clTRID>
     <svTRID>F8471712-28F7-11DE-900C-B7CCEEA560E0</svTRID>
   </trID>
 </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "DOMAIN_RENEWAL_SUCCESSFUL");
        match data.data {
            super::PollData::DomainRenewData {
                data: ren_data,
                change_data: None,
            } => {
                assert_eq!(ren_data.data.name, "siatki.eu");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn rrpproxy_domain_pan_failed() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1301">
      <msg>Command completed successfully; ack to dequeue</msg>
    </result>
    <msgQ count="1" id="2351">
      <qDate>2015-02-25T14:07:18.0Z</qDate>
      <msg>DOMAIN_RESTORE_FAILED</msg>
    </msgQ>
    <resData>
      <domain:panData xmlns:domain="urn:ietf:params:xml:ns:domain-1.0">
        <domain:name paResult="0">example.com</domain:name>
        <domain:paTRID>
          <clTRID>ECA21919-4B41-40BB-8A9F-ED6849950154</clTRID>
          <svTRID>33a2eb76-4295-43f1-a1f6-c757e8d1be41</svTRID>
        </domain:paTRID>
        <domain:paDate>2015-02-25T14:07:18.0Z</domain:paDate>
      </domain:panData>
    </resData>
    <trID>
      <clTRID>8C8B693B-B5E5-47D9-B40A-0FDC10307DF7</clTRID>
      <svTRID>434ec058-9640-4deb-85c6-8e7f497e4acf</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "DOMAIN_RESTORE_FAILED");
        match data.data {
            super::PollData::DomainPanData {
                data: pan_data,
                change_data: None,
            } => {
                assert_eq!(pan_data.name, "example.com");
                assert!(!pan_data.result);
                assert_eq!(
                    pan_data.server_transaction_id.unwrap(),
                    "33a2eb76-4295-43f1-a1f6-c757e8d1be41"
                );
                assert_eq!(
                    pan_data.client_transaction_id.unwrap(),
                    "ECA21919-4B41-40BB-8A9F-ED6849950154"
                );
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn eurid_poll_0() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:poll-1.2="http://www.eurid.eu/xml/epp/poll-1.2">
  <response>
    <result code="1301">
      <msg>Command completed successfully; ack to dequeue</msg>
    </result>
    <msgQ count="1" id="d6f48a64-862e-4490-9835-31fd02fb74d4">
      <qDate>2019-11-06T12:19:29.223Z</qDate>
      <msg>Suspended domain name: abcabc-1573042768420.eu</msg>
    </msgQ>
    <resData>
      <poll-1.2:pollData>
        <poll-1.2:context>LEGAL</poll-1.2:context>
        <poll-1.2:objectType>DOMAIN</poll-1.2:objectType>
        <poll-1.2:object>abcabc-1573042768420.eu</poll-1.2:object>
        <poll-1.2:objectUnicode>abcabc-1573042768420.eu</poll-1.2:objectUnicode>
        <poll-1.2:action>SUSPENDED</poll-1.2:action>
        <poll-1.2:code>2110</poll-1.2:code>
        <poll-1.2:detail>A really good reason for suspending</poll-1.2:detail>
      </poll-1.2:pollData>
    </resData>
    <trID>
      <clTRID>poll01-req</clTRID>
      <svTRID>e0b0ae1b7-66b9-475f-a28a-40f94e727061</svTRID>
    </trID>
  </response>
</epp>
"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(
            data.message,
            "Suspended domain name: abcabc-1573042768420.eu"
        );
        match data.data {
            super::PollData::EURIDPoll(data) => {
                assert_eq!(data.context, "LEGAL");
                assert_eq!(data.object_type, "DOMAIN");
                assert_eq!(data.object, "abcabc-1573042768420.eu");
                assert_eq!(data.object_unicode.unwrap(), "abcabc-1573042768420.eu");
                assert_eq!(data.action, "SUSPENDED");
                assert_eq!(data.code, 2110);
                assert_eq!(data.detail.unwrap(), "A really good reason for suspending");
                assert!(data.registrar.is_none());
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn eurid_poll_1() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:poll-1.2="http://www.eurid.eu/xml/epp/poll-1.2">
  <response>
    <result code="1301">
      <msg>Command completed successfully; ack to dequeue</msg>
    </result>
    <msgQ count="1" id="d6108143-3dd4-4c46-b6be-604027dbea54">
      <qDate>2019-11-08T12:19:40.822Z</qDate>
      <msg>Domain name quarantined: abcabc-1573042778986.eu</msg>
    </msgQ>
    <resData>
      <poll-1.2:pollData>
        <poll-1.2:context>DOMAIN</poll-1.2:context>
        <poll-1.2:objectType>DOMAIN</poll-1.2:objectType>
        <poll-1.2:object>abcabc-1573042778986.eu</poll-1.2:object>
        <poll-1.2:objectUnicode>abcabc-1573042778986.eu</poll-1.2:objectUnicode>
        <poll-1.2:action>QUARANTINED</poll-1.2:action>
        <poll-1.2:code>1700</poll-1.2:code>
      </poll-1.2:pollData>
    </resData>
    <trID>
      <clTRID>poll03-req</clTRID>
      <svTRID>eaa2ed561-3e37-42ae-ad6f-2992a87bb0ba</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(
            data.message,
            "Domain name quarantined: abcabc-1573042778986.eu"
        );
        match data.data {
            super::PollData::EURIDPoll(data) => {
                assert_eq!(data.context, "DOMAIN");
                assert_eq!(data.object_type, "DOMAIN");
                assert_eq!(data.object, "abcabc-1573042778986.eu");
                assert_eq!(data.object_unicode.unwrap(), "abcabc-1573042778986.eu");
                assert_eq!(data.action, "QUARANTINED");
                assert_eq!(data.code, 1700);
                assert!(data.detail.is_none());
                assert!(data.registrar.is_none());
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn eurid_poll_2() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0" xmlns:poll-1.2="http://www.eurid.eu/xml/epp/poll-1.2">
  <response>
    <result code="1301">
      <msg>Command completed successfully; ack to dequeue</msg>
    </result>
    <msgQ count="5" id="7fe53591-f556-4408-a962-bbb79dacd00a">
      <qDate>2019-11-30T23:00:15.662Z</qDate>
      <msg>Watermark level reached: 7</msg>
    </msgQ>
    <resData>
      <poll-1.2:pollData>
        <poll-1.2:context>REGISTRATION_LIMIT</poll-1.2:context>
        <poll-1.2:objectType>WATERMARK</poll-1.2:objectType>
        <poll-1.2:object>7</poll-1.2:object>
        <poll-1.2:objectUnicode>7</poll-1.2:objectUnicode>
        <poll-1.2:action>REACHED</poll-1.2:action>
        <poll-1.2:code>2620</poll-1.2:code>
      </poll-1.2:pollData>
    </resData>
    <trID>
      <clTRID>poll07-req</clTRID>
      <svTRID>e24e57eb2-4291-4776-9032-4143785dcfb3</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA.trim()).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_poll_response(
            *res, &crate::metrics::DummyMetrics::default()).unwrap().unwrap();
        assert_eq!(data.message, "Watermark level reached: 7");
        match data.data {
            super::PollData::EURIDPoll(data) => {
                assert_eq!(data.context, "REGISTRATION_LIMIT");
                assert_eq!(data.object_type, "WATERMARK");
                assert_eq!(data.object, "7");
                assert_eq!(data.object_unicode.unwrap(), "7");
                assert_eq!(data.action, "REACHED");
                assert_eq!(data.code, 2620);
                assert!(data.detail.is_none());
                assert!(data.registrar.is_none());
            }
            _ => unreachable!(),
        }
    }
}
