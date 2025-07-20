use super::{HeaderInfo, Response};
use crate::client;

#[derive(serde::Deserialize)]
pub struct HostCreate {
    name: String,
    #[serde(default)]
    ipv4: Vec<String>,
    #[serde(default)]
    ipv6: Vec<String>,
}

crate::rpp_method!(
    name = host_check,
    method = HEAD,
    url = "/<registry_id>/hosts/<host>",
    args = (host: &str),
    return_type = (),
    handler = |mut c, h: HeaderInfo, host| async move {
        let res = client::host::check(host, h.client_transaction_id, &mut c).await?;
        let mut resp = Response::from(&res);
        resp.check_availability = Some(res.response.avail);
        Ok(resp)
    }
);

crate::rpp_method!(
    name = host_create,
    method = POST,
    url = "/<registry_id>/hosts",
    data_type = HostCreate,
    return_type = (),
    handler = |mut c, h: HeaderInfo, request: HostCreate| async move {
        let mut addresses = vec![];
        for v4 in request.ipv4 {
            addresses.push(client::host::Address {
                address: v4,
                ip_version: client::host::AddressVersion::IPv4,
            })
        }
        for v6 in request.ipv6 {
            addresses.push(client::host::Address {
                address: v6,
                ip_version: client::host::AddressVersion::IPv6,
            })
        }

        let res = client::host::create(&request.name, addresses, None, None, h.client_transaction_id, &mut c).await?;
        let resp = Response::from(&res);
        Ok(resp)
    }
);

crate::rpp_method!(
    name = host_delete,
    method = DELETE,
    url = "/<registry_id>/hosts/<host>",
    args = (host: &str),
    return_type = (),
    handler = |mut c, h: HeaderInfo, host| async move {
        let res = client::host::delete(host, h.client_transaction_id, &mut c).await?;
        Ok(Response::from(&res))
    }
);
