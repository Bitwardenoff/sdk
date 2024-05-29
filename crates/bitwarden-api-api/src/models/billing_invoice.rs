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
pub struct BillingInvoice {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "pdfUrl", skip_serializing_if = "Option::is_none")]
    pub pdf_url: Option<String>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "paid", skip_serializing_if = "Option::is_none")]
    pub paid: Option<bool>,
}

impl BillingInvoice {
    pub fn new() -> BillingInvoice {
        BillingInvoice {
            amount: None,
            date: None,
            url: None,
            pdf_url: None,
            number: None,
            paid: None,
        }
    }
}
