#[derive(Debug)]
pub struct DomainInfo {
    pub zone_contact: Option<String>,
}

#[derive(Debug)]
pub enum PaymentInfo {
    Prepaid(u32),
    Card { id: u32, cvc: String },
}

#[derive(Debug)]
pub struct DomainUpdate {
    pub remove_all_ns: bool,
    pub new_master_ns: Vec<String>,
}

#[derive(Debug)]
pub struct ContactInfo {
    pub statuses: Vec<ContactStatus>,
    pub mobile: Option<super::contact::Phone>,
    pub sid: Option<String>,
    pub auto_update_from_national_registry: bool,
    pub paper_invoices: bool,
}

#[derive(Debug)]
pub struct ContactCreate {
    pub mobile: Option<super::contact::Phone>,
    pub sid: Option<String>,
    pub auto_update_from_national_registry: bool,
    pub paper_invoices: bool,
    pub lang: Option<String>,
}

#[derive(Debug)]
pub struct ContactUpdate {
    pub mobile: Option<super::contact::Phone>,
    pub auto_update_from_national_registry: Option<bool>,
    pub paper_invoices: Option<bool>,
    pub lang: Option<String>,
    pub password: String,
}

#[derive(Debug)]
pub enum ContactStatus {
    Ok,
    OkUnconfirmed,
    PendingCreate,
    ServerExpired,
    ServerSuspended,
}

#[derive(Debug)]
pub struct HostInfo {
    pub zone_contact: Option<String>,
}
