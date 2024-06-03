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

///
#[repr(i64)]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
)]
pub enum CipherType {
    Login = 1,
    SecureNote = 2,
    Card = 3,
    Identity = 4,
}

impl std::fmt::Display for CipherType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Login => write!(f, "1"),
            Self::SecureNote => write!(f, "2"),
            Self::Card => write!(f, "3"),
            Self::Identity => write!(f, "4"),
        }
    }
}

impl Default for CipherType {
    fn default() -> CipherType {
        Self::Login
    }
}
