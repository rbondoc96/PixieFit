use super::{Error, Exercise, Result};
use crate::prelude::*;
use crate::enums::{ExerciseForce, ExerciseMechanic, ExerciseMuscleTarget, ExerciseType};
use async_trait::async_trait;
use database::{DatabaseManager, Model};
use sqlx::{postgres::PgPool, FromRow};

#[cfg(test)]
pub use builder::*;

#[derive(Clone, Debug, FromRow)]
pub struct ExerciseInstruction {
    #[sqlx(skip)]
    database: Option<DatabaseManager>,
    pub id: i16,
    pub exercise_id: i16,
    pub sequence_number: i16,
    pub content: String,
    pub created_at: ISO8601DateTimeUTC,
    pub updated_at: ISO8601DateTimeUTC,
}

mod builder {
    use super::{ExerciseInstruction, Result};
    use crate::models::Exercise;
    use database::{DatabaseManager, Model};

    // region Type States

    #[derive(Default)]
    pub struct NoExerciseId;
    #[derive(Default)]
    pub struct ExerciseId(i16);

    #[derive(Default)]
    pub struct NoSequenceNumber;
    #[derive(Default)]
    pub struct SequenceNumber(i16);

    #[derive(Default)]
    pub struct NoContent;
    #[derive(Default)]
    pub struct Content(String);

    // endregion

    // region Builder

    #[derive(Default)]
    pub struct ExerciseInstructionBuilder<E, S, C> {
        exercise_id: E,
        sequence_number: S,
        content: C,
    }

    impl ExerciseInstructionBuilder<NoExerciseId, NoSequenceNumber, NoContent> {
        pub fn new() -> Self {
            Self::default()
        }
    }

    // impl<S, C> ExerciseInstructionBuilder<NoExerciseId, S, C> {
    //     pub fn exercise_id(self, id: i16) -> ExerciseInstructionBuilder<ExerciseId, S, C> {
    //         ExerciseInstructionBuilder {
    //             exercise_id: ExerciseId(id),
    //             sequence_number: self.sequence_number,
    //             content: self.content,
    //         }
    //     }
    //
    //     pub fn exercise(self, exercise: &Exercise) -> ExerciseInstructionBuilder<ExerciseId, S, C> {
    //         ExerciseInstructionBuilder {
    //             exercise_id: ExerciseId(exercise.id),
    //             sequence_number: self.sequence_number,
    //             content: self.content,
    //         }
    //     }
    // }
    //
    // impl<E, C> ExerciseInstructionBuilder<E, NoSequenceNumber, C> {
    //     pub fn sequence_number(self, number: i16) -> ExerciseInstructionBuilder<E, SequenceNumber, C> {
    //         ExerciseInstructionBuilder {
    //             exercise_id: self.exercise_id,
    //             sequence_number: SequenceNumber(number),
    //             content: self.content,
    //         }
    //     }
    // }
    //
    // impl<E, S> ExerciseInstructionBuilder<E, S, NoContent> {
    //     pub fn content(self, content: impl Into<String>) -> ExerciseInstructionBuilder<E, S, Content> {
    //         ExerciseInstructionBuilder {
    //             exercise_id: self.exercise_id,
    //             sequence_number: self.sequence_number,
    //             content: Content(content.into()),
    //         }
    //     }
    // }

    impl<E, S, C,> ExerciseInstructionBuilder<E, S, C> {
        pub fn exercise_id(self, id: i16) -> ExerciseInstructionBuilder<ExerciseId, S, C> {
            ExerciseInstructionBuilder {
                exercise_id: ExerciseId(id),
                sequence_number: self.sequence_number,
                content: self.content,
            }
        }

        pub fn exercise(self, exercise: &Exercise) -> ExerciseInstructionBuilder<ExerciseId, S, C> {
            ExerciseInstructionBuilder {
                exercise_id: ExerciseId(exercise.id),
                sequence_number: self.sequence_number,
                content: self.content,
            }
        }

        pub fn sequence_number(self, number: i16) -> ExerciseInstructionBuilder<E, SequenceNumber, C> {
            ExerciseInstructionBuilder {
                exercise_id: self.exercise_id,
                sequence_number: SequenceNumber(number),
                content: self.content,
            }
        }

        pub fn content(self, content: impl Into<String>) -> ExerciseInstructionBuilder<E, S, Content> {
            ExerciseInstructionBuilder {
                exercise_id: self.exercise_id,
                sequence_number: self.sequence_number,
                content: Content(content.into()),
            }
        }
    }

    impl ExerciseInstructionBuilder<ExerciseId, SequenceNumber, Content> {
        pub async fn create(self, database: &DatabaseManager) -> Result<ExerciseInstruction> {
            let model = sqlx::query_as::<_, ExerciseInstruction>(format!(
                "INSERT INTO {} (exercise_id, sequence_number, content) VALUES ($1, $2, $3) RETURNING *",
                ExerciseInstruction::TABLE_NAME,
            ).as_str())
                .bind(self.exercise_id.0)
                .bind(self.sequence_number.0)
                .bind(self.content.0)
                .fetch_one(database.connection())
                .await?;

            Ok(model)
        }
    }

    // endregion
}

use builder::*;

#[async_trait]
impl Model for ExerciseInstruction {
    const MODEL_NAME: &'static str = "ExerciseInstruction";
    const TABLE_NAME: &'static str = "exercise_instructions";

    type PrimaryKey = i16;
    fn pk(&self) -> Self::PrimaryKey {
        self.id
    }

    type RouteKey = i16;
    fn rk(&self) -> Self::RouteKey {
        self.id
    }
}

impl ExerciseInstruction {
    pub fn new() -> ExerciseInstructionBuilder<NoExerciseId, NoSequenceNumber, NoContent> {
        ExerciseInstructionBuilder::new()
    }

    // region Relationships

    pub async fn exercise(&self, database: &DatabaseManager) -> Result<Exercise> {
        let exercise = Exercise::find_by_pk(self.exercise_id, database).await?;

        Ok(exercise)
    }

    // endregion

    // region Instance Methods

    pub async fn save(&mut self, database: &DatabaseManager) -> Result<()> {
        let model = sqlx::query_as::<_, Self>(format!(
            "UPDATE {} SET (exercise_id, sequence_number, content, updated_at) = ($1, $2, $3, $4) WHERE {} = {} RETURNING *",
            Self::TABLE_NAME, Self::PRIMARY_KEY, &self.pk(),
        ).as_str())
            .bind(self.exercise_id)
            .bind(self.sequence_number)
            .bind(self.content.clone())
            .bind(chrono::Utc::now())
            .fetch_one(database.connection())
            .await?;

        self.exercise_id = model.exercise_id;
        self.sequence_number = model.sequence_number;
        self.content = model.content;
        self.updated_at = model.updated_at;

        Ok(())
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::ExerciseInstruction;
    use crate::models::Exercise;
    use crate::prelude::*;

    #[sqlx::test]
    async fn create_exercise_instruction_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let exercise = Exercise::mocked(&database).await?;

        let count = ExerciseInstruction::count(&database).await?;

        let instruction = ExerciseInstruction::new()
            .exercise(&exercise)
            .sequence_number(1)
            .content("Some content")
            .create(&database)
            .await?;

        assert_eq!(count + 1, ExerciseInstruction::count(&database).await?);
        assert_eq!(exercise.id, instruction.exercise_id);
        assert_eq!(1, instruction.sequence_number);
        assert_eq!("Some content", instruction.content);

        Ok(())
    }

    #[sqlx::test]
    async fn cannot_create_exercise_instructions_with_same_exercise_and_sequence(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let exercise = Exercise::mocked(&database).await?;
        let instructions = ExerciseInstruction::fake()
            .exercise(&exercise)
            .sequence_number(1)
            .create(&database)
            .await?;

        let result = ExerciseInstruction::fake()
            .exercise(&exercise)
            .sequence_number(1)
            .create(&database)
            .await;

        assert!(result.is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn edit_exercise_instruction_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let mut instructions = ExerciseInstruction::mocked(&database).await?;
        let another_exercise = Exercise::mocked(&database).await?;

        instructions.exercise_id = another_exercise.id;
        instructions.sequence_number = 24;
        instructions.content = "New content!".to_string();

        instructions.save(&database).await?;

        assert_eq!(another_exercise.id, instructions.exercise_id);
        assert_eq!(24, instructions.sequence_number);
        assert_eq!("New content!", instructions.content);

        Ok(())
    }
}
