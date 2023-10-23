use crate::Error;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone, Debug)]
pub struct DatabaseManager {
    connection: PgPool,
}

impl DatabaseManager {
    pub fn new() -> DatabaseManagerBuilder<NoUrl> {
        DatabaseManagerBuilder::new()
    }

    pub fn connection(&self) -> &PgPool {
        &self.connection
    }
}

// region DatabaseManagerBuilder states

#[derive(Default)]
pub struct NoUrl;
#[derive(Default)]
pub struct Url(String);

// endregion

pub struct DatabaseManagerBuilder<U> {
    max_connections: u32,
    min_connections: u32,
    url: U,
}

impl<U> Default for DatabaseManagerBuilder<U>
where
    U: Default,
{
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 2,
            url: U::default(),
        }
    }
}

impl DatabaseManagerBuilder<NoUrl> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<U> DatabaseManagerBuilder<U> {
    pub fn url(self, url: impl Into<String>) -> DatabaseManagerBuilder<Url> {
        DatabaseManagerBuilder {
            max_connections: self.max_connections,
            min_connections: self.min_connections,
            url: Url(url.into()),
        }
    }

    pub fn max_connections(mut self, max_connections: u32) -> Self {
        self.max_connections = max_connections;
        self
    }

    pub fn min_connections(mut self, min_connections: u32) -> Self {
        self.min_connections = min_connections;
        self
    }
}

impl DatabaseManagerBuilder<Url> {
    pub async fn build(self) -> Result<DatabaseManager, Error> {
        let connection = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .connect(&self.url.0)
            .await
            .map_err(Error::DatabasePoolCreationFailure)?;

        Ok(DatabaseManager { connection })
    }
}
