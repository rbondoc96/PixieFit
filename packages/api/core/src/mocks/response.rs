use crate::prelude::StatusCode;
use axum_test::TestResponse;
use serde_json::Value;

pub struct MockResponse(pub TestResponse);

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
