use super::{Error, Model, Result};
use crate::sys::DatabaseManager;
use sqlx::postgres::PgRow;
use sqlx::{Any, Encode, FromRow, Postgres, Type};

pub async fn all<TModel>(database: &DatabaseManager) -> Result<Vec<TModel>>
where
    TModel: Model,
{
    let connection = database.connection();

    let data = sqlx::query_as::<_, TModel::Attributes>(
        format!("SELECT * FROM {}", TModel::TABLE_NAME).as_str(),
    )
    .fetch_all(connection)
    .await?;

    Ok(data
        .into_iter()
        .map(|data| TModel::from_database(data, database))
        .collect())
}

pub async fn find_by_id<TModel>(id: i64, database: &DatabaseManager) -> Result<TModel>
where
    TModel: Model,
{
    find::<TModel, i64>("id", id, database).await
}

pub async fn exists<TModel, TKey>(
    key: &'static str,
    value: TKey,
    database: &DatabaseManager,
) -> Result<bool>
where
    TKey: Type<Postgres> + for<'q> Encode<'q, Postgres> + Send + std::fmt::Display + Clone,
    TModel: Model,
{
    let connection = database.connection();
    let value_clone = value.clone();

    let record = sqlx::query_as::<_, TModel::Attributes>(
        format!("SELECT * FROM {} WHERE {} = $1", TModel::TABLE_NAME, key).as_str(),
    )
        .bind(value)
        .fetch_optional(connection)
        .await?;

    Ok(record.is_some())
}

pub async fn find<TModel, TKey>(
    key: &'static str,
    value: TKey,
    database: &DatabaseManager,
) -> Result<TModel>
where
    TKey: Type<Postgres> + for<'q> Encode<'q, Postgres> + Send + std::fmt::Display + Clone,
    TModel: Model,
{
    let connection = database.connection();
    let value_clone = value.clone();

    let record = sqlx::query_as::<_, TModel::Attributes>(
        format!("SELECT * FROM {} WHERE {} = $1", TModel::TABLE_NAME, key).as_str(),
    )
    .bind(value)
    .fetch_optional(connection)
    .await?
    .ok_or(Error::ModelNotFound {
        model: TModel::TABLE_NAME,
        search_key: value_clone.to_string(),
        search_value: key.to_string(),
    })?;

    Ok(TModel::from_database(record, database))
}
