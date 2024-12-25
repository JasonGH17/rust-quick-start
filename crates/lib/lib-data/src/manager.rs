use crate::{create_database_pool, error::Result, Db};

#[derive(Clone)]
pub struct ModelManager {
    pool: Db,
}

impl ModelManager {
    pub async fn new(connection_url: &str) -> Result<Self> {
        Ok(ModelManager {
            pool: create_database_pool(connection_url).await?,
        })
    }

    pub fn new_from_pool(pool: Db) -> Self {
        Self { pool }
    }

    pub(crate) fn db(&self) -> &Db {
        &self.pool
    }
}
