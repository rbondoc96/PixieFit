#[derive(Debug, strum_macros::Display)]
pub enum ClientError {
    ActionNotAuthorized,
    InvalidCredentials,
    InvalidPasswordFormat,
    NotAuthenticated,
    ResourceNotFound,
    UnexpectedSystemFailure,
    UnknownSystemFailure,
    ValidationError,
}