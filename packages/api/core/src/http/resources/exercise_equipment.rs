use super::{ModelResource, ResourceResult};
use crate::models::ExerciseEquipment;
use async_trait::async_trait;
use database::DatabaseManager;
use serde::Serialize;

#[derive(Serialize)]
pub struct ExerciseEquipmentResource {
    id: i16,
    name: String,
}

#[async_trait]
impl ModelResource for ExerciseEquipmentResource {
    type Model = ExerciseEquipment;

    async fn default(equipment: ExerciseEquipment, database: &DatabaseManager) -> ResourceResult<Self> {
        Ok(Self {
            id: equipment.id,
            name: equipment.name,
        })
    }

    async fn simple(equipment: ExerciseEquipment, database: &DatabaseManager) -> ResourceResult<Self> {
        Ok(Self {
            id: equipment.id,
            name: equipment.name,
        })
    }
}
