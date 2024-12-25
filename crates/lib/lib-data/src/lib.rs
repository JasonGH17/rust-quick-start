pub mod dto;
pub mod error;
mod manager;
mod models;

pub use manager::ModelManager;
pub use models::*;

use std::str::FromStr;

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};
use url::Url;

pub type Db = Pool<Postgres>;

// Date/Time type def helpers
pub type DateTimeLocal = chrono::DateTime<chrono::Local>;
pub type DateTimeUtc = chrono::DateTime<chrono::Utc>;
pub type Date = chrono::NaiveDate;

pub async fn create_database_pool(uri: &str) -> error::Result<Db> {
    let options =
        PgConnectOptions::from_url(&Url::from_str(uri).map_err(error::Error::InvalidUrl)?)
            .map_err(error::Error::PoolInit)?
            .application_name("server")
            .log_statements(log::LevelFilter::Trace);

    PgPoolOptions::new()
        .max_connections(3)
        .connect_with(options)
        .await
        .map_err(error::Error::PoolInit)
}
