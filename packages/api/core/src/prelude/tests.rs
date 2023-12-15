use super::DynError;
use crate::tests::actions::auth::{login, login_as_admin};
use crate::http::router;
use crate::models::User;

pub use axum::http::StatusCode;
pub use axum_test::{TestResponse, TestServer, TestServerConfig};
pub use database::{DatabaseManager, HasRouteKey, Model};
pub use serde_json::{json, Value};
pub use sqlx::postgres::PgPool;

pub type Result<T> = core::result::Result<T, Box<DynError>>;

pub const NULL: Option<()> = None;

pub fn assert_ok<T, E>(result: &core::result::Result<T, E>) {
    assert!(result.is_ok());
}

pub fn assert_ok_eq<T, E>(expected: impl Into<T>, actual: core::result::Result<T, E>)
where
    T: PartialEq + std::fmt::Debug,
    E: std::fmt::Debug,
{
    assert!(actual.is_ok());
    assert_eq!(expected.into(), actual.unwrap());
}

pub fn assert_some<T>(result: Option<T>) {
    assert!(result.is_some());
}

pub fn assert_some_eq<T: PartialEq + std::fmt::Debug>(expected: impl Into<T>, actual: Option<T>) {
    assert!(actual.is_some());
    assert_eq!(expected.into(), actual.unwrap());
}

pub struct MockUser {
    user: User,
    password: &'static str,
}

pub struct MockServer {
    server: TestServer,
    database: DatabaseManager,
    user: Option<MockUser>,
}

pub struct MockResponse(TestResponse);

impl MockUser {
    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn password(&self) -> &'static str {
        self.password
    }
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
        let user = User::fake()
            .password("#Password1234")
            .create(server.database())
            .await
            .unwrap();

        login(&server, json!({
            "email": user.email,
            "password": "#Password1234",
        }))
        .await;

        server.user = Some(MockUser {
            user,
            password: "#Password1234",
        });

        server
    }

    pub async fn authenticated_admin(pool: PgPool) -> Self {
        let mut server = Self::init(pool).await;
        let user = User::fake()
            .admin()
            .password("#AdminPassword1234")
            .create(server.database())
            .await
            .unwrap();

        let response = login_as_admin(&server, json!({
            "email": user.email,
            "password": "#AdminPassword1234",
        }))
            .await;

        server.user = Some(MockUser {
            user,
            password: "#AdminPassword1234",
        });

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

impl MockResponse {
    pub fn assert_ok(&self) {
        self.0.assert_status_ok();
    }

    pub fn assert_created(&self) {
        self.0.assert_status(StatusCode::CREATED);
    }

    pub fn assert_no_content(&self) {
        self.0.assert_status(StatusCode::NO_CONTENT);
    }

    pub fn assert_bad_request(&self) {
        self.0.assert_status(StatusCode::BAD_REQUEST)
    }

    pub fn assert_unauthorized(&self) {
        self.0.assert_status_unauthorized();
    }

    pub fn assert_forbidden(&self) {
        self.0.assert_status_forbidden();
    }

    pub fn assert_not_found(&self) {
        self.0.assert_status_not_found();
    }

    pub fn assert_unprocessable(&self) {
        self.0.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    pub fn assert_server_error(&self) {
        self.0.assert_status(StatusCode::INTERNAL_SERVER_ERROR);
    }

    pub fn assert_json(&self, json: Value) {
        self.0.assert_json(&json);
    }
}
