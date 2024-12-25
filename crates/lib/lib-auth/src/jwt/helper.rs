use crate::{
    error::{Error, Result},
    scheme::{argon::ArgonScheme, AuthScheme},
};

use super::{generate_jwt, JwtCustomClaims};

pub async fn verify_password_and_generate_jwt(
    claim_data: JwtCustomClaims,
    plain_password: String,
    hash: String,
) -> Result<String> {
    ArgonScheme::verify_password(plain_password, hash)
        .await?
        .then_some(generate_jwt(claim_data)?)
        .ok_or(Error::WrongPassword)
}

/**
 * returns: (Hash, JWT)
 */
pub async fn generate_password_and_jwt(
    claim_data: JwtCustomClaims,
    plain_password: String,
) -> Result<(String, String)> {
    let jwt = generate_jwt(claim_data)?;
    ArgonScheme::hash_password(plain_password)
        .await
        .map(|hash| (hash, jwt))
}
