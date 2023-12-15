use super::{
    ExerciseEquipmentResource,
    ExerciseInstructionResource,
    MeasurementResource,
    ModelResource,
    MuscleResource,
    MuscleGroupResource,
    ResourceResult,
};
use crate::enums::{ExerciseForce, ExerciseMechanic, ExerciseType};
use crate::models::Exercise;
use async_trait::async_trait;
use database::DatabaseManager;
use serde::Serialize;

#[derive(Serialize)]
pub struct ExerciseResource {
    id: String,
    #[serde(rename = "type")]
    exercise_type: ExerciseType,
    target_muscle_group: Option<MuscleGroupResource>,
    name: String,
    name_alternative: Option<String>,
    description: Option<String>,
    equipment: Option<ExerciseEquipmentResource>,
    mechanic: Option<ExerciseMechanic>,
    force: Option<ExerciseForce>,
    measurement: Option<MeasurementResource>,
    primary_muscles: Vec<MuscleResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_muscles: Option<Vec<MuscleResource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tertiary_muscles: Option<Vec<MuscleResource>>,
    instructions: Option<Vec<ExerciseInstructionResource>>,
}

#[async_trait]
impl ModelResource for ExerciseResource {
    type Model = Exercise;

    async fn default(exercise: Exercise, database: &DatabaseManager) -> ResourceResult<Self> {
        let equipment = match exercise.equipment(database).await? {
            Some(equipment) => Some(ExerciseEquipmentResource::simple(equipment, database).await?),
            None => None,
        };
        let target_muscle_group = match exercise.target_muscle_group(database).await? {
            Some(muscle_group) => Some(MuscleGroupResource::simple(muscle_group, database).await?),
            None => None,
        };
        let primary_muscles = MuscleResource::list(
            exercise.primary_muscles(database).await?,
            database).await?;
        let secondary_muscles = MuscleResource::list(
            exercise.secondary_muscles(database).await?,
            database).await?;
        let tertiary_muscles = MuscleResource::list(
            exercise.tertiary_muscles(database).await?,
            database).await?;
        let instructions = ExerciseInstructionResource::list(
            exercise.instructions(database).await?,
            database).await?;

        let measurement = match exercise.measurement {
            Some(measurement) => Some(MeasurementResource::new(measurement)),
            None => None,
        };

        Ok(Self {
            id: exercise.ulid,
            exercise_type: exercise.exercise_type,
            target_muscle_group,
            name: exercise.name,
            name_alternative: exercise.name_alternative,
            description: exercise.description,
            equipment,
            mechanic: exercise.mechanic,
            force: exercise.force,
            measurement,
            primary_muscles,
            secondary_muscles: Some(secondary_muscles),
            tertiary_muscles: Some(tertiary_muscles),
            instructions: Some(instructions),
        })
    }

    async fn simple(exercise: Exercise, database: &DatabaseManager) -> ResourceResult<Self> {
        let equipment = match exercise.equipment(database).await? {
            Some(equipment) => Some(ExerciseEquipmentResource::simple(equipment, database).await?),
            None => None,
        };
        let target_muscle_group = match exercise.target_muscle_group(database).await? {
            Some(muscle_group) => Some(MuscleGroupResource::simple(muscle_group, database).await?),
            None => None,
        };
        let primary_muscles = MuscleResource::list(
            exercise.primary_muscles(database).await?,
            database).await?;

        let measurement = match exercise.measurement {
            Some(measurement) => Some(MeasurementResource::new(measurement)),
            None => None,
        };

        Ok(Self {
            id: exercise.ulid,
            exercise_type: exercise.exercise_type,
            target_muscle_group,
            name: exercise.name,
            name_alternative: exercise.name_alternative,
            description: exercise.description,
            equipment,
            mechanic: exercise.mechanic,
            force: exercise.force,
            measurement,
            primary_muscles,
            secondary_muscles: None,
            tertiary_muscles: None,
            instructions: None,
        })
    }
}
