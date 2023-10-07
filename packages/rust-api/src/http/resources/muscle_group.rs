use super::ModelResource;
use crate::{
    models::MuscleGroup,
};
use async_trait::async_trait;
use serde::Serialize;

#[derive(Serialize)]
pub struct MuscleGroupResource {
    id: i32,
    name: String,
    image_source: Option<String>,
}

#[async_trait]
impl ModelResource for MuscleGroupResource {
    type Model = MuscleGroup;

    async fn default(group: MuscleGroup) -> Self {
        Self {
            id: group.id(),
            name: group.name(),
            image_source: group.image_source(),
        }
    }

    async fn simple(group: MuscleGroup) -> Self {
        Self {
            id: group.id(),
            name: group.name(),
            image_source: group.image_source(),
        }
    }
}