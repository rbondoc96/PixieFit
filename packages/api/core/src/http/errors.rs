use crate::prelude::__;
use crate::error::{ClientError, Domain};
use axum::http::StatusCode;

#[derive(Debug, strum_macros::Display, thiserror::Error)]
pub enum Error {
    NoMatchingSessionUserFound,
    RequestExtensionMissingContext,
}

impl From<Error> for crate::http::Error {
    fn from(error: Error) -> Self {
        Self::not_authenticated(
            ClientError::NotAuthenticated,
            Domain::UserAuthentication,
        )
    }
}
