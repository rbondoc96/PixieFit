pub use crate::types::*;
pub use crate::utils::__;

#[cfg(test)]
mod test_utils {
    pub use database::{DatabaseManager, Model};
    pub use sqlx::postgres::PgPool;

    pub type Result<T> = core::result::Result<T, crate::http::Error>;

    pub fn assert_some_eq<T>(expected: impl Into<T>, actual: Option<T>)
    where
        T: PartialEq + std::fmt::Debug,
    {
        assert!(actual.is_some());
        assert_eq!(expected.into(), actual.unwrap());
    }
}

#[cfg(test)]
pub(crate) use test_utils::*;
