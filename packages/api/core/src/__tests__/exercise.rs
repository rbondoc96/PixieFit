use crate::__tests__::actions::exercise as actions;
use crate::enums::{ExerciseType, ExerciseForce, ExerciseMechanic, Measurement};
use crate::http::resources::{ExerciseResource, ModelResource};
use crate::models::{Exercise, ExerciseEquipment, MuscleGroup};
use crate::prelude::*;

#[sqlx::test]
async fn create_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;
    let database = server.database();
    let current_count = Exercise::count(database).await?;
    let exercise_type = ExerciseType::Strength;
    let equipment = ExerciseEquipment::mocked(database).await?;
    let force = ExerciseForce::Pull;
    let group = MuscleGroup::mocked(database).await?;
    let mechanic = ExerciseMechanic::Compound;
    let measurement = Measurement::WeightedRepetitions;
    let payload = json!({
        "type": exercise_type,
        "target_muscle_group_id": Some(group.id),
        "name": "Test Exercise",
        "name_alternative": NULL,
        "description": NULL,
        "equipment_id": Some(equipment.id),
        "mechanic": Some(mechanic),
        "force": Some(force),
        "measurement": Some(measurement),
        "muscles": [],
    });

    // Act
    let response = actions::create_exercise(&server, payload).await;

    // Assert
    let exercise = Exercise::find("name", "Test Exercise", database).await?;
    let resource = ExerciseResource::default(exercise, database).await;
    response.assert_created();
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
    let database = server.database();
    let current_count = Exercise::count(database).await?;
    let exercise_type = ExerciseType::Strength;
    let equipment = ExerciseEquipment::mocked(database).await?;
    let force = ExerciseForce::Pull;
    let group = MuscleGroup::mocked(database).await?;
    let mechanic = ExerciseMechanic::Compound;
    let measurement = Measurement::WeightedRepetitions;
    let payload = json!({
        "type": exercise_type,
        "target_muscle_group_id": Some(group.id),
        "name": "Test Exercise",
        "name_alternative": NULL,
        "description": NULL,
        "equipment_id": Some(equipment.id),
        "mechanic": Some(mechanic),
        "force": Some(force),
        "measurement": Some(measurement),
        "muscles": [],
    });

    // Act
    let response = actions::create_exercise(&server, payload).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}

#[sqlx::test]
async fn read_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;
    let exercise = Exercise::mocked(server.database()).await?;

    // Act
    let response = actions::read_exercise(&server, &exercise).await;

    // Assert
    let resource = ExerciseResource::default(exercise, server.database()).await;
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
    let exercise = Exercise::mocked(server.database()).await?;

    // Act
    let response = actions::read_exercise(&server, &exercise).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}

#[sqlx::test]
async fn list_success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;

    // Act
    let response = actions::list_exercises(&server, None).await;

    // Assert
    response.assert_ok();

    Ok(())
}

#[sqlx::test]
async fn list_exercises_with_specific_muscle_group(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::authenticated(pool).await;
    let group = MuscleGroup::mocked(server.database()).await?;
    let exercise = Exercise::fake()
        .target_muscle_group_id(Some(group.id))
        .create(server.database())
        .await?;

    // Act
    let response = actions::list_exercises(&server, Some(json!({
        "muscle_group": group.id,
    }))).await;

    // Assert
    response.assert_ok();

    Ok(())
}

#[sqlx::test]
async fn list_fails_if_not_authenticated(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;

    // Act
    let response = actions::list_exercises(&server, None).await;

    // Assert
    response.assert_unauthorized();

    Ok(())
}
