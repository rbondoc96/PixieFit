use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

use crate::types::ErrorMap;
use crate::ErrorContext;

pub struct JsonResponse<TData>
where
    TData: Serialize,
{
    code: StatusCode,
    success: bool,
    data: Option<TData>,
    error_context: Option<ErrorContext>,
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
    errors: Option<ErrorMap>,
}

impl<TData> JsonResponse<TData>
where
    TData: Serialize,
{
    fn new(
        code: StatusCode,
        success: bool,
        data: Option<TData>,
        error_context: Option<ErrorContext>,
    ) -> Self {
        Self {
            code,
            success,
            data,
            error_context,
        }
    }

    pub fn error(context: ErrorContext) -> Self {
        Self::new(context.code(), false, None, Some(context))
    }

    pub fn success(data: Option<TData>, code: StatusCode) -> Self {
        Self::new(code, true, data, None)
    }

    pub fn json(self) -> Json<Value> {
        Json(json!(ApiResponse {
            success: self.success,
            data: self.data,
            error: self
                .error_context
                .as_ref()
                .map(|context| context.client_error().as_ref().to_string()),
            message: self
                .error_context
                .as_ref()
                .map(|context| context.client_error().message()),
            errors: match self.error_context {
                Some(context) => context.client_error().errors(),
                None => None,
            },
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
