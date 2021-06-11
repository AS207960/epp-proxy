#[derive(Debug)]
pub struct LowBalanceData {
    pub registrar_name: String,
    pub credit_limit: String,
    pub credit_threshold: CreditThreshold,
    pub available_credit: String,
}

#[derive(PartialEq, Debug)]
pub enum CreditThreshold {
    Fixed(String),
    Percentage(u8),
}

#[derive(Debug)]
pub struct InfoWhois {
    pub registrar: String,
    pub whois_server: Option<String>,
    pub url: Option<String>,
    pub iris_server: Option<String>,
}
