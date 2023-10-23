use super::{Error, Model, Profile, Result};
use crate::prelude::*;
use crate::{
    enums::Role,
};
use async_trait::async_trait;
use chrono::Utc;
use database::DatabaseManager;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};

#[derive(Clone, Debug, FromRow)]
pub struct UserRecord {
    id: i64,
    password: String,
    role: Role,
    email: String,
    first_name: String,
    last_name: String,
    last_logged_in_at: Option<ISO8601DateTimeUTC>,
    created_at: ISO8601DateTimeUTC,
    updated_at: ISO8601DateTimeUTC,
}

#[derive(Debug)]
pub struct NewUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Role,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct User {
    database: DatabaseManager,
    data: UserRecord,
}

#[async_trait]
impl Model for User {
    const MODEL_NAME: &'static str = "User";
    const TABLE_NAME: &'static str = "users";
    type Attributes = UserRecord;

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

impl User {
    pub fn id(&self) -> i64 {
        self.data.id
    }

    pub fn role(&self) -> Role {
        self.data.role.clone()
    }

    pub fn email(&self) -> String {
        self.data.email.clone()
    }

    pub fn first_name(&self) -> String {
        self.data.first_name.clone()
    }

    pub fn last_name(&self) -> String {
        self.data.last_name.clone()
    }

    pub fn last_logged_in_at(&self) -> Option<ISO8601DateTimeUTC> {
        self.data.last_logged_in_at
    }

    pub fn created_at(&self) -> ISO8601DateTimeUTC {
        self.data.created_at
    }

    pub fn updated_at(&self) -> ISO8601DateTimeUTC {
        self.data.updated_at
    }

    pub fn password(&self) -> String {
        self.data.password.clone()
    }

    // region Relationships

    pub async fn profile(&self) -> Result<Profile> {
        Profile::find_by_user(self.id(), &self.database).await
    }

    // endregion

    pub async fn create(
        attributes: NewUser,
        database: &DatabaseManager,
    ) -> Result<User> {
        let mut transaction = database.connection().begin().await?;

        let user = sqlx::query_as::<_, UserRecord>("INSERT INTO users (email, first_name, last_name, role, password) VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(attributes.email)
            .bind(attributes.first_name)
            .bind(attributes.last_name)
            .bind(attributes.role)
            .bind(attributes.password)
            .fetch_one(&mut *transaction)
            .await;

        match user {
            Ok(user) => {
                transaction.commit().await?;
                Ok(User {
                    database: database.clone(),
                    data: user,
                })
            }
            Err(err) => {
                transaction.rollback().await?;
                Err(err.into())
            }
        }
    }

    pub async fn update_last_logged_in(
        &mut self,
        database: &DatabaseManager,
    ) -> Result<()> {
        let mut transaction = database.connection().begin().await?;

        let now = Utc::now();
        let result = sqlx::query("UPDATE users SET last_logged_in_at = $1 WHERE id = $2")
            .bind(now)
            .bind(self.id())
            .execute(&mut *transaction)
            .await;

        match result {
            Ok(result) => {
                if result.rows_affected() != 1 {
                    transaction.rollback().await?;
                    Err(Error::UnknownError)
                } else {
                    transaction.commit().await?;
                    self.data.last_logged_in_at = Some(now);
                    Ok(())
                }
            }
            Err(err) => {
                transaction.rollback().await?;
                Err(err.into())
            }
        }
    }

    pub async fn exists(email: &str, database: &DatabaseManager) -> Result<bool> {
        Self::has("email", email, database).await
    }

    pub async fn find_by_email(email: String, database: &DatabaseManager) -> Result<User> {
        Self::find("email", email, database).await
    }
}
