use std::{fmt::Display, str::FromStr};

use base64::Engine;
use serde::{de::Visitor, Deserialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{decrypt_aes256, Decryptable, Encryptable, SymmetricCryptoKey},
    error::{CryptoError, EncStringParseError, Error, Result},
    util::BASE64_ENGINE,
};

#[allow(unused, non_camel_case_types)]
pub enum EncString {
    // 0
    AesCbc256_B64 {
        iv: [u8; 16],
        data: Vec<u8>,
    },
    // 1
    AesCbc128_HmacSha256_B64 {
        iv: [u8; 16],
        mac: [u8; 32],
        data: Vec<u8>,
    },
    // 2
    AesCbc256_HmacSha256_B64 {
        iv: [u8; 16],
        mac: [u8; 32],
        data: Vec<u8>,
    },
    // 3
    Rsa2048_OaepSha256_B64 {
        data: Vec<u8>,
    },
    // 4
    Rsa2048_OaepSha1_B64 {
        data: Vec<u8>,
    },
    // 5
    Rsa2048_OaepSha256_HmacSha256_B64 {
        mac: [u8; 32],
        data: Vec<u8>,
    },
    // 6
    Rsa2048_OaepSha1_HmacSha256_B64 {
        mac: [u8; 32],
        data: Vec<u8>,
    },
}

// We manually implement these to make sure we don't print any sensitive data
impl std::fmt::Debug for EncString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncString").finish()
    }
}

impl FromStr for EncString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (enc_type, data) = s.split_once('.').ok_or(EncStringParseError::NoType)?;

        let parts: Vec<_> = data.split('|').collect();
        match (enc_type, parts.len()) {
            ("0", 2) => unimplemented!(),

            ("1" | "2", 3) => {
                let iv_str = parts[0];
                let data_str = parts[1];
                let mac_str = parts[2];

                let iv = BASE64_ENGINE
                    .decode(iv_str)
                    .map_err(EncStringParseError::InvalidBase64)?
                    .try_into()
                    .map_err(invalid_len_error(16))?;

                let mac = BASE64_ENGINE
                    .decode(mac_str)
                    .map_err(EncStringParseError::InvalidBase64)?
                    .try_into()
                    .map_err(invalid_len_error(32))?;

                let data = BASE64_ENGINE
                    .decode(data_str)
                    .map_err(EncStringParseError::InvalidBase64)?;

                if enc_type == "1" {
                    Ok(EncString::AesCbc128_HmacSha256_B64 { iv, mac, data })
                } else {
                    Ok(EncString::AesCbc256_HmacSha256_B64 { iv, mac, data })
                }
            }

            ("3" | "4", 1) => {
                let data = BASE64_ENGINE
                    .decode(data)
                    .map_err(EncStringParseError::InvalidBase64)?;
                if enc_type == "3" {
                    Ok(EncString::Rsa2048_OaepSha256_B64 { data })
                } else {
                    Ok(EncString::Rsa2048_OaepSha1_B64 { data })
                }
            }
            ("5" | "6", 2) => {
                unimplemented!()
            }

            (enc_type, parts) => Err(EncStringParseError::InvalidType {
                enc_type: enc_type.to_string(),
                parts,
            }
            .into()),
        }
    }
}

impl Display for EncString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.", self.enc_type())?;

        let mut parts = Vec::<&[u8]>::new();

        match self {
            EncString::AesCbc256_B64 { iv, data } => {
                parts.push(iv);
                parts.push(data);
            }
            EncString::AesCbc128_HmacSha256_B64 { iv, mac, data } => {
                parts.push(iv);
                parts.push(data);
                parts.push(mac);
            }
            EncString::AesCbc256_HmacSha256_B64 { iv, mac, data } => {
                parts.push(iv);
                parts.push(data);
                parts.push(mac);
            }
            EncString::Rsa2048_OaepSha256_B64 { data } => {
                parts.push(data);
            }
            EncString::Rsa2048_OaepSha1_B64 { data } => {
                parts.push(data);
            }
            EncString::Rsa2048_OaepSha256_HmacSha256_B64 { mac, data } => {
                parts.push(data);
                parts.push(mac);
            }
            EncString::Rsa2048_OaepSha1_HmacSha256_B64 { mac, data } => {
                parts.push(data);
                parts.push(mac);
            }
        }

        for i in 0..parts.len() {
            if i == parts.len() - 1 {
                write!(f, "{}", BASE64_ENGINE.encode(parts[i]))?;
            } else {
                write!(f, "{}|", BASE64_ENGINE.encode(parts[i]))?;
            }
        }

        Ok(())
    }
}

impl<'de> Deserialize<'de> for EncString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CSVisitor;
        impl Visitor<'_> for CSVisitor {
            type Value = EncString;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "A valid string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                EncString::from_str(v).map_err(|e| E::custom(format!("{:?}", e)))
            }
        }

        deserializer.deserialize_str(CSVisitor)
    }
}

impl serde::Serialize for EncString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl EncString {
    fn enc_type(&self) -> u8 {
        match self {
            EncString::AesCbc256_B64 { .. } => 0,
            EncString::AesCbc128_HmacSha256_B64 { .. } => 1,
            EncString::AesCbc256_HmacSha256_B64 { .. } => 2,
            EncString::Rsa2048_OaepSha256_B64 { .. } => 3,
            EncString::Rsa2048_OaepSha1_B64 { .. } => 4,
            EncString::Rsa2048_OaepSha256_HmacSha256_B64 { .. } => 5,
            EncString::Rsa2048_OaepSha1_HmacSha256_B64 { .. } => 6,
        }
    }

    pub fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<Vec<u8>> {
        match self {
            EncString::AesCbc256_HmacSha256_B64 { iv, mac, data } => {
                let dec = decrypt_aes256(iv, mac, data.clone(), key.mac_key, key.key)?;
                Ok(dec)
            }
            _ => Err(CryptoError::InvalidKey.into()),
        }
    }
}

fn invalid_len_error(expected: usize) -> impl Fn(Vec<u8>) -> EncStringParseError {
    move |e: Vec<_>| EncStringParseError::InvalidBase64Length {
        expected,
        got: e.len(),
    }
}

impl Encryptable<EncString> for String {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<EncString> {
        enc.encrypt(self.as_bytes(), org_id)
    }
}

impl Decryptable<String> for EncString {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<String> {
        enc.decrypt(self, org_id)
    }
}

#[cfg(test)]
mod tests {
    use super::EncString;

    #[test]
    fn test_enc_string_serialization() {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Test {
            key: EncString,
        }

        let cipher = "2.pMS6/icTQABtulw52pq2lg==|XXbxKxDTh+mWiN1HjH2N1w==|Q6PkuT+KX/axrgN9ubD5Ajk2YNwxQkgs3WJM0S0wtG8=";
        let serialized = format!("{{\"key\":\"{cipher}\"}}");

        let t = serde_json::from_str::<Test>(&serialized).unwrap();
        assert_eq!(t.key.enc_type(), 2);
        assert_eq!(t.key.to_string(), cipher);
        assert_eq!(serde_json::to_string(&t).unwrap(), serialized);
    }
}
