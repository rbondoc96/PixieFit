use super::{Error, Model, Result, User};
use crate::prelude::*;
use crate::{
    enums::Gender,
    sys::DatabaseManager,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use ulid::Ulid;

#[derive(Clone, Debug, FromRow)]
pub struct UserProfileRecord {
    id: i64,
    ulid: String,
    user_id: i64,
    birthday: chrono::NaiveDate,
    gender: Gender,
    created_at: ISO8601DateTimeUTC,
    updated_at: ISO8601DateTimeUTC,
}

pub struct CreateUserProfileData {
    pub user_id: i64,
    pub birthday: chrono::NaiveDate,
    pub gender: Gender,
}

pub struct Profile {
    database: DatabaseManager,
    data: UserProfileRecord,
}

#[async_trait]
impl Model for Profile {
    const ROUTE_KEY: &'static str = "ulid";
    const MODEL_NAME: &'static str = "Profile";
    const TABLE_NAME: &'static str = "user_profiles";
    type Attributes = UserProfileRecord;

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

impl Profile {
    pub fn route_key(&self) -> String {
        self.data.ulid.clone()
    }

    pub fn birthday(&self) -> chrono::NaiveDate {
        self.data.birthday
    }

    pub fn gender(&self) -> Gender {
        self.data.gender.clone()
    }

    pub async fn create(
        attributes: CreateUserProfileData,
        database: &DatabaseManager,
    ) -> Result<Profile> {
        let mut transaction = database.connection().begin().await?;

        let ulid = Ulid::new();

        let profile = sqlx::query_as::<_, UserProfileRecord>("INSERT INTO user_profiles (ulid, user_id, birthday, gender) VALUES ($1, $2, $3, $4) RETURNING *")
            .bind(ulid.to_string().to_lowercase())
            .bind(attributes.user_id)
            .bind(attributes.birthday)
            .bind(attributes.gender)
            .fetch_one(&mut *transaction)
            .await;

        match profile {
            Ok(profile) => {
                transaction.commit().await?;
                Ok(Profile {
                    database: database.clone(),
                    data: profile,
                })
            }
            Err(err) => {
                transaction.rollback().await?;
                Err(err.into())
            }
        }
    }

    pub async fn find_by_user(
        user_id: i64,
        database: &DatabaseManager,
    ) -> Result<Profile> {
        super::base::find::<Self, i64>("user_id", user_id, database).await
    }
}
