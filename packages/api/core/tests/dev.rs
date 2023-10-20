#![allow(unused)]

use serde_json::json;

#[tokio::test]
async fn dev() -> anyhow::Result<()> {
    let client = httpc_test::new_client("http://localhost:4000")?;

    // client.do_get("/ping").await?.print().await?;
    // client.do_get("/hello?name=Rodrigo").await?.print().await?;
    // client.do_get("/hello/Bondoc").await?.print().await?;
    // client.do_get("/api/ping").await?.print().await?;

    client
        .do_post(
            "/api/auth",
            json!({
                "email": "test_userd@example.com",
                "password": "password1234",
            }),
        )
        .await?
        .print()
        .await?;

    client
        .do_post(
            "/api/auth",
            json!({
                "email": "test_user@example.com",
                "password": "password12345",
            }),
        )
        .await?
        .print()
        .await?;

    client
        .do_post(
            "/api/auth",
            json!({
                "email": "test_user@example.com",
                "password": "password1234",
            }),
        )
        .await?
        .print()
        .await?;

    Ok(())
}
