use crate::http::router;
use crate::models::User;
use crate::prelude::{MockResponse, MockUser, PgPool};
use crate::tests::actions::auth::{login, login_as_admin};
use axum_test::{TestServer, TestServerConfig};
use database::DatabaseManager;
use serde_json::{json, Value};

pub struct MockServer {
    server: TestServer,
    database: DatabaseManager,
    user: Option<MockUser>,
}

impl MockServer {
    // region Static Methods

    pub async fn init(pool: PgPool) -> Self {
        let database = DatabaseManager::from_pool(pool);
        let router = router(database.clone()).await;

        let config = TestServerConfig::builder()
            .save_cookies()
            .default_content_type("application/json")
            .build();

        Self {
            database,
            server: TestServer::new_with_config(router, config).unwrap(),
            user: None,
        }
    }

    pub async fn authenticated(pool: PgPool) -> Self {
        let mut server = Self::init(pool).await;
        let user = MockUser::create(server.database()).await;

        login(&server, json!({
            "email": user.email(),
            "password": user.password(),
        })).await;

        server.user = Some(user);
        server
    }

    pub async fn authenticated_admin(pool: PgPool) -> Self {
        let mut server = Self::init(pool).await;
        let user = MockUser::create_admin(server.database()).await;

        login_as_admin(&server, json!({
            "email": user.email(),
            "password": user.password(),
        })).await;

        server.user = Some(user);
        server
    }

    // endregion

    // region Instance Methods

    pub fn connection(&self) -> &TestServer {
        &self.server
    }

    pub fn database(&self) -> &DatabaseManager {
        &self.database
    }

    pub fn user(&self) -> Option<&User> {
        self.user.as_ref().map(|user| user.user())
    }

    pub fn user_password(&self) -> Option<&'static str> {
        self.user.as_ref().map(|user| user.password())
    }

    pub async fn get(&self, path: &str) -> MockResponse {
        MockResponse(self.server.get(path).await)
    }

    pub async fn get_with_route_key(&self, path: &str, route_key: impl std::fmt::Display) -> MockResponse {
        let trimmed_path = path.trim_end_matches('/');

        MockResponse(self.server.get(format!("{}/{}", trimmed_path, route_key).as_str()).await)
    }

    pub async fn get_with_params(&self, path: &str, params: Value) -> MockResponse {
        MockResponse(self.server.get(path).add_query_params(&params).await)
    }

    pub async fn post(&self, path: &str, body: Value) -> MockResponse {
        MockResponse(self.server.post(path).json(&body).await)
    }

    pub async fn delete(&self, path: &str) -> MockResponse {
        MockResponse(self.server.delete(path).await)
    }

    // endregion
}
