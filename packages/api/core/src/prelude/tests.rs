use super::DynError;

pub use crate::mocks::{MockResponse, MockServer, MockUser};
pub use axum::http::StatusCode;
pub use axum_test::{TestResponse, TestServer, TestServerConfig};
pub use database::{DatabaseManager, HasRouteKey, Model};
pub use serde_json::{json, Value};
pub use sqlx::postgres::PgPool;

pub type Result<T> = core::result::Result<T, Box<DynError>>;

pub const NULL: Option<()> = None;

pub fn assert_ok<T, E>(result: &core::result::Result<T, E>) {
    assert!(result.is_ok());
}

pub fn assert_ok_eq<T, E>(expected: impl Into<T>, actual: core::result::Result<T, E>)
where
    T: PartialEq + std::fmt::Debug,
    E: std::fmt::Debug,
{
    assert!(actual.is_ok());
    assert_eq!(expected.into(), actual.unwrap());
}

pub fn assert_some<T>(result: Option<T>) {
    assert!(result.is_some());
}

pub fn assert_some_eq<T: PartialEq + std::fmt::Debug>(expected: impl Into<T>, actual: Option<T>) {
    assert!(actual.is_some());
    assert_eq!(expected.into(), actual.unwrap());
}
