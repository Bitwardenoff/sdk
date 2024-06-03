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
pub struct PeopleAccessPoliciesRequestModel {
    #[serde(
        rename = "userAccessPolicyRequests",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_access_policy_requests: Option<Vec<models::AccessPolicyRequest>>,
    #[serde(
        rename = "groupAccessPolicyRequests",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_access_policy_requests: Option<Vec<models::AccessPolicyRequest>>,
}

impl PeopleAccessPoliciesRequestModel {
    pub fn new() -> PeopleAccessPoliciesRequestModel {
        PeopleAccessPoliciesRequestModel {
            user_access_policy_requests: None,
            group_access_policy_requests: None,
        }
    }
}
