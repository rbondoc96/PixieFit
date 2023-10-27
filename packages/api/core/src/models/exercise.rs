use super::{Error, ExerciseEquipment, ExerciseMuscleMap, Muscle, MuscleGroup, Result};
use crate::prelude::*;
use crate::enums::{ExerciseForce, ExerciseMechanic, ExerciseMuscleTarget, ExerciseType, Measurement};
use async_trait::async_trait;
use database::{DatabaseManager, Model};
use serde::Deserialize;
use sqlx::{postgres::PgPool, FromRow};

#[cfg(test)]
pub(crate) use builder::*;

#[derive(Clone, Debug, FromRow)]
pub struct Exercise {
    pub id: i16,
    pub ulid: String,
    pub external_id: Option<i16>,
    #[sqlx(rename = "type")]
    pub exercise_type: ExerciseType,
    pub target_muscle_group_id: Option<i16>,
    pub equipment_id: Option<i16>,
    pub name: String,
    pub name_alternative: Option<String>,
    pub description: Option<String>,
    pub mechanic: Option<ExerciseMechanic>,
    pub force: Option<ExerciseForce>,
    pub measurement: Option<Measurement>,
    pub created_at: ISO8601DateTimeUTC,
    pub updated_at: ISO8601DateTimeUTC,
}

mod builder {
    use super::{Error, Exercise, ExerciseEquipment, MuscleGroup, Result};
    use crate::enums::{ExerciseForce, ExerciseMechanic, ExerciseType, Measurement};
    use database::{DatabaseManager, Model};

    // region Type States

    #[derive(Default)]
    pub struct NoType;
    #[derive(Default)]
    pub struct Type(ExerciseType);

    #[derive(Default)]
    pub struct NoName;
    #[derive(Default)]
    pub struct Name(String);

    // endregion

    // region Builder

    #[derive(Default)]
    pub struct ExerciseBuilder<T, N> {
        exercise_type: T,
        name: N,
        name_alternative: Option<String>,
        external_id: Option<i16>,
        target_muscle_group_id: Option<i16>,
        equipment_id: Option<i16>,
        mechanic: Option<ExerciseMechanic>,
        force: Option<ExerciseForce>,
        measurement: Option<Measurement>,
        description: Option<String>,
    }

    impl ExerciseBuilder<NoType, NoName> {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl<T, N> ExerciseBuilder<T, N> {
        pub fn name(self, name: impl Into<String>) -> ExerciseBuilder<T, Name> {
            ExerciseBuilder {
                exercise_type: self.exercise_type,
                name: Name(name.into()),
                name_alternative: self.name_alternative,
                external_id: self.external_id,
                target_muscle_group_id: self.target_muscle_group_id,
                equipment_id: self.equipment_id,
                mechanic: self.mechanic,
                force: self.force,
                measurement: self.measurement,
                description: self.description,
            }
        }

        pub fn exercise_type(self, exercise_type: ExerciseType) -> ExerciseBuilder<Type, N> {
            ExerciseBuilder {
                exercise_type: Type(exercise_type),
                name: self.name,
                name_alternative: self.name_alternative,
                external_id: self.external_id,
                target_muscle_group_id: self.target_muscle_group_id,
                equipment_id: self.equipment_id,
                mechanic: self.mechanic,
                force: self.force,
                measurement: self.measurement,
                description: self.description,
            }
        }

        pub fn external_id(mut self, id: Option<i16>) -> Self {
            self.external_id = id;
            self
        }

        pub fn target_muscle_group_id(mut self, id: Option<i16>) -> Self {
            self.target_muscle_group_id = id;
            self
        }

        pub fn target_muscle_group(mut self, group: &MuscleGroup) -> Self {
            self.target_muscle_group_id = Some(group.id);
            self
        }

        pub fn name_alternative(mut self, name: Option<String>) -> Self {
            self.name_alternative = name;
            self
        }

        pub fn description(mut self, description: Option<String>) -> Self {
            self.description = description;
            self
        }

        pub fn equipment_id(mut self, id: Option<i16>) -> Self {
            self.equipment_id = id;
            self
        }

        pub fn equipment(mut self, equipment: &ExerciseEquipment) -> Self {
            self.equipment_id = Some(equipment.id);
            self
        }

        pub fn mechanic(mut self, mechanic: Option<ExerciseMechanic>) -> Self {
            self.mechanic = mechanic;
            self
        }

        pub fn force(mut self, force: Option<ExerciseForce>) -> Self {
            self.force = force;
            self
        }

        pub fn measurement(mut self, measurement: Option<Measurement>) -> Self {
            self.measurement = measurement;
            self
        }
    }

    impl ExerciseBuilder<Type, Name> {
        pub async fn create(self, database: &DatabaseManager) -> Result<Exercise> {
            let model = sqlx::query_as::<_, Exercise>(format!(
                "INSERT INTO {} (external_id, type, target_muscle_group_id, name, name_alternative, description, equipment_id, mechanic, force, measurement) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *",
                Exercise::TABLE_NAME,
            ).as_str())
                .bind(self.external_id)
                .bind(self.exercise_type.0)
                .bind(self.target_muscle_group_id)
                .bind(self.name.0)
                .bind(self.name_alternative)
                .bind(self.description)
                .bind(self.equipment_id)
                .bind(self.mechanic)
                .bind(self.force)
                .bind(self.measurement)
                .fetch_one(database.connection())
                .await?;

            Ok(model)
        }
    }

    // endregion
}

use builder::*;

#[async_trait]
impl Model for Exercise {
    const MODEL_NAME: &'static str = "Exercise";
    const TABLE_NAME: &'static str = "exercises";

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

impl Exercise {
    pub fn new() -> ExerciseBuilder<NoType, NoName> {
        ExerciseBuilder::new()
    }

    // region Relationships

    pub async fn equipment(&self, database: &DatabaseManager) -> Result<Option<ExerciseEquipment>> {
        let equipment_id = self.equipment_id;

        if equipment_id.is_none() {
            return Ok(None);
        }

        let equipment = ExerciseEquipment::find(
            "name",
            equipment_id.unwrap(),
            database,
        ).await?;

        Ok(Some(equipment))
    }

    pub async fn target_muscle_group(&self, database: &DatabaseManager) -> Result<Option<MuscleGroup>> {
        let group_id = self.target_muscle_group_id;

        if group_id.is_none() {
            return Ok(None);
        }

        let group = MuscleGroup::find_by_pk(
            group_id.unwrap().into(),
            database,
        )
        .await?;

        Ok(Some(group))
    }

    async fn muscles(&self, target: ExerciseMuscleTarget, database: &DatabaseManager) -> Result<Vec<Muscle>> {
        let results = sqlx::query_as::<_, Muscle>(
            "SELECT muscles.* FROM muscles INNER JOIN exercises_muscles ON muscles.id = exercises_muscles.muscle_id WHERE exercises_muscles.exercise_id = $1 AND exercises_muscles.target = $2",
        )
        .bind(self.id)
        .bind(target)
        .fetch_all(database.connection())
        .await?;

        Ok(results)
    }

    pub async fn primary_muscles(&self, database: &DatabaseManager) -> Result<Vec<Muscle>> {
        self.muscles(ExerciseMuscleTarget::Primary, database).await
    }

    pub async fn secondary_muscles(&self, database: &DatabaseManager) -> Result<Vec<Muscle>> {
        self.muscles(ExerciseMuscleTarget::Secondary, database).await
    }

    pub async fn tertiary_muscles(&self, database: &DatabaseManager) -> Result<Vec<Muscle>> {
        self.muscles(ExerciseMuscleTarget::Tertiary, database).await
    }

    // endregion

    // region Instance Methods

    pub async fn save(&mut self, database: &DatabaseManager) -> Result<()> {
        let model = sqlx::query_as::<_, Self>(format!(
            "UPDATE {} SET (external_id, type, target_muscle_group_id, name, name_alternative, description, equipment_id, mechanic, force, measurement, updated_at) = ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) WHERE {} = {} RETURNING *",
            Self::TABLE_NAME, Self::PRIMARY_KEY, &self.pk(),
        ).as_str())
            .bind(self.external_id)
            .bind(self.exercise_type.clone())
            .bind(self.target_muscle_group_id)
            .bind(self.name.clone())
            .bind(self.name_alternative.clone())
            .bind(self.description.clone())
            .bind(self.equipment_id)
            .bind(self.mechanic.clone())
            .bind(self.force.clone())
            .bind(self.measurement.clone())
            .bind(chrono::Utc::now())
            .fetch_one(database.connection())
            .await?;

        self.external_id = model.external_id;
        self.exercise_type = model.exercise_type;
        self.target_muscle_group_id = model.target_muscle_group_id;
        self.name = model.name;
        self.name_alternative = model.name_alternative;
        self.description = model.description;
        self.equipment_id = model.equipment_id;
        self.mechanic = model.mechanic;
        self.force = model.force;
        self.measurement = model.measurement;
        self.updated_at = model.updated_at;

        Ok(())
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::Exercise;
    use crate::enums::{ExerciseForce, ExerciseMechanic, ExerciseType, Measurement};
    use crate::models::{ExerciseEquipment, MuscleGroup};
    use crate::prelude::*;

    #[sqlx::test]
    async fn create_exercise_success(pool: PgPool) -> Result<()> {
        // Arrange
        let database = DatabaseManager::from_pool(pool);
        let group = MuscleGroup::mocked(&database).await?;
        let equipment = ExerciseEquipment::mocked(&database).await?;
        let count = Exercise::count(&database).await?;

        // Act
        let exercise = Exercise::new()
            .exercise_type(ExerciseType::Strength)
            .target_muscle_group_id(Some(group.id))
            .equipment_id(Some(equipment.id))
            .name("My Exercise")
            .name_alternative(Some("My other exercise".to_string()))
            .description(Some("My description".to_string()))
            .force(Some(ExerciseForce::Hold))
            .mechanic(Some(ExerciseMechanic::Compound))
            .measurement(Some(Measurement::WeightedRepetitions))
            .create(&database)
            .await?;

        // Assert
        assert!(exercise.external_id.is_none());
        assert_eq!(ExerciseType::Strength, exercise.exercise_type);
        assert_some_eq(group.id, exercise.target_muscle_group_id);
        assert_some_eq(equipment.id, exercise.equipment_id);
        assert_eq!("My Exercise", exercise.name);
        assert_eq!(count + 1, Exercise::count(&database).await?);

        Ok(())
    }

    #[sqlx::test]
    async fn edit_exercise_success(pool: PgPool) -> Result<()> {
        // Arrange
        let database = DatabaseManager::from_pool(pool);
        let mut exercise = Exercise::mocked(&database).await?;

        // Act
        exercise.exercise_type = ExerciseType::Class;
        exercise.name = "New Exercise".to_string();
        exercise.name_alternative = Some("New alternative".to_string());
        exercise.description = Some("New description".to_string());
        exercise.force = Some(ExerciseForce::Pull);
        exercise.mechanic = Some(ExerciseMechanic::Isolation);
        exercise.measurement = Some(Measurement::Duration);

        exercise.save(&database).await?;

        // Assert
        assert_eq!(ExerciseType::Class, exercise.exercise_type);
        assert_eq!("New Exercise", exercise.name);
        assert_some_eq("New alternative", exercise.name_alternative);
        assert_some_eq("New description", exercise.description);
        assert_some_eq(ExerciseForce::Pull, exercise.force);
        assert_some_eq(ExerciseMechanic::Isolation, exercise.mechanic);
        assert_some_eq(Measurement::Duration, exercise.measurement);

        Ok(())
    }

    #[sqlx::test]
    async fn cannot_create_exercise_with_duplicate_name(pool: PgPool) -> Result<()> {
        // Arrange
        let database = DatabaseManager::from_pool(pool);
        let exercise = Exercise::mocked(&database).await?;

        // Act
        let result = Exercise::fake()
            .name(exercise.name)
            .create(&database)
            .await;

        // Assert
        assert!(result.is_err());

        Ok(())
    }
}
