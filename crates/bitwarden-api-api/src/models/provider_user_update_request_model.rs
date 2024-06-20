/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderUserUpdateRequestModel {
    #[serde(rename = "type")]
    pub r#type: models::ProviderUserType,
}

impl ProviderUserUpdateRequestModel {
    pub fn new(r#type: models::ProviderUserType) -> ProviderUserUpdateRequestModel {
        ProviderUserUpdateRequestModel { r#type }
    }
}
