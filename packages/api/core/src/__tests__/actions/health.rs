use crate::models::User;
use crate::prelude::*;

pub async fn ping(server: &MockServer) -> MockResponse {
    server.get("/ping").await
}
