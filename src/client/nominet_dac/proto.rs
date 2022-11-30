use chrono::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum DACRequest {
    Domain(String),
    Usage,
    Limits,
    Exit,
}

impl From<DACRequest> for Vec<u8> {
    fn from(from: DACRequest) -> Vec<u8> {
        match from {
            DACRequest::Domain(d) => {
                return [d.trim_start_matches('#').as_bytes(), &[0xd, 0xa]].concat()
            }
            DACRequest::Usage => b"#usage\r\n".to_vec(),
            DACRequest::Limits => b"#limits\r\n".to_vec(),
            DACRequest::Exit => b"#exit\r\n".to_vec(),
        }
    }
}

#[derive(Debug)]
pub enum DACResponse {
    DomainRT(DomainRT),
    DomainTD(DomainTD),
    Usage(Usage),
    Limits(Usage),
    Aub(Aub),
}

#[derive(Debug)]
pub struct Aub {
    pub domain: String,
    pub delay: u64,
}

#[derive(Debug)]
pub struct Usage {
    pub usage_60: u64,
    pub usage_24: u64,
}

#[derive(Debug)]
pub struct DomainRT {
    pub domain: String,
    pub registered: bool,
    pub detagged: bool,
    pub created: NaiveDate,
    pub expiry: NaiveDate,
    pub tag: String,
}

#[derive(Debug)]
pub enum DomainRegistered {
    Registered,
    Available,
    NotWithinRegistry,
    RulesPrevent,
}

#[derive(Debug)]
pub enum DomainStatus {
    Unknown,
    RegisteredUntilExpiry,
    RenewalRequired,
    NoLongerRequired,
}

#[derive(Debug)]
pub struct DomainTD {
    pub domain: String,
    pub registered: DomainRegistered,
    pub detagged: bool,
    pub suspended: bool,
    pub created: NaiveDate,
    pub expiry: NaiveDate,
    pub status: DomainStatus,
    pub tag: String,
}
