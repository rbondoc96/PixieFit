use super::{Controller, Result};
use crate::prelude::*;
use crate::{
    http::resources::{ModelResource, ExerciseEquipmentResource},
    http::response::JsonResponse,
    models::{Model, ExerciseEquipment},
};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, Router},
};
use database::DatabaseManager;

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
    pub async fn list(
        State(database): State<DatabaseManager>,
    ) -> Result<JsonResponse<Vec<ExerciseEquipmentResource>>> {
        let groups = ExerciseEquipment::all(&database).await?;

        Ok(JsonResponse::success(
            Some(ExerciseEquipmentResource::list(groups).await),
            StatusCode::OK,
        ))
    }
}