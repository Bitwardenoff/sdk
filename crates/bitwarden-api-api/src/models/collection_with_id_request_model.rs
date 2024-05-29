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
pub struct CollectionWithIdRequestModel {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<models::SelectionReadOnlyRequestModel>>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<models::SelectionReadOnlyRequestModel>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
}

impl CollectionWithIdRequestModel {
    pub fn new(name: String) -> CollectionWithIdRequestModel {
        CollectionWithIdRequestModel {
            name,
            external_id: None,
            groups: None,
            users: None,
            id: None,
        }
    }
}
