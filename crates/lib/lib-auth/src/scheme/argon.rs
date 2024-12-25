use std::sync::OnceLock;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use tokio::task::spawn_blocking;

use crate::error::{Error, Result};

use super::AuthScheme;

pub struct ArgonScheme {
    pub(crate) instance: Argon2<'static>,
    pub(crate) secret: &'static str,
}

static INSTANCE: OnceLock<ArgonScheme> = OnceLock::new();

impl ArgonScheme {
    pub fn init_instance(secret: &'static str) {
        let secret_bytes = secret.as_bytes();

        INSTANCE.get_or_init(|| ArgonScheme {
            instance: Argon2::new_with_secret(
                secret_bytes,
                argon2::Algorithm::Argon2id,
                argon2::Version::V0x13,
                Default::default(),
            )
            .unwrap(),
            secret,
        });
    }

    pub(crate) fn get_instance() -> Result<&'static ArgonScheme> {
        INSTANCE.get().ok_or(Error::NotInitialized)
    }
}

impl AuthScheme for ArgonScheme {
    async fn hash_password(plain_password: String) -> Result<String> {
        spawn_blocking(move || {
            let bytes = plain_password.as_bytes();
            let salt = SaltString::generate(&mut OsRng);
            Ok(Self::get_instance()?
                .instance
                .hash_password(bytes, &salt)?
                .to_string())
        })
        .await?
    }

    async fn verify_password(plain_password: String, hash: String) -> Result<bool> {
        spawn_blocking(move || {
            let parsed = PasswordHash::new(hash.as_str())?;
            Ok(Self::get_instance()?
                .instance
                .verify_password(plain_password.as_bytes(), &parsed)
                .is_ok())
        })
        .await?
    }

    fn get_secret_key() -> Result<&'static str> {
        Self::get_instance().map(|instance| instance.secret)
    }
}
