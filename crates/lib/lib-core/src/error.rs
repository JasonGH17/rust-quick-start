use axum::http::StatusCode;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Deserialize(serde_json::Error),
    Database(lib_data::error::Error),
    NotInitialized,
    TemplateNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<lib_data::error::Error> for Error {
    fn from(value: lib_data::error::Error) -> Self {
        Error::Database(value)
    }
}

impl<T> From<Error> for ApiResult<T> {
    fn from(_value: Error) -> Self {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub type ApiResult<T> = ::core::result::Result<T, StatusCode>;
pub type Result<T> = ::core::result::Result<T, Error>;
