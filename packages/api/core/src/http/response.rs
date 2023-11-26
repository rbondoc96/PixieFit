use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use database::DatabaseManager;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct JsonResponse {
    body: Value,
    code: StatusCode,
    success: bool,
}

#[derive(Serialize)]
pub struct ApiSuccessResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
}

#[derive(Serialize)]
pub struct ApiErrorContext {
    pub name: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub success: bool,
    pub error: ApiErrorContext,
}

impl JsonResponse {
    // region Static Methods

    fn new(
        body: Value,
        code: StatusCode,
        success: bool,
    ) -> Self {
        Self {
            body,
            code,
            success,
        }
    }

    pub fn error(error: super::Error) -> Self {
        Self::new(
            json!(ApiErrorContext {
                name: error.client().to_string(),
                message: error.message(),
                errors: error.messages(),
            }),
            error.code(),
            false,
        )
    }

    pub fn success(code: StatusCode) -> Self {
        Self::new(
            json!(None::<()>),
            code,
            true,
        )
    }

    pub fn ok() -> Self {
        Self::success(StatusCode::OK)
    }

    pub fn created() -> Self {
        Self::success(StatusCode::CREATED)
    }

    pub fn no_content() -> Self {
        Self::success(StatusCode::NO_CONTENT)
    }

    // endregion

    // region Instance Accessor Methods

    pub fn code(&self) -> StatusCode {
        self.code
    }

    // endregion

    // region Instance Mutator Methods

    pub fn with_data(mut self, data: impl Serialize) -> Self {
        self.body = json!(data);
        self
    }

    // endregion
}

impl IntoResponse for JsonResponse {
    fn into_response(self) -> Response {
        if self.code == StatusCode::NO_CONTENT {
            return self.code.into_response()
        }

        let body = Json(match self.success {
            true => json!({
                "success": true,
                "data": self.body,
            }),
            false => json!({
                "success": false,
                "error": self.body,
            }),
        });

        (self.code, body).into_response()
    }
}
