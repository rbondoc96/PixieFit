use super::{Error, Exercise, Muscle, Result};
use crate::prelude::*;
use crate::enums::ExerciseMuscleTarget;
use async_trait::async_trait;
use database::{DatabaseManager, Model, SqlxAction};
use serde::Deserialize;
use sqlx::{FromRow, PgPool};

#[cfg(test)]
pub use builder::*;

#[derive(Clone, Debug, FromRow)]
pub struct ExerciseMuscleMap {
    pub id: i16,
    pub exercise_id: i16,
    pub muscle_id: i16,
    pub target: ExerciseMuscleTarget,
}

mod builder {
    use super::{ExerciseMuscleMap, Result};
    use crate::enums::ExerciseMuscleTarget;
    use crate::models::{Exercise, Muscle};
    use database::{DatabaseManager, Model};

    // region Type States

    #[derive(Default)]
    pub struct NoExerciseId;
    #[derive(Default)]
    pub struct ExerciseId(i16);

    #[derive(Default)]
    pub struct NoMuscleId;
    #[derive(Default)]
    pub struct MuscleId(i16);

    #[derive(Default)]
    pub struct NoTarget;
    #[derive(Default)]
    pub struct Target(ExerciseMuscleTarget);

    // endregion

    #[derive(Default)]
    pub struct ExerciseMuscleMapBuilder<E, M, T> {
        exercise_id: E,
        muscle_id: M,
        target: T,
    }

    impl ExerciseMuscleMapBuilder<NoExerciseId, NoMuscleId, NoTarget> {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl<E, M, T> ExerciseMuscleMapBuilder<E, M, T> {
        pub fn exercise_id(self, id: i16) -> ExerciseMuscleMapBuilder<ExerciseId, M, T> {
            ExerciseMuscleMapBuilder {
                exercise_id: ExerciseId(id),
                muscle_id: self.muscle_id,
                target: self.target,
            }
        }

        pub fn exercise(self, exercise: &Exercise) -> ExerciseMuscleMapBuilder<ExerciseId, M, T> {
            ExerciseMuscleMapBuilder {
                exercise_id: ExerciseId(exercise.id),
                muscle_id: self.muscle_id,
                target: self.target,
            }
        }

        pub fn muscle_id(self, id: i16) -> ExerciseMuscleMapBuilder<E, MuscleId, T> {
            ExerciseMuscleMapBuilder {
                exercise_id: self.exercise_id,
                muscle_id: MuscleId(id),
                target: self.target,
            }
        }

        pub fn muscle(self, muscle: &Muscle) -> ExerciseMuscleMapBuilder<E, MuscleId, T> {
            ExerciseMuscleMapBuilder {
                exercise_id: self.exercise_id,
                muscle_id: MuscleId(muscle.id),
                target: self.target,
            }
        }

        pub fn target(self, target: ExerciseMuscleTarget) -> ExerciseMuscleMapBuilder<E, M, Target> {
            ExerciseMuscleMapBuilder {
                exercise_id: self.exercise_id,
                muscle_id: self.muscle_id,
                target: Target(target),
            }
        }
    }

    impl ExerciseMuscleMapBuilder<ExerciseId, MuscleId, Target> {
        pub async fn create(self, database: &DatabaseManager) -> Result<ExerciseMuscleMap> {
            let mut model = sqlx::query_as::<_, ExerciseMuscleMap>(format!(
                "INSERT INTO {} (exercise_id, muscle_id, target) VALUES ($1, $2, $3) RETURNING *",
                ExerciseMuscleMap::TABLE_NAME,
            ).as_str())
                .bind(self.exercise_id.0)
                .bind(self.muscle_id.0)
                .bind(self.target.0)
                .fetch_one(database.connection())
                .await?;

            Ok(model)
        }
    }
}

use builder::*;

#[async_trait]
impl Model for ExerciseMuscleMap {
    const MODEL_NAME: &'static str = "ExerciseMuscleMap";
    const TABLE_NAME: &'static str = "exercises_muscles";

    type PrimaryKey = i16;
    fn pk(&self) -> Self::PrimaryKey {
        self.id
    }

    type RouteKey = i16;
    fn rk(&self) -> Self::RouteKey {
        self.id
    }
}

impl ExerciseMuscleMap {
    pub fn new() -> ExerciseMuscleMapBuilder<NoExerciseId, NoMuscleId, NoTarget> {
        ExerciseMuscleMapBuilder::new()
    }

    pub async fn find_by_exercise_and_target(
        exercise_id: i16,
        target: ExerciseMuscleTarget,
        database: &DatabaseManager,
    ) -> Result<Vec<ExerciseMuscleMap>> {
        let relations = Self::query()
            .select(&["*"])
            .and_where("exercise_id", "=", exercise_id)
            .and_where("target", "=", target)
            .all(database.connection())
            .await?;

        Ok(relations)
    }

    // region Instance Methods

    pub async fn save(&mut self, database: &DatabaseManager) -> Result<()> {
        let model = sqlx::query_as::<_, Self>(format!(
            "UPDATE {} SET (exercise_id, muscle_id, target) = ($1, $2, $3) WHERE {} = {} RETURNING *",
            Self::TABLE_NAME, Self::PRIMARY_KEY, &self.pk(),
        ).as_str())
            .bind(self.exercise_id)
            .bind(self.muscle_id)
            .bind(self.target.clone())
            .fetch_one(database.connection())
            .await?;

        self.exercise_id = model.exercise_id;
        self.muscle_id = model.muscle_id;
        self.target = model.target;

        Ok(())
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::ExerciseMuscleMap;
    use crate::enums::ExerciseMuscleTarget;
    use crate::models::{Exercise, Muscle};
    use crate::prelude::*;

    #[sqlx::test]
    async fn create_exercise_muscle_map_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let exercise = Exercise::mocked(&database).await?;
        let muscle = Muscle::mocked(&database).await?;

        let count = ExerciseMuscleMap::count(&database).await?;

        let map = ExerciseMuscleMap::new()
            .exercise(&exercise)
            .muscle(&muscle)
            .target(ExerciseMuscleTarget::Primary)
            .create(&database)
            .await?;

        assert_eq!(count + 1, ExerciseMuscleMap::count(&database).await?);
        assert_eq!(exercise.id, map.exercise_id);
        assert_eq!(muscle.id, map.muscle_id);
        assert_eq!(ExerciseMuscleTarget::Primary, map.target);

        Ok(())
    }

    #[sqlx::test]
    async fn cannot_create_exercise_muscle_map_with_duplicate_columns(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let exercise = Exercise::mocked(&database).await?;
        let muscle = Muscle::mocked(&database).await?;

        let map = ExerciseMuscleMap::fake()
            .exercise(&exercise)
            .muscle(&muscle)
            .target(ExerciseMuscleTarget::Primary)
            .create(&database)
            .await?;

        let result = ExerciseMuscleMap::fake()
            .exercise(&exercise)
            .muscle(&muscle)
            .target(ExerciseMuscleTarget::Primary)
            .create(&database)
            .await;

        assert!(result.is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn edit_exercise_muscle_map_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let exercise = Exercise::mocked(&database).await?;
        let muscle = Muscle::mocked(&database).await?;

        let mut map = ExerciseMuscleMap::fake()
            .target(ExerciseMuscleTarget::Secondary)
            .create(&database)
            .await?;

        map.exercise_id = exercise.id;
        map.muscle_id = muscle.id;
        map.target = ExerciseMuscleTarget::Tertiary;

        map.save(&database).await?;

        assert_eq!(exercise.id, map.exercise_id);
        assert_eq!(muscle.id, map.muscle_id);
        assert_eq!(ExerciseMuscleTarget::Tertiary, map.target);

        Ok(())
    }
}
