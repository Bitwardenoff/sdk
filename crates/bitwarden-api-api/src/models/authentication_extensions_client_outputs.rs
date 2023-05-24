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
pub struct AuthenticationExtensionsClientOutputs {
    #[serde(rename = "example.extension", skip_serializing_if = "Option::is_none")]
    pub example_period_extension: Option<serde_json::Value>,
    #[serde(rename = "appid", skip_serializing_if = "Option::is_none")]
    pub appid: Option<bool>,
    #[serde(rename = "authnSel", skip_serializing_if = "Option::is_none")]
    pub authn_sel: Option<bool>,
    #[serde(rename = "exts", skip_serializing_if = "Option::is_none")]
    pub exts: Option<Vec<String>>,
    #[serde(rename = "uvm", skip_serializing_if = "Option::is_none")]
    pub uvm: Option<Vec<Vec<i64>>>,
}

impl AuthenticationExtensionsClientOutputs {
    pub fn new() -> AuthenticationExtensionsClientOutputs {
        AuthenticationExtensionsClientOutputs {
            example_period_extension: None,
            appid: None,
            authn_sel: None,
            exts: None,
            uvm: None,
        }
    }
}
