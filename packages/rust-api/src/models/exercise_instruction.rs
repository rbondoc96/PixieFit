use super::{Error, Model};
use crate::{
    enums::{ExerciseForce, ExerciseMechanic, ExerciseMuscleTarget, ExerciseType},
    sys::DatabaseManager,
    types::ISO8601DateTimeUTC,
};
use async_trait::async_trait;
use sqlx::{postgres::PgPool, FromRow};

#[derive(Clone, Debug, FromRow)]
pub struct ExerciseInstructionRecord {
    exercise_id: i64,
    order: i16,
    content: String,
    created_at: ISO8601DateTimeUTC,
    updated_at: ISO8601DateTimeUTC,
}

pub struct ExerciseInstruction {
    database: DatabaseManager,
    data: ExerciseInstructionRecord,
}

#[async_trait]
impl Model for ExerciseInstruction {
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

    pub fn order(&self) -> i16 {
        self.data.order
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

    pub fn exercise(&self) {
        todo!()
    }

    // endregion
}
