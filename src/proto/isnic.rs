#[derive(Debug, Deserialize)]
pub struct DomainInfo {
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-domain-1.0}contact", default)]
    pub contacts: Vec<DomainContact>,
}

#[derive(Debug, Deserialize)]
pub struct DomainContact {
    #[serde(rename = "$attr:type")]
    pub contact_type: DomainContactType,
    #[serde(rename = "$value")]
    pub contact_id: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum DomainContactType {
    #[serde(rename = "zone")]
    Zone,
}

#[derive(Debug, Serialize)]
pub struct DomainCreateRenew {
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-domain-1.0}is-domain:cardID")]
    pub card_id: u32,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-domain-1.0}is-domain:cardCVC")]
    pub card_cvc: String,
}

#[derive(Debug, Serialize)]
pub struct DomainUpdate {
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-domain-1.0}is-domain:rem",
        skip_serializing_if = "Option::is_none"
    )]
    pub remove: Option<DomainUpdateRemove>,
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-domain-1.0}is-domain:chg",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<DomainUpdateChange>,
}

#[derive(Debug, Serialize)]
pub struct DomainUpdateRemove {
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-domain-1.0}is-domain:ns_all")]
    pub ns_all: bool,
}

#[derive(Debug, Serialize)]
pub struct DomainUpdateChange {
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-domain-1.0}is-domain:master_ns",
        skip_serializing_if = "Option::is_none"
    )]
    pub master_ns: Option<DomainUpdateNS>,
}

#[derive(Debug, Serialize)]
pub struct DomainUpdateNS {
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-domain-1.0}is-domain:hostObj")]
    pub hosts: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContactInfo {
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}type")]
    pub contact_type: ContactType,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}status")]
    pub status: Vec<ContactStatus>,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}mobile", default)]
    pub mobile: Option<super::contact::EPPContactPhone>,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}sid", default)]
    pub sid: Option<String>,
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}autoUpdateFromNationalRegistry",
        default
    )]
    pub auto_update_from_national_registry: Option<bool>,
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}cancelPaper",
        default
    )]
    pub cancel_paper: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ContactCreate {
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:type")]
    pub contact_type: ContactType,
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:mobile",
        skip_serializing_if = "Option::is_none"
    )]
    pub mobile: Option<super::contact::EPPContactPhone>,
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub sid: Option<String>,
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:autoUpdateFromNationalRegistry",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_update_from_national_registry: Option<bool>,
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:cancelPaper",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancel_paper: Option<bool>,
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:lang",
        skip_serializing_if = "Option::is_none"
    )]
    pub lang: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ContactUpdate {
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:chg",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<ContactUpdateChange>,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:oldPW")]
    pub old_password: String,
}

#[derive(Debug, Serialize)]
pub struct ContactUpdateChange {
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:mobile",
        skip_serializing_if = "Option::is_none"
    )]
    pub mobile: Option<super::contact::EPPContactPhone>,
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:autoUpdateFromNationalRegistry",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_update_from_national_registry: Option<bool>,
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:cancelPaper",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancel_paper: Option<bool>,
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-contact-1.0}is-contact:lang",
        skip_serializing_if = "Option::is_none"
    )]
    pub lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContactType {
    #[serde(rename = "person")]
    Person,
    #[serde(rename = "role")]
    Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactStatus {
    #[serde(rename = "$attr:s")]
    pub status_type: ContactStatusType,
    #[serde(rename = "$attr:lang", default)]
    pub language: Option<String>,
    #[serde(rename = "$value", default)]
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContactStatusType {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "okUnconfirmed")]
    OkUnconfirmed,
    #[serde(rename = "pendingCreate")]
    PendingCreate,
    #[serde(rename = "serverExpired")]
    ServerExpired,
    #[serde(rename = "serverSuspended")]
    ServerSuspended,
}

#[derive(Debug, Serialize)]
pub struct HostCreateUpdate {
    #[serde(
        rename = "{urn:is.isnic:xml:ns:is-ext-host-1.0}is-host:contact",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contacts: Vec<HostContact>,
}

#[derive(Debug, Serialize)]
pub struct HostContact {
    #[serde(rename = "$value")]
    pub id: String,
    #[serde(rename = "$attr:type")]
    pub contact_type: HostContactType,
}

#[derive(Debug, Serialize)]
pub enum HostContactType {
    #[serde(rename = "admin")]
    Admin,
}

#[derive(Debug, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}account", default)]
    pub accounts: Vec<Account>,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}id")]
    pub id: u32,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}contactHandle")]
    pub handle: String,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}allowedContactHandles")]
    pub allowed_contact_handles: AccountAllowedContactHandles,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}number", default)]
    pub number: Option<String>,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}owner")]
    pub owner: String,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}expireYear")]
    pub expiry_year: u32,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}expireMonth")]
    pub expiry_month: u32,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}brand")]
    pub brand: AccountBrand,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}currency", default)]
    pub currency: Option<String>,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}balance")]
    pub balance: f64,
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}backupAccountID")]
    pub backup_account_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct AccountAllowedContactHandles {
    #[serde(rename = "{urn:is.isnic:xml:ns:is-ext-account-1.0}contact")]
    pub contacts: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub enum AccountBrand {
    ISNICPrePaid,
    Visa,
    Mastercard,
    Jcb,
}
