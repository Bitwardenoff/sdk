#[cfg(feature = "secrets")]
use bitwarden::{
    auth::request::AccessTokenLoginRequest,
    secrets_manager::{
        projects::{
            ProjectCreateRequest, ProjectGetRequest, ProjectPutRequest, ProjectsDeleteRequest,
            ProjectsListRequest,
        },
        secrets::{
            SecretCreateRequest, SecretGetRequest, SecretIdentifiersRequest, SecretPutRequest,
            SecretsDeleteRequest,
        },
    },
};

#[cfg(feature = "internal")]
use bitwarden::{
    auth::request::{ApiKeyLoginRequest, PasswordLoginRequest},
    platform::{FingerprintRequest, SecretVerificationRequest, SyncRequest},
};

#[cfg(feature = "mobile")]
use bitwarden::mobile::{
    kdf::PasswordHashRequest,
    vault::{
        CipherDecryptListRequest, CipherDecryptRequest, CipherEncryptRequest,
        CollectionDecryptListRequest, CollectionDecryptRequest, FolderDecryptListRequest,
        FolderDecryptRequest, FolderEncryptRequest, PasswordHistoryDecryptListRequest,
        PasswordHistoryEncryptRequest,
    },
};

#[cfg(all(feature = "mobile", feature = "internal"))]
use bitwarden::mobile::crypto::InitCryptoRequest;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum Command {
    #[cfg(feature = "internal")]
    /// Login with username and password
    ///
    /// This command is for initiating an authentication handshake with Bitwarden.
    /// Authorization may fail due to requiring 2fa or captcha challenge completion
    /// despite accurate credentials.
    ///
    /// This command is not capable of handling authentication requiring 2fa or captcha.
    ///
    /// Returns: [PasswordLoginResponse](bitwarden::auth::response::PasswordLoginResponse)
    ///
    PasswordLogin(PasswordLoginRequest),

    #[cfg(feature = "internal")]
    /// Login with API Key
    ///
    /// This command is for initiating an authentication handshake with Bitwarden.
    ///
    /// Returns: [ApiKeyLoginResponse](bitwarden::auth::response::ApiKeyLoginResponse)
    ///
    ApiKeyLogin(ApiKeyLoginRequest),

    #[cfg(feature = "secrets")]
    /// Login with Secrets Manager Access Token
    ///
    /// This command is for initiating an authentication handshake with Bitwarden.
    ///
    /// Returns: [ApiKeyLoginResponse](bitwarden::auth::response::ApiKeyLoginResponse)
    ///
    AccessTokenLogin(AccessTokenLoginRequest),

    #[cfg(feature = "internal")]
    /// > Requires Authentication
    /// Get the API key of the currently authenticated user
    ///
    /// Returns: [UserApiKeyResponse](bitwarden::platform::UserApiKeyResponse)
    ///
    GetUserApiKey(SecretVerificationRequest),

    #[cfg(feature = "internal")]
    /// Get the user's passphrase
    ///
    /// Returns: String
    ///
    Fingerprint(FingerprintRequest),

    #[cfg(feature = "internal")]
    /// > Requires Authentication
    /// Retrieve all user data, ciphers and organizations the user is a part of
    ///
    /// Returns: [SyncResponse](bitwarden::platform::SyncResponse)
    ///
    Sync(SyncRequest),

    #[cfg(feature = "secrets")]
    Secrets(SecretsCommand),
    #[cfg(feature = "secrets")]
    Projects(ProjectsCommand),

    #[cfg(feature = "mobile")]
    Mobile(MobileCommand),
}

#[cfg(feature = "secrets")]
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum SecretsCommand {
    /// > Requires Authentication
    /// > Requires using an Access Token for login or calling Sync at least once
    /// Retrieve a secret by the provided identifier
    ///
    /// Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
    ///
    Get(SecretGetRequest),

    /// > Requires Authentication
    /// > Requires using an Access Token for login or calling Sync at least once
    /// Creates a new secret in the provided organization using the given data
    ///
    /// Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
    ///
    Create(SecretCreateRequest),

    /// > Requires Authentication
    /// > Requires using an Access Token for login or calling Sync at least once
    /// Lists all secret identifiers of the given organization, to then retrieve each secret, use `CreateSecret`
    ///
    /// Returns: [SecretIdentifiersResponse](bitwarden::secrets_manager::secrets::SecretIdentifiersResponse)
    ///
    List(SecretIdentifiersRequest),

    /// > Requires Authentication
    /// > Requires using an Access Token for login or calling Sync at least once
    /// Updates an existing secret with the provided ID using the given data
    ///
    /// Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
    ///
    Update(SecretPutRequest),

    /// > Requires Authentication
    /// > Requires using an Access Token for login or calling Sync at least once
    /// Deletes all the secrets whose IDs match the provided ones
    ///
    /// Returns: [SecretsDeleteResponse](bitwarden::secrets_manager::secrets::SecretsDeleteResponse)
    ///
    Delete(SecretsDeleteRequest),
}

#[cfg(feature = "secrets")]
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum ProjectsCommand {
    /// > Requires Authentication
    /// > Requires using an Access Token for login or calling Sync at least once
    /// Retrieve a project by the provided identifier
    ///
    /// Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
    ///
    Get(ProjectGetRequest),

    /// > Requires Authentication
    /// > Requires using an Access Token for login or calling Sync at least once
    /// Creates a new project in the provided organization using the given data
    ///
    /// Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
    ///
    Create(ProjectCreateRequest),

    /// > Requires Authentication
    /// > Requires using an Access Token for login or calling Sync at least once
    /// Lists all projects of the given organization
    ///
    /// Returns: [ProjectsResponse](bitwarden::secrets_manager::projects::ProjectsResponse)
    ///
    List(ProjectsListRequest),

    /// > Requires Authentication
    /// > Requires using an Access Token for login or calling Sync at least once
    /// Updates an existing project with the provided ID using the given data
    ///
    /// Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
    ///
    Update(ProjectPutRequest),

    /// > Requires Authentication
    /// > Requires using an Access Token for login or calling Sync at least once
    /// Deletes all the projects whose IDs match the provided ones
    ///
    /// Returns: [ProjectsDeleteResponse](bitwarden::secrets_manager::projects::ProjectsDeleteResponse)
    ///
    Delete(ProjectsDeleteRequest),
}

#[cfg(feature = "mobile")]
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum MobileCommand {
    Kdf(MobileKdfCommand),
    Crypto(MobileCryptoCommand),

    Vault(MobileVaultCommand),
}

#[cfg(feature = "mobile")]
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum MobileKdfCommand {
    /// Calculates the user master password hash based on the provided password and KDF parametes
    ///
    /// Returns: String
    ///
    HashPassword(PasswordHashRequest),
}

#[cfg(feature = "mobile")]
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum MobileCryptoCommand {
    #[cfg(feature = "internal")]
    /// Decrypts the users keys and initializes the user crypto, allowing for the encryption/decryption of the users vault
    ///
    InitCrypto(InitCryptoRequest),
}

#[cfg(feature = "mobile")]
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum MobileVaultCommand {
    Folders(MobileFoldersCommand),
    Ciphers(MobileCiphersCommand),
    PasswordHistory(MobilePasswordHistoryCommand),
    Collections(MobileCollectionsCommand),
}

#[cfg(feature = "mobile")]
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum MobileFoldersCommand {
    /// > Requires having previously initialized the cryptography parameters
    /// Encrypts the provided folder
    ///
    /// Returns: [FolderEncryptResponse](bitwarden::mobile::vault::FolderEncryptResponse)
    ///
    Encrypt(FolderEncryptRequest),
    /// > Requires having previously initialized the cryptography parameters
    /// Decrypts the provided folder
    ///
    /// Returns: [FolderDecryptResponse](bitwarden::mobile::vault::FolderDecryptResponse)
    ///
    Decrypt(FolderDecryptRequest),
    /// > Requires having previously initialized the cryptography parameters
    /// Decrypts the provided folders
    ///
    /// Returns: [FolderDecryptListResponse](bitwarden::mobile::vault::FolderDecryptListResponse)
    ///
    DecryptList(FolderDecryptListRequest),
}

#[cfg(feature = "mobile")]
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum MobileCiphersCommand {
    /// > Requires having previously initialized the cryptography parameters
    /// Encrypts the provided cipher
    ///
    /// Returns: [CipherEncryptResponse](bitwarden::mobile::vault::CipherEncryptResponse)
    ///
    Encrypt(Box<CipherEncryptRequest>),
    /// > Requires having previously initialized the cryptography parameters
    /// Decrypts the provided cipher
    ///
    /// Returns: [CipherDecryptResponse](bitwarden::mobile::vault::CipherDecryptResponse)
    ///
    Decrypt(Box<CipherDecryptRequest>),
    /// > Requires having previously initialized the cryptography parameters
    /// Decrypts the provided ciphers. Note that some sensitive fields might not be included in the response.
    /// To get them, use `DecryptCipher` for each cipher individually when those fields are needed
    ///
    /// Returns: [CipherDecryptListResponse](bitwarden::mobile::vault::CipherDecryptListResponse)
    ///
    DecryptList(Box<CipherDecryptListRequest>),
}

#[cfg(feature = "mobile")]
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum MobilePasswordHistoryCommand {
    /// > Requires having previously initialized the cryptography parameters
    /// Encrypts the provided password history entry
    ///
    /// Returns: [PasswordHistoryEncryptResponse](bitwarden::mobile::vault::PasswordHistoryEncryptResponse)
    ///
    Encrypt(PasswordHistoryEncryptRequest),
    /// > Requires having previously initialized the cryptography parameters
    /// Decrypts the provided password history
    ///
    /// Returns: [PasswordHistoryDecryptListResponse](bitwarden::mobile::vault::PasswordHistoryDecryptListResponse)
    ///
    DecryptList(PasswordHistoryDecryptListRequest),
}

#[cfg(feature = "mobile")]
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum MobileCollectionsCommand {
    /// > Requires having previously initialized the cryptography parameters
    /// Decrypts the provided collection
    ///
    /// Returns: [CollectionDecryptResponse](bitwarden::mobile::vault::CollectionDecryptResponse)
    ///
    Decrypt(CollectionDecryptRequest),
    /// > Requires having previously initialized the cryptography parameters
    /// Decrypts the provided collections
    ///
    /// Returns: [CollectionDecryptListResponse](bitwarden::mobile::vault::CollectionDecryptListResponse)
    ///
    DecryptList(CollectionDecryptListRequest),
}
