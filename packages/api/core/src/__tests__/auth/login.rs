use super::{perform_admin_login, perform_standard_login};
use crate::__tests__::prelude::*;
use crate::enums::Gender;
use crate::models::{Profile, User};

#[sqlx::test]
async fn standard_login_success(pool: PgPool) -> Result<()> {
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
    let response = perform_standard_login(&server, &payload).await;

    // Assert
    response.assert_status_ok();

    Ok(())
}

#[sqlx::test]
async fn standard_login_fails_with_incorrect_password(pool: PgPool) -> Result<()> {
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
    let response = perform_standard_login(&server, &payload).await;

    // Assert
    response.assert_status(StatusCode::BAD_REQUEST);

    Ok(())
}

#[sqlx::test]
async fn standard_login_fails_with_non_existent_email(pool: PgPool) -> Result<()> {
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
    let response = perform_standard_login(&server, &payload).await;


    // Assert
    response.assert_status(StatusCode::BAD_REQUEST);

    Ok(())
}

#[sqlx::test]
async fn standard_login_fails_with_admin_user(pool: PgPool) -> Result<()> {
    // Arrange
    let (server, database) = init(pool).await;
    let user = User::fake()
        .admin()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(&database)
        .await?;

    let payload = json!({
        "email": "test_user@example.com",
        "password": "#TestPassword1234",
    });

    // Act
    let response = perform_standard_login(&server, &payload).await;

    // Assert
    response.assert_status_forbidden();

    Ok(())
}

#[sqlx::test]
async fn admin_login_success(pool: PgPool) -> Result<()> {
    // Arrange
    let (server, database) = init(pool).await;
    let user = User::fake()
        .admin()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(&database)
        .await?;

    let payload = json!({
        "email": "test_user@example.com",
        "password": "#TestPassword1234",
    });

    // Act
    let response = perform_admin_login(&server, &payload).await;

    // Assert
    response.assert_status_ok();

    Ok(())
}

#[sqlx::test]
async fn admin_login_fails_with_standard_user(pool: PgPool) -> Result<()> {
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
    let response = perform_admin_login(&server, &payload).await;

    // Assert
    response.assert_status_forbidden();

    Ok(())
}

#[sqlx::test]
async fn admin_login_fails_with_incorrect_password(pool: PgPool) -> Result<()> {
    // Arrange
    let (server, database) = init(pool).await;
    let user = User::fake()
        .admin()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(&database)
        .await?;

    let payload = json!({
        "email": "test_user@example.com",
        "password": "#TestPassword123",
    });

    // Act
    let response = perform_admin_login(&server, &payload).await;

    // Assert
    response.assert_status_bad_request();

    Ok(())
}

#[sqlx::test]
async fn admin_login_fails_with_non_existent_email(pool: PgPool) -> Result<()> {
    // Arrange
    let (server, database) = init(pool).await;
    let user = User::fake()
        .admin()
        .email("test_user@example.com")
        .password("#TestPassword1234")
        .create(&database)
        .await?;

    let payload = json!({
        "email": "i_dont_belong_to_anyone@example.com",
        "password": "#TestPassword123",
    });

    // Act
    let response = perform_admin_login(&server, &payload).await;


    // Assert
    response.assert_status_bad_request();

    Ok(())
}
