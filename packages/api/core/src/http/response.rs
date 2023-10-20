use crate::error::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

pub struct JsonResponse<TData>
where
    TData: Serialize,
{
    code: StatusCode,
    success: bool,
    data: Option<TData>,
    error: Option<Error>,
}

#[derive(Serialize)]
pub struct ApiSuccessResponse<TData>
where
    TData: Serialize,
{
    success: bool,
    data: Option<TData>,
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

#[skip_serializing_none]
#[derive(Serialize)]
pub struct ApiResponse<TData>
where
    TData: Serialize,
{
    success: bool,
    data: Option<TData>,
    error: Option<String>,
    message: Option<String>,
    errors: Option<std::collections::HashMap<String, Vec<String>>>,
}

impl<TData> JsonResponse<TData>
where
    TData: Serialize,
{
    fn new(
        code: StatusCode,
        success: bool,
        data: Option<TData>,
        error: Option<Error>,
    ) -> Self {
        Self {
            code,
            success,
            data,
            error,
        }
    }

    pub fn error(error: Error) -> Self {
        Self::new(
            error.code(),
            false,
            None,
            Some(error),
        )
    }

    pub fn success(data: Option<TData>, code: StatusCode) -> Self {
        Self::new(code, true, data, None)
    }

    pub fn json(self) -> Json<Value> {
        if let Some(error) = self.error {
            println!("error: {}", error);
            return Json(json!(ApiErrorResponse {
                success: self.success,
                error: ApiErrorContext {
                    name: error.client_name(),
                    message: error.message(),
                    errors: error.messages(),
                },
            }));
        }

        Json(json!(ApiSuccessResponse {
            success: self.success,
            data: self.data,
        }))
    }
}

impl<TData> IntoResponse for JsonResponse<TData>
where
    TData: Serialize,
{
    fn into_response(self) -> Response {
        let status_code = self.code;

        if status_code == StatusCode::NO_CONTENT {
            return status_code.into_response();
        }

        let body = self.json();

        (status_code, body).into_response()
    }
}
