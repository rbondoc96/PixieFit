use crate::__tests__::actions::auth as actions;
use crate::models::User;
use crate::prelude::*;

#[sqlx::test]
async fn success_as_standard_user(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;

    // Act
    let response = actions::logout(&server).await;

    // Assert
    response.assert_ok();

    Ok(())
}

#[sqlx::test]
async fn success_as_admin_user(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated_admin(pool).await;

    // Act
    let response = actions::logout(&server).await;

    // Assert
    response.assert_ok();

    Ok(())
}

#[sqlx::test]
async fn fails_if_not_authenticated_user(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;

    // Act
    let response = actions::logout(&server).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}
