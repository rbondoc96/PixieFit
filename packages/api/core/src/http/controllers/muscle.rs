use super::{Controller, Result};
use crate::prelude::*;
use crate::http::Context;
use crate::http::extractors::Pagination;
use crate::http::resources::{ModelResource, MuscleResource};
use crate::http::response::JsonResponse;
use crate::models::Muscle;
use axum::extract::{Path, State};
use axum::response::Json;
use axum::routing::{get, post, Router};
use database::{DatabaseManager, HasRouteKey, Model};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateMusclePayload {
    group_id: i16,
    parent_id: Option<i16>,
    name: String,
    simple_name: Option<String>,
    description: Option<String>,
    image_source: Option<String>,
}

pub struct MuscleController;

impl Controller for MuscleController {
    fn router(state: DatabaseManager) -> Router {
        Router::new()
            .route("/", get(Self::list).post(Self::create))
            .route("/:id", get(Self::read))
            .with_state(state)
    }
}

impl MuscleController {
    pub async fn create(
        State(database): State<DatabaseManager>,
        Json(payload): Json<CreateMusclePayload>,
    ) -> Result<JsonResponse> {
        let muscle = Muscle::new()
            .group_id(payload.group_id)
            .parent_id(payload.parent_id)
            .name(payload.name)
            .simple_name(payload.simple_name)
            .description(payload.description)
            .image_source(payload.image_source)
            .create(&database)
            .await?;

        Ok(JsonResponse::created()
            .with_data(MuscleResource::default(muscle, &database).await?)
        )
    }

    pub async fn read(
        context: Context,
        State(database): State<DatabaseManager>,
        Path(ulid): Path<String>,
    ) -> Result<JsonResponse> {
        let muscle = Muscle::find_by_route_key(ulid, &database).await?;

        eprintln!("{:?}", context);

        Ok(JsonResponse::ok()
            .with_data(MuscleResource::default(muscle, &database).await?)
        )
    }

    pub async fn list(
        pagination: Pagination,
        State(database): State<DatabaseManager>,
    ) -> Result<JsonResponse> {
        let muscles = Muscle::query()
            .select(&["*"])
            .order_by("name", true)
            .all(database.connection())
            .await?;

        Ok(JsonResponse::ok()
            .with_data(MuscleResource::list(muscles, &database).await?)
        )
    }
}
