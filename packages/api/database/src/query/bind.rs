use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use sqlx::{
    Encode, Type,
    postgres::{PgArguments, Postgres},
    query::Query,
};

pub trait SqlxBindable {
    fn bind_to_query<'q>(
        &'q self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> Query<'q, Postgres, PgArguments>;
}

#[macro_export]
macro_rules! impl_bindable {
    ($($t:ty), *) => {
        $(impl $crate::SqlxBindable for $t {
            fn bind_to_query<'q>(
                &'q self,
                query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
            ) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
                let query = query.bind(self.clone());
                query
            }
        }

        impl $crate::SqlxBindable for &$t {
            fn bind_to_query<'q>(
                &'q self,
                query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
            ) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
                let query = query.bind(<$t>::clone(self));
                query
            }
        })*
    }
}

// Must be re-exported AFTER the macro is defined
pub(crate) use impl_bindable;

impl_bindable!(bool);
impl_bindable!(i16, i32, i64);
impl_bindable!(f32, f64);
impl_bindable!(DateTime<Utc>, NaiveDate, NaiveDateTime, NaiveTime);

impl<T> SqlxBindable for Option<T>
where
    T: for<'r> Encode<'r, Postgres> + Type<Postgres> + Clone + Send + SqlxBindable,
{
    fn bind_to_query<'q>(
        &'q self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> Query<'q, Postgres, PgArguments> {
        let query = query.bind(self.clone());
        query
    }
}

impl SqlxBindable for String {
    fn bind_to_query<'q>(
        &'q self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> Query<'q, Postgres, PgArguments> {
        let query = query.bind(self.to_string());
        query
    }
}

impl SqlxBindable for &String {
    fn bind_to_query<'q>(
        &'q self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> Query<'q, Postgres, PgArguments> {
        let query = query.bind(self.to_string());
        query
    }
}

impl SqlxBindable for str {
    fn bind_to_query<'q>(
        &'q self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> Query<'q, Postgres, PgArguments> {
        let query = query.bind(self.to_string());
        query
    }
}

impl SqlxBindable for &str {
    fn bind_to_query<'q>(
        &'q self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> Query<'q, Postgres, PgArguments> {
        let query = query.bind(self.to_string());
        query
    }
}
