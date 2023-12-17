use crate::error::{ClientError, Domain};
use crate::prelude::__;
use crate::types::ErrorMap;
use axum::http::StatusCode;

#[derive(Debug, strum_macros::Display, thiserror::Error)]
pub enum Error {
    InvalidPasswordFormat(Vec<String>),
    PasswordMismatch,
    UserWithEmailAlreadyExists,
}

impl From<Error> for crate::http::Error {
    fn from(error: Error) -> Self {
        match &error {
            Error::InvalidPasswordFormat(messages) => {
                let mut map = ErrorMap::new();
                map.insert("password".to_string(), messages.clone());

                Self::unprocessable(ClientError::Validation, Domain::UserRegistration)
                    .with_messages(map)
            },
            Error::PasswordMismatch => Self::unprocessable(ClientError::Validation, Domain::UserRegistration)
                .with_message(__("errors.auth.passwordMismatch")),
            Error::UserWithEmailAlreadyExists => Self::unprocessable(ClientError::Validation, Domain::UserRegistration)
        }
    }
}
