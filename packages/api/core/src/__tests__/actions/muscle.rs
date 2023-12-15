use crate::prelude::*;

pub async fn create_muscle(server: &MockServer, payload: Value) -> MockResponse {
    server.post("/api/muscles", payload).await
}

pub async fn read_muscle(server: &MockServer, muscle: &impl HasRouteKey) -> MockResponse {
    server.get_with_route_key("/api/muscles", muscle.route_key()).await
}

pub async fn list_muscles(server: &MockServer, params: Option<Value>) -> MockResponse {
    match params {
        Some(params) => server.get_with_params("/api/muscles", params).await,
        None => server.get("/api/muscles").await,
    }
}
