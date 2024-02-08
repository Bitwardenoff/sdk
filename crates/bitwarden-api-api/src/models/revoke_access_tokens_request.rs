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
pub struct RevokeAccessTokensRequest {
    #[serde(rename = "ids")]
    pub ids: Vec<uuid::Uuid>,
}

impl RevokeAccessTokensRequest {
    pub fn new(ids: Vec<uuid::Uuid>) -> RevokeAccessTokensRequest {
        RevokeAccessTokensRequest { ids }
    }
}
