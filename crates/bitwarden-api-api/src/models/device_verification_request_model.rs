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
pub struct DeviceVerificationRequestModel {
    #[serde(rename = "unknownDeviceVerificationEnabled")]
    pub unknown_device_verification_enabled: bool,
}

impl DeviceVerificationRequestModel {
    pub fn new(unknown_device_verification_enabled: bool) -> DeviceVerificationRequestModel {
        DeviceVerificationRequestModel {
            unknown_device_verification_enabled,
        }
    }
}
