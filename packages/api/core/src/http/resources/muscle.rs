use super::{LinkResource, ModelResource, ResourceResult};
use crate::models::{Link, Muscle};
use async_trait::async_trait;
use database::{DatabaseManager, HasRouteKey, Model};
use serde::Serialize;

#[derive(Serialize)]
pub struct MuscleResource {
    id: String,
    muscle_group: String,
    name: String,
    simple_name: Option<String>,
    description: Option<String>,
    image_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<Box<MuscleResource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Vec<LinkResource>>,
}

#[async_trait]
impl ModelResource for MuscleResource {
    type Model = Muscle;

    async fn default(muscle: Muscle, database: &DatabaseManager) -> ResourceResult<Self> {
        let links = muscle.links(database).await?;

        let group = muscle.muscle_group(database).await?;
        let parent = match muscle.parent(database).await? {
            Some(parent) => Some(Box::new(MuscleResource::simple(parent, database).await?)),
            None => None,
        };
        let links = Some(LinkResource::list(links, database).await?);

        Ok(Self {
            id: muscle.route_key(),
            muscle_group: group.name,
            name: muscle.name,
            simple_name: muscle.simple_name,
            description: muscle.description,
            image_source: muscle.image_source,
            parent,
            links,
        })
    }

    async fn simple(muscle: Muscle, database: &DatabaseManager) -> ResourceResult<Self> {
        let group = muscle.muscle_group(database).await?;

        Ok(Self {
            id: muscle.route_key(),
            muscle_group: group.name,
            name: muscle.name,
            simple_name: muscle.simple_name,
            description: muscle.description,
            image_source: muscle.image_source,
            parent: None,
            links: None,
        })
    }
}
