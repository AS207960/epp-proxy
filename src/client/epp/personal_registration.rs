impl From<&super::proto::personal_registration::PersonalRegistrationInfoData> for super::super::personal_registration::PersonalRegistrationInfo {
    fn from(from: &super::proto::personal_registration::PersonalRegistrationInfoData) -> Self {
        super::super::personal_registration::PersonalRegistrationInfo {
            consent_id: from.consent_id.clone()
        }
    }
}

impl From<&super::super::personal_registration::PersonalRegistrationInfo> for super::proto::personal_registration::PersonalRegistrationCreate {
    fn from(from: &super::super::personal_registration::PersonalRegistrationInfo) -> Self {
        super::proto::personal_registration::PersonalRegistrationCreate {
            consent_id: from.consent_id.clone()
        }
    }
}

impl From<&super::proto::personal_registration::PersonalRegistrationCreateData> for super::super::personal_registration::PersonalRegistrationCreate {
    fn from(from: &super::proto::personal_registration::PersonalRegistrationCreateData) -> Self {
        super::super::personal_registration::PersonalRegistrationCreate {
            bundled_rate: from.bundled_rate
        }
    }
}