use super::{Controller, Result};
use crate::prelude::*;
use crate::enums::{ExerciseForce, ExerciseMechanic, ExerciseMuscleTarget, ExerciseType, Measurement};
use crate::http::extractors::Pagination;
use crate::http::resources::{ModelResource, ExerciseResource};
use crate::http::response::JsonResponse;
use crate::models::{Exercise, ExerciseMuscleMap};
use axum::extract::{Path, State, Query};
use axum::response::Json;
use axum::routing::{get, post, Router};
use database::{DatabaseManager, Model};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MuscleData {
    muscle_id: i16,
    target: ExerciseMuscleTarget,
}

#[derive(Deserialize)]
pub struct CreateExercisePayload {
    #[serde(rename = "type")]
    exercise_type: ExerciseType,
    target_muscle_group_id: Option<i16>,
    name: String,
    name_alternative: Option<String>,
    description: Option<String>,
    equipment_id: Option<i16>,
    mechanic: Option<ExerciseMechanic>,
    force: Option<ExerciseForce>,
    measurement: Option<Measurement>,
    muscles: Vec<MuscleData>
}

#[derive(Debug, Deserialize)]
pub struct ListExerciseParams {
    muscle: Option<i16>,
    muscle_group: Option<i16>,
}

pub struct ExerciseController;

impl Controller for ExerciseController {
    fn router(state: DatabaseManager) -> Router {
        Router::new()
            .route("/", get(Self::list).post(Self::create))
            .route("/:ulid", get(Self::read))
            .with_state(state)
    }
}

impl ExerciseController {
    pub async fn create(
        State(database): State<DatabaseManager>,
        Json(payload): Json<CreateExercisePayload>,
    ) -> Result<JsonResponse> {
        let exercise = Exercise::new()
            .exercise_type(payload.exercise_type)
            .target_muscle_group_id(payload.target_muscle_group_id)
            .equipment_id(payload.equipment_id)
            .name(payload.name)
            .name_alternative(payload.name_alternative)
            .description(payload.description)
            .mechanic(payload.mechanic)
            .force(payload.force)
            .measurement(payload.measurement)
            .create(&database)
            .await?;

        for muscle in payload.muscles {
            ExerciseMuscleMap::new()
                .exercise_id(exercise.id)
                .muscle_id(muscle.muscle_id)
                .target(muscle.target)
                .create(&database)
                .await?;
        }

        Ok(JsonResponse::created()
            .with_data(ExerciseResource::default(exercise, &database).await?)
        )
    }

    pub async fn read(
        State(database): State<DatabaseManager>,
        Path(ulid): Path<String>,
    ) -> Result<JsonResponse> {
        let exercise = Exercise::find_by_route_key(ulid, &database).await?;

        Ok(JsonResponse::ok()
            .with_data(ExerciseResource::default(exercise, &database).await?)
        )
    }

    pub async fn list(
        pagination: Pagination,
        Query(params): Query<ListExerciseParams>,
        State(database): State<DatabaseManager>,
    ) -> Result<JsonResponse> {
        let mut query = Exercise::query()
            .select(&["*"])
            .limit(pagination.limit())
            .offset(pagination.offset());

        if let Some(group) = params.muscle_group {
            query = query.and_where("target_muscle_group_id", "=", group);
        }

        let exercises = query
            .all(database.connection())
            .await?;

        Ok(JsonResponse::ok()
            .with_data(ExerciseResource::list(exercises, &database).await?)
        )
    }
}