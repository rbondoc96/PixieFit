use super::{ClientError, ErrorDomain, ErrorSubdomain};
use crate::http::response::JsonResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct ErrorContext {
    code: StatusCode,
    domain: ErrorDomain,
    subdomain: Option<ErrorSubdomain>,
    client_error: ClientError,
}

impl ErrorContext {
    pub fn new(
        code: StatusCode,
        domain: ErrorDomain,
        subdomain: Option<ErrorSubdomain>,
        client_error: ClientError,
    ) -> Self {
        Self {
            code,
            domain,
            subdomain,
            client_error,
        }
    }

    pub fn code(&self) -> StatusCode {
        self.code
    }

    pub fn domain(&self) -> ErrorDomain {
        self.domain.clone()
    }

    pub fn subdomain(&self) -> Option<ErrorSubdomain> {
        self.subdomain.clone()
    }

    pub fn client_error(&self) -> ClientError {
        self.client_error.clone()
    }
}

impl IntoResponse for ErrorContext {
    fn into_response(self) -> Response {
        println!("->> {:<12}", "ERROR_CONTEXT_INTO_RESPONSE");

        JsonResponse::<()>::error(self).into_response()
    }
}
