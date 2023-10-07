use super::{LinkResource, ModelResource};
use crate::{
    models::{Link, Muscle},
};
use async_trait::async_trait;
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

    async fn default(muscle: Muscle) -> Self {
        let links = match muscle.links().await {
            Ok(links) => links,
            Err(_) => vec![],
        };

        let group = muscle.muscle_group().await.unwrap();
        let parent = muscle.parent().await;

        Self {
            id: muscle.ulid(),
            muscle_group: group.name(),
            name: muscle.name(),
            simple_name: muscle.simple_name(),
            description: muscle.description(),
            image_source: muscle.image_source(),
            parent: match parent {
                Some(parent) => Some(
                    Box::new(MuscleResource::simple(parent).await)
                ),
                None => None,
            },
            links: Some(LinkResource::list(links).await),
        }
    }

    async fn simple(muscle: Muscle) -> Self {
        let group = muscle.muscle_group().await.unwrap();

        Self {
            id: muscle.ulid(),
            muscle_group: group.name(),
            name: muscle.name(),
            simple_name: muscle.simple_name(),
            description: muscle.description(),
            image_source: muscle.image_source(),
            parent: None,
            links: None,
        }
    }
}
