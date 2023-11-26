use super::{Error, Result};
use crate::prelude::*;
use crate::enums::{LinkFormat, LinkType, Table};
use async_trait::async_trait;
use database::{DatabaseManager, Model};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[cfg(test)]
pub use builder::*;

#[derive(Clone, Debug, FromRow)]
pub struct Link {
    pub id: i32,
    pub ulid: String,
    pub model_name: Table,
    pub model_id: i16,
    #[sqlx(rename = "type")]
    pub link_type: LinkType,
    pub format: LinkFormat,
    pub label: String,
    pub description: Option<String>,
    pub src: String,
    pub created_at: ISO8601DateTimeUTC,
    pub updated_at: ISO8601DateTimeUTC,
}

mod builder {
    use super::{Link, Result};
    use crate::enums::{LinkFormat, LinkType, Table};
    use database::{DatabaseManager, Model};

    // region Builder type states

    #[derive(Default)]
    pub struct NoModelData;
    #[derive(Default)]
    pub struct ModelData(Table, i16);

    #[derive(Default)]
    pub struct NoType;
    #[derive(Default)]
    pub struct Type(LinkType);

    #[derive(Default)]
    pub struct NoFormat;
    #[derive(Default)]
    pub struct Format(LinkFormat);

    #[derive(Default)]
    pub struct NoLabel;
    #[derive(Default)]
    pub struct Label(String);

    #[derive(Default)]
    pub struct NoSrc;
    #[derive(Default)]
    pub struct Src(String);

    // endregion

    #[derive(Default)]
    pub struct LinkBuilder<M, T, F, L, S> {
        model: M,
        link_type: T,
        format: F,
        label: L,
        src: S,
        description: Option<String>,
    }

    impl LinkBuilder<NoModelData, NoType, NoFormat, NoLabel, NoSrc> {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl<M, T, F, L, S> LinkBuilder<M, T, F, L, S> {
        pub fn model(self, model_name: Table, model_id: i16) -> LinkBuilder<ModelData, T, F, L, S> {
            LinkBuilder {
                model: ModelData(model_name, model_id),
                link_type: self.link_type,
                format: self.format,
                label: self.label,
                src: self.src,
                description: self.description,
            }
        }

        pub fn link_type(self, link_type: LinkType) -> LinkBuilder<M, Type, F, L, S> {
            LinkBuilder {
                model: self.model,
                link_type: Type(link_type),
                format: self.format,
                label: self.label,
                src: self.src,
                description: self.description,
            }
        }

        pub fn format(self, format: LinkFormat) -> LinkBuilder<M, T, Format, L, S> {
            LinkBuilder {
                model: self.model,
                link_type: self.link_type,
                format: Format(format),
                label: self.label,
                src: self.src,
                description: self.description,
            }
        }

        pub fn label(self, label: impl Into<String>) -> LinkBuilder<M, T, F, Label, S> {
            LinkBuilder {
                model: self.model,
                link_type: self.link_type,
                format: self.format,
                label: Label(label.into()),
                src: self.src,
                description: self.description,
            }
        }

        pub fn src(self, src: impl Into<String>) -> LinkBuilder<M, T, F, L, Src> {
            LinkBuilder {
                model: self.model,
                link_type: self.link_type,
                format: self.format,
                label: self.label,
                src: Src(src.into()),
                description: self.description,
            }
        }

        pub fn description(mut self, description: Option<impl Into<String>>) -> Self {
            self.description = description.map(|d| d.into());
            self
        }
    }

    impl LinkBuilder<ModelData, Type, Format, Label, Src> {
        pub async fn create(self, database: &DatabaseManager) -> Result<Link> {
            let model = sqlx::query_as::<_, Link>(format!(
                "INSERT INTO {} (model_name, model_id, type, format, label, description, src) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
                Link::TABLE_NAME,
            ).as_str())
                .bind(self.model.0)
                .bind(self.model.1)
                .bind(self.link_type.0)
                .bind(self.format.0)
                .bind(self.label.0)
                .bind(self.description)
                .bind(self.src.0)
                .fetch_one(database.connection())
                .await?;

            Ok(model)
        }
    }
}

use builder::*;

#[async_trait]
impl Model for Link {
    const MODEL_NAME: &'static str = "Link";
    const TABLE_NAME: &'static str = "links";

    type PrimaryKey = i32;
    fn pk(&self) -> Self::PrimaryKey {
        self.id
    }

    const ROUTE_KEY: &'static str = "ulid";
    type RouteKey = String;
    fn rk(&self) -> Self::RouteKey {
        self.ulid.clone()
    }
}

impl Link {
    pub fn new() -> LinkBuilder<NoModelData, NoType, NoFormat, NoLabel, NoSrc> {
        LinkBuilder::new()
    }

    // region Relationships

    async fn model_links(id: i16, model_name: Table, database: &DatabaseManager) -> Result<Vec<Link>> {
        let links = Self::query()
            .select(&["*"])
            .and_where("model_name", "=", model_name.clone())
            .and_where("model_id", "=", id)
            .all::<&PgPool, Self>(database.connection())
            .await?;

        Ok(links)
    }

    pub async fn muscle_links(id: i16, database: &DatabaseManager) -> Result<Vec<Link>> {
        Self::model_links(id, Table::Muscles, database).await
    }

    // endregion

    // region Instance Methods

    pub async fn save(&mut self, database: &DatabaseManager) -> Result<()> {
        let model = sqlx::query_as::<_, Self>(format!(
            "UPDATE {} SET (model_name, model_id, type, format, label, description, src, updated_at) = ($1, $2, $3, $4, $5, $6, $7, $8) WHERE {} = {} RETURNING *",
            Self::TABLE_NAME, Self::PRIMARY_KEY, &self.pk(),
        ).as_str())
            .bind(self.model_name.clone())
            .bind(self.model_id.clone())
            .bind(self.link_type.clone())
            .bind(self.format.clone())
            .bind(self.label.clone())
            .bind(self.description.clone())
            .bind(self.src.clone())
            .bind(chrono::Utc::now())
            .fetch_one(database.connection())
            .await?;

        self.link_type = model.link_type;
        self.format = model.format;
        self.label = model.label;
        self.description = model.description;
        self.src = model.src;
        self.updated_at = model.updated_at;

        Ok(())
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::Link;
    use crate::prelude::*;
    use crate::enums::{LinkFormat, LinkType, Table};
    use crate::models::{Exercise, Muscle, Profile};

    #[sqlx::test]
    async fn create_link_success(pool: PgPool) -> Result<()> {
        // Arrange
        let database = DatabaseManager::from_pool(pool);
        let model = Exercise::mocked(&database).await?;

        let count = Link::count(&database).await?;

        // Act
        let link = Link::new()
            .link_type(LinkType::Image)
            .format(LinkFormat::Png)
            .label("My label")
            .description(Some("My description"))
            .src("My source")
            .model(Table::Exercises, model.id)
            .create(&database)
            .await?;

        // Assert
        assert_eq!(count + 1, Link::count(&database).await?);
        assert_eq!(LinkType::Image, link.link_type);
        assert_eq!(LinkFormat::Png, link.format);
        assert_eq!("My label", link.label);
        assert_some_eq("My description", link.description);
        assert_eq!("My source", link.src);
        assert_eq!(Table::Exercises, link.model_name);
        assert_eq!(model.id, link.model_id);

        Ok(())
    }

    #[sqlx::test]
    async fn edit_link_success(pool: PgPool) -> Result<()> {
        // Arrange
        let database = DatabaseManager::from_pool(pool);
        let model = Exercise::mocked(&database).await?;
        let mut link = Link::fake()
            .model(Table::Muscles, Muscle::mocked(&database).await?.id)
            .link_type(LinkType::Image)
            .format(LinkFormat::Png)
            .label("My Label")
            .description(Some("My description"))
            .src("My src")
            .create(&database)
            .await?;

        // Act
        link.model_name = Table::Exercises;
        link.model_id = model.id;
        link.link_type = LinkType::Webpage;
        link.format = LinkFormat::Html;
        link.label = "A new label".to_string();
        link.description = Some("A new description".to_string());
        link.src = "A new source".to_string();

        link.save(&database).await?;

        // Assert
        assert_eq!(Table::Exercises, link.model_name);
        assert_eq!(model.id, link.model_id);
        assert_eq!(LinkType::Webpage, link.link_type);
        assert_eq!(LinkFormat::Html, link.format);
        assert_eq!("A new label", link.label);
        assert_some_eq("A new description", link.description);
        assert_eq!("A new source", link.src);

        Ok(())
    }
}
