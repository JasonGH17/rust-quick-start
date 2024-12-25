use lib_data::ModelManager;
use serde::Deserialize;

use crate::error::*;

#[derive(Clone)]
pub struct AppState {
    pub mm: ModelManager,
}

#[derive(Deserialize)]
struct Configuration {
    database_url: String,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let configuration = std::fs::read_to_string("config.json").map_err(Error::Io)?;

        let configuration: Configuration =
            serde_json::from_str(&configuration).map_err(Error::Deserialize)?;

        let mm = ModelManager::new(&configuration.database_url).await?;

        Ok(Self { mm })
    }
}
