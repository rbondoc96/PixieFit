#[derive(Clone, Debug, strum_macros::Display)]
pub enum ClientError {
    // Generic Errors
    Conflict,
    #[strum(serialize = "InternalError")]
    Internal,
    InvalidRequest,
    #[strum(serialize = "NetworkError")]
    Network,
    NotAuthenticated,
    RequestTooLarge,
    Unavailable,
    #[strum(serialize = "UnexpectedError")]
    /// A known one-in-a-million/impossible error, but it was somehow triggered.
    Unexpected,
    #[strum(serialize = "UnknownError")]
    /// An error that is completely unknown,
    Unknown,

    // Standard Errors
    ResourceNotFound,
    UnauthorizedAction,
    #[strum(serialize = "ValidationError")]
    Validation,

    // Specific Errors
    InvalidCredentials,
}
