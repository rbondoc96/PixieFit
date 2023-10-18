mod client;

pub use client::ClientError;

use crate::http::JsonResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct Error {
    code: StatusCode,
    message: String,
    messages: Option<std::collections::HashMap<String, Vec<String>>>,
    name: String,
    client_name: String,
}

impl Error {
    pub fn new(
        code: StatusCode,
        message: impl Into<String>,
        messages: Option<std::collections::HashMap<String, Vec<String>>>,
        name: impl Into<String>,
        client_error: ClientError,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            messages,
            name: name.into(),
            client_name: client_error.to_string(),
        }
    }

    pub fn code(&self) -> StatusCode {
        self.code
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn messages(&self) -> Option<std::collections::HashMap<String, Vec<String>>> {
        self.messages.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn client_name(&self) -> String {
        self.client_name.clone()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        JsonResponse::<()>::error(self).into_response()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(
            f,
            "Error {{ code: {}, message: {}, name: {} }}",
            self.code, self.message, self.name
        )
    }
}
