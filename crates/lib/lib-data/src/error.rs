#[derive(Debug)]
pub enum Error {
    Auth(lib_auth::error::Error),
    PoolInit(sqlx::Error),
    Sqlx(sqlx::Error),
    InvalidUrl(url::ParseError),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<lib_auth::error::Error> for Error {
    fn from(value: lib_auth::error::Error) -> Self {
        Self::Auth(value)
    }
}

pub type Result<T> = ::core::result::Result<T, Error>;
