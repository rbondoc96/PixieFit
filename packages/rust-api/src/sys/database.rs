use super::{config, Error, Result};
use crate::types::DatabasePool;
use sqlx::postgres::{PgPool, PgPoolOptions};

async fn create_pool() -> Result<DatabasePool> {
    let db_config = config().database();

    PgPoolOptions::new()
        .min_connections(db_config.max_connections())
        .max_connections(db_config.min_connections())
        .connect(db_config.url())
        .await
        .map_err(Error::DatabasePoolCreationFailure)
}

#[derive(Clone, Debug)]
pub struct DatabaseManager {
    connection: DatabasePool,
}

impl DatabaseManager {
    pub async fn new() -> Result<Self> {
        let connection = create_pool().await?;

        Ok(Self { connection })
    }

    pub fn connection(&self) -> &DatabasePool {
        &self.connection
    }
}
