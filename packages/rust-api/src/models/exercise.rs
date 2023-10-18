use super::{Error, ExerciseEquipment, ExerciseMuscleMap, Model, Muscle, MuscleGroup, MuscleRecord, Result};
use crate::prelude::*;
use crate::{
    enums::{ExerciseForce, ExerciseMechanic, ExerciseMuscleTarget, ExerciseType, Measurement},
    sys::DatabaseManager,
};
use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{postgres::PgPool, FromRow};

#[derive(Clone, Debug, FromRow)]
pub struct ExerciseRecord {
    id: i64,
    ulid: String,
    #[sqlx(rename = "type")]
    exercise_type: ExerciseType,
    target_muscle_group_id: i32,
    name: String,
    name_alternative: Option<String>,
    description: Option<String>,
    equipment: String,
    mechanic: ExerciseMechanic,
    force: ExerciseForce,
    measurement: Measurement,
    created_at: ISO8601DateTimeUTC,
    updated_at: ISO8601DateTimeUTC,
}

#[derive(Debug)]
pub struct CreateExerciseData {
    pub exercise_type: ExerciseType,
    pub target_muscle_group_id: i32,
    pub name: String,
    pub name_alternative: Option<String>,
    pub description: Option<String>,
    pub equipment: String,
    pub mechanic: ExerciseMechanic,
    pub force: ExerciseForce,
    pub measurement: Measurement,
}

pub struct Exercise {
    database: DatabaseManager,
    data: ExerciseRecord,
}

#[async_trait]
impl Model for Exercise {
    const ROUTE_KEY: &'static str = "ulid";
    const MODEL_NAME: &'static str = "Exercise";
    const TABLE_NAME: &'static str = "exercises";
    type Attributes = ExerciseRecord;

    fn connection(&self) -> &PgPool {
        self.database.connection()
    }

    fn from_database(attributes: Self::Attributes, database: &DatabaseManager) -> Self {
        Self {
            database: database.clone(),
            data: attributes,
        }
    }
}

impl Exercise {
    pub fn id(&self) -> i64 {
        self.data.id
    }

    pub fn ulid(&self) -> String {
        self.data.ulid.clone()
    }

    pub fn exercise_type(&self) -> ExerciseType {
        self.data.exercise_type.clone()
    }

    pub fn target_muscle_group_id(&self) -> i32 {
        self.data.target_muscle_group_id
    }

    pub fn name(&self) -> String {
        self.data.name.clone()
    }

    pub fn name_alternative(&self) -> Option<String> {
        self.data.name_alternative.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.data.description.clone()
    }

    pub fn equipment_key(&self) -> String {
        self.data.equipment.clone()
    }

    pub fn mechanic(&self) -> ExerciseMechanic {
        self.data.mechanic.clone()
    }

    pub fn force(&self) -> ExerciseForce {
        self.data.force.clone()
    }

    pub fn measurement(&self) -> Measurement {
        self.data.measurement.clone()
    }

    pub fn created_at(&self) -> ISO8601DateTimeUTC {
        self.data.created_at
    }

    pub fn updated_at(&self) -> ISO8601DateTimeUTC {
        self.data.updated_at
    }

    // region Relationships

    pub async fn equipment(&self) -> Result<ExerciseEquipment> {
        ExerciseEquipment::find("name", self.equipment_key(), &self.database).await
    }

    pub async fn target_muscle_group(&self) -> Result<MuscleGroup> {
        MuscleGroup::find_by_id(
            self.target_muscle_group_id().into(),
            &self.database,
        )
        .await
    }

    async fn muscles(&self, target: ExerciseMuscleTarget) -> Result<Vec<Muscle>> {
        let results = sqlx::query_as::<_, MuscleRecord>(
            "SELECT muscles.* FROM muscles INNER JOIN exercises_muscles ON muscles.id = exercises_muscles.muscle_id WHERE exercises_muscles.exercise_id = $1 AND exercises_muscles.target = $2",
        )
        .bind(self.id())
        .bind(target)
        .fetch_all(self.connection())
        .await?;

        Ok(results
            .into_iter()
            .map(|record| Muscle::from_database(record, &self.database))
            .collect()
        )
    }

    pub async fn primary_muscles(&self) -> Result<Vec<Muscle>> {
        self.muscles(ExerciseMuscleTarget::Primary).await
    }

    pub async fn secondary_muscles(&self) -> Result<Vec<Muscle>> {
        self.muscles(ExerciseMuscleTarget::Secondary).await
    }

    pub async fn tertiary_muscles(&self) -> Result<Vec<Muscle>> {
        self.muscles(ExerciseMuscleTarget::Tertiary).await
    }

    // endregion

    pub async fn create(
        attributes: CreateExerciseData,
        database: &DatabaseManager,
    ) -> Result<Exercise> {
        let connection = database.connection();

        let record = sqlx::query_as::<_, ExerciseRecord>(
            "INSERT INTO exercises (type, target_muscle_group_id, name, name_alternative, description, equipment, mechanic, force, measurement) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
        )
        .bind(attributes.exercise_type)
        .bind(attributes.target_muscle_group_id)
        .bind(attributes.name)
        .bind(attributes.name_alternative)
        .bind(attributes.description)
        .bind(attributes.equipment)
        .bind(attributes.mechanic)
        .bind(attributes.force)
        .bind(attributes.measurement)
        .fetch_one(connection)
        .await?;

        Ok(Exercise {
            database: database.clone(),
            data: record,
        })
    }
}