use crate::prelude::*;
use crate::error::ClientError;
use axum::http::StatusCode;
use bcrypt::BcryptError;
use std::collections::HashMap;

#[derive(Debug, strum_macros::Display)]
pub enum Error {
    StringDecryption(BcryptError),
    StringEncryption(BcryptError),
    InvalidPasswordFormat(Vec<String>),
}

impl From<Error> for crate::error::Error {
    fn from(error: Error) -> Self {
        let name = error.to_string();
        match error {
            Error::InvalidPasswordFormat(messages) => {
                let mut errors = HashMap::with_capacity(1);
                errors.insert("password".to_owned(), messages);

                Self::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    __("error.validation.invalidPasswordFormat"),
                    Some(errors),
                    name,
                    ClientError::ValidationError,
                )
            }
            _ => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                __("error.unexpectedSystemFailure"),
                None,
                name,
                ClientError::UnexpectedSystemFailure,
            )
        }
    }
}
