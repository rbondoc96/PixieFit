use super::{Controller, Result};
use crate::prelude::*;
use crate::{
    http::extractors::Pagination,
    http::resources::{ModelResource, MuscleResource},
    http::response::JsonResponse,
    models::{CreateMuscleData, Model, Muscle},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, Router},
};
use database::DatabaseManager;

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
    ) -> Result<JsonResponse<Vec<MuscleResource>>> {
        let muscles = Muscle::all(&database).await?;

        println!("{:?}", pagination);
        println!("({:?}, {:?})", pagination.page(), pagination.per_page());

        Ok(JsonResponse::success(
            Some(MuscleResource::list(muscles).await),
            StatusCode::OK,
        ))
    }

    pub async fn create(
        State(database): State<DatabaseManager>,
        Json(payload): Json<CreateMuscleData>,
    ) -> Result<JsonResponse<MuscleResource>> {
        let muscle = Muscle::create(payload, &database).await?;

        Ok(JsonResponse::success(
            Some(MuscleResource::default(muscle).await),
            StatusCode::CREATED,
        ))
    }

    pub async fn read(
        State(database): State<DatabaseManager>,
        Path(ulid): Path<String>,
    ) -> Result<JsonResponse<MuscleResource>> {
        let muscle = Muscle::find_by_key(ulid, &database).await?;

        Ok(JsonResponse::success(
            Some(MuscleResource::default(muscle).await),
            StatusCode::OK,
        ))
    }
}
