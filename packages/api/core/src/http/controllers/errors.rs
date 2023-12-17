use crate::error::{ClientError, Domain};
use axum::http::StatusCode;

#[derive(Debug, strum_macros::Display, thiserror::Error)]
pub enum Error {
    NoMatchingCredentialsFound,
    RequestExtensionMissingContext,
    UnauthorizedUserRole,
}

impl From<Error> for crate::http::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::NoMatchingCredentialsFound => Self::bad_request(
                ClientError::InvalidCredentials,
                Domain::UserAuthentication,
            ),
            Error::RequestExtensionMissingContext => Self::not_authenticated(
                ClientError::NotAuthenticated,
                Domain::UserAuthentication,
            ),
            // TODO: Re-work
            Error::UnauthorizedUserRole => Self::forbidden(
                ClientError::UnauthorizedAction,
                Domain::UserAuthentication
            ),
        }
    }
}
