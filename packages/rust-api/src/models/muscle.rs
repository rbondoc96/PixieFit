use super::{Error, Link, Model};
use crate::{enums::MuscleGroup, sys::DatabaseManager, types::ISO8601DateTimeUTC};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Clone, Debug, FromRow)]
pub struct MuscleRecord {
    id: i64,
    parent_id: Option<i64>,
    muscle_group: MuscleGroup,
    name: String,
    simple_name: Option<String>,
    created_at: ISO8601DateTimeUTC,
    updated_at: ISO8601DateTimeUTC,
}

#[derive(Deserialize)]
pub struct CreateMuscleData {
    pub parent_id: Option<i64>,
    pub muscle_group: MuscleGroup,
    pub name: String,
    pub simple_name: Option<String>,
}

pub struct Muscle {
    database: DatabaseManager,
    data: MuscleRecord,
}

impl super::Model for Muscle {
    const TABLE_NAME: &'static str = "muscles";
    type Attributes = MuscleRecord;

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

impl Muscle {
    pub fn id(&self) -> i64 {
        self.data.id
    }

    pub fn parent_id(&self) -> Option<i64> {
        self.data.parent_id
    }

    pub fn muscle_group(&self) -> MuscleGroup {
        self.data.muscle_group.clone()
    }

    pub fn name(&self) -> String {
        self.data.name.clone()
    }

    pub fn simple_name(&self) -> Option<String> {
        self.data.simple_name.clone()
    }

    pub fn created_at(&self) -> ISO8601DateTimeUTC {
        self.data.created_at
    }

    pub fn updated_at(&self) -> ISO8601DateTimeUTC {
        self.data.updated_at
    }

    // region Relationships

    pub async fn links(&self) -> Result<Vec<Link>, Error> {
        Link::muscle_links(self.id(), &self.database).await
    }

    // endregion

    pub async fn create(
        attributes: CreateMuscleData,
        database: &DatabaseManager,
    ) -> Result<Muscle, sqlx::Error> {
        let mut transaction = database.connection().begin().await?;

        let muscle = sqlx::query_as::<_, MuscleRecord>(
            "INSERT INTO muscles (muscle_group, name, simple_name) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(attributes.muscle_group)
        .bind(attributes.name)
        .bind(attributes.simple_name)
        .fetch_one(&mut *transaction)
        .await;

        match muscle {
            Ok(muscle) => {
                transaction.commit().await?;
                Ok(Muscle {
                    database: database.clone(),
                    data: muscle,
                })
            }
            Err(err) => {
                transaction.rollback().await?;
                Err(err)
            }
        }
    }

    pub async fn find_by_id(id: i64, database: &DatabaseManager) -> Result<Muscle, Error> {
        super::base::find_by_id(id, database).await
    }

    pub async fn all(database: &DatabaseManager) -> Result<Vec<Muscle>, Error> {
        super::base::all(database).await
    }
}
