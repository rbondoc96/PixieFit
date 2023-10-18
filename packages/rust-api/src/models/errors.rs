use crate::error::ClientError;
use axum::http::StatusCode;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    ModelNotCreated {
        message: String,
    },
    ModelNotFound {
        model: &'static str,
        search_key: String,
        search_value: String,
    },
    UnexpectedDatabaseError {
        message: String,
    },
    UnknownError,
}

impl From<Error> for crate::error::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::ModelNotCreated {
                message,
            } => Self::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                message,
                None,
                "ModelNotCreated",
                ClientError::ValidationError,
            ),
            Error::ModelNotFound {
                model,
                search_key,
                search_value,
            } => Self::new(
                StatusCode::NOT_FOUND,
                format!(
                    "A {} with {} = {} could not be found.",
                    model, search_key, search_value
                ),
                None,
                "ModelNotFound",
                ClientError::ResourceNotFound,
            ),
            Error::UnexpectedDatabaseError {
                message,
            } => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                message,
                None,
                "UnexpectedDatabaseError",
                ClientError::UnexpectedSystemFailure,
            ),
            Error::UnknownError => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "An unknown error has occurred.".to_string(),
                None,
                "UnknownError",
                ClientError::UnknownSystemFailure,
            ),
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
                return Self::ModelNotCreated {
                    message: message.to_owned(),
                };
            }

            return Self::UnexpectedDatabaseError {
                message: message.to_owned(),
            };
        }

        Self::UnknownError
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for Error {}
