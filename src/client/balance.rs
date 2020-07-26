//! EPP commands relating to balance enquiries

use super::router::HandleReqReturn;
use super::{proto, EPPClientServerFeatures, Error, Request, Response, Sender};

#[derive(Debug)]
pub struct BalanceRequest {
    pub return_path: Sender<BalanceResponse>,
}

#[derive(Debug)]
pub struct BalanceResponse {
    pub balance: String,
    pub currency: String,
    pub credit_limit: Option<String>,
    pub available_credit: Option<String>,
    pub credit_threshold: Option<CreditThreshold>,
}

#[derive(Debug, PartialEq)]
pub enum CreditThreshold {
    Fixed(String),
    Percentage(u8),
}

pub fn handle_balance(
    client: &EPPClientServerFeatures,
    _req: &BalanceRequest,
) -> HandleReqReturn<BalanceResponse> {
    if client.switch_balance {
        Ok((
            proto::EPPCommandType::Info(proto::EPPInfo::SwitchBalace {}),
            None,
        ))
    } else if client.verisign_balance || client.has_erratum("rrpproxy") {
        Ok((
            proto::EPPCommandType::Info(proto::EPPInfo::VerisignBalace {}),
            None,
        ))
    } else {
        Err(Err(Error::Unsupported))
    }
}

pub fn handle_balance_response(response: proto::EPPResponse) -> Response<BalanceResponse> {
    match response.data {
        Some(value) => match value.value {
            proto::EPPResultDataValue::SwitchBalanceInfoResult(switch_balance) => {
                Response::Ok(BalanceResponse {
                    balance: switch_balance.balance,
                    currency: switch_balance.currency,
                    credit_limit: None,
                    available_credit: None,
                    credit_threshold: None,
                })
            }
            proto::EPPResultDataValue::VerisignBalanceInfoResult(verisign_balance) => {
                Response::Ok(BalanceResponse {
                    balance: verisign_balance.balance,
                    currency: "USD".to_string(),
                    credit_limit: Some(verisign_balance.credit_limit),
                    available_credit: Some(verisign_balance.available_credit),
                    credit_threshold: Some(match verisign_balance.credit_threshold {
                        proto::verisign::EPPCreditThreshold::Fixed(f) => CreditThreshold::Fixed(f),
                        proto::verisign::EPPCreditThreshold::Percentage(p) => {
                            CreditThreshold::Percentage(p)
                        }
                    }),
                })
            }
            _ => Err(Error::InternalServerError),
        },
        None => Err(Error::InternalServerError),
    }
}

/// Makes a balance enquiry to the registry
///
/// # Arguments
/// * `client_sender` - Reference to the tokio channel into the client
pub async fn balance_info(
    client_sender: &mut futures::channel::mpsc::Sender<Request>,
) -> Result<BalanceResponse, super::Error> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    super::send_epp_client_request(
        client_sender,
        Request::Balance(Box::new(BalanceRequest {
            return_path: sender,
        })),
        receiver,
    )
    .await
}

#[cfg(test)]
mod balance_tests {
    #[test]
    fn switch_balance() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
    <response>
        <result code="1000">
            <msg lang="en">Command completed successfully</msg>
        </result>
        <resData>
            <infData xmlns="https://www.nic.ch/epp/balance-1.0">
                <balance>27.05</balance>
                <currency>CHF</currency>
            </infData>
        </resData>
        <trID>
            <clTRID>b4e118c9-b2ea-41f3-bfa7-d8238b5a224d</clTRID>
            <svTRID>20200615.116639549.1185125979</svTRID>
        </trID>
    </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_balance_response(*res).unwrap();
        assert_eq!(data.balance, "27.05");
        assert_eq!(data.currency, "CHF");
    }

    #[test]
    fn verisign_percent_balance() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <balance:infData
        xmlns:balance="http://www.verisign.com/epp/balance-1.0">
        <balance:creditLimit>1000.00</balance:creditLimit>
        <balance:balance>200.00</balance:balance>
        <balance:availableCredit>800.00</balance:availableCredit>
        <balance:creditThreshold>
          <balance:percent>50</balance:percent>
        </balance:creditThreshold>
      </balance:infData>
    </resData>
    <trID>
      <clTRID>ABC-12345</clTRID>
      <svTRID>54322-XYZ</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_balance_response(*res).unwrap();
        assert_eq!(data.balance, "200.00");
        assert_eq!(data.currency, "USD");
        assert_eq!(data.credit_limit.unwrap(), "1000.00");
        assert_eq!(data.available_credit.unwrap(), "800.00");
        assert_eq!(data.credit_threshold.unwrap(), super::CreditThreshold::Percentage(50));
    }

    #[test]
    fn verisign_fixed_balance() {
        const XML_DATA: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<epp xmlns="urn:ietf:params:xml:ns:epp-1.0">
  <response>
    <result code="1000">
      <msg>Command completed successfully</msg>
    </result>
    <resData>
      <balance:infData
        xmlns:balance="http://www.verisign.com/epp/balance-1.0">
        <balance:creditLimit>1000.00</balance:creditLimit>
        <balance:balance>200.00</balance:balance>
        <balance:availableCredit>800.00</balance:availableCredit>
        <balance:creditThreshold>
          <balance:fixed>500.00</balance:fixed>
        </balance:creditThreshold>
      </balance:infData>
    </resData>
    <trID>
      <clTRID>ABC-12345</clTRID>
      <svTRID>54322-XYZ</svTRID>
    </trID>
  </response>
</epp>"#;
        let res: super::proto::EPPMessage = xml_serde::from_str(XML_DATA).unwrap();
        let res = match res.message {
            super::proto::EPPMessageType::Response(r) => r,
            _ => unreachable!(),
        };
        let data = super::handle_balance_response(*res).unwrap();
        assert_eq!(data.balance, "200.00");
        assert_eq!(data.currency, "USD");
        assert_eq!(data.credit_limit.unwrap(), "1000.00");
        assert_eq!(data.available_credit.unwrap(), "800.00");
        assert_eq!(data.credit_threshold.unwrap(), super::CreditThreshold::Fixed("500.00".to_string()));
    }
}
