pub(self) mod base;
mod exercise;
mod exercise_equipment;
mod exercise_instruction;
mod exercise_muscle_map;
mod errors;
mod link;
mod muscle;
mod muscle_group;
mod user;
mod profile;

pub use exercise::{CreateExerciseData, Exercise};
pub use exercise_equipment::ExerciseEquipment;
pub use exercise_instruction::ExerciseInstruction;
pub use exercise_muscle_map::{CreateExerciseMuscleMapData, ExerciseMuscleMap};
pub use errors::Error;
pub use link::{CreateLinkData, Link};
pub use muscle::{CreateMuscleData, Muscle, MuscleRecord};
pub use muscle_group::MuscleGroup;
pub use profile::{CreateUserProfileData, Profile};
pub use user::{NewUser, User};
pub(self) type Result<TValue> = ::core::result::Result<TValue, Error>;

use crate::sys::DatabaseManager;
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::{Encode, FromRow, PgPool, Postgres, Type};

#[async_trait]
pub trait Model
where
    Self: Send + Sized + Unpin,
{
    const TABLE_NAME: &'static str;
    type Attributes: for<'r> FromRow<'r, PgRow> + Unpin + Send;

    fn connection(&self) -> &PgPool;
    fn from_database(attributes: Self::Attributes, database: &DatabaseManager) -> Self;

    async fn find<TKey>(key: &'static str, value: TKey, database: &DatabaseManager) -> Result<Self>
    where
        TKey: Type<Postgres> + for<'q> Encode<'q, Postgres> + Send + std::fmt::Display + Clone,
    {
        base::find(key, value, database).await
    }

    async fn find_by_id(id: i64, database: &DatabaseManager) -> Result<Self> {
        base::find_by_id(id, database).await
    }

    async fn all(database: &DatabaseManager) -> Result<Vec<Self>> {
        base::all(database).await
    }
}
