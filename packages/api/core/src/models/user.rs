use super::{Error, Profile, Result};
use crate::enums::Role;
use crate::prelude::*;
use database::{DatabaseManager, HasRouteKey, Model};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};

#[cfg(test)]
pub(crate) use builder::*;

#[derive(Clone, Debug, FromRow)]
pub struct User {
    pub id: i16,
    pub email: String,
    pub role: Role,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub last_logged_in_at: Option<ISO8601DateTimeUTC>,
    pub created_at: ISO8601DateTimeUTC,
    pub updated_at: ISO8601DateTimeUTC,
}

mod builder {
    use super::{Error, User, Result};
    use crate::enums::Role;
    use crate::utils::crypt;
    use database::{DatabaseManager, Model};

    // region Type States

    #[derive(Default)]
    pub struct NoPassword;
    #[derive(Default)]
    pub struct Password(String);

    #[derive(Default)]
    pub struct NoUserRole;
    #[derive(Default)]
    pub struct UserRole(Role);

    #[derive(Default)]
    pub struct NoEmail;
    #[derive(Default)]
    pub struct Email(String);

    #[derive(Default)]
    pub struct NoName;
    #[derive(Default)]
    pub struct Name(String, String);

    // endregion

    #[derive(Default)]
    pub struct UserBuilder<P, R, E, N> {
        password: P,
        role: R,
        email: E,
        name: N,
    }

    impl UserBuilder<NoPassword, NoUserRole, NoEmail, NoName> {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl<P, R, E, N> UserBuilder<P, R, E, N> {
        pub fn password(self, password: impl Into<String>) -> UserBuilder<Password, R, E, N> {
            UserBuilder {
                password: Password(password.into()),
                role: self.role,
                email: self.email,
                name: self.name,
            }
        }

        pub fn role(self, role: Role) -> UserBuilder<P, UserRole, E, N> {
            UserBuilder {
                password: self.password,
                role: UserRole(role),
                email: self.email,
                name: self.name,
            }
        }

        pub fn admin(self) -> UserBuilder<P, UserRole, E, N> {
            UserBuilder {
                password: self.password,
                role: UserRole(Role::Admin),
                email: self.email,
                name: self.name,
            }
        }

        pub fn email(self, email: impl Into<String>) -> UserBuilder<P, R, Email, N> {
            UserBuilder {
                password: self.password,
                role: self.role,
                email: Email(email.into()),
                name: self.name,
            }
        }

        pub fn name(self, first: impl Into<String>, last: impl Into<String>) -> UserBuilder<P, R, E, Name> {
            UserBuilder {
                password: self.password,
                role: self.role,
                email: self.email,
                name: Name(first.into(), last.into()),
            }
        }
    }

    impl UserBuilder<Password, UserRole, Email, Name> {
        pub async fn create(self, database: &DatabaseManager) -> Result<User> {
            let password = crypt::encrypt(self.password.0.as_ref())?;

            let model = sqlx::query_as::<_, User>(format!(
                "INSERT INTO {} (email, role, first_name, last_name, password) VALUES ($1, $2, $3, $4, $5) RETURNING *",
                User::TABLE_NAME,
            ).as_str())
                .bind(self.email.0)
                .bind(self.role.0)
                .bind(self.name.0)
                .bind(self.name.1)
                .bind(password)
                .fetch_one(database.connection())
                .await?;

            Ok(model)
        }
    }
}

use builder::*;

#[async_trait]
impl Model for User {
    const MODEL_NAME: &'static str = "User";
    const TABLE_NAME: &'static str = "users";

    type PrimaryKey = i16;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}

impl HasRouteKey for User {
    const ROUTE_KEY: &'static str = "id";
    type RouteKey = i16;

    fn route_key(&self) -> Self::RouteKey {
        self.id
    }
}

impl User {
    // region Static Methods

    pub fn new() -> UserBuilder<NoPassword, NoUserRole, NoEmail, NoName> {
        UserBuilder::new()
    }

    pub async fn exists_with_email(email: impl ToString, database: &DatabaseManager) -> Result<bool> {
        let result = Self::query()
            .select(&["*"])
            .and_where("email", "=", email.to_string())
            .optional::<&PgPool, Self>(database.connection())
            .await
            .map(|user| user.is_some())?;

        Ok(result)
    }

    pub async fn find_by_email(email: impl ToString, database: &DatabaseManager) -> Result<User> {
        let user = Self::find(
            "email",
            email.to_string(),
            database
        ).await?;

        Ok(user)
    }

    // endregion

    // region Instance Methods

    pub async fn save(&mut self, database: &DatabaseManager) -> Result<()> {
        let model = sqlx::query_as::<_, Self>(format!(
            "UPDATE {} SET (email, first_name, last_name, password, updated_at) = ($1, $2, $3, $4, $5) WHERE {} = {} RETURNING *",
            Self::TABLE_NAME, Self::PRIMARY_KEY, self.primary_key(),
        ).as_str())
            .bind(self.email.clone())
            .bind(self.first_name.clone())
            .bind(self.last_name.clone())
            .bind(self.password.clone())
            .bind(chrono::Utc::now())
            .fetch_one(database.connection())
            .await?;

        self.email = model.email;
        self.first_name = model.first_name;
        self.last_name = model.last_name;
        self.password = model.password;
        self.updated_at = model.updated_at;

        Ok(())
    }

    pub async fn update_last_logged_in(&mut self, database: &DatabaseManager) -> Result<()> {
        let now = chrono::Utc::now();

        let result = sqlx::query(format!(
            "UPDATE {} SET last_logged_in_at = $1 WHERE {} = {}",
            Self::TABLE_NAME, Self::PRIMARY_KEY, self.primary_key()
        ).as_str())
            .bind(now)
            .execute(database.connection())
            .await?;

        self.last_logged_in_at = Some(now);

        Ok(())
    }

    // endregion

    // region Relationships

    pub async fn profile(&self, database: &DatabaseManager) -> Result<Profile> {
        Profile::find_by_user(self.id, database).await
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::User;
    use crate::enums::Role;
    use crate::models::Profile;
    use crate::prelude::*;
    use crate::utils::crypt::decrypt_and_verify;

    #[sqlx::test]
    async fn create_user_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);

        let user = User::new()
            .name("Test", "User")
            .email("test_user@example.com")
            .role(Role::User)
            .password("password")
            .create(&database)
            .await?;

        let count = User::count(&database).await?;

        assert_eq!("Test", user.first_name);
        assert_eq!("User", user.last_name);
        assert_eq!("test_user@example.com", user.email);
        assert!(decrypt_and_verify("password", user.password.as_ref())?);
        assert_eq!(Role::User, user.role);
        assert_eq!(1, count);

        Ok(())
    }

    #[sqlx::test]
    async fn edit_user_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);

        let mut user = User::mocked(&database).await?;

        user.email = "different_email@example.com".to_string();
        user.role = Role::Admin;
        user.first_name = "Bob".to_string();
        user.last_name = "Smith".to_string();
        user.password = "different".to_string();

        user.save(&database).await?;

        assert_eq!("Bob", user.first_name);
        assert_eq!("Smith", user.last_name);
        assert_eq!("different_email@example.com", user.email);
        assert_eq!("different", user.password);
        assert_eq!(Role::Admin, user.role);

        Ok(())
    }

    #[sqlx::test]
    async fn find_by_email_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);

        let user = User::new()
            .name("Test", "User")
            .email("test_user@example.com")
            .role(Role::User)
            .password("password")
            .create(&database)
            .await?;

        let result = User::find_by_email("test_user@example.com".to_string(), &database).await?;

        assert_eq!(result.id, user.id);

        Ok(())
    }

    #[sqlx::test]
    async fn exists_with_email_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);

        let user = User::new()
            .name("Test", "User")
            .email("test_user@example.com")
            .role(Role::User)
            .password("password")
            .create(&database)
            .await?;

        let exists = User::exists_with_email("test_user@example.com".to_string(), &database).await?;

        assert!(exists);

        Ok(())
    }

    #[sqlx::test]
    async fn updates_last_logged_in(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);

        let start_time = chrono::Utc::now();

        let mut user = User::mocked(&database).await?;

        user.update_last_logged_in(&database).await?;

        assert!(user.last_logged_in_at.is_some());
        assert!(user.last_logged_in_at.unwrap() > start_time);

        Ok(())
    }

    #[sqlx::test]
    async fn cannot_create_user_with_duplicate_email(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);

        let user = User::new()
            .name("Test", "User")
            .email("test_user@example.com")
            .role(Role::User)
            .password("password")
            .create(&database)
            .await?;

        let result = User::new()
            .name("Test", "User")
            .email("test_user@example.com")
            .role(Role::User)
            .password("password")
            .create(&database)
            .await;

        assert_eq!(true, result.is_err());

        Ok(())
    }
}
