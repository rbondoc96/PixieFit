use super::{Error, Exercise, Result};
use async_trait::async_trait;
use database::{DatabaseManager, HasRouteKey, Model, SqlxAction};
use sqlx::{FromRow, PgPool};

#[cfg(test)]
pub(crate) use builder::*;

#[derive(Clone, Debug, FromRow)]
pub struct ExerciseEquipment {
    pub id: i16,
    pub name: String,
}

mod builder {
    use super::{ExerciseEquipment, Result};
    use database::{DatabaseManager, Model};

    // region Type States

    #[derive(Default)]
    pub struct NoName;
    #[derive(Default)]
    pub struct Name(String);

    // endregion

    #[derive(Default)]
    pub struct ExerciseEquipmentBuilder<N> {
        name: N,
    }

    impl ExerciseEquipmentBuilder<NoName> {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl<N> ExerciseEquipmentBuilder<N> {
        pub fn name(mut self, name: impl Into<String>) -> ExerciseEquipmentBuilder<Name> {
            ExerciseEquipmentBuilder {
                name: Name(name.into())
            }
        }
    }

    impl ExerciseEquipmentBuilder<Name> {
        pub async fn create(self, database: &DatabaseManager) -> Result<ExerciseEquipment> {
            let model = sqlx::query_as::<_, ExerciseEquipment>(format!(
                "INSERT INTO {} (name) VALUES ($1) RETURNING *",
                ExerciseEquipment::TABLE_NAME,
            ).as_str())
                .bind(self.name.0)
                .fetch_one(database.connection())
                .await?;

            Ok(model)
        }
    }
}

use builder::*;

#[async_trait]
impl Model for ExerciseEquipment {
    const MODEL_NAME: &'static str = "ExerciseEquipment";
    const TABLE_NAME: &'static str = "exercise_equipment";

    type PrimaryKey = i16;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}

impl HasRouteKey for ExerciseEquipment {
    const ROUTE_KEY: &'static str = "id";
    type RouteKey = i16;

    fn route_key(&self) -> Self::RouteKey {
        self.id
    }
}

impl ExerciseEquipment {
    pub fn new() -> ExerciseEquipmentBuilder<NoName> {
        ExerciseEquipmentBuilder::new()
    }

    // region Relationships

    pub async fn exercises(&self, database: &DatabaseManager) -> Result<Vec<Exercise>> {
        let results = Exercise::query()
            .select(&["*"])
            .and_where("equipment_id", "=", self.id)
            .all(database.connection())
            .await?;

        Ok(results)
    }

    // endregion

    // region Instance Methods

    pub async fn save(&mut self, database: &DatabaseManager) -> Result<()> {
        let model = sqlx::query_as::<_, Self>(format!(
            "UPDATE {} SET (name) = ($1) WHERE {} = {} RETURNING *",
            Self::TABLE_NAME, Self::PRIMARY_KEY, &self.primary_key(),
        ).as_str())
            .bind(self.name.clone())
            .fetch_one(database.connection())
            .await?;

        self.name = model.name;

        Ok(())
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::ExerciseEquipment;
    use crate::prelude::*;

    #[sqlx::test]
    async fn create_exercise_equipment_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);

        let count = ExerciseEquipment::count(&database).await?;

        let equipment = ExerciseEquipment::new()
            .name("My Equipment")
            .create(&database)
            .await?;

        assert_eq!("My Equipment", equipment.name);
        assert_eq!(count + 1, ExerciseEquipment::count(&database).await?);

        Ok(())
    }

    async fn edit_exercise_equipment_success(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let mut equipment = ExerciseEquipment::mocked(&database).await?;

        equipment.name = "Another name".to_string();

        equipment.save(&database);

        assert_eq!("Another name", equipment.name);

        Ok(())
    }

    #[sqlx::test]
    async fn cannot_create_equipment_with_duplicate_name(pool: PgPool) -> Result<()> {
        let database = DatabaseManager::from_pool(pool);
        let equipment = ExerciseEquipment::mocked(&database).await?;

        let result = ExerciseEquipment::fake()
            .name(equipment.name)
            .create(&database)
            .await;

        assert!(result.is_err());

        Ok(())
    }
}
