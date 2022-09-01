use chrono::prelude::*;

#[derive(Debug)]
pub struct ContactInfo {
    pub validated: bool,
    pub verification_requested: bool,
    pub verified: bool,
}

#[derive(Debug)]
pub struct ContactCreate {
    pub check_only: bool,
    pub force_duplication: bool,
    pub pre_verify: bool,
}

#[derive(Debug)]
pub struct ContactUpdate {
    pub check_only: bool,
    pub pre_verify: bool,
    pub trigger_foa: bool,
}

#[derive(Debug)]
pub struct DomainCheck {
    pub allocation_token: Option<String>
}

#[derive(Debug)]
pub struct DomainCreate {
    pub accept_premium_price: bool,
    pub accept_ssl_requirements: bool,
    pub allocation_token: Option<String>,
    pub renewal_mode: RenewalMode,
    pub transfer_mode: TransferMode,
    pub whois_banner: Vec<String>,
    pub whois_rsp: Option<String>,
    pub whois_url: Option<String>,
    pub tld: Option<DomainCreateTLD>,
}

#[derive(Debug)]
pub struct DomainUpdate {
    pub renewal_mode: Option<RenewalMode>,
    pub transfer_mode: Option<TransferMode>,
    pub whois_banner: Vec<String>,
    pub whois_rsp: Option<String>,
    pub whois_url: Option<String>,
    pub tld: Option<DomainUpdateTLD>,
}

#[derive(Debug)]
pub struct DomainInfo {
    pub renewal_date: DateTime<Utc>,
    pub paid_until_date: DateTime<Utc>,
    pub roid: Option<String>,
    pub renewal_mode: RenewalMode,
    pub transfer_mode: TransferMode,
    pub whois_banner: Vec<String>,
    pub whois_rsp: Option<String>,
    pub whois_url: Option<String>,
    pub tld: Option<DomainInfoTLD>,
}

#[derive(Debug)]
pub struct DomainRenew {
    pub accept_premium_price: bool,
    pub promotion_code: Option<String>,
}

#[derive(Debug)]
pub struct DomainTransfer {
    pub accept_premium_price: bool,
    pub accept_quarantine: bool,
    pub accept_trade: bool,
    pub allocation_token: Option<String>,
    pub at_request_authcode: bool,
    pub be_request_authcode: bool,
    pub promotion_code: Option<String>,
}

#[derive(Debug)]
pub struct DomainDelete {
    pub action: DomainDeleteAction,
    pub target: Option<String>
}

#[derive(Debug)]
pub enum DomainDeleteAction {
    Default,
    Instant,
    AutoDelete,
    AutoExpire,
    Push
}

#[derive(Debug)]
pub enum DomainCreateTLD {
    CA(DomainCreateCA),
    DE(DomainCreateDE),
    EU(DomainCreateEU),
    FR(DomainCreateFR),
    Gay(DomainCreateGay),
    Name(DomainName),
    RS(DomainCreateRS),
    US(DomainCreateUS),
}

#[derive(Debug)]
pub enum DomainUpdateTLD {
    CA(DomainUpdateCA),
    DE(DomainUpdateDE),
    EU(DomainUpdateEU),
    FR(DomainUpdateFR),
    Name(DomainName),
    RS(DomainUpdateRS),
    US(DomainUpdateUS),
}

#[derive(Debug)]
pub enum DomainInfoTLD {
    CA(DomainCreateCA),
    DE(DomainCreateDE),
    EU(DomainCreateEU),
    FR(DomainCreateFR),
    Name(DomainName),
    RS(DomainUpdateRS),
    US(DomainCreateUS),
}

#[derive(Debug)]
pub struct DomainCreateCA {
    pub legal_type: CALegalType,
    pub trademark: bool,
}

#[derive(Debug)]
pub struct DomainUpdateCA {
    pub legal_type: Option<CALegalType>,
    pub trademark: Option<bool>,
}

#[derive(Debug)]
pub struct DomainCreateEU {
    pub accept_trustee_tac: bool,
    pub registrant_lang: Option<EULanguage>,
    pub registrant_citizenship: Option<EUCountry>
}

#[derive(Debug)]
pub struct DomainUpdateEU {
    pub accept_trustee_tac: Option<bool>,
    pub registrant_lang: Option<EULanguage>,
    pub registrant_citizenship: Option<EUCountry>
}

#[derive(Debug)]
pub struct DomainCreateDE {
    pub abuse_contact: Option<String>,
    pub general_request: Option<String>,
    pub accept_trustee_tac: DETrustee,
    pub holder_person: bool,
}

#[derive(Debug)]
pub struct DomainUpdateDE {
    pub abuse_contact: Option<String>,
    pub general_request: Option<String>,
    pub accept_trustee_tac: Option<DETrustee>,
    pub holder_person: Option<bool>,
}

#[derive(Debug)]
pub struct DomainCreateFR {
    pub accept_trustee_tac: bool
}

#[derive(Debug)]
pub struct DomainUpdateFR {
    pub accept_trustee_tac: Option<bool>
}

#[derive(Debug)]
pub struct DomainCreateGay {
    pub accept_requirements: bool
}

#[derive(Debug)]
pub struct DomainName {
    pub email_forward: Option<String>
}

#[derive(Debug)]
pub struct DomainCreateRS {
    pub owner: RsId,
    pub admin: RsId,
    pub tech: RsId,
}

#[derive(Debug)]
pub struct DomainUpdateRS {
    pub owner: Option<RsId>,
    pub admin: Option<RsId>,
    pub tech: Option<RsId>,
}

#[derive(Debug)]
pub enum RsId {
    IDCard(String),
    CompanyNumber(String)
}

#[derive(Debug)]
pub struct DomainCreateUS {
    pub purpose: USPurpose,
    pub category: USCategory,
    pub validator: Option<String>
}

#[derive(Debug)]
pub struct DomainUpdateUS {
    pub purpose: Option<USPurpose>,
    pub category: Option<USCategory>,
    pub validator: Option<String>
}

#[derive(Debug, Eq, PartialEq)]
pub enum RenewalMode {
    Default,
    AutoRenew,
    AutoDelete,
    AutoExpire,
    AutoRenewMonthly,
    AutoRenewQuarterly,
    ExpireAuction,
    RenewOnce,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TransferMode {
    Default,
    AutoApprove,
    AutoDeny,
}

#[derive(Debug, Eq, PartialEq)]
pub enum DETrustee {
    None,
    Monthly,
    Annually
}

#[derive(Debug, Eq, PartialEq)]
pub enum CALegalType {
    AboriginalPeoples,
    CanadianUnincorporatedAssociation,
    Corporation,
    Citizen,
    CanadianEducationalInstitution,
    Government,
    CanadianHospital,
    IndianBand,
    CanadianLibraryArchiveMuseum,
    LegalRepOfCanadianCitizenOrPermanentResident,
    TheQueen,
    OfficialMark,
    CanadianPoliticalParty,
    Partnership,
    PermanentResident,
    TradeMark,
    TradeUnion,
    Trust
}

#[derive(Debug, Eq, PartialEq)]
pub enum EULanguage {
    Bulgarian,
    Czech,
    Danish,
    German,
    ModernGreek,
    English,
    Spanish,
    Estonian,
    Finnish,
    French,
    Gaelic,
    Croatian,
    Hungarian,
    Italian,
    Lithuanian,
    Latvian,
    Maltese,
    DutchFlemish,
    Polish,
    Portuguese,
    Romanian,
    Slovak,
    Slovene,
    Swedish,
}

#[derive(Debug, Eq, PartialEq)]
pub enum EUCountry {
    Austria,
    Belgium,
    Bulgaria,
    Czech,
    Cyprus,
    Germany,
    Denmark,
    Spain,
    Estonia,
    Finland,
    France,
    Greece,
    Hungary,
    Ireland,
    Italy,
    Liechtenstein,
    Lithuania,
    Luxembourg,
    Latvia,
    Malta,
    Netherlands,
    Poland,
    Portugal,
    Romania,
    Sweden,
    Slovakia,
    Slovenia,
    Croatia
}

#[derive(Debug, Eq, PartialEq)]
pub enum USPurpose {
    Business,
    NonProfit,
    Personal,
    Educational,
    Government,
}


#[derive(Debug, Eq, PartialEq)]
pub enum USCategory {
    Citizen,
    PermanentResident,
    USOrganisation,
    RegularActivity,
    OfficeOrFacility,
}