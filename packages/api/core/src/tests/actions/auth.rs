use crate::models::User;
use crate::prelude::*;

pub async fn login(server: &MockServer, payload: Value) -> MockResponse {
    server.post("/api/auth", payload).await
}

pub async fn login_as_admin(server: &MockServer, payload: Value) -> MockResponse {
    server.post("/api/auth/admin", payload).await
}

pub async fn logout(server: &MockServer) -> MockResponse {
    server.delete("/api/auth").await
}

pub async fn register(server: &MockServer, payload: Value) -> MockResponse {
    server.post("/api/auth/register", payload).await
}
