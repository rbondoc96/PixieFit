use super::{Error, Result};
use crate::sys::DatabaseManager;
use async_trait::async_trait;
use sqlx::{
    postgres::PgRow,
    Encode, FromRow, PgPool, Postgres, Type,
};

#[async_trait]
pub trait Model
where
    Self: Send + Sized + Unpin,
{
    const ROUTE_KEY: &'static str = "id";
    const MODEL_NAME: &'static str;
    const TABLE_NAME: &'static str;
    type Attributes: for<'r> FromRow<'r, PgRow> + Unpin + Send;

    fn connection(&self) -> &PgPool;
    fn from_database(attributes: Self::Attributes, database: &DatabaseManager) -> Self;

    async fn all(database: &DatabaseManager) -> Result<Vec<Self>> {
        let connection = database.connection();

        let data = sqlx::query_as::<_, Self::Attributes>(
            format!("SELECT * FROM {}", Self::TABLE_NAME).as_str(),
        )
            .fetch_all(connection)
            .await?;

        Ok(data
            .into_iter()
            .map(|data| Self::from_database(data, database))
            .collect())
    }

    async fn count(database: &DatabaseManager) -> Result<i64> {
        let row_count = sqlx::query_as::<_, (i64,)>(format!("SELECT count(*) FROM {}", Self::TABLE_NAME).as_str())
            .fetch_one(database.connection())
            .await?;

        Ok(row_count.0)
    }

    async fn has<TKey>(
        key: &'static str,
        value: TKey,
        database: &DatabaseManager,
    ) -> Result<bool>
    where
        TKey: Type<Postgres> + for<'q> Encode<'q, Postgres> + Send + std::fmt::Display + Clone,
    {
        let connection = database.connection();
        let value_clone = value.clone();

        let record = sqlx::query_as::<_, Self::Attributes>(
            format!("SELECT * FROM {} WHERE {} = $1", Self::TABLE_NAME, key).as_str(),
        )
            .bind(value)
            .fetch_optional(connection)
            .await?;

        Ok(record.is_some())
    }

    async fn find<TKey>(
        key: &'static str,
        value: TKey,
        database: &DatabaseManager,
    ) -> Result<Self>
    where
        TKey: Type<Postgres> + for<'q> Encode<'q, Postgres> + Send + std::fmt::Display + Clone,
    {
        let connection = database.connection();
        let value_clone = value.clone();

        let record = sqlx::query_as::<_, Self::Attributes>(
            format!("SELECT * FROM {} WHERE {} = $1", Self::TABLE_NAME, key).as_str(),
        )
            .bind(value)
            .fetch_optional(connection)
            .await?
            .ok_or(Error::ModelNotFound {
                model: Self::TABLE_NAME,
                search_key: value_clone.to_string(),
                search_value: key.to_string(),
            })?;

        Ok(Self::from_database(record, database))
    }

    async fn find_by_id(id: i64, database: &DatabaseManager) -> Result<Self> {
        Self::find::<i64>("id", id, database).await
    }

    async fn find_by_key<TKey>(value: TKey, database: &DatabaseManager) -> Result<Self>
    where
        TKey: Type<Postgres> + for<'q> Encode<'q, Postgres> + Send + std::fmt::Display + Clone,
    {
        Self::find(Self::ROUTE_KEY, value, database).await
    }
}