#[derive(Debug)]
pub struct TTLInfo {
    pub ttl: Vec<TTL>
}

#[derive(Debug)]
pub struct TTL {
    pub ttl: Option<u32>,
    pub record: Record,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub default: Option<u32>
}

#[derive(Debug)]
pub enum Record {
    NS,
    DS,
    DNAME,
    A,
    AAAA,
    Custom(String)
}

#[derive(Debug)]
pub struct TTLSet {
    pub ttl: Vec<TTLCommand>
}

#[derive(Debug)]
pub struct TTLCommand {
    pub ttl: Option<u32>,
    pub record: Record,
}