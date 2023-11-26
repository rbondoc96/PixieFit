mod prelude;
mod auth;

use self::prelude::*;
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct PingResponse {
    success: bool,
    message: String,
}

#[sqlx::test]
async fn test_ping(pool: PgPool) -> Result<()> {
    let (server, _)= init(pool).await;

    let response = server.get("/ping").await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let expected = PingResponse {
        success: true,
        message: "pong".to_string(),
    };

    response.assert_json(&expected);

    Ok(())
}
