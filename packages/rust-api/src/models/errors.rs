use crate::root::errors::{ClientError, ErrorContext, ErrorDomain, ErrorSubdomain};
use axum::http::StatusCode;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    DatabaseError(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
    ModelNotFound {
        model: &'static str,
        search_key: String,
        search_value: String,
    },
}

impl From<Error> for crate::root::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::ModelNotFound {
                model,
                search_key,
                search_value,
            } => Self::ModelFailure(ErrorContext::new(
                StatusCode::NOT_FOUND,
                ErrorDomain::Database,
                None,
                ClientError::RESOURCE_NOT_FOUND {
                    resource_name: model,
                    search_key,
                    search_value,
                },
            )),
            Error::DatabaseError(error) => {
                let db_error = error.into_database_error();

                if db_error.is_none() {
                    return Self::UnknownFailure(ErrorContext::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ErrorDomain::Database,
                        None,
                        ClientError::UNEXPECTED_SERVICE,
                    ));
                }

                let db_error = db_error.unwrap();
                let message = db_error.message();

                if db_error.is_unique_violation()
                    || db_error.is_foreign_key_violation()
                    || db_error.is_check_violation()
                {
                    return Self::ModelFailure(ErrorContext::new(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        ErrorDomain::Database,
                        None,
                        ClientError::VALIDATION_ERROR {
                            message: message.to_owned(),
                            errors: None,
                        },
                    ));
                }

                Self::UnknownFailure(ErrorContext::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorDomain::Database,
                    None,
                    ClientError::UNEXPECTED_SERVICE,
                ))
            }
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Error::DatabaseError(error)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for Error {}
