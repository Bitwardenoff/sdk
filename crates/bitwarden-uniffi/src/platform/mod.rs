use std::sync::Arc;

use bitwarden::platform::FingerprintRequest;

use crate::{error::Result, Client};

mod passkeys;

#[derive(uniffi::Object)]
pub struct ClientPlatform(pub(crate) Arc<Client>);

#[uniffi::export(async_runtime = "tokio")]
impl ClientPlatform {
    /// Fingerprint (public key)
    pub async fn fingerprint(&self, req: FingerprintRequest) -> Result<String> {
        Ok(self
            .0
             .0
            .write()
            .await
            .platform()
            .fingerprint(&req)?
            .fingerprint)
    }

    /// Fingerprint using logged in user's public key
    pub async fn user_fingerprint(&self, fingerprint_material: String) -> Result<String> {
        Ok(self
            .0
             .0
            .write()
            .await
            .platform()
            .user_fingerprint(fingerprint_material)?)
    }

    /// Load feature flags into the client
    pub async fn load_flags(&self, flags: std::collections::HashMap<String, bool>) -> Result<()> {
        self.0 .0.write().await.load_flags(flags);
        Ok(())
    }

    /// Passkey operations
    pub fn passkeys(self: Arc<Self>) -> Arc<passkeys::ClientPasskeys> {
        Arc::new(passkeys::ClientPasskeys(self.0.clone()))
    }
}
