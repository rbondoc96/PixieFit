use super::{Controller, Result};
use crate::prelude::*;
use crate::http::resources::{ModelResource, ExerciseEquipmentResource};
use crate::http::response::JsonResponse;
use crate::models::ExerciseEquipment;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, Router};
use database::{DatabaseManager, Model};

pub struct ExerciseEquipmentController;

impl Controller for ExerciseEquipmentController {
    type State = DatabaseManager;

    fn router(state: Self::State) -> Router {
        Router::new()
            .route("/", get(Self::list))
            .with_state(state)
    }
}

impl ExerciseEquipmentController {
    pub async fn list(State(database): State<DatabaseManager>) -> Result<JsonResponse> {
        let groups = ExerciseEquipment::all(&database).await?;

        Ok(JsonResponse::success(
            Some(ExerciseEquipmentResource::list(groups, &database).await),
            StatusCode::OK,
        ))
    }
}