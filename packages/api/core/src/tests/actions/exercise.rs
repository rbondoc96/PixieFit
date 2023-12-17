use crate::prelude::*;

pub async fn create_exercise(server: &MockServer, payload: Value) -> MockResponse {
    server.post("/api/exercises", payload).await
}

pub async fn read_exercise(server: &MockServer, exercise: &impl HasRouteKey) -> MockResponse {
    server.get_with_route_key("/api/exercises", exercise.route_key()).await
}

pub async fn list_exercises(server: &MockServer, params: Option<Value>) -> MockResponse {
    match params {
        Some(params) => server.get_with_params("/api/exercises", params).await,
        None => server.get("/api/exercises").await,
    }
}
