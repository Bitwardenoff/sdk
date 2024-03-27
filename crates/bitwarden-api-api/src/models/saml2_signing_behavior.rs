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
pub enum Saml2SigningBehavior {
    IfIdpWantAuthnRequestsSigned = 0,
    Always = 1,
    Never = 3,

    #[serde(other)]
    UnknownValue = -1337,
}

impl ToString for Saml2SigningBehavior {
    fn to_string(&self) -> String {
        match self {
            Self::IfIdpWantAuthnRequestsSigned => String::from("0"),
            Self::Always => String::from("1"),
            Self::Never => String::from("3"),
            Self::UnknownValue => String::from("UnknownValue"),
        }
    }
}

impl Default for Saml2SigningBehavior {
    fn default() -> Saml2SigningBehavior {
        Self::IfIdpWantAuthnRequestsSigned
    }
}
