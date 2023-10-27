use super::bind::SqlxBindable;
use super::select::SelectAction;
use async_trait::async_trait;
use sqlx::{
    Executor, FromRow,
    postgres::{PgRow, Postgres},
};

pub struct SqlxQuery {
    table: &'static str,
}

impl SqlxQuery {
    pub fn table(table: &'static str) -> Self {
        Self {
            table,
        }
    }

    pub fn select(self, columns: &'static[&'static str]) -> SelectAction {
        SelectAction::new(self.table, columns)
    }
}

#[async_trait]
pub trait SqlxAction<'a> {
    fn sql(&self) -> String;

    // Making the items be references to a Box rather than just a box
    // prevents the value from being dropped when putting together the SQL string
    // Not sure why.
    fn binds(&self) -> Vec<&Box<dyn SqlxBindable + 'a + Send + Sync>>;

    async fn one<D, R>(&'a self, db: D) -> Result<R, sqlx::Error>
    where
        D: for<'e> Executor<'e, Database = Postgres>,
        R: for<'r> FromRow<'r, PgRow> + Send + Unpin;

    async fn optional<D, R>(&'a self, db: D) -> Result<Option<R>, sqlx::Error>
    where
        D: for<'e> Executor<'e, Database = Postgres>,
        R: for<'r> FromRow<'r, PgRow> + Send + Unpin;

    async fn all<D, R>(&'a self, db: D) -> Result<Vec<R>, sqlx::Error>
    where
        D: for<'e> Executor<'e, Database = Postgres>,
        R: for<'r> FromRow<'r, PgRow> + Send + Unpin;
}
