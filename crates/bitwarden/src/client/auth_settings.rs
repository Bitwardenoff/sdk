use std::num::NonZeroU32;

use base64::Engine;
use bitwarden_api_identity::models::{KdfType, PreloginResponseModel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    crypto::{PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE},
    util::{
        default_argon2_iterations, default_argon2_memory, default_argon2_parallelism,
        default_pbkdf2_iterations, BASE64_ENGINE,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuthSettings {
    pub email: String,
    #[serde(flatten)]
    pub(crate) kdf: Kdf,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Kdf {
    PBKDF2 {
        iterations: NonZeroU32,
    },
    Argon2id {
        iterations: NonZeroU32,
        memory: NonZeroU32,
        parallelism: NonZeroU32,
    },
}

impl AuthSettings {
    pub fn new(response: PreloginResponseModel, email: String) -> Self {
        let kdf = match response.kdf.unwrap_or_default() {
            KdfType::Variant0 => Kdf::PBKDF2 {
                iterations: response
                    .kdf_iterations
                    .and_then(|e| NonZeroU32::new(e as u32))
                    .unwrap_or_else(default_pbkdf2_iterations),
            },
            KdfType::Variant1 => Kdf::Argon2id {
                iterations: response
                    .kdf_iterations
                    .and_then(|e| NonZeroU32::new(e as u32))
                    .unwrap_or_else(default_argon2_iterations),

                memory: response
                    .kdf_memory
                    .and_then(|e| NonZeroU32::new(e as u32))
                    .unwrap_or_else(default_argon2_memory),
                parallelism: response
                    .kdf_parallelism
                    .and_then(|e| NonZeroU32::new(e as u32))
                    .unwrap_or_else(default_argon2_parallelism),
            },
        };

        Self { email, kdf }
    }

    pub fn make_user_password_hash(&self, password: &str) -> String {
        self.make_password_hash(password, &self.email)
    }

    pub fn make_password_hash(&self, password: &str, salt: &str) -> String {
        let hash = match self.kdf {
            Kdf::PBKDF2 { iterations } => pbkdf2::pbkdf2_array::<
                PbkdfSha256Hmac,
                PBKDF_SHA256_HMAC_OUT_SIZE,
            >(
                password.as_bytes(), salt.as_bytes(), iterations.get()
            ),
            Kdf::Argon2id {
                iterations,
                memory,
                parallelism,
            } => {
                todo!("Implement argon2id");
            }
        }
        .unwrap();

        // Server expects hash + 1 iteration
        let login_hash = pbkdf2::pbkdf2_array::<PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE>(
            &hash,
            password.as_bytes(),
            1,
        )
        .unwrap();

        BASE64_ENGINE.encode(login_hash)
    }
}

#[cfg(test)]
mod tests {
    use bitwarden_api_identity::models::{KdfType, PreloginResponseModel};

    use super::AuthSettings;

    #[test]
    fn test_password_hash() {
        let res = PreloginResponseModel {
            kdf: Some(KdfType::Variant0),
            kdf_iterations: Some(100_000),
            kdf_memory: None,
            kdf_parallelism: None,
        };
        let settings = AuthSettings::new(res, "test@bitwarden.com".into());

        assert_eq!(
            settings.make_password_hash("asdfasdf", "test_salt"),
            "ZF6HjxUTSyBHsC+HXSOhZoXN+UuMnygV5YkWXCY4VmM="
        );
        assert_eq!(
            settings.make_user_password_hash("asdfasdf"),
            "wmyadRMyBZOH7P/a/ucTCbSghKgdzDpPqUnu/DAVtSw="
        );
    }
}
