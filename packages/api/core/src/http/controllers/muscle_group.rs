use super::{Controller, Result};
use crate::prelude::*;
use crate::http::resources::{ModelResource, MuscleGroupResource};
use crate::http::response::JsonResponse;
use crate::models::MuscleGroup;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use axum::routing::{get, post, Router};
use database::{DatabaseManager, Model};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateMuscleGroupPayload {
    name: String,
    image_source: Option<String>,
}

pub struct MuscleGroupController;

impl Controller for MuscleGroupController {
    fn router(state: DatabaseManager) -> Router {
        Router::new()
            .route("/", get(Self::list).post(Self::create))
            .route("/:id", get(Self::read))
            .with_state(state)
    }
}

impl MuscleGroupController {
    pub async fn create(
        State(database): State<DatabaseManager>,
        Json(payload): Json<CreateMuscleGroupPayload>,
    ) -> Result<JsonResponse> {
        let mut group_builder = MuscleGroup::new()
            .name(payload.name);

        if let Some(source) = payload.image_source {
            group_builder = group_builder.image_source(source);
        }

        let group = group_builder.create(&database).await?;

        Ok(JsonResponse::created()
            .with_data(MuscleGroupResource::default(group, &database).await?)
        )
    }

    pub async fn read(
        Path(id): Path<i16>,
        State(database): State<DatabaseManager>,
    ) -> Result<JsonResponse> {
        let group = MuscleGroup::find_by_pk(id, &database).await?;

        Ok(JsonResponse::ok()
            .with_data(MuscleGroupResource::default(group, &database).await?)
        )
    }

    pub async fn list(
        State(database): State<DatabaseManager>,
    ) -> Result<JsonResponse> {
        let groups = MuscleGroup::query()
            .select(&["*"])
            .order_by("name", true)
            .all(database.connection())
            .await?;

        Ok(JsonResponse::ok()
            .with_data(MuscleGroupResource::list(groups, &database).await?)
        )
    }
}