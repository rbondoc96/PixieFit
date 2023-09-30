pub(self) mod base;
mod errors;
mod link;
mod muscle;
mod user;
mod profile;

pub use errors::Error;
pub use link::{CreateLinkData, Link};
pub use muscle::{CreateMuscleData, Muscle};
pub use user::{NewUser, User};
pub use profile::{CreateUserProfileData, Profile};
pub(self) type Result<TValue> = ::core::result::Result<TValue, Error>;

use crate::sys::DatabaseManager;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool};

pub trait Model {
    const TABLE_NAME: &'static str;
    type Attributes: for<'r> FromRow<'r, PgRow> + Unpin + Send;

    fn connection(&self) -> &PgPool;
    fn from_database(attributes: Self::Attributes, database: &DatabaseManager) -> Self;
}
