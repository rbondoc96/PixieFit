use crate::manager::DatabaseManager;
use crate::query::{SqlxAction, SqlxBindable, SqlxQuery};
use async_trait::async_trait;
use sqlx::{
    Encode, FromRow, Type,
    postgres::{PgPool, PgRow, Postgres},
};

#[async_trait]
pub trait Model
where
    Self: for<'r> FromRow<'r, PgRow> + Send + Sized + Sync + Unpin + std::fmt::Debug
{
    const MODEL_NAME: &'static str;
    const TABLE_NAME: &'static str;

    const PRIMARY_KEY: &'static str = "id";
    type PrimaryKey: SqlxBindable + Send + Sync + std::fmt::Display + Clone;
    fn pk(&self) -> Self::PrimaryKey;

    const ROUTE_KEY: &'static str = "id";
    type RouteKey: SqlxBindable + Send + Sync + std::fmt::Display + Clone;
    fn rk(&self) ->  Self::RouteKey;

    async fn all(database: &DatabaseManager) -> Result<Vec<Self>, sqlx::Error> {
        Self::query()
            .select(&["*"])
            .all::<&PgPool, Self>(database.connection())
            .await
    }

    async fn count(database: &DatabaseManager) -> Result<i64, sqlx::Error> {
        sqlx::query_as::<_, (i64,)>(format!(
            "SELECT count(*) FROM {}",
            Self::TABLE_NAME
        ).as_str())
        .fetch_one(database.connection())
        .await
        .map(|result| result.0)
    }

    async fn find<K>(key: &'static str, value: K, database: &DatabaseManager) -> Result<Self, sqlx::Error>
    where
        K: SqlxBindable + Send + Sync + std::fmt::Display + Clone,
    {
        let model = Self::query()
            .select(&["*"])
            .and_where(key, "=", value)
            .one::<&PgPool, Self>(database.connection())
            .await?;

        Ok(model)
    }

    async fn find_by_pk(pk: Self::PrimaryKey, database: &DatabaseManager) -> Result<Self, sqlx::Error> {
        Self::find(Self::PRIMARY_KEY, pk, database).await
    }

    async fn find_by_route_key(key: Self::RouteKey, database: &DatabaseManager) -> Result<Self, sqlx::Error> {
        Self::find(Self::ROUTE_KEY, key, database).await
    }

    async fn has<TKey>(key: &'static str, value: TKey, database: &DatabaseManager) -> Result<bool, sqlx::Error>
    where
        TKey: SqlxBindable + Send + Sync + std::fmt::Display + Clone,
    {
        Self::query()
            .select(&["*"])
            .and_where(key, "=", value)
            .optional::<&PgPool, Self>(database.connection())
            .await
            .map(|result| result.is_some())
    }

    fn query<'q>() -> SqlxQuery {
        SqlxQuery::table(Self::TABLE_NAME)
    }
}
