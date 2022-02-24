#[derive(Clone, Debug)]
pub struct PersonalRegistrationCreate {
    pub bundled_rate: bool
}

#[derive(Clone, Debug)]
pub struct PersonalRegistrationInfo {
    pub consent_id: String
}