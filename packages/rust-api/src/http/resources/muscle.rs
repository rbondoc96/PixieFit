use super::{LinkResource, ModelResource};
use crate::{
    enums::MuscleGroup,
    models::{Link, Muscle},
};
use async_trait::async_trait;
use serde::Serialize;

#[derive(Serialize)]
pub struct MuscleResource {
    id: i64,
    muscle_group: MuscleGroup,
    name: String,
    simple_name: Option<String>,

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

        Self {
            id: muscle.id(),
            muscle_group: muscle.muscle_group(),
            name: muscle.name(),
            simple_name: muscle.simple_name(),
            links: Some(LinkResource::list(links).await),
        }
    }

    async fn simple(muscle: Muscle) -> Self {
        Self {
            id: muscle.id(),
            muscle_group: muscle.muscle_group(),
            name: muscle.name(),
            simple_name: muscle.simple_name(),
            links: None,
        }
    }
}
