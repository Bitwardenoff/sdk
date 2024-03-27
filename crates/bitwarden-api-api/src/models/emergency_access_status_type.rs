/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[repr(i64)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum EmergencyAccessStatusType {
    Invited = 0,
    Accepted = 1,
    Confirmed = 2,
    RecoveryInitiated = 3,
    RecoveryApproved = 4,

    #[serde(other)]
    UnknownValue = -1337,
}

impl ToString for EmergencyAccessStatusType {
    fn to_string(&self) -> String {
        match self {
            Self::Invited => String::from("0"),
            Self::Accepted => String::from("1"),
            Self::Confirmed => String::from("2"),
            Self::RecoveryInitiated => String::from("3"),
            Self::RecoveryApproved => String::from("4"),
            Self::UnknownValue => String::from("UnknownValue"),
        }
    }
}

impl Default for EmergencyAccessStatusType {
    fn default() -> EmergencyAccessStatusType {
        Self::Invited
    }
}
