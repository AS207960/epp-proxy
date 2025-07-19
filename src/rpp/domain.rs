use super::{HeaderInfo, Response};
use crate::client;

#[derive(serde::Deserialize)]
pub struct DomainCreate {
    name: String,
    // TODO: process
    #[serde(default)]
    ns: DomainHosts,
    #[serde(default)]
    contacts: Vec<DomainContactReference>,
    // TODO: dnsSEC
    #[serde(rename = "authInfo")]
    auth_info: DomainAuthInfo,
}

#[derive(serde::Deserialize)]
enum DomainHosts {
    #[serde(rename = "hostObj")]
    HostObject(Vec<DomainHostObject>),
    #[serde(rename = "hostAttr")]
    HostAttr(Vec<DomainHostAttr>),
}

impl Default for DomainHosts {
    fn default() -> Self {
        DomainHosts::HostObject(vec![])
    }
}

#[derive(serde::Deserialize)]
struct DomainHostObject {
    name: String,
}

#[derive(serde::Deserialize)]
struct DomainHostAttr {
    name: String,
    #[serde(default)]
    ipv4: Vec<String>,
    #[serde(default)]
    ipv6: Vec<String>,
}

#[derive(serde::Deserialize)]
struct DomainContactReference {
    value: String,
    #[serde(rename = "type")]
    contact_type: Vec<ContactType>,
}

#[derive(serde::Deserialize, Eq, PartialEq)]
enum ContactType {
    #[serde(rename = "registrant")]
    Registrant,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "tech")]
    Tech,
    #[serde(rename = "billing")]
    Billing,
}

#[derive(serde::Deserialize)]
struct DomainAuthInfo {
    pw: String,
}

crate::rpp_method!(
    name = domain_check,
    method = HEAD,
    url = "/rpp/v0/<registry_id>/domains/<domain>",
    return_type = &'static str,
    handler = |mut c, h: HeaderInfo, domain| async move {
        let res = client::domain::check(domain, None, None, None, h.client_transaction_id, &mut c).await?;
        let mut resp = Response::from(&res);
        resp.check_availability = Some(res.response.avail);
        Ok(resp)
    },
    args = domain: &str
);


crate::rpp_method!(
    name = domain_create,
    method = POST,
    url = "/rpp/v0/<registry_id>/domains",
    data_type = DomainCreate,
    return_type = &'static str,
    handler = |mut c, h: HeaderInfo, request: DomainCreate| async move {
        let registrant = request.contacts.iter()
            .filter(|c| c.contact_type.contains(&ContactType::Registrant))
            .map(|c| c.value.as_str())
            .next()
            .unwrap_or_default();

        let mut contacts = vec![];
        for contact in request.contacts.iter() {
            if contact.contact_type.contains(&ContactType::Admin) {
                contacts.push(client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: contact.value.clone(),
                });
            }
            if contact.contact_type.contains(&ContactType::Tech) {
                contacts.push(client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: contact.value.clone(),
                });
            }
            if contact.contact_type.contains(&ContactType::Billing) {
                contacts.push(client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: contact.value.clone(),
                });
            }
        }

        let mut nameservers = vec![];
        match request.ns {
            DomainHosts::HostObject(obj) => {
                for obj in obj {
                    nameservers.push(client::domain::InfoNameserver::HostOnly(obj.name))
                }
            }
            DomainHosts::HostAttr(attr) => {
                for obj in attr {
                    let mut addresses = vec![];
                    for v4 in obj.ipv4 {
                        addresses.push(client::host::Address {
                            address: v4,
                            ip_version: client::host::AddressVersion::IPv4,
                        })
                    }
                    for v6 in obj.ipv6 {
                        addresses.push(client::host::Address {
                            address: v6,
                            ip_version: client::host::AddressVersion::IPv6,
                        })
                    }
                    nameservers.push(client::domain::InfoNameserver::HostAndAddress {
                        host: obj.name,
                        addresses,
                        eurid_idn: None,
                    })
                }
            }
        }

        let res = client::domain::create(client::domain::CreateInfo {
            domain: &request.name,
            period: None,
            registrant,
            contacts,
            nameservers,
            auth_info: &request.auth_info.pw,
            sec_dns: None,
            launch_create: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            eurid_data: None,
            isnic_payment: None,
            personal_registration: None,
            keysys: None,
            nominet_ext: None,
            ttl: None
        }, h.client_transaction_id, &mut c).await?;
        let resp = Response::from(&res);
        Ok(resp)
    }
);