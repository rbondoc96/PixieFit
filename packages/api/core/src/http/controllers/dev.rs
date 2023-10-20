use super::{Controller, Result};
use crate::sys::DatabaseManager;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Json};
use axum::routing::{get, Router};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

pub struct DevController;

impl Controller for DevController {
    type State = DatabaseManager;

    fn router(_state: Self::State) -> Router {
        Router::new()
            .route("/ping", get(Self::pong))
            .route("/hello", get(Self::hello))
            .route("/hello/:path", get(Self::hello_path))
    }
}

impl DevController {
    pub async fn pong() -> Result<Json<Value>> {
        let body = Json(json!({
            "success": true,
            "message": "pong",
        }));

        Ok(body)
    }

    pub async fn hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
        let name = params.name.as_deref().unwrap_or("World");

        Html(format!("Hello, {}!", name))
    }

    pub async fn hello_path(Path(path): Path<String>) -> impl IntoResponse {
        Html(format!("Hello, {}!", path))
    }
}
