use super::Error;
use crate::http::JsonResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// region States

#[derive(Default)]
pub struct NoErrorSource;
#[derive(Default)]
pub struct Source<TError>(TError) where TError: std::error::Error + Send + Sync + 'static;

#[derive(Default)]
pub struct NoErrorName;
#[derive(Default)]
pub struct Name(String);

// endregion

#[derive(Default)]
pub struct ErrorBuilder<TError, TName>
where
    TError: std::error::Error + Send + Sync + 'static,
{
    code: Option<StatusCode>,
    message: Option<String>,
    messages: Option<std::collections::HashMap<String, String>>,
    name: TName,
    source: TError,
}

impl ErrorBuilder<NoErrorSource, NoErrorName> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<TError> ErrorBuilder<TError, NoErrorName>
where
    TError: std::error::Error + Send + Sync + 'static,
{
    pub fn name(self, name: impl Into<String>) -> ErrorBuilder<TError, Name> {
        ErrorBuilder {
            code: self.code,
            message: self.message,
            messages: self.messages,
            name: Name(name.into()),
            source: self.source,
        }
    }
}

impl<TName> ErrorBuilder<NoErrorSource, TName> {
    pub fn source<TError>(
        self,
        source: impl Into<TError>
    ) -> ErrorBuilder<Source<TError>, TName>
    where
        TError: std::error::Error + Send + Sync + 'static,
    {
        ErrorBuilder {
            code: self.code,
            message: self.message,
            messages: self.messages,
            name: self.name,
            source: Source(source.into()),
        }
    }
}

impl<TError, TName> ErrorBuilder<TError, TName>
where
    TError: std::error::Error + Send + Sync + 'static,
{
    pub fn code(mut self, code: StatusCode) -> Self {
        self.code = Some(code);
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn messages(mut self, messages: std::collections::HashMap<String, String>) -> Self {
        self.messages = Some(messages);
        self
    }

    pub fn build(self) -> Error {
        Error {
            code: self.code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            message: self.message.unwrap_or("An unknown error has occurred.".to_string()),
            messages: self.messages,
            name: self.name,
            source: self.source,
        }
    }
}
