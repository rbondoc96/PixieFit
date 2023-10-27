use super::{Controller, Result};
use crate::prelude::*;
use crate::http::Context;
use crate::http::extractors::Pagination;
use crate::http::resources::{ModelResource, MuscleResource};
use crate::http::response::JsonResponse;
use crate::models::Muscle;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use axum::routing::{get, post, Router};
use database::{DatabaseManager, Model};
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
    type State = DatabaseManager;

    fn router(state: Self::State) -> Router {
        Router::new()
            .route("/", get(Self::list).post(Self::create))
            .route("/:id", get(Self::read))
            .with_state(state)
    }
}

impl MuscleController {
    pub async fn list(
        pagination: Pagination,
        State(database): State<DatabaseManager>,
    ) -> Result<JsonResponse> {
        let muscles = Muscle::all(&database).await?;

        Ok(JsonResponse::success(
            Some(MuscleResource::list(muscles, &database).await),
            StatusCode::OK,
        ))
    }

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

        Ok(JsonResponse::success(
            Some(MuscleResource::default(muscle, &database).await),
            StatusCode::CREATED,
        ))
    }

    pub async fn read(
        context: Context,
        State(database): State<DatabaseManager>,
        Path(ulid): Path<String>,
    ) -> Result<JsonResponse> {
        let muscle = Muscle::find_by_route_key(ulid, &database).await?;

        eprintln!("{:?}", context);

        Ok(JsonResponse::success(
            Some(MuscleResource::default(muscle, &database).await),
            StatusCode::OK,
        ))
    }
}
