use super::{
    ExerciseEquipmentResource,
    MeasurementResource,
    ModelResource,
    MuscleResource,
    MuscleGroupResource,
};
use crate::enums::{ExerciseForce, ExerciseMechanic, ExerciseType};
use crate::models::Exercise;
use async_trait::async_trait;
use serde::Serialize;

#[derive(Serialize)]
pub struct ExerciseResource {
    id: String,
    #[serde(rename = "type")]
    exercise_type: ExerciseType,
    target_muscle_group: MuscleGroupResource,
    name: String,
    name_alternative: Option<String>,
    description: Option<String>,
    equipment: ExerciseEquipmentResource,
    mechanic: ExerciseMechanic,
    force: ExerciseForce,
    measurement: MeasurementResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_muscles: Option<Vec<MuscleResource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_muscles: Option<Vec<MuscleResource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tertiary_muscles: Option<Vec<MuscleResource>>,
}

#[async_trait]
impl ModelResource for ExerciseResource {
    type Model = Exercise;

    async fn default(exercise: Exercise) -> Self {
        let equipment = exercise.equipment().await.unwrap();
        let muscle_group = exercise.target_muscle_group().await.unwrap();
        let primary_muscles = exercise.primary_muscles().await.unwrap();
        let secondary_muscles = exercise.secondary_muscles().await.unwrap();
        let tertiary_muscles = exercise.tertiary_muscles().await.unwrap();

        Self {
            id: exercise.ulid(),
            exercise_type: exercise.exercise_type(),
            target_muscle_group: MuscleGroupResource::simple(muscle_group).await,
            name: exercise.name(),
            name_alternative: exercise.name_alternative(),
            description: exercise.description(),
            equipment: ExerciseEquipmentResource::simple(equipment).await,
            mechanic: exercise.mechanic(),
            force: exercise.force(),
            measurement: MeasurementResource::new(exercise.measurement()),
            primary_muscles: Some(MuscleResource::list(primary_muscles).await),
            secondary_muscles: Some(MuscleResource::list(secondary_muscles).await),
            tertiary_muscles: Some(MuscleResource::list(tertiary_muscles).await),
        }
    }

    async fn simple(exercise: Exercise) -> Self {
        let equipment = exercise.equipment().await.unwrap();
        let muscle_group = exercise.target_muscle_group().await.unwrap();

        Self {
            id: exercise.ulid(),
            exercise_type: exercise.exercise_type(),
            target_muscle_group: MuscleGroupResource::simple(muscle_group).await,
            name: exercise.name(),
            name_alternative: exercise.name_alternative(),
            description: exercise.description(),
            equipment: ExerciseEquipmentResource::simple(equipment).await,
            mechanic: exercise.mechanic(),
            force: exercise.force(),
            measurement: MeasurementResource::new(exercise.measurement()),
            primary_muscles: None,
            secondary_muscles: None,
            tertiary_muscles: None,
        }
    }
}
