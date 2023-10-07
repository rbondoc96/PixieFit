use super::{Error, Link, Model, MuscleGroup};
use crate::{sys::DatabaseManager, types::ISO8601DateTimeUTC};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Clone, Debug, FromRow)]
pub struct MuscleRecord {
    id: i64,
    ulid: String,
    group_id: i32,
    parent_id: Option<i64>,
    name: String,
    simple_name: Option<String>,
    description: Option<String>,
    image_source: Option<String>,
    created_at: ISO8601DateTimeUTC,
    updated_at: ISO8601DateTimeUTC,
}

#[derive(Deserialize)]
pub struct CreateMuscleData {
    pub group_id: i32,
    pub parent_id: Option<i64>,
    pub name: String,
    pub simple_name: Option<String>,
    pub description: Option<String>,
    pub image_source: Option<String>,
}

#[derive(Debug)]
pub struct Muscle {
    database: DatabaseManager,
    data: MuscleRecord,
}

#[async_trait]
impl Model for Muscle {
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
    pub fn route_key(&self) -> String {
        self.data.ulid.clone()
    }

    pub fn id(&self) -> i64 {
        self.data.id
    }

    pub fn ulid(&self) -> String {
        self.data.ulid.clone()
    }

    pub fn group_id(&self) -> i32 {
        self.data.group_id
    }

    pub fn parent_id(&self) -> Option<i64> {
        self.data.parent_id
    }

    pub fn name(&self) -> String {
        self.data.name.clone()
    }

    pub fn simple_name(&self) -> Option<String> {
        self.data.simple_name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.data.description.clone()
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

    // region Relationships

    pub async fn links(&self) -> Result<Vec<Link>, Error> {
        Link::muscle_links(self.id(), &self.database).await
    }

    pub async fn muscle_group(&self) -> Result<MuscleGroup, Error> {
        MuscleGroup::find_by_id(self.group_id().into(), &self.database).await
    }

    pub async fn parent(&self) -> Option<Self> {
        match self.parent_id() {
            Some(id) => Muscle::find_by_id(id, &self.database).await.ok(),
            None => None,
        }
    }

    // endregion

    pub async fn create(
        attributes: CreateMuscleData,
        database: &DatabaseManager,
    ) -> Result<Muscle, sqlx::Error> {
        let mut transaction = database.connection().begin().await?;

        let muscle = sqlx::query_as::<_, MuscleRecord>(
            "INSERT INTO muscles (group_id, parent_id, name, simple_name, description, image_source) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(attributes.group_id)
        .bind(attributes.parent_id)
        .bind(attributes.name)
        .bind(attributes.simple_name)
        .bind(attributes.description)
        .bind(attributes.image_source)
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

    pub async fn find_by_ulid(ulid: String, database: &DatabaseManager) -> Result<Muscle, sqlx::Error> {
        let muscle = sqlx::query_as::<_, MuscleRecord>(
            "SELECT * FROM muscles WHERE ulid = $1",
        )
        .bind(ulid)
        .fetch_one(database.connection())
        .await?;

        Ok(Muscle {
            database: database.clone(),
            data: muscle,
        })
    }
}
