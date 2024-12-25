use jwt_simple::prelude::*;
use scheme::argon::ArgonScheme;
use serde::{Deserialize, Serialize};

pub mod helper;

use crate::{
    error::{Error, Result},
    scheme::{self, AuthScheme},
};

pub enum Scheme {
    Argon2,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JwtCustomClaims {
    pub uid: i32,
}

pub fn init_authentication(scheme: Scheme, secret: &'static str) {
    match scheme {
        Scheme::Argon2 => ArgonScheme::init_instance(secret),
    }
}

pub fn generate_jwt(claim_data: JwtCustomClaims) -> Result<String> {
    let key = HS256Key::from_bytes(ArgonScheme::get_secret_key()?.as_bytes());

    let claims =
        Claims::with_custom_claims(claim_data, Duration::from_days(14)).with_issuer("__WEB_AUTH__");
    key.authenticate(claims).map_err(Error::Jwt)
}

pub fn verify_jwt(token: &str) -> Result<i32> {
    let key = HS256Key::from_bytes(ArgonScheme::get_secret_key()?.as_bytes());

    let options = VerificationOptions {
        allowed_issuers: Some(HashSet::from_strings(&["__WEB_AUTH__"])),
        ..Default::default()
    };

    key.verify_token::<JwtCustomClaims>(token, Some(options))
        .map_err(Error::Jwt)
        .map(|data| data.custom.uid)
}
