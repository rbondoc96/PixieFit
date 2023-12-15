use super::{Error, Link, MuscleGroup, Result};
use crate::prelude::*;
use async_trait::async_trait;
use database::{DatabaseManager, HasRouteKey, Model};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[cfg(test)]
pub use builder::*;

#[derive(Clone, Debug, FromRow)]
pub struct Muscle {
    pub id: i16,
    pub ulid: String,
    pub group_id: i16,
    pub parent_id: Option<i16>,
    pub name: String,
    pub simple_name: Option<String>,
    pub description: Option<String>,
    pub image_source: Option<String>,
    pub created_at: ISO8601DateTimeUTC,
    pub updated_at: ISO8601DateTimeUTC,
}

mod builder {
    use super::{Muscle, Result};
    use crate::models::MuscleGroup;
    use database::{DatabaseManager, Model};

    // region Type States

    #[derive(Default)]
    pub struct NoGroupId;
    #[derive(Default)]
    pub struct GroupId(i16);

    #[derive(Default)]
    pub struct NoName;
    #[derive(Default)]
    pub struct Name(String);

    // endregion

    #[derive(Default)]
    pub struct MuscleBuilder<G, N> {
        group_id: G,
        name: N,
        parent_id: Option<i16>,
        simple_name: Option<String>,
        description: Option<String>,
        image_source: Option<String>,
    }

    impl MuscleBuilder<NoGroupId, NoName> {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl<G, N> MuscleBuilder<G, N> {
        pub fn group_id(self, id: i16) -> MuscleBuilder<GroupId, N> {
            MuscleBuilder {
                group_id: GroupId(id),
                name: self.name,
                parent_id: self.parent_id,
                simple_name: self.simple_name,
                description: self.description,
                image_source: self.image_source,
            }
        }

        pub fn group(self, group: &MuscleGroup) -> MuscleBuilder<GroupId, N> {
            MuscleBuilder {
                group_id: GroupId(group.id),
                name: self.name,
                parent_id: self.parent_id,
                simple_name: self.simple_name,
                description: self.description,
                image_source: self.image_source,
            }
        }

        pub fn name(self, name: impl Into<String>) -> MuscleBuilder<G, Name> {
            MuscleBuilder {
                group_id: self.group_id,
                name: Name(name.into()),
                parent_id: self.parent_id,
                simple_name: self.simple_name,
                description: self.description,
                image_source: self.image_source,
            }
        }

        pub fn parent_id(mut self, id: Option<i16>) -> Self {
            self.parent_id = id;
            self
        }

        pub fn parent(mut self, parent: Option<&Muscle>) -> Self {
            self.parent_id = parent.map(|p| p.id.clone());
            self
        }

        pub fn simple_name(mut self, name: Option<impl Into<String>>) -> Self {
            self.simple_name = name.map(|n| n.into());
            self
        }

        pub fn description(mut self, description: Option<impl Into<String>>) -> Self {
            self.description = description.map(|d| d.into());
            self
        }

        pub fn image_source(mut self, source: Option<impl Into<String>>) -> Self {
            self.image_source = source.map(|s| s.into());
            self
        }
    }

    impl MuscleBuilder<GroupId, Name> {
        pub async fn create(self, database: &DatabaseManager) -> Result<Muscle> {
            let mut model = sqlx::query_as::<_, Muscle>(format!(
                "INSERT INTO {} (group_id, parent_id, name, simple_name, description, image_source) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
                Muscle::TABLE_NAME,
            ).as_str())
                .bind(self.group_id.0)
                .bind(self.parent_id)
                .bind(self.name.0)
                .bind(self.simple_name)
                .bind(self.description)
                .bind(self.image_source)
                .fetch_one(database.connection())
                .await?;

            Ok(model)
        }
    }
}

use builder::*;

#[async_trait]
impl Model for Muscle {
    const MODEL_NAME: &'static str = "Muscle";
    const TABLE_NAME: &'static str = "muscles";

    type PrimaryKey = i16;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}

impl HasRouteKey for Muscle {
    const ROUTE_KEY: &'static str = "ulid";
    type RouteKey = String;

    fn route_key(&self) -> Self::RouteKey {
        self.ulid.clone()
    }
}

impl Muscle {
    pub fn new() -> MuscleBuilder<NoGroupId, NoName> {
        MuscleBuilder::new()
    }

    // region Relationships

    pub async fn links(&self, database: &DatabaseManager) -> Result<Vec<Link>> {
        Link::muscle_links(self.id, database).await
    }

    pub async fn muscle_group(&self, database: &DatabaseManager) -> Result<MuscleGroup> {
        let group = MuscleGroup::find_by_pk(self.group_id, database).await?;

        Ok(group)
    }

    pub async fn parent(&self, database: &DatabaseManager) -> Result<Option<Self>> {
        if self.parent_id.is_none() {
            return Ok(None);
        }

        Ok(Self::find_by_pk(self.parent_id.unwrap(), database).await.ok())
    }

    // endregion

    // region Instance Methods

    pub async fn save(&mut self, database: &DatabaseManager) -> Result<()> {
        let model = sqlx::query_as::<_, Self>(format!(
            "UPDATE {} SET (group_id, parent_id, name, simple_name, description, image_source, updated_at) = ($1, $2, $3, $4, $5, $6, $7) WHERE {} = {} RETURNING *",
            Self::TABLE_NAME, Self::PRIMARY_KEY, &self.primary_key(),
        ).as_str())
            .bind(self.group_id)
            .bind(self.parent_id)
            .bind(self.name.clone())
            .bind(self.simple_name.clone())
            .bind(self.description.clone())
            .bind(self.image_source.clone())
            .bind(chrono::Utc::now())
            .fetch_one(database.connection())
            .await?;

        self.group_id = model.group_id;
        self.parent_id = model.parent_id;
        self.name = model.name;
        self.simple_name = model.simple_name;
        self.description = model.description;
        self.image_source = model.image_source;
        self.updated_at = model.updated_at;

        Ok(())
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::Muscle;
    use crate::models::MuscleGroup;
    use crate::prelude::*;

    #[sqlx::test]
    async fn create_muscle_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let group = MuscleGroup::mocked(&database).await?;

        let count = Muscle::count(&database).await?;

        let muscle = Muscle::new()
            .group(&group)
            .name("My Muscle")
            .simple_name(Some("My simple name"))
            .description(Some("My description"))
            .image_source(Some("Image source"))
            .create(&database)
            .await?;

        assert_eq!(count + 1, Muscle::count(&database).await?);
        assert_eq!(group.id, muscle.group_id);
        assert_eq!("My Muscle", muscle.name);
        assert_some_eq("My simple name", muscle.simple_name);
        assert_some_eq("My description", muscle.description);
        assert_some_eq("Image source", muscle.image_source);

        Ok(())
    }

    #[sqlx::test]
    async fn create_muscle_with_parent_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let group = MuscleGroup::mocked(&database).await?;
        let parent = Muscle::mocked(&database).await?;

        let count = Muscle::count(&database).await?;

        let muscle = Muscle::new()
            .parent(Some(&parent))
            .group(&group)
            .name("My Muscle")
            .simple_name(Some("My simple name"))
            .description(Some("My description"))
            .image_source(Some("Image source"))
            .create(&database)
            .await?;

        assert_eq!(count + 1, Muscle::count(&database).await?);
        assert_some_eq(parent.id, muscle.parent_id);
        assert_eq!(group.id, muscle.group_id);
        assert_eq!("My Muscle", muscle.name);
        assert_some_eq("My simple name", muscle.simple_name);
        assert_some_eq("My description", muscle.description);
        assert_some_eq("Image source", muscle.image_source);

        Ok(())
    }

    #[sqlx::test]
    async fn cannot_create_muscle_with_duplicate_name(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let parent = Muscle::mocked(&database).await?;

        let result = Muscle::fake()
            .name(parent.name)
            .create(&database)
            .await;

        assert!(result.is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn edit_muscle_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let parent = Muscle::mocked(&database).await?;
        let group = MuscleGroup::mocked(&database).await?;

        let another_parent = Muscle::mocked(&database).await?;
        let another_group = MuscleGroup::mocked(&database).await?;

        let mut muscle = Muscle::fake()
            .parent(Some(&parent))
            .group(&group)
            .create(&database)
            .await?;

        muscle.group_id = another_group.id;
        muscle.parent_id = Some(another_parent.id);
        muscle.name = "Another muscle".to_string();
        muscle.simple_name = Some("Another simple name".to_string());
        muscle.description = Some("Another description".to_string());
        muscle.image_source = Some("Another source".to_string());

        muscle.save(&database).await?;

        assert_eq!(another_group.id, muscle.group_id);
        assert_some_eq(another_parent.id, muscle.parent_id);
        assert_eq!("Another muscle", muscle.name);
        assert_some_eq("Another simple name", muscle.simple_name);
        assert_some_eq("Another description", muscle.description);
        assert_some_eq("Another source", muscle.image_source);

        Ok(())
    }
}
