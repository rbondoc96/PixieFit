use super::{Controller, Result};
use axum::response::Json;
use axum::routing::{get, Router};
use database::DatabaseManager;
use serde_json::{json, Value};

pub struct HealthController;

impl Controller for HealthController {
    fn router(_state: DatabaseManager) -> Router {
        Router::new()
            .route("/ping", get(Self::pong))
    }
}

impl HealthController {
    pub async fn pong() -> Result<Json<Value>> {
        let body = Json(json!({
            "success": true,
            "message": "pong",
        }));

        Ok(body)
    }
}
