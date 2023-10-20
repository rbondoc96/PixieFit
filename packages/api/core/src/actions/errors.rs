use crate::error::ClientError;
use crate::prelude::__;
use axum::http::StatusCode;

#[derive(Debug, strum_macros::Display)]
pub enum Error {
    PasswordMismatch,
    UserWithEmailAlreadyExists,
}

impl From<Error> for crate::error::Error {
    fn from(error: Error) -> Self {
        let name = error.to_string();

        match error {
            Error::PasswordMismatch => Self::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                __("error.user.passwordMismatch"),
                None,
                name,
                ClientError::ValidationError,
            ),
            Error::UserWithEmailAlreadyExists => Self::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                __("error.user.emailAlreadyExists"),
                None,
                name,
                ClientError::ValidationError,
            ),
        }
    }
}
