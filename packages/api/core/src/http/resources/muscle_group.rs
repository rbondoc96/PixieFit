use super::ModelResource;
use crate::models::MuscleGroup;
use async_trait::async_trait;
use database::DatabaseManager;
use serde::Serialize;

#[derive(Serialize)]
pub struct MuscleGroupResource {
    id: i16,
    name: String,
    image_source: Option<String>,
}

#[async_trait]
impl ModelResource for MuscleGroupResource {
    type Model = MuscleGroup;

    async fn default(group: MuscleGroup, database: &DatabaseManager) -> Self {
        Self {
            id: group.id,
            name: group.name,
            image_source: group.image_source,
        }
    }

    async fn simple(group: MuscleGroup, database: &DatabaseManager) -> Self {
        Self {
            id: group.id,
            name: group.name,
            image_source: group.image_source,
        }
    }
}