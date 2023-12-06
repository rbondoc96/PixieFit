use super::actions::muscle_group as actions;
use crate::http::resources::{ModelResource, MuscleGroupResource};
use crate::models::MuscleGroup;
use crate::prelude::*;

#[sqlx::test]
async fn create_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;
    let payload = json!({
        "name": "Test Group",
        "image_source": None::<()>,
    });
    let current_count = MuscleGroup::count(server.database()).await?;

    // Act
    let response = actions::create_muscle_group(&server, payload).await;

    // Assert
    response.assert_created();
    response.assert_json(json!({
        "success": true,
        "data": {
            "id": current_count + 1,
            "name": "Test Group",
            "image_source": None::<()>,
        },
    }));

    Ok(())
}

#[sqlx::test]
async fn create_fails_if_not_authenticated(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let payload = json!({
        "name": "Test Group",
        "image_source": None::<()>,
    });

    // Act
    let response = actions::create_muscle_group(&server, payload).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}

#[sqlx::test]
async fn create_fails_if_name_already_exists(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;

    MuscleGroup::new()
        .name("Test Group")
        .create(server.database())
        .await?;

    let payload = json!({
        "name": "Test Group",
        "image_source": None::<()>,
    });

    // Act
    let response = actions::create_muscle_group(&server, payload).await;

    // Assert
    response.assert_unprocessable();

    Ok(())
}

#[sqlx::test]
async fn read_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;
    let group = MuscleGroup::mocked(server.database()).await?;

    // Act
    let response = actions::read_muscle_group(&server, group.id).await;

    // Assert
    let resource = MuscleGroupResource::default(group, server.database()).await;
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
    let group = MuscleGroup::mocked(server.database()).await?;

    // Act
    let response = actions::read_muscle_group(&server, group.id).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}

#[sqlx::test]
async fn list_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;
    let groups = vec![
        MuscleGroup::mocked(server.database()).await?,
        MuscleGroup::mocked(server.database()).await?,
    ];

    // Act
    let response = actions::list_muscle_groups(&server, None).await;

    // Assert
    response.assert_ok();

    Ok(())
}

#[sqlx::test]
async fn list_fails_if_not_authenticated(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;

    // Act
    let response = actions::list_muscle_groups(&server, None).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}
