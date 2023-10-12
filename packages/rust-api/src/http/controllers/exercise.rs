use super::Controller;
use crate::{
    enums::{ExerciseForce, ExerciseMechanic, ExerciseMuscleTarget, ExerciseType, Measurement},
    http::resources::{ModelResource, ExerciseResource},
    http::response::JsonResponse,
    models::{CreateExerciseData, CreateExerciseMuscleMapData, Model, Exercise},
    sys::DatabaseManager,
    Error, Result,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, Router},
};
use serde::Deserialize;
use crate::models::ExerciseMuscleMap;

#[derive(Deserialize)]
pub struct MuscleData {
    muscle_id: i64,
    target: ExerciseMuscleTarget,
}

#[derive(Deserialize)]
pub struct CreateExercisePayload {
    #[serde(rename = "type")]
    exercise_type: ExerciseType,
    target_muscle_group_id: i32,
    name: String,
    name_alternative: Option<String>,
    description: Option<String>,
    equipment: String,
    mechanic: ExerciseMechanic,
    force: ExerciseForce,
    measurement: Measurement,
    muscles: Vec<MuscleData>
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
        let exercise = Exercise::find_by_ulid(ulid, &database).await?;

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