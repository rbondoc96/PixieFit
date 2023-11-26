use super::prelude::{TestResponse, TestServer, Value};

// region Authentication

pub async fn login(server: &TestServer, payload: &Value) -> TestResponse {
    server.post("/api/auth").json(payload).await
}

pub async fn login_as_admin(server: &TestServer, payload: &Value) -> TestResponse {
    server.post("/api/auth/admin").json(payload).await
}

pub async fn logout(server: &TestServer) -> TestResponse {
    server.delete("/api/auth").await
}

pub async fn register(server: &TestServer, payload: &Value) -> TestResponse {
    server.post("/api/auth/register").json(payload).await
}

// endregion

// region Health

pub async fn ping(server: &TestServer) -> TestResponse {
    server.get("/ping").await
}

// endregion
