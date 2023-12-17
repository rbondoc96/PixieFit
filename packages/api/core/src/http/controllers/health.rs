use super::{Controller, Result};
use crate::http::response::JsonResponse;
use axum::routing::{get, Router};
use database::DatabaseManager;
use serde_json::json;

pub struct HealthController;

impl Controller for HealthController {
    fn router(_state: DatabaseManager) -> Router {
        Router::new()
            .route("/ping", get(Self::pong))
    }
}

impl HealthController {
    pub async fn pong() -> Result<JsonResponse> {
        Ok(JsonResponse::ok()
            .with_data(json!({
                "message": "pong"
            }))
        )
    }
}
