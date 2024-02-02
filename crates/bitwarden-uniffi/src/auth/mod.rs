use std::sync::Arc;

use bitwarden::auth::{
    password::MasterPasswordPolicyOptions, AuthRequestResponse, RegisterKeyResponse,
};
use bitwarden_crypto::{AsymmetricEncString, HashPurpose, Kdf};

use crate::{error::Result, Client};

#[derive(uniffi::Object)]
pub struct ClientAuth(pub(crate) Arc<Client>);

#[uniffi::export(async_runtime = "tokio")]
impl ClientAuth {
    /// **API Draft:** Calculate Password Strength
    pub async fn password_strength(
        &self,
        password: String,
        email: String,
        additional_inputs: Vec<String>,
    ) -> u8 {
        self.0
             .0
            .write()
            .await
            .auth()
            .password_strength(password, email, additional_inputs)
            .await
    }

    /// Evaluate if the provided password satisfies the provided policy
    pub async fn satisfies_policy(
        &self,
        password: String,
        strength: u8,
        policy: MasterPasswordPolicyOptions,
    ) -> bool {
        self.0
             .0
            .write()
            .await
            .auth()
            .satisfies_policy(password, strength, &policy)
            .await
    }

    /// Hash the user password
    pub async fn hash_password(
        &self,
        email: String,
        password: String,
        kdf_params: Kdf,
        purpose: HashPurpose,
    ) -> Result<String> {
        Ok(self
            .0
             .0
            .read()
            .await
            .kdf()
            .hash_password(email, password, kdf_params, purpose)
            .await?)
    }

    /// Generate keys needed for registration process
    pub async fn make_register_keys(
        &self,
        email: String,
        password: String,
        kdf: Kdf,
    ) -> Result<RegisterKeyResponse> {
        Ok(self
            .0
             .0
            .write()
            .await
            .auth()
            .make_register_keys(email, password, kdf)?)
    }

    /// Validate the user password
    ///
    /// To retrieve the user's password hash, use [`ClientAuth::hash_password`] with
    /// `HashPurpose::LocalAuthentication` during login and persist it. If the login method has no
    /// password, use the email OTP.
    pub async fn validate_password(&self, password: String, password_hash: String) -> Result<bool> {
        Ok(self
            .0
             .0
            .write()
            .await
            .auth()
            .validate_password(password, password_hash.to_string())?)
    }

    /// Validate the user password without knowing the password hash
    ///
    /// Used for accounts that we know have master passwords but that have not logged in with a
    /// password. Some example are login with device or TDE.
    ///
    /// This works by comparing the provided password against the encrypted user key.
    pub async fn validate_password_user_key(
        &self,
        password: String,
        encrypted_user_key: String,
    ) -> Result<String> {
        Ok(self
            .0
             .0
            .write()
            .await
            .auth()
            .validate_password_user_key(password, encrypted_user_key)?)
    }

    /// Initialize a new auth request
    pub async fn new_auth_request(&self, email: String) -> Result<AuthRequestResponse> {
        Ok(self.0 .0.write().await.auth().new_auth_request(&email)?)
    }

    /// Approve an auth request
    pub async fn approve_auth_request(&self, public_key: String) -> Result<AsymmetricEncString> {
        Ok(self
            .0
             .0
            .write()
            .await
            .auth()
            .approve_auth_request(public_key)?)
    }
}
