use crate::__tests__::actions::exercise_equipment as actions;
use crate::http::resources::{ModelResource, ExerciseEquipmentResource};
use crate::models::ExerciseEquipment;
use crate::prelude::*;

#[sqlx::test]
async fn create_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;
    let payload = json!({
        "name": "Test Equipment",
    });

    // Act
    let response = actions::create_exercise_equipment(&server, payload).await;

    // Assert
    response.assert_created();
    let equipment = ExerciseEquipment::find("name", "Test Equipment", server.database()).await?;
    let resource = ExerciseEquipmentResource::default(equipment, server.database()).await;
    response.assert_json(json!({
        "success": true,
        "data": resource,
    }));

    Ok(())
}

#[sqlx::test]
async fn create_fails_if_not_authenticated(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let payload = json!({
        "name": "Test Equipment",
    });

    // Act
    let response = actions::create_exercise_equipment(&server, payload).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}

#[sqlx::test]
async fn read_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;
    let equipment = ExerciseEquipment::mocked(server.database()).await?;

    // Act
    let response = actions::read_exercise_equipment(&server, &equipment).await;

    // Assert
    response.assert_ok();
    let resource = ExerciseEquipmentResource::default(equipment, server.database()).await;
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
    let equipment = ExerciseEquipment::mocked(server.database()).await?;

    // Act
    let response = actions::read_exercise_equipment(&server, &equipment).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}

#[sqlx::test]
async fn list_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;

    // Act
    let response = actions::list_exercise_equipment(&server).await;

    // Assert
    response.assert_ok();

    Ok(())
}

#[sqlx::test]
async fn list_fails_if_not_authenticated(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;

    // Act
    let response = actions::list_exercise_equipment(&server).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}
