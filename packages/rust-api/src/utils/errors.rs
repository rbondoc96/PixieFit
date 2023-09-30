use crate::root::errors::{ClientError, ErrorContext, ErrorDomain, ErrorSubdomain};
use crate::__;
use axum::http::StatusCode;
use bcrypt::BcryptError;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    StringDecryption(BcryptError),
    StringEncryption(BcryptError),
    InvalidPasswordFormat(Vec<String>),
}

impl From<Error> for crate::root::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::InvalidPasswordFormat(messages) => {
                let mut errors = HashMap::with_capacity(1);
                errors.insert("password".to_owned(), messages);

                Self::ValidatorFailure(ErrorContext::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    ErrorDomain::Validation,
                    Some(ErrorSubdomain::UserRegistration),
                    ClientError::VALIDATION_ERROR {
                        message: __("errors.validation.invalidPasswordFormat"),
                        errors: Some(errors),
                    },
                ))
            }
            _ => Self::UnknownFailure(ErrorContext::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDomain::System,
                Some(ErrorSubdomain::Utilities),
                ClientError::UNEXPECTED_SERVICE,
            )),
        }
    }
}
