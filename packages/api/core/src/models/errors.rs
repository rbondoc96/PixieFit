use crate::error::{ClientError, Domain};
use crate::utils::__;
use axum::http::StatusCode;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, strum_macros::Display, thiserror::Error, Serialize)]
pub enum Error {
    ModelNotCreated(String),
    ModelNotFound(String),
    Unknown(String),
}

impl From<crate::utils::Error> for Error {
    fn from(error: crate::utils::Error) -> Self {
        Self::Unknown(__("errors.unknownSystemError"))
    }
}

impl From<database::Error> for Error {
    fn from(error: database::Error) -> Self {
        use database::Error as DatabaseError;

        match error {
            DatabaseError::ModelNotCreated(message) => Self::ModelNotCreated(message),
            DatabaseError::ModelNotFound { .. } => Self::ModelNotFound(error.to_string()),
            DatabaseError::Unknown(err) => Self::Unknown(err.to_string()),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        if let Some(db_error) = error.into_database_error() {
            let message = db_error.message();

            if db_error.is_check_violation()
                || db_error.is_foreign_key_violation()
                || db_error.is_unique_violation()
            {
                return Self::ModelNotCreated(message.to_owned());
            }
        }

        Self::Unknown(__("errors.unknownSystemError"))
    }
}

impl From<Error> for crate::http::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::ModelNotCreated(message) => Self::unprocessable(ClientError::Validation, Domain::Database)
                .with_message(message),
            Error::ModelNotFound(message) => Self::not_found(ClientError::ResourceNotFound, Domain::Database)
                .with_message(message),
            Error::Unknown(message) => Self::internal_error(ClientError::Unknown, Domain::Database)
                .with_message(message),
        }
    }
}
