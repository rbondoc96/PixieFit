use crate::prelude::*;

pub async fn create_exercise_equipment(server: &MockServer, payload: Value) -> MockResponse {
    server.post("/api/exercise-equipment", payload).await
}

pub async fn read_exercise_equipment(server: &MockServer, equipment: &impl HasRouteKey) -> MockResponse {
    server.get_with_route_key("/api/exercise-equipment", equipment.route_key()).await
}

pub async fn list_exercise_equipment(server: &MockServer) -> MockResponse {
    server.get("/api/exercise-equipment").await
}
