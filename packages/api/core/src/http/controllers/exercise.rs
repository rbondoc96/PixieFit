use super::{Controller, Result};
use crate::prelude::*;
use crate::{
    enums::{ExerciseForce, ExerciseMechanic, ExerciseMuscleTarget, ExerciseType, Measurement},
    http::resources::{ModelResource, ExerciseResource},
    http::response::JsonResponse,
    models::{CreateExerciseData, CreateExerciseMuscleMapData, Model, Exercise, ExerciseMuscleMap},
};
use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post, Router},
};
use database::DatabaseManager;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MuscleData {
    muscle_id: i64,
    target: ExerciseMuscleTarget,
}

#[derive(Deserialize)]
pub struct CreateExercisePayload {
    #[serde(rename = "type")]
    exercise_type: ExerciseType,
    target_muscle_group_id: Option<i32>,
    name: String,
    name_alternative: Option<String>,
    description: Option<String>,
    equipment: Option<String>,
    mechanic: Option<ExerciseMechanic>,
    force: Option<ExerciseForce>,
    measurement: Option<Measurement>,
    muscles: Vec<MuscleData>
}

#[derive(Deserialize)]
pub struct ListExerciseParams {
    muscle: Option<i64>,
    muscle_group: Option<i32>,
}

pub struct ExerciseController;

impl Controller for ExerciseController {
    type State = DatabaseManager;

    fn router(state: Self::State) -> Router {
        Router::new()
            .route("/", get(Self::list).post(Self::create))
            .route("/:ulid", get(Self::read))
            .with_state(state)
    }
}

impl ExerciseController {
    pub async fn list(
        Query(params): Query<ListExerciseParams>,
        State(database): State<DatabaseManager>,
    ) -> Result<JsonResponse<Vec<ExerciseResource>>> {
        let exercises = Exercise::all(&database).await?;

        Ok(JsonResponse::success(
            Some(ExerciseResource::list(exercises).await),
            StatusCode::OK,
        ))
    }

    pub async fn read(
        State(database): State<DatabaseManager>,
        Path(ulid): Path<String>,
    ) -> Result<JsonResponse<ExerciseResource>> {
        let exercise = Exercise::find_by_key(ulid, &database).await?;

        Ok(JsonResponse::success(
            Some(ExerciseResource::default(exercise).await),
            StatusCode::OK,
        ))
    }

    pub async fn create(
        State(database): State<DatabaseManager>,
        Json(payload): Json<CreateExercisePayload>,
    ) -> Result<JsonResponse<ExerciseResource>> {
        let exercise = Exercise::create(
            CreateExerciseData {
                external_id: None,
                exercise_type: payload.exercise_type,
                target_muscle_group_id: payload.target_muscle_group_id,
                name: payload.name,
                name_alternative: payload.name_alternative,
                description: payload.description,
                equipment: payload.equipment,
                mechanic: payload.mechanic,
                force: payload.force,
                measurement: payload.measurement,
            },
            &database,
        ).await?;

        for muscle in payload.muscles {
            ExerciseMuscleMap::create(
                CreateExerciseMuscleMapData {
                    exercise_id: exercise.id(),
                    muscle_id: muscle.muscle_id,
                    target: muscle.target,
                },
                &database,
            ).await?;
        }

        Ok(JsonResponse::success(
            Some(ExerciseResource::default(exercise).await),
            StatusCode::CREATED,
        ))
    }
}