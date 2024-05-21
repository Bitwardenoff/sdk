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
pub struct UpdateClientOrganizationRequestBody {
    #[serde(rename = "assignedSeats")]
    pub assigned_seats: i32,
    #[serde(rename = "name")]
    pub name: String,
}

impl UpdateClientOrganizationRequestBody {
    pub fn new(assigned_seats: i32, name: String) -> UpdateClientOrganizationRequestBody {
        UpdateClientOrganizationRequestBody {
            assigned_seats,
            name,
        }
    }
}