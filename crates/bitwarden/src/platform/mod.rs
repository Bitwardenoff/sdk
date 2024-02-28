pub mod client_platform;
mod domain;
mod fido2;
mod generate_fingerprint;
mod get_user_api_key;
mod secret_verification_request;
mod sync;

pub(crate) use fido2::client_get_assertion;
pub use fido2::Fido2ClientGetAssertionRequest;
pub use generate_fingerprint::{FingerprintRequest, FingerprintResponse};
pub(crate) use get_user_api_key::get_user_api_key;
pub use get_user_api_key::UserApiKeyResponse;
pub use secret_verification_request::SecretVerificationRequest;
pub(crate) use sync::sync;
pub use sync::{SyncRequest, SyncResponse};
