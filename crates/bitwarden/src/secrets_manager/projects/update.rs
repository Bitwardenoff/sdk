use bitwarden_api_api::models::ProjectUpdateRequestModel;
use bitwarden_core::VaultLocked;
use bitwarden_crypto::KeyEncryptable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ProjectResponse;
use crate::{client::Client, error::Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectPutRequest {
    /// ID of the project to modify
    pub id: Uuid,
    /// Organization ID of the project to modify
    pub organization_id: Uuid,

    pub name: String,
}

pub(crate) async fn update_project(
    client: &Client,
    input: &ProjectPutRequest,
) -> Result<ProjectResponse> {
    let enc = client.get_encryption_settings()?;
    let key = enc
        .get_key(&Some(input.organization_id))
        .ok_or(VaultLocked)?;

    let project = Some(ProjectUpdateRequestModel {
        name: input.name.clone().encrypt_with_key(key)?.to_string(),
    });

    let config = client.get_api_configurations().await;
    let res =
        bitwarden_api_api::apis::projects_api::projects_id_put(&config.api, input.id, project)
            .await?;

    ProjectResponse::process_response(res, &enc)
}
