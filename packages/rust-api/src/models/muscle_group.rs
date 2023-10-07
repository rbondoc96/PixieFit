use super::{Error, Link, Model};
use crate::{sys::DatabaseManager, types::ISO8601DateTimeUTC};
use async_trait::async_trait;
use sqlx::{FromRow, PgPool};

#[derive(Clone, Debug, FromRow)]
pub struct MuscleGroupRecord {
    id: i32,
    name: String,
    image_source: Option<String>,
    created_at: ISO8601DateTimeUTC,
    updated_at: ISO8601DateTimeUTC,
}

pub struct MuscleGroup {
    database: DatabaseManager,
    data: MuscleGroupRecord,
}

#[async_trait]
impl Model for MuscleGroup {
    const TABLE_NAME: &'static str = "muscle_groups";
    type Attributes = MuscleGroupRecord;

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

impl MuscleGroup {
    pub fn id(&self) -> i32 {
        self.data.id
    }

    pub fn name(&self) -> String {
        self.data.name.clone()
    }

    pub fn image_source(&self) -> Option<String> {
        self.data.image_source.clone()
    }

    pub fn created_at(&self) -> ISO8601DateTimeUTC {
        self.data.created_at
    }

    pub fn updated_at(&self) -> ISO8601DateTimeUTC {
        self.data.updated_at
    }
}
