/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAccountCreateRequestModel {
    #[serde(rename = "name")]
    pub name: String,
}

impl ServiceAccountCreateRequestModel {
    pub fn new(name: String) -> ServiceAccountCreateRequestModel {
        ServiceAccountCreateRequestModel { name }
    }
}
