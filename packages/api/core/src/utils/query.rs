use crate::sys::DatabaseManager;
use sqlx::{
    FromRow,
    postgres::PgRow,
    query_builder::QueryBuilder,
};

pub struct DatabaseQuery<TResult> {
    table: &'static str,
    result: Option<TResult>,
}

impl<TResult> DatabaseQuery<TResult>
where
    TResult: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    pub fn table(table: &'static str) -> Self {
        todo!()
    }

    pub fn select() -> Self {
        todo!()
    }

    pub fn insert() -> Self {
        todo!()
    }

    pub fn update() -> Self {
        todo!()
    }

    pub fn delete() -> Self {
        todo!()
    }

    pub fn and_where() -> Self {
        todo!()
    }

    pub fn execute(database: &DatabaseManager) -> Self {
        todo!()
    }
}

