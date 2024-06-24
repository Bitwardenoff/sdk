use std::collections::HashMap;

use bitwarden_crypto::{AsymmetricEncString, EncString};
#[cfg(feature = "internal")]
use bitwarden_crypto::{Kdf, KeyDecryptable, KeyEncryptable, MasterKey, SymmetricCryptoKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "internal")]
use crate::client::{LoginMethod, UserLoginMethod};
use crate::{
    error::{Error, Result},
    Client, VaultLocked,
};

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct InitUserCryptoRequest {
    /// The user's KDF parameters, as received from the prelogin request
    pub kdf_params: Kdf,
    /// The user's email address
    pub email: String,
    /// The user's encrypted private key
    pub private_key: String,
    /// The initialization method to use
    pub method: InitUserCryptoMethod,
}

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum InitUserCryptoMethod {
    Password {
        /// The user's master password
        password: String,
        /// The user's encrypted symmetric crypto key
        user_key: String,
    },
    DecryptedKey {
        /// The user's decrypted encryption key, obtained using `get_user_encryption_key`
        decrypted_user_key: String,
    },
    Pin {
        /// The user's PIN
        pin: String,
        /// The user's symmetric crypto key, encrypted with the PIN. Use `derive_pin_key` to obtain
        /// this.
        pin_protected_user_key: EncString,
    },
    AuthRequest {
        /// Private Key generated by the `crate::auth::new_auth_request`.
        request_private_key: String,

        method: AuthRequestMethod,
    },
    DeviceKey {
        /// The device's DeviceKey
        device_key: String,
        /// The Device Private Key
        protected_device_private_key: EncString,
        /// The user's symmetric crypto key, encrypted with the Device Key.
        device_protected_user_key: AsymmetricEncString,
    },
}

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum AuthRequestMethod {
    UserKey {
        /// User Key protected by the private key provided in `AuthRequestResponse`.
        protected_user_key: AsymmetricEncString,
    },
    MasterKey {
        /// Master Key protected by the private key provided in `AuthRequestResponse`.
        protected_master_key: AsymmetricEncString,
        /// User Key protected by the MasterKey, provided by the auth response.
        auth_request_key: EncString,
    },
}

#[cfg(feature = "internal")]
pub async fn initialize_user_crypto(client: &Client, req: InitUserCryptoRequest) -> Result<()> {
    use bitwarden_crypto::DeviceKey;

    use crate::auth::{auth_request_decrypt_master_key, auth_request_decrypt_user_key};

    let private_key: EncString = req.private_key.parse()?;

    match req.method {
        InitUserCryptoMethod::Password { password, user_key } => {
            let user_key: EncString = user_key.parse()?;

            let master_key =
                MasterKey::derive(password.as_bytes(), req.email.as_bytes(), &req.kdf_params)?;
            client
                .internal
                .initialize_user_crypto_master_key(master_key, user_key, private_key)?;
        }
        InitUserCryptoMethod::DecryptedKey { decrypted_user_key } => {
            let user_key = SymmetricCryptoKey::try_from(decrypted_user_key)?;
            client
                .internal
                .initialize_user_crypto_decrypted_key(user_key, private_key)?;
        }
        InitUserCryptoMethod::Pin {
            pin,
            pin_protected_user_key,
        } => {
            let pin_key = MasterKey::derive(pin.as_bytes(), req.email.as_bytes(), &req.kdf_params)?;
            client.internal.initialize_user_crypto_pin(
                pin_key,
                pin_protected_user_key,
                private_key,
            )?;
        }
        InitUserCryptoMethod::AuthRequest {
            request_private_key,
            method,
        } => {
            let user_key = match method {
                AuthRequestMethod::UserKey { protected_user_key } => {
                    auth_request_decrypt_user_key(request_private_key, protected_user_key)?
                }
                AuthRequestMethod::MasterKey {
                    protected_master_key,
                    auth_request_key,
                } => auth_request_decrypt_master_key(
                    request_private_key,
                    protected_master_key,
                    auth_request_key,
                )?,
            };
            client
                .internal
                .initialize_user_crypto_decrypted_key(user_key, private_key)?;
        }
        InitUserCryptoMethod::DeviceKey {
            device_key,
            protected_device_private_key,
            device_protected_user_key,
        } => {
            let device_key = DeviceKey::try_from(device_key)?;
            let user_key = device_key
                .decrypt_user_key(protected_device_private_key, device_protected_user_key)?;

            client
                .internal
                .initialize_user_crypto_decrypted_key(user_key, private_key)?;
        }
    }

    client
        .internal
        .set_login_method(crate::client::LoginMethod::User(
            crate::client::UserLoginMethod::Username {
                client_id: "".to_string(),
                email: req.email,
                kdf: req.kdf_params,
            },
        ));

    Ok(())
}

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct InitOrgCryptoRequest {
    /// The encryption keys for all the organizations the user is a part of
    pub organization_keys: HashMap<uuid::Uuid, AsymmetricEncString>,
}

#[cfg(feature = "internal")]
pub async fn initialize_org_crypto(client: &Client, req: InitOrgCryptoRequest) -> Result<()> {
    let organization_keys = req.organization_keys.into_iter().collect();
    client.internal.initialize_org_crypto(organization_keys)?;
    Ok(())
}

#[cfg(feature = "internal")]
pub async fn get_user_encryption_key(client: &Client) -> Result<String> {
    let enc = client.internal.get_encryption_settings()?;
    let user_key = enc.get_key(&None).ok_or(VaultLocked)?;

    Ok(user_key.to_base64())
}

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct UpdatePasswordResponse {
    /// Hash of the new password
    password_hash: String,
    /// User key, encrypted with the new password
    new_key: EncString,
}

pub fn update_password(client: &Client, new_password: String) -> Result<UpdatePasswordResponse> {
    let enc = client.internal.get_encryption_settings()?;
    let user_key = enc.get_key(&None).ok_or(VaultLocked)?;

    let login_method = client
        .internal
        .get_login_method()
        .ok_or(Error::NotAuthenticated)?;

    // Derive a new master key from password
    let new_master_key = match login_method.as_ref() {
        LoginMethod::User(
            UserLoginMethod::Username { email, kdf, .. }
            | UserLoginMethod::ApiKey { email, kdf, .. },
        ) => MasterKey::derive(new_password.as_bytes(), email.as_bytes(), kdf)?,
        #[cfg(feature = "secrets")]
        LoginMethod::ServiceAccount(_) => return Err(Error::NotAuthenticated),
    };

    let new_key = new_master_key.encrypt_user_key(user_key)?;

    let password_hash = new_master_key.derive_master_key_hash(
        new_password.as_bytes(),
        bitwarden_crypto::HashPurpose::ServerAuthorization,
    )?;

    Ok(UpdatePasswordResponse {
        password_hash,
        new_key,
    })
}

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct DerivePinKeyResponse {
    /// [UserKey](bitwarden_crypto::UserKey) protected by PIN
    pin_protected_user_key: EncString,
    /// PIN protected by [UserKey](bitwarden_crypto::UserKey)
    encrypted_pin: EncString,
}

#[cfg(feature = "internal")]
pub fn derive_pin_key(client: &Client, pin: String) -> Result<DerivePinKeyResponse> {
    let enc = client.internal.get_encryption_settings()?;
    let user_key = enc.get_key(&None).ok_or(VaultLocked)?;

    let login_method = client
        .internal
        .get_login_method()
        .ok_or(Error::NotAuthenticated)?;

    let pin_protected_user_key = derive_pin_protected_user_key(&pin, &login_method, user_key)?;

    Ok(DerivePinKeyResponse {
        pin_protected_user_key,
        encrypted_pin: pin.encrypt_with_key(user_key)?,
    })
}

#[cfg(feature = "internal")]
pub fn derive_pin_user_key(client: &Client, encrypted_pin: EncString) -> Result<EncString> {
    let enc = client.internal.get_encryption_settings()?;
    let user_key = enc.get_key(&None).ok_or(VaultLocked)?;

    let pin: String = encrypted_pin.decrypt_with_key(user_key)?;
    let login_method = client
        .internal
        .get_login_method()
        .ok_or(Error::NotAuthenticated)?;

    derive_pin_protected_user_key(&pin, &login_method, user_key)
}

#[cfg(feature = "internal")]
fn derive_pin_protected_user_key(
    pin: &str,
    login_method: &LoginMethod,
    user_key: &SymmetricCryptoKey,
) -> Result<EncString> {
    let derived_key = match login_method {
        LoginMethod::User(
            UserLoginMethod::Username { email, kdf, .. }
            | UserLoginMethod::ApiKey { email, kdf, .. },
        ) => MasterKey::derive(pin.as_bytes(), email.as_bytes(), kdf)?,
        #[cfg(feature = "secrets")]
        LoginMethod::ServiceAccount(_) => return Err(Error::NotAuthenticated),
    };

    Ok(derived_key.encrypt_user_key(user_key)?)
}

#[cfg(feature = "internal")]
pub(super) fn enroll_admin_password_reset(
    client: &Client,
    public_key: String,
) -> Result<AsymmetricEncString> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use bitwarden_crypto::AsymmetricPublicCryptoKey;

    let public_key = AsymmetricPublicCryptoKey::from_der(&STANDARD.decode(public_key)?)?;
    let enc = client.internal.get_encryption_settings()?;
    let key = enc.get_key(&None).ok_or(VaultLocked)?;

    Ok(AsymmetricEncString::encrypt_rsa2048_oaep_sha1(
        &key.to_vec(),
        &public_key,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Client;

    #[tokio::test]
    async fn test_update_password() {
        let client = Client::new(None);

        let priv_key = "2.kmLY8NJVuiKBFJtNd/ZFpA==|qOodlRXER+9ogCe3yOibRHmUcSNvjSKhdDuztLlucs10jLiNoVVVAc+9KfNErLSpx5wmUF1hBOJM8zwVPjgQTrmnNf/wuDpwiaCxNYb/0v4FygPy7ccAHK94xP1lfqq7U9+tv+/yiZSwgcT+xF0wFpoxQeNdNRFzPTuD9o4134n8bzacD9DV/WjcrXfRjbBCzzuUGj1e78+A7BWN7/5IWLz87KWk8G7O/W4+8PtEzlwkru6Wd1xO19GYU18oArCWCNoegSmcGn7w7NDEXlwD403oY8Oa7ylnbqGE28PVJx+HLPNIdSC6YKXeIOMnVs7Mctd/wXC93zGxAWD6ooTCzHSPVV50zKJmWIG2cVVUS7j35H3rGDtUHLI+ASXMEux9REZB8CdVOZMzp2wYeiOpggebJy6MKOZqPT1R3X0fqF2dHtRFPXrNsVr1Qt6bS9qTyO4ag1/BCvXF3P1uJEsI812BFAne3cYHy5bIOxuozPfipJrTb5WH35bxhElqwT3y/o/6JWOGg3HLDun31YmiZ2HScAsUAcEkA4hhoTNnqy4O2s3yVbCcR7jF7NLsbQc0MDTbnjxTdI4VnqUIn8s2c9hIJy/j80pmO9Bjxp+LQ9a2hUkfHgFhgHxZUVaeGVth8zG2kkgGdrp5VHhxMVFfvB26Ka6q6qE/UcS2lONSv+4T8niVRJz57qwctj8MNOkA3PTEfe/DP/LKMefke31YfT0xogHsLhDkx+mS8FCc01HReTjKLktk/Jh9mXwC5oKwueWWwlxI935ecn+3I2kAuOfMsgPLkoEBlwgiREC1pM7VVX1x8WmzIQVQTHd4iwnX96QewYckGRfNYWz/zwvWnjWlfcg8kRSe+68EHOGeRtC5r27fWLqRc0HNcjwpgHkI/b6czerCe8+07TWql4keJxJxhBYj3iOH7r9ZS8ck51XnOb8tGL1isimAJXodYGzakwktqHAD7MZhS+P02O+6jrg7d+yPC2ZCuS/3TOplYOCHQIhnZtR87PXTUwr83zfOwAwCyv6KP84JUQ45+DItrXLap7nOVZKQ5QxYIlbThAO6eima6Zu5XHfqGPMNWv0bLf5+vAjIa5np5DJrSwz9no/hj6CUh0iyI+SJq4RGI60lKtypMvF6MR3nHLEHOycRUQbZIyTHWl4QQLdHzuwN9lv10ouTEvNr6sFflAX2yb6w3hlCo7oBytH3rJekjb3IIOzBpeTPIejxzVlh0N9OT5MZdh4sNKYHUoWJ8mnfjdM+L4j5Q2Kgk/XiGDgEebkUxiEOQUdVpePF5uSCE+TPav/9FIRGXGiFn6NJMaU7aBsDTFBLloffFLYDpd8/bTwoSvifkj7buwLYM+h/qcnfdy5FWau1cKav+Blq/ZC0qBpo658RTC8ZtseAFDgXoQZuksM10hpP9bzD04Bx30xTGX81QbaSTNwSEEVrOtIhbDrj9OI43KH4O6zLzK+t30QxAv5zjk10RZ4+5SAdYndIlld9Y62opCfPDzRy3ubdve4ZEchpIKWTQvIxq3T5ogOhGaWBVYnkMtM2GVqvWV//46gET5SH/MdcwhACUcZ9kCpMnWH9CyyUwYvTT3UlNyV+DlS27LMPvaw7tx7qa+GfNCoCBd8S4esZpQYK/WReiS8=|pc7qpD42wxyXemdNPuwxbh8iIaryrBPu8f/DGwYdHTw=";

        let kdf = Kdf::PBKDF2 {
            iterations: 100_000.try_into().unwrap(),
        };

        initialize_user_crypto(
            & client,
            InitUserCryptoRequest {
                kdf_params: kdf.clone(),
                email: "test@bitwarden.com".into(),
                private_key: priv_key.to_owned(),
                method: InitUserCryptoMethod::Password {
                    password: "asdfasdfasdf".into(),
                    user_key: "2.u2HDQ/nH2J7f5tYHctZx6Q==|NnUKODz8TPycWJA5svexe1wJIz2VexvLbZh2RDfhj5VI3wP8ZkR0Vicvdv7oJRyLI1GyaZDBCf9CTBunRTYUk39DbZl42Rb+Xmzds02EQhc=|rwuo5wgqvTJf3rgwOUfabUyzqhguMYb3sGBjOYqjevc=".into(),
                },
            },
        )
        .await
        .unwrap();

        let new_password_response = update_password(&client, "123412341234".into()).unwrap();

        let client2 = Client::new(None);

        initialize_user_crypto(
            &client2,
            InitUserCryptoRequest {
                kdf_params: kdf.clone(),
                email: "test@bitwarden.com".into(),
                private_key: priv_key.to_owned(),
                method: InitUserCryptoMethod::Password {
                    password: "123412341234".into(),
                    user_key: new_password_response.new_key.to_string(),
                },
            },
        )
        .await
        .unwrap();

        let new_hash = client2
            .kdf()
            .hash_password(
                "test@bitwarden.com".into(),
                "123412341234".into(),
                kdf.clone(),
                bitwarden_crypto::HashPurpose::ServerAuthorization,
            )
            .await
            .unwrap();

        assert_eq!(new_hash, new_password_response.password_hash);

        assert_eq!(
            client
                .internal
                .get_encryption_settings()
                .unwrap()
                .get_key(&None)
                .unwrap()
                .to_base64(),
            client2
                .internal
                .get_encryption_settings()
                .unwrap()
                .get_key(&None)
                .unwrap()
                .to_base64()
        );
    }

    #[tokio::test]
    async fn test_initialize_user_crypto_pin() {
        let client = Client::new(None);

        let priv_key = "2.kmLY8NJVuiKBFJtNd/ZFpA==|qOodlRXER+9ogCe3yOibRHmUcSNvjSKhdDuztLlucs10jLiNoVVVAc+9KfNErLSpx5wmUF1hBOJM8zwVPjgQTrmnNf/wuDpwiaCxNYb/0v4FygPy7ccAHK94xP1lfqq7U9+tv+/yiZSwgcT+xF0wFpoxQeNdNRFzPTuD9o4134n8bzacD9DV/WjcrXfRjbBCzzuUGj1e78+A7BWN7/5IWLz87KWk8G7O/W4+8PtEzlwkru6Wd1xO19GYU18oArCWCNoegSmcGn7w7NDEXlwD403oY8Oa7ylnbqGE28PVJx+HLPNIdSC6YKXeIOMnVs7Mctd/wXC93zGxAWD6ooTCzHSPVV50zKJmWIG2cVVUS7j35H3rGDtUHLI+ASXMEux9REZB8CdVOZMzp2wYeiOpggebJy6MKOZqPT1R3X0fqF2dHtRFPXrNsVr1Qt6bS9qTyO4ag1/BCvXF3P1uJEsI812BFAne3cYHy5bIOxuozPfipJrTb5WH35bxhElqwT3y/o/6JWOGg3HLDun31YmiZ2HScAsUAcEkA4hhoTNnqy4O2s3yVbCcR7jF7NLsbQc0MDTbnjxTdI4VnqUIn8s2c9hIJy/j80pmO9Bjxp+LQ9a2hUkfHgFhgHxZUVaeGVth8zG2kkgGdrp5VHhxMVFfvB26Ka6q6qE/UcS2lONSv+4T8niVRJz57qwctj8MNOkA3PTEfe/DP/LKMefke31YfT0xogHsLhDkx+mS8FCc01HReTjKLktk/Jh9mXwC5oKwueWWwlxI935ecn+3I2kAuOfMsgPLkoEBlwgiREC1pM7VVX1x8WmzIQVQTHd4iwnX96QewYckGRfNYWz/zwvWnjWlfcg8kRSe+68EHOGeRtC5r27fWLqRc0HNcjwpgHkI/b6czerCe8+07TWql4keJxJxhBYj3iOH7r9ZS8ck51XnOb8tGL1isimAJXodYGzakwktqHAD7MZhS+P02O+6jrg7d+yPC2ZCuS/3TOplYOCHQIhnZtR87PXTUwr83zfOwAwCyv6KP84JUQ45+DItrXLap7nOVZKQ5QxYIlbThAO6eima6Zu5XHfqGPMNWv0bLf5+vAjIa5np5DJrSwz9no/hj6CUh0iyI+SJq4RGI60lKtypMvF6MR3nHLEHOycRUQbZIyTHWl4QQLdHzuwN9lv10ouTEvNr6sFflAX2yb6w3hlCo7oBytH3rJekjb3IIOzBpeTPIejxzVlh0N9OT5MZdh4sNKYHUoWJ8mnfjdM+L4j5Q2Kgk/XiGDgEebkUxiEOQUdVpePF5uSCE+TPav/9FIRGXGiFn6NJMaU7aBsDTFBLloffFLYDpd8/bTwoSvifkj7buwLYM+h/qcnfdy5FWau1cKav+Blq/ZC0qBpo658RTC8ZtseAFDgXoQZuksM10hpP9bzD04Bx30xTGX81QbaSTNwSEEVrOtIhbDrj9OI43KH4O6zLzK+t30QxAv5zjk10RZ4+5SAdYndIlld9Y62opCfPDzRy3ubdve4ZEchpIKWTQvIxq3T5ogOhGaWBVYnkMtM2GVqvWV//46gET5SH/MdcwhACUcZ9kCpMnWH9CyyUwYvTT3UlNyV+DlS27LMPvaw7tx7qa+GfNCoCBd8S4esZpQYK/WReiS8=|pc7qpD42wxyXemdNPuwxbh8iIaryrBPu8f/DGwYdHTw=";

        initialize_user_crypto(
            & client,
            InitUserCryptoRequest {
                kdf_params: Kdf::PBKDF2 {
                    iterations: 100_000.try_into().unwrap(),
                },
                email: "test@bitwarden.com".into(),
                private_key: priv_key.to_owned(),
                method: InitUserCryptoMethod::Password {
                    password: "asdfasdfasdf".into(),
                    user_key: "2.u2HDQ/nH2J7f5tYHctZx6Q==|NnUKODz8TPycWJA5svexe1wJIz2VexvLbZh2RDfhj5VI3wP8ZkR0Vicvdv7oJRyLI1GyaZDBCf9CTBunRTYUk39DbZl42Rb+Xmzds02EQhc=|rwuo5wgqvTJf3rgwOUfabUyzqhguMYb3sGBjOYqjevc=".into(),
                },
            },
        )
        .await
        .unwrap();

        let pin_key = derive_pin_key(&client, "1234".into()).unwrap();

        // Verify we can unlock with the pin
        let client2 = Client::new(None);
        initialize_user_crypto(
            &client2,
            InitUserCryptoRequest {
                kdf_params: Kdf::PBKDF2 {
                    iterations: 100_000.try_into().unwrap(),
                },
                email: "test@bitwarden.com".into(),
                private_key: priv_key.to_owned(),
                method: InitUserCryptoMethod::Pin {
                    pin: "1234".into(),
                    pin_protected_user_key: pin_key.pin_protected_user_key,
                },
            },
        )
        .await
        .unwrap();

        assert_eq!(
            client
                .internal
                .get_encryption_settings()
                .unwrap()
                .get_key(&None)
                .unwrap()
                .to_base64(),
            client2
                .internal
                .get_encryption_settings()
                .unwrap()
                .get_key(&None)
                .unwrap()
                .to_base64()
        );

        // Verify we can derive the pin protected user key from the encrypted pin
        let pin_protected_user_key = derive_pin_user_key(&client, pin_key.encrypted_pin).unwrap();

        let client3 = Client::new(None);

        initialize_user_crypto(
            &client3,
            InitUserCryptoRequest {
                kdf_params: Kdf::PBKDF2 {
                    iterations: 100_000.try_into().unwrap(),
                },
                email: "test@bitwarden.com".into(),
                private_key: priv_key.to_owned(),
                method: InitUserCryptoMethod::Pin {
                    pin: "1234".into(),
                    pin_protected_user_key,
                },
            },
        )
        .await
        .unwrap();

        assert_eq!(
            client
                .internal
                .get_encryption_settings()
                .unwrap()
                .get_key(&None)
                .unwrap()
                .to_base64(),
            client3
                .internal
                .get_encryption_settings()
                .unwrap()
                .get_key(&None)
                .unwrap()
                .to_base64()
        );
    }

    #[cfg(feature = "internal")]
    #[test]
    fn test_enroll_admin_password_reset() {
        use std::num::NonZeroU32;

        use base64::{engine::general_purpose::STANDARD, Engine};
        use bitwarden_crypto::AsymmetricCryptoKey;

        let client = Client::new(None);

        let master_key = bitwarden_crypto::MasterKey::derive(
            "asdfasdfasdf".as_bytes(),
            "test@bitwarden.com".as_bytes(),
            &Kdf::PBKDF2 {
                iterations: NonZeroU32::new(600_000).unwrap(),
            },
        )
        .unwrap();

        let user_key = "2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=".parse().unwrap();
        let private_key ="2.yN7l00BOlUE0Sb0M//Q53w==|EwKG/BduQRQ33Izqc/ogoBROIoI5dmgrxSo82sgzgAMIBt3A2FZ9vPRMY+GWT85JiqytDitGR3TqwnFUBhKUpRRAq4x7rA6A1arHrFp5Tp1p21O3SfjtvB3quiOKbqWk6ZaU1Np9HwqwAecddFcB0YyBEiRX3VwF2pgpAdiPbSMuvo2qIgyob0CUoC/h4Bz1be7Qa7B0Xw9/fMKkB1LpOm925lzqosyMQM62YpMGkjMsbZz0uPopu32fxzDWSPr+kekNNyLt9InGhTpxLmq1go/pXR2uw5dfpXc5yuta7DB0EGBwnQ8Vl5HPdDooqOTD9I1jE0mRyuBpWTTI3FRnu3JUh3rIyGBJhUmHqGZvw2CKdqHCIrQeQkkEYqOeJRJVdBjhv5KGJifqT3BFRwX/YFJIChAQpebNQKXe/0kPivWokHWwXlDB7S7mBZzhaAPidZvnuIhalE2qmTypDwHy22FyqV58T8MGGMchcASDi/QXI6kcdpJzPXSeU9o+NC68QDlOIrMVxKFeE7w7PvVmAaxEo0YwmuAzzKy9QpdlK0aab/xEi8V4iXj4hGepqAvHkXIQd+r3FNeiLfllkb61p6WTjr5urcmDQMR94/wYoilpG5OlybHdbhsYHvIzYoLrC7fzl630gcO6t4nM24vdB6Ymg9BVpEgKRAxSbE62Tqacxqnz9AcmgItb48NiR/He3n3ydGjPYuKk/ihZMgEwAEZvSlNxYONSbYrIGDtOY+8Nbt6KiH3l06wjZW8tcmFeVlWv+tWotnTY9IqlAfvNVTjtsobqtQnvsiDjdEVtNy/s2ci5TH+NdZluca2OVEr91Wayxh70kpM6ib4UGbfdmGgCo74gtKvKSJU0rTHakQ5L9JlaSDD5FamBRyI0qfL43Ad9qOUZ8DaffDCyuaVyuqk7cz9HwmEmvWU3VQ+5t06n/5kRDXttcw8w+3qClEEdGo1KeENcnXCB32dQe3tDTFpuAIMLqwXs6FhpawfZ5kPYvLPczGWaqftIs/RXJ/EltGc0ugw2dmTLpoQhCqrcKEBDoYVk0LDZKsnzitOGdi9mOWse7Se8798ib1UsHFUjGzISEt6upestxOeupSTOh0v4+AjXbDzRUyogHww3V+Bqg71bkcMxtB+WM+pn1XNbVTyl9NR040nhP7KEf6e9ruXAtmrBC2ah5cFEpLIot77VFZ9ilLuitSz+7T8n1yAh1IEG6xxXxninAZIzi2qGbH69O5RSpOJuJTv17zTLJQIIc781JwQ2TTwTGnx5wZLbffhCasowJKd2EVcyMJyhz6ru0PvXWJ4hUdkARJs3Xu8dus9a86N8Xk6aAPzBDqzYb1vyFIfBxP0oO8xFHgd30Cgmz8UrSE3qeWRrF8ftrI6xQnFjHBGWD/JWSvd6YMcQED0aVuQkuNW9ST/DzQThPzRfPUoiL10yAmV7Ytu4fR3x2sF0Yfi87YhHFuCMpV/DsqxmUizyiJuD938eRcH8hzR/VO53Qo3UIsqOLcyXtTv6THjSlTopQ+JOLOnHm1w8dzYbLN44OG44rRsbihMUQp+wUZ6bsI8rrOnm9WErzkbQFbrfAINdoCiNa6cimYIjvvnMTaFWNymqY1vZxGztQiMiHiHYwTfwHTXrb9j0uPM=|09J28iXv9oWzYtzK2LBT6Yht4IT4MijEkk0fwFdrVQ4=".parse().unwrap();
        client
            .internal
            .initialize_user_crypto_master_key(master_key, user_key, private_key)
            .unwrap();

        let public_key = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAsy7RFHcX3C8Q4/OMmhhbFReYWfB45W9PDTEA8tUZwZmtOiN2RErIS2M1c+K/4HoDJ/TjpbX1f2MZcr4nWvKFuqnZXyewFc+jmvKVewYi+NAu2++vqKq2kKcmMNhwoQDQdQIVy/Uqlp4Cpi2cIwO6ogq5nHNJGR3jm+CpyrafYlbz1bPvL3hbyoGDuG2tgADhyhXUdFuef2oF3wMvn1lAJAvJnPYpMiXUFmj1ejmbwtlxZDrHgUJvUcp7nYdwUKaFoi+sOttHn3u7eZPtNvxMjhSS/X/1xBIzP/mKNLdywH5LoRxniokUk+fV3PYUxJsiU3lV0Trc/tH46jqd8ZGjmwIDAQAB";

        let encrypted = enroll_admin_password_reset(&client, public_key.to_owned()).unwrap();

        let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCzLtEUdxfcLxDj84yaGFsVF5hZ8Hjlb08NMQDy1RnBma06I3ZESshLYzVz4r/gegMn9OOltfV/Yxlyvida8oW6qdlfJ7AVz6Oa8pV7BiL40C7b76+oqraQpyYw2HChANB1AhXL9SqWngKmLZwjA7qiCrmcc0kZHeOb4KnKtp9iVvPVs+8veFvKgYO4ba2AAOHKFdR0W55/agXfAy+fWUAkC8mc9ikyJdQWaPV6OZvC2XFkOseBQm9Rynudh3BQpoWiL6w620efe7t5k+02/EyOFJL9f/XEEjM/+Yo0t3LAfkuhHGeKiRST59Xc9hTEmyJTeVXROtz+0fjqOp3xkaObAgMBAAECggEACs4xhnO0HaZhh1/iH7zORMIRXKeyxP2LQiTR8xwN5JJ9wRWmGAR9VasS7EZFTDidIGVME2u/h4s5EqXnhxfO+0gGksVvgNXJ/qw87E8K2216g6ZNo6vSGA7H1GH2voWwejJ4/k/cJug6dz2S402rRAKh2Wong1arYHSkVlQp3diiMa5FHAOSE+Cy09O2ZsaF9IXQYUtlW6AVXFrBEPYH2kvkaPXchh8VETMijo6tbvoKLnUHe+wTaDMls7hy8exjtVyI59r3DNzjy1lNGaGb5QSnFMXR+eHhPZc844Wv02MxC15zKABADrl58gpJyjTl6XpDdHCYGsmGpVGH3X9TQQKBgQDz/9beFjzq59ve6rGwn+EtnQfSsyYT+jr7GN8lNEXb3YOFXBgPhfFIcHRh2R00Vm9w2ApfAx2cd8xm2I6HuvQ1Os7g26LWazvuWY0Qzb+KaCLQTEGH1RnTq6CCG+BTRq/a3J8M4t38GV5TWlzv8wr9U4dl6FR4efjb65HXs1GQ4QKBgQC7/uHfrOTEHrLeIeqEuSl0vWNqEotFKdKLV6xpOvNuxDGbgW4/r/zaxDqt0YBOXmRbQYSEhmO3oy9J6XfE1SUln0gbavZeW0HESCAmUIC88bDnspUwS9RxauqT5aF8ODKN/bNCWCnBM1xyonPOs1oT1nyparJVdQoG//Y7vkB3+wKBgBqLqPq8fKAp3XfhHLfUjREDVoiLyQa/YI9U42IOz9LdxKNLo6p8rgVthpvmnRDGnpUuS+KOWjhdqDVANjF6G3t3DG7WNl8Rh5Gk2H4NhFswfSkgQrjebFLlBy9gjQVCWXt8KSmjvPbiY6q52Aaa8IUjA0YJAregvXxfopxO+/7BAoGARicvEtDp7WWnSc1OPoj6N14VIxgYcI7SyrzE0d/1x3ffKzB5e7qomNpxKzvqrVP8DzG7ydh8jaKPmv1MfF8tpYRy3AhmN3/GYwCnPqT75YYrhcrWcVdax5gmQVqHkFtIQkRSCIftzPLlpMGKha/YBV8c1fvC4LD0NPh/Ynv0gtECgYEAyOZg95/kte0jpgUEgwuMrzkhY/AaUJULFuR5MkyvReEbtSBQwV5tx60+T95PHNiFooWWVXiLMsAgyI2IbkxVR1Pzdri3gWK5CTfqb7kLuaj/B7SGvBa2Sxo478KS5K8tBBBWkITqo+wLC0mn3uZi1dyMWO1zopTA+KtEGF2dtGQ=";
        let private_key =
            AsymmetricCryptoKey::from_der(&STANDARD.decode(private_key).unwrap()).unwrap();
        let decrypted: Vec<u8> = encrypted.decrypt_with_key(&private_key).unwrap();

        let enc = client.internal.get_encryption_settings().unwrap();
        let expected = enc.get_key(&None).unwrap();
        assert_eq!(&decrypted, &expected.to_vec());
    }
}
