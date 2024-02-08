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
pub struct BulkCollectionAccessRequestModel {
    #[serde(rename = "collectionIds", skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::SelectionReadOnlyRequestModel>>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::SelectionReadOnlyRequestModel>>,
}

impl BulkCollectionAccessRequestModel {
    pub fn new() -> BulkCollectionAccessRequestModel {
        BulkCollectionAccessRequestModel {
            collection_ids: None,
            groups: None,
            users: None,
        }
    }
}
