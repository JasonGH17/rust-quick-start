use crate::error::Result;

pub mod argon;

pub trait AuthScheme {
    fn hash_password(plain_password: String) -> impl std::future::Future<Output = Result<String>> + Send;
    fn verify_password(plain_password: String, hash: String) -> impl std::future::Future<Output = Result<bool>> + Send;
    fn get_secret_key() -> Result<&'static str>;
}
