use axum::http::StatusCode;
use crate::error::ClientError;

#[derive(Debug, strum_macros::Display)]
pub enum Error {
    RequestExtensionMissingContext,
    UserLoginFailed,
}

impl From<Error> for crate::error::Error {
    fn from(error: Error) -> Self {
        let name = error.to_string();
        match error {
            Error::RequestExtensionMissingContext => Self::new(
                StatusCode::UNAUTHORIZED,
                "Not authenticated",
                None,
                name,
                ClientError::NotAuthenticated,
            ),
            Error::UserLoginFailed => Self::new(
                StatusCode::BAD_REQUEST,
                "",
                None,
                name,
                ClientError::InvalidCredentials,
            ),
        }
    }
}
