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
    auth_info: super::AuthInfo,
}

#[derive(serde::Serialize)]
pub struct DomainInfo {
    name: String,
    ns: DomainHosts,
    contacts: Vec<DomainContactReference>,
    // TODO: dnsSEC
    #[serde(rename = "authInfo")]
    auth_info: super::AuthInfo,
    status: Vec<DomainStatus>,
    #[serde(rename = "crDate", skip_serializing_if = "Option::is_none")]
    creation_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "exDate", skip_serializing_if = "Option::is_none")]
    expiry_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "upDate", skip_serializing_if = "Option::is_none")]
    update_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "trDate", skip_serializing_if = "Option::is_none")]
    transfer_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "clId")]
    client_id: String,
    #[serde(rename = "crId", skip_serializing_if = "Option::is_none")]
    client_created_id: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum DomainStatus {
    #[serde(rename = "clientDeleteProhibited")]
    ClientDeleteProhibited,
    #[serde(rename = "clientHold")]
    ClientHold,
    #[serde(rename = "clientRenewProhibited")]
    ClientRenewProhibited,
    #[serde(rename = "clientTransferProhibited")]
    ClientTransferProhibited,
    #[serde(rename = "clientUpdateProhibited")]
    ClientUpdateProhibited,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "pendingCreate")]
    PendingCreate,
    #[serde(rename = "pendingDelete")]
    PendingDelete,
    #[serde(rename = "pendingRenew")]
    PendingRenew,
    #[serde(rename = "pendingTransfer")]
    PendingTransfer,
    #[serde(rename = "pendingUpdate")]
    PendingUpdate,
    #[serde(rename = "serverDeleteProhibited")]
    ServerDeleteProhibited,
    #[serde(rename = "serverHold")]
    ServerHold,
    #[serde(rename = "serverRenewProhibited")]
    ServerRenewProhibited,
    #[serde(rename = "serverTransferProhibited")]
    ServerTransferProhibited,
    #[serde(rename = "serverUpdateProhibited")]
    ServerUpdateProhibited,
}

impl From<client::domain::Status> for DomainStatus {
    fn from(status: client::domain::Status) -> Self {
        match status {
            client::domain::Status::ClientDeleteProhibited => DomainStatus::ClientDeleteProhibited,
            client::domain::Status::ClientHold => DomainStatus::ClientHold,
            client::domain::Status::ClientRenewProhibited => DomainStatus::ClientRenewProhibited,
            client::domain::Status::ClientTransferProhibited => DomainStatus::ClientTransferProhibited,
            client::domain::Status::ClientUpdateProhibited => DomainStatus::ClientUpdateProhibited,
            client::domain::Status::Inactive => DomainStatus::Inactive,
            client::domain::Status::Ok => DomainStatus::Ok,
            client::domain::Status::PendingCreate => DomainStatus::PendingCreate,
            client::domain::Status::PendingDelete => DomainStatus::PendingDelete,
            client::domain::Status::PendingRenew => DomainStatus::PendingRenew,
            client::domain::Status::PendingTransfer => DomainStatus::PendingTransfer,
            client::domain::Status::PendingUpdate => DomainStatus::PendingUpdate,
            client::domain::Status::ServerDeleteProhibited => DomainStatus::ServerDeleteProhibited,
            client::domain::Status::ServerHold => DomainStatus::ServerHold,
            client::domain::Status::ServerRenewProhibited => DomainStatus::ServerRenewProhibited,
            client::domain::Status::ServerTransferProhibited => DomainStatus::ServerTransferProhibited,
            client::domain::Status::ServerUpdateProhibited => DomainStatus::ServerUpdateProhibited,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
struct DomainHostObject {
    name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DomainHostAttr {
    name: String,
    #[serde(default)]
    ipv4: Vec<String>,
    #[serde(default)]
    ipv6: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DomainContactReference {
    value: String,
    #[serde(rename = "type")]
    contact_type: Vec<ContactType>,
}

#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
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

crate::rpp_method!(
    name = domain_check,
    method = HEAD,
    url = "/<registry_id>/domains/<domain>",
    args = (domain: &str),
    return_type = (),
    handler = |mut c, h: HeaderInfo, domain| async move {
        let res = client::domain::check(domain, None, None, None, h.client_transaction_id, &mut c).await?;
        let mut resp = Response::from(&res);
        resp.check_availability = Some(res.response.avail);
        Ok(resp)
    }
);

crate::rpp_method!(
    name = domain_info,
    method = GET,
    url = "/<registry_id>/domains/<domain>",
    args = (domain: &str),
    return_type = DomainInfo,
    handler = |mut c, h: HeaderInfo, domain| async move {
        let res = client::domain::info(domain, None, None, None, None, h.client_transaction_id, &mut c).await?;
        let mut resp = Response::from(&res);

        let ns = if res.response.nameservers.is_empty() {
            Default::default()
        } else if matches!(res.response.nameservers[0], client::domain::InfoNameserver::HostAndAddress { .. }) {
            DomainHosts::HostAttr(res.response.nameservers.into_iter().map(|h| match h {
                client::domain::InfoNameserver::HostAndAddress { host: name, addresses, .. } => {
                    let mut ipv4 = vec![];
                    let mut ipv6 = vec![];

                    for addr in addresses {
                        match addr.ip_version {
                            client::host::AddressVersion::IPv4 => ipv4.push(addr.address),
                            client::host::AddressVersion::IPv6 => ipv6.push(addr.address),
                        }
                    }

                    DomainHostAttr {
                        name,
                        ipv4,
                        ipv6
                    }
                },
                _ => unreachable!()
            }).collect())
        } else if matches!(res.response.nameservers[0], client::domain::InfoNameserver::HostOnly(_)) {
            DomainHosts::HostObject(res.response.nameservers.into_iter().map(|h| match h {
                client::domain::InfoNameserver::HostOnly(name) => DomainHostObject {
                    name
                },
                _ => unreachable!()
            }).collect())
        } else {
            unreachable!();
        };

        let mut contacts = std::collections::HashMap::<String, std::collections::HashSet<ContactType>>::new();
        contacts.insert(res.response.registrant, {
            let mut s = std::collections::HashSet::new();
            s.insert(ContactType::Registrant);
            s
        });

        for contact in res.response.contacts {
            let contact_type = match contact.contact_type.as_str() {
                "admin" => ContactType::Admin,
                "tech" => ContactType::Tech,
                "billing" => ContactType::Billing,
                _ => continue
            };
            contacts.entry(contact.contact_id)
                .or_insert_with(std::collections::HashSet::new)
                .insert(contact_type);
        }

        resp.body = Some(DomainInfo {
            name: res.response.name,
            ns,
            contacts: contacts.into_iter().map(|(k, v)| {
                DomainContactReference {
                    value: k,
                    contact_type: v.into_iter().collect()
                }
            }).collect(),
            auth_info: super::AuthInfo {
                pw: res.response.auth_info
            },
            status: res.response.statuses.into_iter().map(Into::into).collect(),
            creation_date: res.response.creation_date,
            expiry_date: res.response.expiry_date,
            update_date: res.response.last_updated_date,
            transfer_date: res.response.last_transfer_date,
            client_id: res.response.client_id,
            client_created_id: res.response.client_created_id,
        });
        Ok(resp)
    }
);

crate::rpp_method!(
    name = domain_create,
    method = POST,
    url = "/<registry_id>/domains",
    data_type = DomainCreate,
    return_type = (),
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
            auth_info: request.auth_info.pw.as_deref().unwrap_or_default(),
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

crate::rpp_method!(
    name = domain_delete,
    method = DELETE,
    url = "/<registry_id>/domains/<domain>",
    args = (domain: &str),
    return_type = (),
    handler = |mut c, h: HeaderInfo, domain| async move {
        let res = client::domain::delete(domain, None, None, None, None, h.client_transaction_id, &mut c).await?;
        Ok(Response::from(&res))
    }
);