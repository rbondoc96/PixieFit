use crate::tests::actions::auth as actions;
use crate::enums::Gender;
use crate::models::{Profile, User};
use crate::prelude::*;

#[sqlx::test]
async fn standard_login_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let user = User::fake()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(server.database())
        .await?;

    Profile::fake()
        .user(&user)
        .create(server.database())
        .await?;

    let payload = json!({
        "email": "test_user@example.com",
        "password": "#TestPassword1234",
    });

    // Act
    let response = actions::login(&server, payload).await;

    // Assert
    response.assert_ok();

    Ok(())
}

#[sqlx::test]
async fn standard_login_fails_with_incorrect_password(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let user = User::fake()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(server.database())
        .await?;

    let payload = json!({
        "email": "test_user@example.com",
        "password": "#TestPassword123",
    });

    // Act
    let response = actions::login(&server, payload).await;

    // Assert
    response.assert_bad_request();

    Ok(())
}

#[sqlx::test]
async fn standard_login_fails_with_non_existent_email(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let user = User::fake()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(server.database())
        .await?;

    let payload = json!({
        "email": "i_dont_belong_to_anyone@example.com",
        "password": "#TestPassword123",
    });

    // Act
    let response = actions::login(&server, payload).await;

    // Assert
    response.assert_bad_request();

    Ok(())
}

#[sqlx::test]
async fn standard_login_fails_with_admin_user(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let user = User::fake()
        .admin()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(server.database())
        .await?;

    let payload = json!({
        "email": "test_user@example.com",
        "password": "#TestPassword1234",
    });

    // Act
    let response = actions::login(&server, payload).await;

    // Assert
    response.assert_forbidden();

    Ok(())
}

#[sqlx::test]
async fn admin_login_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let user = User::fake()
        .admin()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(server.database())
        .await?;

    let payload = json!({
        "email": "test_user@example.com",
        "password": "#TestPassword1234",
    });

    // Act
    let response = actions::login_as_admin(&server, payload).await;

    // Assert
    response.assert_ok();

    Ok(())
}

#[sqlx::test]
async fn admin_login_fails_with_standard_user(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let user = User::fake()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(server.database())
        .await?;

    let payload = json!({
        "email": "test_user@example.com",
        "password": "#TestPassword1234",
    });

    // Act
    let response = actions::login_as_admin(&server, payload).await;

    // Assert
    response.assert_forbidden();

    Ok(())
}

#[sqlx::test]
async fn admin_login_fails_with_incorrect_password(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let user = User::fake()
        .admin()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(server.database())
        .await?;

    let payload = json!({
        "email": "test_user@example.com",
        "password": "#TestPassword123",
    });

    // Act
    let response = actions::login_as_admin(&server, payload).await;

    // Assert
    response.assert_bad_request();

    Ok(())
}

#[sqlx::test]
async fn admin_login_fails_with_non_existent_email(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let user = User::fake()
        .admin()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(server.database())
        .await?;

    let payload = json!({
        "email": "i_dont_belong_to_anyone@example.com",
        "password": "#TestPassword123",
    });

    // Act
    let response = actions::login_as_admin(&server, payload).await;

    // Assert
    response.assert_bad_request();

    Ok(())
}
