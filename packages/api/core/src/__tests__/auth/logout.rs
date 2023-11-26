use crate::__tests__::actions;
use crate::__tests__::prelude::*;
use crate::models::User;

async fn perform_logout(server: &TestServer) -> TestResponse {
    server.delete("/api/auth").await
}

#[sqlx::test]
async fn success_as_standard_user(pool: PgPool) -> Result<()> {
    // Arrange
    let (server, database) = init(pool).await;

    let user = User::fake()
        .email("test_user@example.com")
        .password("#Password1234")
        .create(&database)
        .await?;

    let login_payload = json!({
        "email": "test_user@example.com",
        "password": "#Password1234",
    });

    // Act
    let login_response = actions::login(&server, &login_payload).await;
    let logout_response = perform_logout(&server).await;

    // Assert
    login_response.assert_status_ok();
    logout_response.assert_status_ok();

    Ok(())
}

#[sqlx::test]
async fn success_as_admin_user(pool: PgPool) -> Result<()> {
    // Arrange
    let (server, database) = init(pool).await;

    let user = User::fake()
        .admin()
        .email("test_user@example.com")
        .password("#Password1234")
        .create(&database)
        .await?;

    let login_payload = json!({
        "email": "test_user@example.com",
        "password": "#Password1234",
    });

    // Act
    let login_response = actions::login_as_admin(&server, &login_payload).await;
    let logout_response = perform_logout(&server).await;

    // Assert
    login_response.assert_status_ok();
    logout_response.assert_status_ok();

    Ok(())
}

#[sqlx::test]
async fn fails_if_not_authenticated_user(pool: PgPool) -> Result<()> {
    // Arrange
    let (server, database) = init(pool).await;

    // Act
    let response = perform_logout(&server).await;

    // Assert
    response.assert_status_unauthorized();

    Ok(())
}
