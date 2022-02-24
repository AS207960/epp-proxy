#[derive(Debug, Serialize)]
pub struct PersonalRegistrationCreate {
    #[serde(rename = "{http://www.nic.name/epp/persReg-1.0}persReg:consentID")]
    pub consent_id: String,
}

#[derive(Debug, Deserialize)]
pub struct PersonalRegistrationCreateData {
    #[serde(rename = "{http://www.nic.name/epp/persReg-1.0}bundledRate")]
    pub bundled_rate: bool,
}

#[derive(Debug, Deserialize)]
pub struct PersonalRegistrationInfoData {
    #[serde(rename = "{http://www.nic.name/epp/persReg-1.0}consentID")]
    pub consent_id: String,
}