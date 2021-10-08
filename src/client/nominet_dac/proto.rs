use chrono::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum DACRequest {
    Domain(String),
    Usage,
    Limits,
    Exit
}

impl Into<Vec<u8>> for DACRequest {
    fn into(self) -> Vec<u8> {
        match self {
            Self::Domain(d) => {
                return [d.trim_start_matches("#").as_bytes(), &[0xd, 0xa]].concat()
            },
            Self::Usage => b"#usage\r\n".to_vec(),
            Self::Limits => b"#limits\r\n".to_vec(),
            Self::Exit => b"#exit\r\n".to_vec(),
        }
    }
}

#[derive(Debug)]
pub enum DACResponse {
    DomainRT(DomainRT),
    DomainTD(DomainTD),
    Usage(Usage),
    Limits(Usage),
    AUB(AUB)
}

#[derive(Debug)]
pub struct AUB {
    pub domain: String,
    pub delay: u64
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
    pub created: Date<Utc>,
    pub expiry: Date<Utc>,
    pub tag: String
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
    NoLongerRequired
}

#[derive(Debug)]
pub struct DomainTD {
    pub domain: String,
    pub registered: DomainRegistered,
    pub detagged: bool,
    pub suspended: bool,
    pub created: Date<Utc>,
    pub expiry: Date<Utc>,
    pub status: DomainStatus,
    pub tag: String
}