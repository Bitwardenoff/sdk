/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`sm_events_service_accounts_service_account_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SmEventsServiceAccountsServiceAccountIdGetError {
    UnknownValue(serde_json::Value),
}

pub async fn sm_events_service_accounts_service_account_id_get(
    configuration: &configuration::Configuration,
    service_account_id: uuid::Uuid,
    start: Option<String>,
    end: Option<String>,
    continuation_token: Option<&str>,
) -> Result<
    crate::models::EventResponseModelListResponseModel,
    Error<SmEventsServiceAccountsServiceAccountIdGetError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/sm/events/service-accounts/{serviceAccountId}",
        local_var_configuration.base_path,
        serviceAccountId = crate::apis::urlencode(service_account_id.to_string())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = start {
        local_var_req_builder =
            local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end {
        local_var_req_builder = local_var_req_builder.query(&[("end", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = continuation_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("continuationToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SmEventsServiceAccountsServiceAccountIdGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
