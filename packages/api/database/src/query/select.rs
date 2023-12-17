use super::SqlxAction;
use super::bind::SqlxBindable;
use super::clause::{OrderClause, WhereClause};
use async_trait::async_trait;
use sqlx::{Execute, Executor, FromRow, postgres::{PgArguments, PgRow, Postgres}};

pub struct SelectAction<'a> {
    table: &'static str,
    columns: &'a[&'static str],
    wheres: Vec<WhereClause<'a>>,
    orders: Vec<OrderClause>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl<'a> SelectAction<'a> {
    pub fn new(table: &'static str, columns: &'a[&'static str]) -> Self {
        Self {
            table,
            columns,
            wheres: Vec::new(),
            orders: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    pub fn and_where<V>(mut self, name: &'static str, operator: &'static str, value: V) -> Self
    where
        V: 'a + Send + SqlxBindable + Sync
    {
        self.wheres.push(WhereClause {
            column: name,
            operator,
            value: Box::new(value),
        });
        self
    }

    pub fn order_by(mut self, name: &'static str, asc: bool) -> Self {
        match asc {
            true => self.orders.push(OrderClause::Ascending(name)),
            false => self.orders.push(OrderClause::Descending(name)),
        }
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub async fn one<D, R>(&'a self, database: D) -> Result<R, sqlx::Error>
    where
        D: for<'e> Executor<'e, Database = Postgres>,
        R: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        let sql = self.sql();
        let values = self.binds();

        let mut query = sqlx::query(&sql);
        for value in values.into_iter() {
            query = value.bind_to_query(query);
        }

        let query = sqlx::query_as_with::<Postgres, R, PgArguments>(&sql, query.take_arguments().unwrap());

        let res = query.fetch_one(database).await?;
        Ok(res)
    }

    pub async fn optional<D, R>(&'a self, database: D) -> Result<Option<R>, sqlx::Error>
    where
        D: for<'e> Executor<'e, Database = Postgres>,
        R: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        let sql = self.sql();
        let values = self.binds();

        let mut query = sqlx::query(&sql);
        for value in values.into_iter() {
            query = value.bind_to_query(query);
        }

        let query = sqlx::query_as_with::<Postgres, R, PgArguments>(&sql, query.take_arguments().unwrap());

        let res = query.fetch_optional(database).await?;
        Ok(res)
    }

    pub async fn all<D, R>(&'a self, database: D) -> Result<Vec<R>, sqlx::Error>
    where
        D: for<'e> Executor<'e, Database = Postgres>,
        R: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        let sql = self.sql();
        let values = self.binds();

        let mut query = sqlx::query(&sql);
        for value in values.into_iter() {
            query = value.bind_to_query(query);
        }

        let query = sqlx::query_as_with::<Postgres, R, PgArguments>(&sql, query.take_arguments().unwrap());

        let res = query.fetch_all(database).await?;
        Ok(res)
    }
}

#[async_trait]
impl<'a> SqlxAction<'a> for SelectAction<'a> {
    fn sql(&self) -> String {
        let mut tokens = vec!["SELECT".to_string()];

        if !self.columns.is_empty() {
            let columns = self.columns.join(", ");
            tokens.push(columns);
        } else {
            tokens.push("*".to_string());
        }

        let table_token = format!("FROM {}", self.table);
        tokens.push(table_token);

        if !self.wheres.is_empty() {
            tokens.push("WHERE".to_string());

            let joined_wheres = self.wheres.iter().enumerate()
                .map(|(index, clause)| {
                    format!("{} {} ${}", clause.column, clause.operator, index + 1)
                })
                .collect::<Vec<String>>()
                .join(" AND ");

            tokens.push(joined_wheres);
        }

        if !self.orders.is_empty() {
            tokens.push("ORDER BY".to_string());

            let joined_orders = self.orders.iter()
                .map(|clause| clause.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            tokens.push(joined_orders);
        }

        if let Some(limit) = &self.limit {
            tokens.push(format!("LIMIT {}", limit));
        }

        if let Some(offset) = &self.offset {
            tokens.push(format!("OFFSET {}", offset));
        }

        let res = tokens.join(" ");

        eprintln!("{}", res);

        res
    }

    fn binds(&self) -> Vec<&Box<dyn SqlxBindable + 'a + Send + Sync>> {
        self.wheres.iter()
            .map(|clause| &clause.value)
            .collect::<Vec<&Box<dyn SqlxBindable + 'a + Send + Sync>>>()
    }

    async fn one<D, R>(&'a self, database: D) -> Result<R, sqlx::Error>
    where
        D: for<'e> Executor<'e, Database = Postgres>,
        R: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        Self::one(self, database).await
    }

    async fn optional<D, R>(&'a self, database: D) -> Result<Option<R>, sqlx::Error>
    where
        D: for<'e> Executor<'e, Database = Postgres>,
        R: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        Self::optional(self, database).await
    }

    async fn all<D, R>(&'a self, database: D) -> Result<Vec<R>, sqlx::Error>
    where
        D: for<'e> Executor<'e, Database = Postgres>,
        R: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        Self::all(self, database).await
    }
}
