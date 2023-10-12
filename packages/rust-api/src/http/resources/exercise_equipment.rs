use super::ModelResource;
use crate::models::ExerciseEquipment;
use async_trait::async_trait;
use serde::Serialize;

#[derive(Serialize)]
pub struct ExerciseEquipmentResource {
    name: String,
}

#[async_trait]
impl ModelResource for ExerciseEquipmentResource {
    type Model = ExerciseEquipment;

    async fn default(equipment: ExerciseEquipment) -> Self {
        Self {
            name: equipment.name(),
        }
    }

    async fn simple(equipment: ExerciseEquipment) -> Self {
        Self {
            name: equipment.name(),
        }
    }
}
