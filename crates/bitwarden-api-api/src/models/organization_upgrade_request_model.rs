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
pub struct OrganizationUpgradeRequestModel {
    #[serde(rename = "businessName", skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<crate::models::PlanType>,
    #[serde(rename = "additionalSeats", skip_serializing_if = "Option::is_none")]
    pub additional_seats: Option<i32>,
    #[serde(
        rename = "additionalStorageGb",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_storage_gb: Option<i32>,
    #[serde(rename = "additionalSmSeats", skip_serializing_if = "Option::is_none")]
    pub additional_sm_seats: Option<i32>,
    #[serde(
        rename = "additionalServiceAccounts",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_service_accounts: Option<i32>,
    #[serde(rename = "useSecretsManager")]
    pub use_secrets_manager: bool,
    #[serde(rename = "premiumAccessAddon", skip_serializing_if = "Option::is_none")]
    pub premium_access_addon: Option<bool>,
    #[serde(
        rename = "billingAddressCountry",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_address_country: Option<String>,
    #[serde(
        rename = "billingAddressPostalCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_address_postal_code: Option<String>,
    #[serde(rename = "keys", skip_serializing_if = "Option::is_none")]
    pub keys: Option<Box<crate::models::OrganizationKeysRequestModel>>,
}

impl OrganizationUpgradeRequestModel {
    pub fn new(use_secrets_manager: bool) -> OrganizationUpgradeRequestModel {
        OrganizationUpgradeRequestModel {
            business_name: None,
            plan_type: None,
            additional_seats: None,
            additional_storage_gb: None,
            additional_sm_seats: None,
            additional_service_accounts: None,
            use_secrets_manager,
            premium_access_addon: None,
            billing_address_country: None,
            billing_address_postal_code: None,
            keys: None,
        }
    }
}
