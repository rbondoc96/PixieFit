use super::{ModelResource, ResourceResult};
use crate::models::ExerciseInstruction;
use async_trait::async_trait;
use database::DatabaseManager;
use serde::Serialize;

#[derive(Serialize)]
pub struct ExerciseInstructionResource {
    sequence_number: i16,
    content: String,
}

#[async_trait]
impl ModelResource for ExerciseInstructionResource {
    type Model = ExerciseInstruction;

    async fn default(instruction: ExerciseInstruction, database: &DatabaseManager) -> ResourceResult<Self> {
        Ok(Self {
            content: instruction.content,
            sequence_number: instruction.sequence_number,
        })
    }

    async fn simple(instruction: ExerciseInstruction, database: &DatabaseManager) -> ResourceResult<Self> {
        Ok(Self {
            content: instruction.content,
            sequence_number: instruction.sequence_number,
        })
    }
}
