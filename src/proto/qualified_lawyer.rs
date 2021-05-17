use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct QualifiedLawyerInfoData {
    #[serde(rename = "{urn:ietf:params:xml:ns:qualifiedLawyer-1.0}qualifiedLawyer:accreditationId")]
    pub accreditation_id: String,
    #[serde(rename = "{urn:ietf:params:xml:ns:qualifiedLawyer-1.0}qualifiedLawyer:accreditationBody")]
    pub accreditation_body: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:qualifiedLawyer-1.0}qualifiedLawyer:accreditationYear",
        deserialize_with = "deserialize_date_year",
        serialize_with = "serialize_date_year",
    )]
    pub accreditation_year: i32,
    #[serde(rename = "{urn:ietf:params:xml:ns:qualifiedLawyer-1.0}qualifiedLawyer:jurisdictionCC")]
    pub jurisdiction_country: String,
    #[serde(
        rename = "{urn:ietf:params:xml:ns:qualifiedLawyer-1.0}qualifiedLawyer:jurisdictionSP",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub jurisdiction_province: Option<String>
}


struct DateYearVisitor;

impl<'de> serde::de::Visitor<'de> for DateYearVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a formatted date string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value.ends_with(&"Z") {
            let len = value.len();
            if len < 2 {
                return Err(E::custom("Invalid year"));
            }
            match i32::from_str(&value[..len-1]) {
                Ok(v) => Ok(v),
                Err(e) => Err(E::custom(e)),
            }
        } else {
            return Err(E::custom("Invalid year"));
        }
    }
}

fn deserialize_date_year<'de, D>(d: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    d.deserialize_str(DateYearVisitor)
}


#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_date_year<S>(d: &i32, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    s.serialize_str(&format!("{}Z", d))
}