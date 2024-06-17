use chrono::Utc;

#[cfg(feature = "internal")]
use crate::{auth::api::request::ApiTokenRequest, client::UserLoginMethod};
use crate::{
    auth::api::{request::AccessTokenRequest, response::IdentityTokenResponse},
    client::{Client, LoginMethod, ServiceAccountLoginMethod},
    error::{Error, Result},
    secrets_manager::state::{self, ClientState},
};

pub(crate) async fn renew_token(client: &Client) -> Result<()> {
    const TOKEN_RENEW_MARGIN_SECONDS: i64 = 5 * 60;

    let tokens = client
        .internal
        .tokens
        .read()
        .expect("RwLock is not poisoned")
        .clone();
    let login_method = client
        .internal
        .login_method
        .read()
        .expect("RwLock is not poisoned")
        .clone();

    if let (Some(expires), Some(login_method)) = (tokens.expires_on, login_method) {
        if Utc::now().timestamp() < expires - TOKEN_RENEW_MARGIN_SECONDS {
            return Ok(());
        }

        let config = client
            .internal
            .__api_configurations
            .read()
            .expect("RwLock is not poisoned")
            .clone();

        let res = match login_method.as_ref() {
            #[cfg(feature = "internal")]
            LoginMethod::User(u) => match u {
                UserLoginMethod::Username { client_id, .. } => {
                    let refresh = tokens.refresh_token.ok_or(Error::NotAuthenticated)?;

                    crate::auth::api::request::RenewTokenRequest::new(refresh, client_id.to_owned())
                        .send(&config)
                        .await?
                }
                UserLoginMethod::ApiKey {
                    client_id,
                    client_secret,
                    ..
                } => {
                    ApiTokenRequest::new(client_id, client_secret)
                        .send(&config)
                        .await?
                }
            },
            LoginMethod::ServiceAccount(s) => match s {
                ServiceAccountLoginMethod::AccessToken {
                    access_token,
                    state_file,
                    ..
                } => {
                    let result = AccessTokenRequest::new(
                        access_token.access_token_id,
                        &access_token.client_secret,
                    )
                    .send(&config)
                    .await?;

                    if let (IdentityTokenResponse::Payload(r), Some(state_file), Ok(enc_settings)) = (
                        &result,
                        state_file,
                        client.internal.get_encryption_settings(),
                    ) {
                        if let Some(enc_key) = enc_settings.get_key(&None) {
                            let state =
                                ClientState::new(r.access_token.clone(), enc_key.to_base64());
                            _ = state::set(state_file, access_token, state);
                        }
                    }

                    result
                }
            },
        };

        match res {
            IdentityTokenResponse::Refreshed(r) => {
                client
                    .internal
                    .set_tokens(r.access_token, r.refresh_token, r.expires_in);
                return Ok(());
            }
            IdentityTokenResponse::Authenticated(r) => {
                client
                    .internal
                    .set_tokens(r.access_token, r.refresh_token, r.expires_in);
                return Ok(());
            }
            IdentityTokenResponse::Payload(r) => {
                client
                    .internal
                    .set_tokens(r.access_token, r.refresh_token, r.expires_in);
                return Ok(());
            }
            _ => {
                // We should never get here
                return Err(Error::InvalidResponse);
            }
        }
    }

    Err(Error::NotAuthenticated)
}