/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrganizationSubscriptionResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "businessName", skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[serde(rename = "businessAddress1", skip_serializing_if = "Option::is_none")]
    pub business_address1: Option<String>,
    #[serde(rename = "businessAddress2", skip_serializing_if = "Option::is_none")]
    pub business_address2: Option<String>,
    #[serde(rename = "businessAddress3", skip_serializing_if = "Option::is_none")]
    pub business_address3: Option<String>,
    #[serde(rename = "businessCountry", skip_serializing_if = "Option::is_none")]
    pub business_country: Option<String>,
    #[serde(rename = "businessTaxNumber", skip_serializing_if = "Option::is_none")]
    pub business_tax_number: Option<String>,
    #[serde(rename = "billingEmail", skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<crate::models::PlanResponseModel>>,
    #[serde(rename = "secretsManagerPlan", skip_serializing_if = "Option::is_none")]
    pub secrets_manager_plan: Option<Box<crate::models::PlanResponseModel>>,
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<crate::models::PlanType>,
    #[serde(rename = "seats", skip_serializing_if = "Option::is_none")]
    pub seats: Option<i32>,
    #[serde(rename = "maxAutoscaleSeats", skip_serializing_if = "Option::is_none")]
    pub max_autoscale_seats: Option<i32>,
    #[serde(rename = "maxCollections", skip_serializing_if = "Option::is_none")]
    pub max_collections: Option<i32>,
    #[serde(rename = "maxStorageGb", skip_serializing_if = "Option::is_none")]
    pub max_storage_gb: Option<i32>,
    #[serde(rename = "usePolicies", skip_serializing_if = "Option::is_none")]
    pub use_policies: Option<bool>,
    #[serde(rename = "useSso", skip_serializing_if = "Option::is_none")]
    pub use_sso: Option<bool>,
    #[serde(rename = "useKeyConnector", skip_serializing_if = "Option::is_none")]
    pub use_key_connector: Option<bool>,
    #[serde(rename = "useScim", skip_serializing_if = "Option::is_none")]
    pub use_scim: Option<bool>,
    #[serde(rename = "useGroups", skip_serializing_if = "Option::is_none")]
    pub use_groups: Option<bool>,
    #[serde(rename = "useDirectory", skip_serializing_if = "Option::is_none")]
    pub use_directory: Option<bool>,
    #[serde(rename = "useEvents", skip_serializing_if = "Option::is_none")]
    pub use_events: Option<bool>,
    #[serde(rename = "useTotp", skip_serializing_if = "Option::is_none")]
    pub use_totp: Option<bool>,
    #[serde(rename = "use2fa", skip_serializing_if = "Option::is_none")]
    pub use2fa: Option<bool>,
    #[serde(rename = "useApi", skip_serializing_if = "Option::is_none")]
    pub use_api: Option<bool>,
    #[serde(rename = "useSecretsManager", skip_serializing_if = "Option::is_none")]
    pub use_secrets_manager: Option<bool>,
    #[serde(rename = "useResetPassword", skip_serializing_if = "Option::is_none")]
    pub use_reset_password: Option<bool>,
    #[serde(rename = "usersGetPremium", skip_serializing_if = "Option::is_none")]
    pub users_get_premium: Option<bool>,
    #[serde(
        rename = "useCustomPermissions",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_custom_permissions: Option<bool>,
    #[serde(rename = "selfHost", skip_serializing_if = "Option::is_none")]
    pub self_host: Option<bool>,
    #[serde(
        rename = "hasPublicAndPrivateKeys",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_public_and_private_keys: Option<bool>,
    #[serde(rename = "usePasswordManager", skip_serializing_if = "Option::is_none")]
    pub use_password_manager: Option<bool>,
    #[serde(rename = "smSeats", skip_serializing_if = "Option::is_none")]
    pub sm_seats: Option<i32>,
    #[serde(rename = "smServiceAccounts", skip_serializing_if = "Option::is_none")]
    pub sm_service_accounts: Option<i32>,
    #[serde(
        rename = "maxAutoscaleSmSeats",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_autoscale_sm_seats: Option<i32>,
    #[serde(
        rename = "maxAutoscaleSmServiceAccounts",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_autoscale_sm_service_accounts: Option<i32>,
    #[serde(rename = "storageName", skip_serializing_if = "Option::is_none")]
    pub storage_name: Option<String>,
    #[serde(rename = "storageGb", skip_serializing_if = "Option::is_none")]
    pub storage_gb: Option<f64>,
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Box<crate::models::BillingSubscription>>,
    #[serde(rename = "upcomingInvoice", skip_serializing_if = "Option::is_none")]
    pub upcoming_invoice: Option<Box<crate::models::BillingSubscriptionUpcomingInvoice>>,
    /// Date when a self-hosted organization's subscription expires, without any grace period.
    #[serde(
        rename = "expirationWithoutGracePeriod",
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_without_grace_period: Option<String>,
    /// Date when a self-hosted organization expires (includes grace period).
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(rename = "secretsManagerBeta", skip_serializing_if = "Option::is_none")]
    pub secrets_manager_beta: Option<bool>,
}

impl OrganizationSubscriptionResponseModel {
    pub fn new() -> OrganizationSubscriptionResponseModel {
        OrganizationSubscriptionResponseModel {
            object: None,
            id: None,
            name: None,
            business_name: None,
            business_address1: None,
            business_address2: None,
            business_address3: None,
            business_country: None,
            business_tax_number: None,
            billing_email: None,
            plan: None,
            secrets_manager_plan: None,
            plan_type: None,
            seats: None,
            max_autoscale_seats: None,
            max_collections: None,
            max_storage_gb: None,
            use_policies: None,
            use_sso: None,
            use_key_connector: None,
            use_scim: None,
            use_groups: None,
            use_directory: None,
            use_events: None,
            use_totp: None,
            use2fa: None,
            use_api: None,
            use_secrets_manager: None,
            use_reset_password: None,
            users_get_premium: None,
            use_custom_permissions: None,
            self_host: None,
            has_public_and_private_keys: None,
            use_password_manager: None,
            sm_seats: None,
            sm_service_accounts: None,
            max_autoscale_sm_seats: None,
            max_autoscale_sm_service_accounts: None,
            storage_name: None,
            storage_gb: None,
            subscription: None,
            upcoming_invoice: None,
            expiration_without_grace_period: None,
            expiration: None,
            secrets_manager_beta: None,
        }
    }
}
