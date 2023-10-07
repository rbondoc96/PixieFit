use super::{Error, Model, Profile};
use crate::{
    enums::Role,
    sys::DatabaseManager,
    types::ISO8601DateTimeUTC,
};
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, NaiveDate, Utc};
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

    pub async fn profile(&self) -> Result<Profile, Error> {
        Profile::find_by_user(self.id(), &self.database).await
    }

    // endregion

    pub async fn create(
        attributes: NewUser,
        database: &DatabaseManager,
    ) -> Result<User, sqlx::Error> {
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
                Err(err)
            }
        }
    }

    pub async fn update_last_logged_in(
        &mut self,
        database: &DatabaseManager,
    ) -> Result<(), sqlx::Error> {
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
                    Err(sqlx::Error::RowNotFound)
                } else {
                    transaction.commit().await?;
                    self.data.last_logged_in_at = Some(now);
                    Ok(())
                }
            }
            Err(err) => {
                transaction.rollback().await?;
                Err(err)
            }
        }
    }

    pub async fn exists(email: &str, database: &DatabaseManager) -> Result<bool, Error> {
        super::base::exists::<Self, &str>("email", email, database).await
    }

    pub async fn find_by_id(id: i64, database: &DatabaseManager) -> Result<User, Error> {
        super::base::find_by_id::<Self>(id, database).await
    }

    pub async fn find_by_email(email: String, database: &DatabaseManager) -> Result<User, Error> {
        super::base::find("email", email, database).await
    }

    pub async fn save(self, database: &DatabaseManager) -> Result<(), sqlx::Error> {
        let mut transaction = database.connection().begin().await?;

        let update_result = sqlx::query_as::<_, (i64,)>("UPDATE users SET email = $1, first_name = $2, last_name = $3, password = $4 WHERE id = $5 RETURNING id")
            .bind(self.data.email)
            .bind(self.data.first_name)
            .bind(self.data.last_name)
            .bind(self.data.password)
            .bind(self.data.id)
            .fetch_one(&mut *transaction)
            .await;

        match update_result {
            Ok(result) => {
                log::info!("Update user succeeded");
                transaction.commit().await?;
                Ok(())
            }
            Err(err) => {
                log::info!("Update user failed");
                log::info!("{}", err);
                transaction.rollback().await?;
                Err(err)
            }
        }
    }
}
