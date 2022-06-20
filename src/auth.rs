use anyhow::bail;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use password_hash::{PasswordHash, PasswordVerifier};
use rand::{thread_rng, Rng};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

const PASSWORD_MIN_LENGTH: usize = 8;
const PASSWORD_MAX_LENGTH: usize = 64;

#[derive(Deserialize)]
#[serde(try_from = "String")]
pub struct Password(Secret<String>);

pub fn gen_auth_token() -> String {
    let token: [u8; 30] = thread_rng().gen();
    base64::encode(&token)
}

impl Password {
    pub fn compute_hash(&self) -> Result<String, anyhow::Error> {
        let salt = SaltString::generate(&mut thread_rng());
        // Values from OWASP cheatsheet
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, Some(32)).unwrap(),
        )
        .hash_password(self.0.expose_secret().as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    pub fn matches_hash(&self, hash: PasswordHash) -> bool {
        Argon2::default()
            .verify_password(self.0.expose_secret().as_bytes(), &hash)
            .is_ok()
    }

    pub fn inner(&self) -> &Secret<String> {
        &self.0
    }
}

impl TryFrom<String> for Password {
    type Error = anyhow::Error;

    fn try_from(password: String) -> Result<Self, Self::Error> {
        if password.len() < PASSWORD_MIN_LENGTH {
            bail!("Password is too short");
        }
        if password.len() > PASSWORD_MAX_LENGTH {
            bail!("Password is too long");
        }

        Ok(Password(Secret::new(password)))
    }
}
