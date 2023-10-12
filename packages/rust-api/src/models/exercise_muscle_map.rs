use super::{Error, Exercise, Model, Muscle};
use crate::{
    enums::ExerciseMuscleTarget,
    sys::DatabaseManager,
    types::ISO8601DateTimeUTC,
};
use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{FromRow, PgPool};

#[derive(Clone, Debug, FromRow)]
pub struct ExerciseMuscleMapRecord {
    exercise_id: i64,
    muscle_id: i64,
    target: ExerciseMuscleTarget,
}

#[derive(Debug)]
pub struct CreateExerciseMuscleMapData {
    pub exercise_id: i64,
    pub muscle_id: i64,
    pub target: ExerciseMuscleTarget,
}

pub struct ExerciseMuscleMap {
    database: DatabaseManager,
    data: ExerciseMuscleMapRecord,
}

#[async_trait]
impl Model for ExerciseMuscleMap {
    const TABLE_NAME: &'static str = "exercises_muscles";
    type Attributes = ExerciseMuscleMapRecord;

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

impl ExerciseMuscleMap {
    pub fn exercise_id(&self) -> i64 {
        self.data.exercise_id
    }

    pub fn muscle_id(&self) -> i64 {
        self.data.muscle_id
    }

    pub fn target(&self) -> ExerciseMuscleTarget {
        self.data.target.clone()
    }

    pub async fn create(
        muscle_data: CreateExerciseMuscleMapData,
        database: &DatabaseManager,
    ) -> Result<ExerciseMuscleMap, sqlx::Error> {
        let mut transaction = database.connection().begin().await?;

        let record = sqlx::query_as::<_, ExerciseMuscleMapRecord>(
            "INSERT INTO exercises_muscles (exercise_id, muscle_id, target) VALUES ($1, $2, $3) RETURNING *"
        )
            .bind(muscle_data.exercise_id)
            .bind(muscle_data.muscle_id)
            .bind(muscle_data.target)
            .fetch_one(&mut *transaction)
            .await;

        match record {
            Ok(record) => {
                transaction.commit().await?;
                Ok(Self::from_database(record, database))
            },
            Err(err) => {
                transaction.rollback().await?;
                Err(err)
            }
        }
    }

    pub async fn find_by_exercise_and_target(
        id: i64,
        target: ExerciseMuscleTarget,
        database: &DatabaseManager,
    ) -> Result<Vec<ExerciseMuscleMap>, sqlx::Error> {
        let data = sqlx::query_as::<_, ExerciseMuscleMapRecord>(
            format!("SELECT * FROM {} WHERE exercise_id = $1 AND target = $2", Self::TABLE_NAME).as_str(),
        )
            .bind(id)
            .bind(target)
            .fetch_all(database.connection())
            .await?;

        Ok(data
            .into_iter()
            .map(|data| Self::from_database(data, database))
            .collect())
    }
}
