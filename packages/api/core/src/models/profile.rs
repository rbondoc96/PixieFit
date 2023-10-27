use super::{Error, Result, User};
use crate::prelude::*;
use crate::enums::Gender;
use async_trait::async_trait;
use chrono::NaiveDate;
use database::{DatabaseManager, Model, SqlxAction};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[cfg(test)]
pub(crate) use builder::*;

#[derive(Clone, Debug, FromRow)]
pub struct Profile {
    pub id: i16,
    pub ulid: String,
    pub user_id: i16,
    pub birthday: NaiveDate,
    pub gender: Gender,
    pub created_at: ISO8601DateTimeUTC,
    pub updated_at: ISO8601DateTimeUTC,
}

mod builder {
    use super::{Error, Profile, Result, User};
    use crate::enums::Gender;
    use chrono::NaiveDate;
    use database::{DatabaseManager, Model};

    // region Type States

    #[derive(Default)]
    pub struct NoUserId;
    #[derive(Default)]
    pub struct UserId(pub i16);

    #[derive(Default)]
    pub struct NoBirthday;
    #[derive(Default)]
    pub struct Birthday(NaiveDate);

    #[derive(Default)]
    pub struct NoUserGender;
    #[derive(Default)]
    pub struct UserGender(Gender);

    // endregion

    #[derive(Default)]
    pub struct ProfileBuilder<U, B, G> {
        pub user_id: U,
        pub birthday: B,
        pub gender: G,
    }

    impl ProfileBuilder<NoUserId, NoBirthday, NoUserGender> {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl<B, G> ProfileBuilder<NoUserId, B, G> {
        pub fn user_id(self, id: i16) -> ProfileBuilder<UserId, B, G> {
            ProfileBuilder {
                user_id: UserId(id),
                birthday: self.birthday,
                gender: self.gender,
            }
        }

        pub fn user(self, user: &User) -> ProfileBuilder<UserId, B, G> {
            ProfileBuilder {
                user_id: UserId(user.id),
                birthday: self.birthday,
                gender: self.gender,
            }
        }
    }

    impl<U, G> ProfileBuilder<U, NoBirthday, G> {
        pub fn birthday(self, birthday: NaiveDate) -> ProfileBuilder<U, Birthday, G> {
            ProfileBuilder {
                user_id: self.user_id,
                birthday: Birthday(birthday),
                gender: self.gender,
            }
        }
    }

    impl<U, B> ProfileBuilder<U, B, NoUserGender> {
        pub fn gender(self, gender: Gender) -> ProfileBuilder<U, B, UserGender> {
            ProfileBuilder {
                user_id: self.user_id,
                birthday: self.birthday,
                gender: UserGender(gender),
            }
        }
    }

    impl ProfileBuilder<UserId, Birthday, UserGender> {
        pub async fn create(self, database: &DatabaseManager) -> Result<Profile> {
            let model = sqlx::query_as::<_, Profile>(format!(
                "INSERT INTO {} (user_id, birthday, gender) VALUES ($1, $2, $3) RETURNING *",
                Profile::TABLE_NAME,
            ).as_str())
                .bind(self.user_id.0)
                .bind(self.birthday.0)
                .bind(self.gender.0)
                .fetch_one(database.connection())
                .await?;

            Ok(model)
        }
    }
}

use builder::*;

#[async_trait]
impl Model for Profile {
    const MODEL_NAME: &'static str = "Profile";
    const TABLE_NAME: &'static str = "user_profiles";

    type PrimaryKey = i16;
    fn pk(&self) -> Self::PrimaryKey {
        self.id
    }

    const ROUTE_KEY: &'static str = "ulid";
    type RouteKey = String;
    fn rk(&self) -> Self::RouteKey {
        self.ulid.clone()
    }
}

impl Profile {
    // region Static Methods

    pub fn new() -> ProfileBuilder<NoUserId, NoBirthday, NoUserGender> {
        ProfileBuilder::new()
    }

    pub async fn find_by_user(user_id: i16, database: &DatabaseManager) -> Result<Self> {
        let user = Self::query()
            .select(&["*"])
            .and_where("user_id", "=", user_id)
            .one(database.connection())
            .await?;

        Ok(user)
    }

    // endregion

    // region Instance Methods

    pub async fn save(&mut self, database: &DatabaseManager) -> Result<()> {
        let model = sqlx::query_as::<_, Self>(format!(
            "UPDATE {} SET (birthday, gender, updated_at) = ($1, $2, $3) WHERE {} = {} RETURNING *",
            Self::TABLE_NAME, Self::PRIMARY_KEY, &self.pk(),
        ).as_str())
            .bind(self.birthday)
            .bind(self.gender.clone())
            .bind(chrono::Utc::now())
            .fetch_one(database.connection())
            .await?;

        self.birthday = model.birthday;
        self.gender = model.gender;
        self.updated_at = model.updated_at;

        Ok(())
    }

    // endregion

    // region Relationships

    pub async fn user(&self, database: &DatabaseManager) -> Result<User> {
        let user = User::find_by_pk(self.user_id, database).await?;

        Ok(user)
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::Profile;
    use crate::enums::{Gender, Role};
    use crate::models::User;
    use crate::prelude::*;

    #[sqlx::test]
    async fn create_profile_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let user = User::mocked(&database).await?;

        let birthday = chrono::NaiveDate::from_ymd_opt(2000, 01, 01).unwrap();
        let profile = Profile::new()
            .user(&user)
            .birthday(birthday)
            .gender(Gender::Other)
            .create(&database)
            .await?;

        let count = Profile::count(&database).await?;

        assert_eq!(user.id, profile.user_id);
        assert_eq!(Gender::Other, profile.gender);
        assert_eq!(birthday, profile.birthday);
        assert_eq!(1, count);

        Ok(())
    }

    #[sqlx::test]
    async fn edit_profile_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let user = User::mocked(&database).await?;

        let mut profile = Profile::new()
            .user(&user)
            .birthday(chrono::NaiveDate::from_ymd_opt(2000, 01, 01).unwrap())
            .gender(Gender::Other)
            .create(&database)
            .await?;

        let birthday = chrono::NaiveDate::from_ymd_opt(1999, 10, 03).unwrap();
        profile.birthday = birthday;
        profile.gender = Gender::NonBinary;

        profile.save(&database).await?;

        assert_eq!(birthday, profile.birthday);
        assert_eq!(Gender::NonBinary, profile.gender);

        Ok(())
    }

    #[sqlx::test]
    async fn find_by_user_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let user = User::mocked(&database).await?;

        let profile = Profile::new()
            .user(&user)
            .birthday(chrono::NaiveDate::from_ymd_opt(2000, 01, 01).unwrap())
            .gender(Gender::Other)
            .create(&database)
            .await?;

        let result = Profile::find_by_user(user.id, &database).await?;

        assert_eq!(profile.id, result.id);

        Ok(())
    }
}
