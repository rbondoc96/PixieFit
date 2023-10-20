use crate::prelude::__;
use crate::error::ClientError;
use axum::http::StatusCode;

#[derive(Debug, strum_macros::Display)]
pub enum Error {
    NoMatchingSessionUserFound,
    RequestExtensionMissingContext,
}

impl From<Error> for crate::error::Error {
    fn from(error: Error) -> Self {
        let name = error.to_string();

        match error {
            Error::NoMatchingSessionUserFound
            | Error::RequestExtensionMissingContext=> Self::new(
                StatusCode::UNAUTHORIZED,
                __("error.auth.notAuthenticated"),
                None,
                name,
                ClientError::NotAuthenticated,
            )
        }
    }
}
