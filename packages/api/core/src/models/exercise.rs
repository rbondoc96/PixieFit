use super::{Error, ExerciseEquipment, ExerciseMuscleMap, Model, Muscle, MuscleGroup, MuscleRecord, Result};
use crate::prelude::*;
use crate::{
    enums::{ExerciseForce, ExerciseMechanic, ExerciseMuscleTarget, ExerciseType, Measurement},
};
use async_trait::async_trait;
use database::DatabaseManager;
use serde::Deserialize;
use sqlx::{postgres::PgPool, FromRow};

#[derive(Clone, Debug, FromRow)]
pub struct ExerciseRecord {
    id: i64,
    ulid: String,
    external_id: Option<i16>,
    #[sqlx(rename = "type")]
    exercise_type: ExerciseType,
    target_muscle_group_id: Option<i32>,
    name: String,
    name_alternative: Option<String>,
    description: Option<String>,
    equipment: Option<String>,
    mechanic: Option<ExerciseMechanic>,
    force: Option<ExerciseForce>,
    measurement: Option<Measurement>,
    created_at: ISO8601DateTimeUTC,
    updated_at: ISO8601DateTimeUTC,
}

#[derive(Debug)]
pub struct CreateExerciseData {
    pub external_id: Option<i16>,
    pub exercise_type: ExerciseType,
    pub target_muscle_group_id: Option<i32>,
    pub name: String,
    pub name_alternative: Option<String>,
    pub description: Option<String>,
    pub equipment: Option<String>,
    pub mechanic: Option<ExerciseMechanic>,
    pub force: Option<ExerciseForce>,
    pub measurement: Option<Measurement>,
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

    pub fn external_id(&self) -> Option<i16> {
        self.data.external_id.clone()
    }

    pub fn exercise_type(&self) -> ExerciseType {
        self.data.exercise_type.clone()
    }

    pub fn target_muscle_group_id(&self) -> Option<i32> {
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

    pub fn equipment_key(&self) -> Option<String> {
        self.data.equipment.clone()
    }

    pub fn mechanic(&self) -> Option<ExerciseMechanic> {
        self.data.mechanic.clone()
    }

    pub fn force(&self) -> Option<ExerciseForce> {
        self.data.force.clone()
    }

    pub fn measurement(&self) -> Option<Measurement> {
        self.data.measurement.clone()
    }

    pub fn created_at(&self) -> ISO8601DateTimeUTC {
        self.data.created_at
    }

    pub fn updated_at(&self) -> ISO8601DateTimeUTC {
        self.data.updated_at
    }

    // region Relationships

    pub async fn equipment(&self) -> Result<Option<ExerciseEquipment>> {
        let key = self.equipment_key();

        if key.is_none() {
            return Ok(None);
        }

        let equipment = ExerciseEquipment::find(
            "name",
            key.unwrap(),
            &self.database
        ).await?;

        Ok(Some(equipment))
    }

    pub async fn target_muscle_group(&self) -> Result<Option<MuscleGroup>> {
        let key = self.target_muscle_group_id();

        if key.is_none() {
            return Ok(None);
        }

        let group = MuscleGroup::find_by_id(
            key.unwrap().into(),
            &self.database,
        )
        .await?;

        Ok(Some(group))
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
            "INSERT INTO exercises (external_id, type, target_muscle_group_id, name, name_alternative, description, equipment, mechanic, force, measurement) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *",
        )
        .bind(attributes.external_id)
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