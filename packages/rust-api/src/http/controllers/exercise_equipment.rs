use super::Controller;
use crate::{
    http::resources::{ModelResource, ExerciseEquipmentResource},
    http::response::JsonResponse,
    models::{Model, ExerciseEquipment},
    sys::DatabaseManager,
    Error, Result,
};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, Router},
};

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