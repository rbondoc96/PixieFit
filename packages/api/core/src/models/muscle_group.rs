use super::{Error, Exercise, Link, Result};
use crate::prelude::*;
use async_trait::async_trait;
use database::{DatabaseManager, Model, SqlxAction};
use sqlx::{FromRow, PgPool};

#[cfg(test)]
pub(crate) use builder::*;

#[derive(Clone, Debug, FromRow)]
pub struct MuscleGroup {
    pub id: i16,
    pub name: String,
    pub image_source: Option<String>,
    pub created_at: ISO8601DateTimeUTC,
    pub updated_at: ISO8601DateTimeUTC,
}

mod builder {
    use super::{Error, MuscleGroup, Result};
    use database::{DatabaseManager, Model};

    // region Type States

    #[derive(Default)]
    pub struct NoName;
    #[derive(Default)]
    pub struct Name(String);

    // endregion

    #[derive(Default)]
    pub struct MuscleGroupBuilder<N> {
        name: N,
        image_source: Option<String>,
    }

    impl MuscleGroupBuilder<NoName> {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl<N> MuscleGroupBuilder<N> {
        pub fn image_source(mut self, source: impl Into<String>) -> Self {
            self.image_source = Some(source.into());
            self
        }

        pub fn name(self, name: impl Into<String>) -> MuscleGroupBuilder<Name> {
            MuscleGroupBuilder {
                name: Name(name.into()),
                image_source: self.image_source,
            }
        }
    }

    impl MuscleGroupBuilder<Name> {
        pub async fn create(self, database: &DatabaseManager) -> Result<MuscleGroup> {
            let model = sqlx::query_as::<_, MuscleGroup>(format!(
                "INSERT INTO {} (name, image_source) VALUES ($1, $2) RETURNING *",
                MuscleGroup::TABLE_NAME,
            ).as_str())
                .bind(self.name.0)
                .bind(self.image_source)
                .fetch_one(database.connection())
                .await?;

            Ok(model)
        }
    }
}

use builder::*;

#[async_trait]
impl Model for MuscleGroup {
    const MODEL_NAME: &'static str = "MuscleGroup";
    const TABLE_NAME: &'static str = "muscle_groups";

    type PrimaryKey = i16;
    fn pk(&self) -> Self::PrimaryKey {
        self.id
    }

    type RouteKey = i16;
    fn rk(&self) -> Self::PrimaryKey {
        self.id
    }
}

impl MuscleGroup {
    pub fn new() -> MuscleGroupBuilder<NoName> {
        MuscleGroupBuilder::new()
    }

    // region Relationships

    pub async fn exercises(&self, database: &DatabaseManager) -> Result<Vec<Exercise>> {
        let results = Exercise::query()
            .select(&["*"])
            .and_where("target_muscle_group_id", "=", self.id)
            .all(database.connection())
            .await?;

        Ok(results)
    }

    // endregion

    // region Instance Methods

    pub async fn save(&mut self, database: &DatabaseManager) -> Result<()> {
        let model = sqlx::query_as::<_, Self>(format!(
            "UPDATE {} SET (name, image_source, updated_at) = ($1, $2, $3) WHERE {} = {} RETURNING *",
            Self::TABLE_NAME, Self::PRIMARY_KEY, &self.pk(),
        ).as_str())
            .bind(self.name.clone())
            .bind(self.image_source.clone())
            .bind(chrono::Utc::now())
            .fetch_one(database.connection())
            .await?;

        self.name = model.name;
        self.image_source = model.image_source;
        self.updated_at = model.updated_at;

        Ok(())
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::MuscleGroup;
    use crate::prelude::*;

    #[sqlx::test]
    async fn create_muscle_group_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let count = MuscleGroup::count(&database).await?;

        let group = MuscleGroup::new()
            .name("My Group")
            .create(&database)
            .await?;

        assert_eq!("My Group", group.name);
        assert_eq!(count + 1, MuscleGroup::count(&database).await?);

        Ok(())
    }

    #[sqlx::test]
    async fn edit_muscle_group_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let mut group = MuscleGroup::mocked(&database).await?;

        group.name = "My New Group".to_string();

        group.save(&database).await?;

        Ok(())
    }

    #[sqlx::test]
    async fn cannot_create_group_with_duplicate_name(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let group = MuscleGroup::mocked(&database).await?;

        let result = MuscleGroup::fake()
            .name(group.name)
            .create(&database)
            .await;

        assert!(result.is_err());

        Ok(())
    }
}
