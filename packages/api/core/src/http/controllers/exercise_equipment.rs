use super::{Controller, Result};
use crate::prelude::*;
use crate::http::resources::{ModelResource, ExerciseEquipmentResource};
use crate::http::response::JsonResponse;
use crate::models::ExerciseEquipment;
use axum::extract::{Path, State};
use axum::response::Json;
use axum::routing::{get, Router};
use database::{DatabaseManager, Model};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateExerciseEquipmentPayload {
    name: String,
}

pub struct ExerciseEquipmentController;

impl Controller for ExerciseEquipmentController {
    fn router(state: DatabaseManager) -> Router {
        Router::new()
            .route("/", get(Self::list).post(Self::create))
            .route("/:id", get(Self::read))
            .with_state(state)
    }
}

impl ExerciseEquipmentController {
    pub async fn create(
        State(database): State<DatabaseManager>,
        Json(payload): Json<CreateExerciseEquipmentPayload>,
    ) -> Result<JsonResponse> {
        let equipment = ExerciseEquipment::new()
            .name(payload.name)
            .create(&database)
            .await?;

        Ok(JsonResponse::created()
            .with_data(ExerciseEquipmentResource::default(equipment, &database).await?)
        )
    }

    pub async fn read(
        State(database): State<DatabaseManager>,
        Path(id): Path<i16>,
    ) -> Result<JsonResponse> {
        let equipment = ExerciseEquipment::find_by_route_key(id, &database).await?;

        Ok(JsonResponse::ok()
            .with_data(ExerciseEquipmentResource::default(equipment, &database).await?)
        )
    }

    pub async fn list(State(database): State<DatabaseManager>) -> Result<JsonResponse> {
        let groups = ExerciseEquipment::all(&database).await?;

        Ok(JsonResponse::ok()
            .with_data(ExerciseEquipmentResource::list(groups, &database).await?)
        )
    }
}