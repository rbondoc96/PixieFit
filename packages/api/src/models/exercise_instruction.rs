use super::{Error, Exercise, Model, Result};
use crate::prelude::*;
use crate::{
    enums::{ExerciseForce, ExerciseMechanic, ExerciseMuscleTarget, ExerciseType},
    sys::DatabaseManager,
};
use async_trait::async_trait;
use sqlx::{postgres::PgPool, FromRow};

#[derive(Clone, Debug, FromRow)]
pub struct ExerciseInstructionRecord {
    exercise_id: i64,
    sequence_number: i16,
    content: String,
    created_at: ISO8601DateTimeUTC,
    updated_at: ISO8601DateTimeUTC,
}

pub struct CreateExerciseInstructionData {
    pub exercise_id: i64,
    pub sequence_number: i16,
    pub content: String,
}

pub struct ExerciseInstruction {
    database: DatabaseManager,
    data: ExerciseInstructionRecord,
}

#[async_trait]
impl Model for ExerciseInstruction {
    const MODEL_NAME: &'static str = "ExerciseInstruction";
    const TABLE_NAME: &'static str = "exercise_instructions";
    type Attributes = ExerciseInstructionRecord;

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

impl ExerciseInstruction {
    pub fn exercise_id(&self) -> i64 {
        self.data.exercise_id
    }

    pub fn sequence_number(&self) -> i16 {
        self.data.sequence_number
    }

    pub fn content(&self) -> String {
        self.data.content.clone()
    }

    pub fn created_at(&self) -> ISO8601DateTimeUTC {
        self.data.created_at
    }

    pub fn updated_at(&self) -> ISO8601DateTimeUTC {
        self.data.updated_at
    }

    // region Relationships

    pub async fn exercise(&self) -> Result<Exercise> {
        Exercise::find_by_id(self.data.exercise_id, &self.database).await
    }

    // endregion

    pub async fn create(
        attributes: CreateExerciseInstructionData,
        database: &DatabaseManager,
    ) -> Result<ExerciseInstruction> {
        let record = sqlx::query_as::<_, ExerciseInstructionRecord>(format!(
            "INSERT INTO {} (exercise_id, sequence_number, content) VALUES ($1, $2, $3) RETURNING *",
            Self::TABLE_NAME,
        ).as_str())
        .bind(attributes.exercise_id)
        .bind(attributes.sequence_number)
        .bind(attributes.content)
        .fetch_one(database.connection())
        .await?;

        Ok(ExerciseInstruction {
            database: database.clone(),
            data: record,
        })
    }
}
