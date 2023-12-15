use super::actions::muscle as actions;
use crate::http::resources::{ModelResource, MuscleResource};
use crate::models::{Muscle, MuscleGroup};
use crate::prelude::*;

#[sqlx::test]
async fn create_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;
    let group = MuscleGroup::mocked(server.database()).await?;
    let payload = json!({
        "group_id": group.id,
        "parent_id": NULL,
        "name": "Test Muscle",
        "simple_name": NULL,
        "description": NULL,
        "image_source": NULL,
    });

    // Act
    let response = actions::create_muscle(&server, payload).await;

    // Assert
    response.assert_created();

    Ok(())
}

#[sqlx::test]
async fn create_fails_if_not_authenticated(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let group = MuscleGroup::mocked(server.database()).await?;
    let payload = json!({
        "group_id": group.id,
        "parent_id": NULL,
        "name": "Test Muscle",
        "simple_name": NULL,
        "description": NULL,
        "image_source": NULL,
    });

    // Act
    let response = actions::create_muscle(&server, payload).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}

#[sqlx::test]
async fn read_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;
    let muscle = Muscle::mocked(server.database()).await?;

    // Act
    let response = actions::read_muscle(&server, &muscle).await;

    // Assert
    let resource = MuscleResource::default(muscle, server.database()).await?;
    response.assert_ok();
    response.assert_json(json!({
        "success": true,
        "data": resource,
    }));

    Ok(())
}

#[sqlx::test]
async fn read_fails_if_not_authenticated(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let muscle = Muscle::mocked(server.database()).await?;

    // Act
    let response = actions::read_muscle(&server, &muscle).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}

#[sqlx::test]
async fn list_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;

    // Act
    let response = actions::list_muscles(&server, None).await;

    // Assert
    response.assert_ok();

    Ok(())
}

#[sqlx::test]
async fn list_fails_if_not_authenticated(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let muscle = Muscle::mocked(server.database()).await?;

    // Act
    let response = actions::list_muscles(&server, None).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}
