use super::{Error, Link, Model};
use crate::sys::DatabaseManager;
use async_trait::async_trait;
use sqlx::{FromRow, PgPool};

#[derive(Clone, Debug, FromRow)]
pub struct ExerciseEquipmentRecord {
    name: String,
}

pub struct ExerciseEquipment {
    database: DatabaseManager,
    data: ExerciseEquipmentRecord,
}

#[async_trait]
impl Model for ExerciseEquipment {
    const TABLE_NAME: &'static str = "exercise_equipment";
    type Attributes = ExerciseEquipmentRecord;

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

impl ExerciseEquipment {
    pub fn name(&self) -> String {
        self.data.name.clone()
    }
}
