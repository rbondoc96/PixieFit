use super::{Error, Model, Result};
use crate::prelude::*;
use crate::{
    enums::{LinkFormat, LinkType, Table},
    sys::DatabaseManager,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use ulid::Ulid;

#[derive(Clone, Debug, FromRow)]
pub struct LinkRecord {
    id: i64,
    model_name: String,
    model_id: i64,
    ulid: String,
    #[sqlx(rename = "type")]
    link_type: LinkType,
    format: LinkFormat,
    label: String,
    description: Option<String>,
    src: String,
    created_at: ISO8601DateTimeUTC,
    updated_at: ISO8601DateTimeUTC,
}

#[derive(Deserialize)]
pub struct CreateLinkData {
    pub model_name: Table,
    pub model_id: i64,
    #[serde(rename = "type")]
    pub link_type: LinkType,
    pub format: LinkFormat,
    pub label: String,
    pub description: Option<String>,
    pub src: String,
}

pub struct Link {
    database: DatabaseManager,
    data: LinkRecord,
}

#[async_trait]
impl Model for Link {
    const MODEL_NAME: &'static str = "Link";
    const TABLE_NAME: &'static str = "links";
    type Attributes = LinkRecord;

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

impl Link {
    pub fn route_key(&self) -> String {
        self.data.ulid.clone()
    }

    pub fn link_type(&self) -> LinkType {
        self.data.link_type.clone()
    }

    pub fn format(&self) -> LinkFormat {
        self.data.format.clone()
    }

    pub fn label(&self) -> String {
        self.data.label.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.data.description.clone()
    }

    pub fn src(&self) -> String {
        self.data.src.clone()
    }

    pub fn created_at(&self) -> ISO8601DateTimeUTC {
        self.data.created_at
    }

    pub fn updated_at(&self) -> ISO8601DateTimeUTC {
        self.data.updated_at
    }

    // region Relationships

    pub async fn muscle_links(id: i64, database: &DatabaseManager) -> Result<Vec<Link>> {
        let connection = database.connection();

        let links = sqlx::query_as::<_, LinkRecord>(
            "SELECT * FROM links WHERE model_name = $1 AND model_id = $2",
        )
        .bind("muscles".to_string())
        .bind(id)
        .fetch_all(connection)
        .await
        .map_err(|err| Error::ModelNotFound {
            model: "links",
            search_key: format!("{}:{}", "muscles", id),
            search_value: id.to_string(),
        })?;

        Ok(links
            .into_iter()
            .map(|link| Link {
                database: database.clone(),
                data: link,
            })
            .collect())
    }

    // endregion

    pub async fn create(
        attributes: CreateLinkData,
        database: &DatabaseManager,
    ) -> Result<Link> {
        let mut transaction = database.connection().begin().await?;

        let ulid = Ulid::new();

        let link = sqlx::query_as::<_, LinkRecord>(
            "INSERT INTO links (ulid, model_name, model_id, type, format, label, description, src) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        )
        .bind(ulid.to_string().to_lowercase())
        .bind(attributes.model_name)
        .bind(attributes.model_id)
        .bind(attributes.link_type)
        .bind(attributes.format)
        .bind(attributes.label)
        .bind(attributes.description)
        .bind(attributes.src)
        .fetch_one(&mut *transaction)
        .await;

        match link {
            Ok(link) => {
                transaction.commit().await?;
                Ok(Link {
                    database: database.clone(),
                    data: link,
                })
            }
            Err(err) => {
                transaction.rollback().await?;
                Err(err.into())
            }
        }
    }
}
