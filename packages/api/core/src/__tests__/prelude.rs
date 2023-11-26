use crate::http::router;
use crate::http::response::{ApiErrorContext, ApiErrorResponse, ApiSuccessResponse};
use crate::types::DynError;
use serde::Serialize;

pub use axum::http::StatusCode;
pub use axum_test::{TestResponse, TestServer, TestServerConfig};
pub use database::{DatabaseManager, Model};
pub use serde_json::{json, Value};
pub use sqlx::postgres::PgPool;

pub type Result<T> = core::result::Result<T, Box<DynError>>;

pub fn assert_some_and_eq<T>(expected: impl Into<T>, actual: Option<T>)
where
    T: PartialEq + std::fmt::Debug,
{
    assert!(actual.is_some());
    assert_eq!(expected.into(), actual.unwrap());
}

pub async fn init(pool: PgPool) -> (TestServer, DatabaseManager) {
    let database = DatabaseManager::from_pool(pool);
    let router = router(database.clone()).await;

    let config = TestServerConfig::builder()
        .save_cookies()
        .default_content_type("application/json")
        .build();

    (TestServer::new_with_config(router, config).unwrap(), database)
}
