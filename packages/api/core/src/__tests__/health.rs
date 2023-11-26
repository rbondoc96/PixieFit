use crate::__tests__::actions;
use crate::__tests__::prelude::*;

#[sqlx::test]
async fn ping(pool: PgPool) -> Result<()> {
    // Arrange
    let (server, _)= init(pool).await;

    // Act
    let response = actions::ping(&server).await;

    // Assert
    response.assert_status_ok();
    response.assert_json(&json!({
        "success": true,
        "data": {
            "message": "pong"
        },
    }));

    Ok(())
}
