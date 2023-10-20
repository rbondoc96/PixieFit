#[derive(Debug, strum_macros::Display)]
pub enum ClientError {
    InvalidCredentials,
    InvalidPasswordFormat,
    NotAuthenticated,
    ResourceNotFound,
    UnexpectedSystemFailure,
    UnknownSystemFailure,
    ValidationError,
}