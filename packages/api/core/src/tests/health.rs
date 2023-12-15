use crate::tests::actions::health as actions;
use crate::prelude::*;

#[sqlx::test]
async fn ping(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;

    // Act
    let response = actions::ping(&server).await;

    // Assert
    response.assert_ok();
    response.assert_json(json!({
        "success": true,
        "data": {
            "message": "pong"
        },
    }));

    Ok(())
}
