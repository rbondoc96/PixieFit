use crate::prelude::*;

pub async fn create_muscle_group(server: &MockServer, payload: Value) -> MockResponse {
    server.post("/api/muscle-groups", payload).await
}

pub async fn read_muscle_group(server: &MockServer, route_key: impl std::fmt::Display) -> MockResponse {
    server.get_with_route_key("/api/muscle-groups", route_key).await
}

pub async fn list_muscle_groups(server: &MockServer, params: Option<Value>) -> MockResponse {
    match params {
        Some(params) => server.get_with_params("/api/muscle-groups", params).await,
        None => server.get("/api/muscle-groups").await,
    }
}
