use crate::__tests__::prelude::*;
use crate::enums::Gender;
use crate::models::{Profile, User};

async fn perform_action(server: TestServer, payload: &Value) -> TestResponse {
    server.post("/api/auth").json(payload).await
}

#[sqlx::test]
async fn success(pool: PgPool) -> Result<()> {
    // Arrange
    let (server, database) = init(pool).await;
    let user = User::fake()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(&database)
        .await?;

    let payload = json!({
        "email": "test_user@example.com",
        "password": "#TestPassword1234",
    });

    // Act
    let response = perform_action(server, &payload).await;

    // Assert
    println!("{:?}", response);
    response.assert_status(StatusCode::OK);

    Ok(())
}

#[sqlx::test]
async fn fails_with_incorrect_password(pool: PgPool) -> Result<()> {
    // Arrange
    let (server, database) = init(pool).await;
    let user = User::fake()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(&database)
        .await?;

    let payload = json!({
        "email": "test_user@example.com",
        "password": "#TestPassword123",
    });

    // Act
    let response = perform_action(server, &payload).await;

    // Assert
    response.assert_status(StatusCode::BAD_REQUEST);

    Ok(())
}

#[sqlx::test]
async fn fails_with_non_existent_email(pool: PgPool) -> Result<()> {
    // Arrange
    let (server, database) = init(pool).await;
    let user = User::fake()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(&database)
        .await?;

    let payload = json!({
        "email": "i_dont_belong_to_anyone@example.com",
        "password": "#TestPassword123",
    });

    // Act
    let response = perform_action(server, &payload).await;


    // Assert
    response.assert_status(StatusCode::BAD_REQUEST);

    Ok(())
}
