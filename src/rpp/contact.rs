use super::{HeaderInfo, Response};
use crate::client;

#[derive(serde::Deserialize)]
pub struct ContactCreate {
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    organisation_name: Option<String>,
    address: Address,
    #[serde(default)]
    email: String,
    #[serde(default)]
    phone: Option<String>,
    #[serde(default)]
    fax: Option<String>,
    #[serde(rename = "authInfo")]
    auth_info: super::AuthInfo,
}

#[derive(serde::Deserialize)]
pub struct Address {
    #[serde(default)]
    street: Vec<String>,
    #[serde(default)]
    city: String,
    #[serde(default, rename = "stateProvidence")]
    province: Option<String>,
    #[serde(default, rename = "postalCode")]
    postal_code: Option<String>,
    #[serde(default)]
    country: String,
}

crate::rpp_method!(
    name = contact_check,
    method = HEAD,
    url = "/<registry_id>/contacts/<id>",
    args = (id: &str),
    return_type = (),
    handler = |mut c, h: HeaderInfo, id| async move {
        let res = client::contact::check(id, h.client_transaction_id, &mut c).await?;
        let mut resp = Response::from(&res);
        resp.check_availability = Some(res.response.avail);
        Ok(resp)
    }
);

crate::rpp_method!(
    name = contact_create,
    method = POST,
    url = "/<registry_id>/contacts",
    data_type = ContactCreate,
    return_type = (),
    handler = |mut c, h: HeaderInfo, request: ContactCreate| async move {
        let res = client::contact::create(&request.id, client::contact::NewContactData {
            local_address: Some(client::contact::Address {
                name: request.name,
                organisation: request.organisation_name,
                streets: request.address.street,
                city: request.address.city,
                province: request.address.province,
                postal_code: request.address.postal_code,
                country_code: request.address.country,
                identity_number: None,
                birth_date: None
            }),
            internationalised_address: None,
            phone: request.phone.map(|p| client::Phone {
                number: p,
                extension: None,
            }),
            fax: request.fax.map(|p| client::Phone {
                number: p,
                extension: None,
            }),
            email: request.email,
            entity_type: None,
            trading_name: None,
            company_number: None,
            disclosure: None,
            auth_info: request.auth_info.pw.unwrap_or_default(),
            eurid_info: None,
            isnic_info: None,
            qualified_lawyer: None,
            keysys: None
        }, h.client_transaction_id, &mut c).await?;
        let resp = Response::from(&res);
        Ok(resp)
    }
);

crate::rpp_method!(
    name = contact_delete,
    method = DELETE,
    url = "/<registry_id>/contacts/<id>",
    args = (id: &str),
    return_type = (),
    handler = |mut c, h: HeaderInfo, id| async move {
        let res = client::contact::delete(id, h.client_transaction_id, &mut c).await?;
        Ok(Response::from(&res))
    }
);
