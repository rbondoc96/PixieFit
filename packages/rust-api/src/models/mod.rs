pub(self) mod base;
mod errors;
mod link;
mod muscle;
mod muscle_group;
mod user;
mod profile;

pub use errors::Error;
pub use link::{CreateLinkData, Link};
pub use muscle::{CreateMuscleData, Muscle};
pub use muscle_group::MuscleGroup;
pub use user::{NewUser, User};
pub use profile::{CreateUserProfileData, Profile};
pub(self) type Result<TValue> = ::core::result::Result<TValue, Error>;

use crate::sys::DatabaseManager;
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool};

#[async_trait]
pub trait Model
where
    Self: Send + Sized + Unpin,
{
    const TABLE_NAME: &'static str;
    type Attributes: for<'r> FromRow<'r, PgRow> + Unpin + Send;

    fn connection(&self) -> &PgPool;
    fn from_database(attributes: Self::Attributes, database: &DatabaseManager) -> Self;

    async fn find_by_id(id: i64, database: &DatabaseManager) -> Result<Self> {
        base::find_by_id(id, database).await
    }

    async fn all(database: &DatabaseManager) -> Result<Vec<Self>> {
        base::all(database).await
    }
}
