use crate::error::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use database::DatabaseManager;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

pub struct JsonResponse {
    code: StatusCode,
    body: Json<Value>,
}

#[derive(Serialize)]
pub struct ApiSuccessResponse<T: Serialize> {
    success: bool,
    data: Option<T>,
}

#[derive(Serialize)]
pub struct ApiErrorContext {
    name: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Serialize)]
pub struct ApiErrorResponse {
    success: bool,
    error: ApiErrorContext,
}

impl JsonResponse {
    fn new(
        body: Json<Value>,
        code: StatusCode,
    ) -> Self {
        Self {
            body,
            code,
        }
    }

    pub fn error(error: Error) -> Self {
        Self::new(
            Json(json!(ApiErrorResponse {
                success: false,
                error: ApiErrorContext {
                    name: error.client_name(),
                    message: error.message(),
                    errors: error.messages(),
                }
            })),
            error.code()
        )
    }

    pub fn success(data: Option<impl Serialize>, code: StatusCode) -> Self {
        Self::new(
            Json(json!(ApiSuccessResponse {
                success: true,
                data: data,
            })),
            code,
        )
    }
}

impl IntoResponse for JsonResponse {
    fn into_response(self) -> Response {
        match self.code {
            StatusCode::NO_CONTENT => self.code.into_response(),
            _ => (self.code, self.body).into_response()
        }
    }
}
