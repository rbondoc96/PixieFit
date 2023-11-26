use crate::http::router;
use crate::http::response::{ApiErrorContext, ApiErrorResponse, ApiSuccessResponse};
use serde::Serialize;

pub use axum::http::StatusCode;
pub use axum_test::{TestResponse, TestServer};
pub use database::{DatabaseManager, Model};
pub use serde_json::{json, Value};
pub use sqlx::postgres::PgPool;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct ApiResponse;

impl ApiResponse {
    pub fn error(
        name: impl ToString,
        message: impl ToString,
        errors: Option<std::collections::HashMap<String, Vec<String>>>
    ) -> Value {
        json!(ApiErrorResponse {
            success: false,
            error: ApiErrorContext {
                name: name.to_string(),
                message: message.to_string(),
                errors: errors,
            }
        })
    }

    pub fn success<T: Serialize>(data: Option<T>) -> Value {
        json!(ApiSuccessResponse {
            success: true,
            data: data
        })
    }
}

pub fn assert_some_eq<T>(expected: impl Into<T>, actual: Option<T>)
where
    T: PartialEq + std::fmt::Debug,
{
    assert!(actual.is_some());
    assert_eq!(expected.into(), actual.unwrap());
}

pub async fn init(pool: PgPool) -> (TestServer, DatabaseManager) {
    let database = DatabaseManager::from_pool(pool);
    let router = router(database.clone()).await;

    (TestServer::new(router).unwrap(), database)
}
