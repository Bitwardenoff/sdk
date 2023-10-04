use base64::Engine;
use reqwest::{header::CONTENT_TYPE, StatusCode};

use crate::{
    error::{Error, Result},
    util::BASE64_ENGINE,
};
pub async fn generate(
    api_token: String,
    domain: String,
    website: Option<String>,
) -> Result<String> {
    if api_token.is_empty() {
        return Err(Error::Internal("Invalid Forward Email API key."));
    }
    if domain.is_empty() {
        return Err(Error::Internal("Invalid Forward Email domain."));
    }

    let api_token_b64 = BASE64_ENGINE.encode(format!("{api_token}:").as_bytes());

    let description = website
        .as_ref()
        .map(|w| format!("Website: {w}. "))
        .unwrap_or_default();
    let description = format!("{description}Generated by Bitwarden.");

    #[derive(serde::Serialize)]
    struct Request {
        labels: Option<String>,
        description: String,
    }

    let response = reqwest::Client::new()
        .post(format!(
            "https://api.forwardemail.net/v1/domains/${domain}/aliases"
        ))
        .header(CONTENT_TYPE, "application/json")
        .bearer_auth(api_token_b64)
        .json(&Request {
            description,
            labels: website,
        })
        .send()
        .await?;

    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(Error::Internal("Invalid Forward Email API key."));
    }

    // Throw any other errors
    response.error_for_status_ref()?;

    #[derive(serde::Deserialize)]
    struct ResponseDomain {
        name: Option<String>,
    }
    #[derive(serde::Deserialize)]
    struct Response {
        name: String,
        domain: ResponseDomain,

        message: Option<String>,
        error: Option<String>,
    }
    let status = response.status();
    let response: Response = response.json().await?;

    if let Some(message) = response.message {
        return Err(Error::ResponseContent { status, message });
    }
    if let Some(message) = response.error {
        return Err(Error::ResponseContent { status, message });
    }

    Ok(format!(
        "{}@{}",
        response.name,
        response.domain.name.unwrap_or(domain)
    ))
}
