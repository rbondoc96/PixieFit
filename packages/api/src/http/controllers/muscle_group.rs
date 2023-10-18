use super::{Controller, Result};
use crate::prelude::*;
use crate::{
    http::resources::{ModelResource, MuscleGroupResource},
    http::response::JsonResponse,
    models::{Model, MuscleGroup},
    sys::DatabaseManager,
};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, Router},
};

pub struct MuscleGroupController;

impl Controller for MuscleGroupController {
    type State = DatabaseManager;

    fn router(state: Self::State) -> Router {
        Router::new()
            .route("/", get(Self::list))
            .with_state(state)
    }
}

impl MuscleGroupController {
    pub async fn list(
        State(database): State<DatabaseManager>,
    ) -> Result<JsonResponse<Vec<MuscleGroupResource>>> {
        let groups = MuscleGroup::all(&database).await?;

        Ok(JsonResponse::success(
            Some(MuscleGroupResource::list(groups).await),
            StatusCode::OK,
        ))
    }
}