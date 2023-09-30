mod context;

pub use context::ErrorContext;

use crate::types::ErrorMap;
use crate::{JsonResponse, __};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::json;

#[derive(Clone, Debug, Serialize)]
pub enum ErrorDomain {
    Authentication,
    Database,
    Network,
    System,
    Validation,
    Other,
}

#[derive(Clone, Debug, Serialize)]
pub enum ErrorSubdomain {
    UserLogin,
    UserRegistration,
    Utilities,
}

#[derive(Clone, Debug, strum_macros::AsRefStr)]
pub enum Error {
    PasswordMismatch,
    InternalServer,
    InvalidPasswordFormat(ErrorMap),
    MissingEnvironmentVariable,
    MissingSessionUserID,
    NoMatchingSessionUserFound,
    PasswordEncryptionFailed,
    RequestContextMissingUser,
    RequestExtMissingContext,
    UserCreationFailed(String),
    UserLoginFailed,
    UserNotFound,

    // Repurposed errors?
    ModelFailure(ErrorContext),
    UnknownFailure(ErrorContext),
    ValidatorFailure(ErrorContext),
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, strum_macros::AsRefStr, Serialize)]
pub enum ClientError {
    INVALID_CREDENTIALS,
    NOT_AUTHENTICATED,
    INVALID_PASSWORD_FORMAT,
    PASSWORD_MISMATCH,
    RESOURCE_NOT_FOUND {
        resource_name: &'static str,
        search_key: String,
        search_value: String,
    },
    UNEXPECTED_SERVICE,
    VALIDATION_ERROR {
        message: String,
        errors: Option<ErrorMap>,
    },
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        println!("{}", error.to_string());

        Self::InternalServer
    }
}

impl From<crate::sys::Error> for Error {
    fn from(error: crate::sys::Error) -> Self {
        Error::InternalServer
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "ERROR_INTO_RESPONSE");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn to_error_context(&self) -> ErrorContext {
        match self {
            Self::InvalidPasswordFormat(errors) => ErrorContext::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorDomain::Validation,
                Some(ErrorSubdomain::UserRegistration),
                ClientError::VALIDATION_ERROR {
                    message: __("errors.validation.invalidPasswordFormat"),
                    errors: Some(errors.to_owned()),
                },
            ),
            Self::PasswordMismatch => ErrorContext::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorDomain::Validation,
                Some(ErrorSubdomain::UserRegistration),
                ClientError::VALIDATION_ERROR {
                    message: __("errors.validation.passwordMismatch"),
                    errors: None,
                },
            ),
            Self::MissingSessionUserID
            | Self::RequestContextMissingUser
            | Self::RequestExtMissingContext => ErrorContext::new(
                StatusCode::UNAUTHORIZED,
                ErrorDomain::Authentication,
                Some(ErrorSubdomain::UserLogin),
                ClientError::NOT_AUTHENTICATED,
            ),
            Self::UserCreationFailed(message) => ErrorContext::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorDomain::Database,
                Some(ErrorSubdomain::UserRegistration),
                ClientError::VALIDATION_ERROR {
                    message: message.to_owned(),
                    errors: None,
                },
            ),
            Self::UserLoginFailed | Self::UserNotFound => ErrorContext::new(
                StatusCode::UNAUTHORIZED,
                ErrorDomain::Authentication,
                Some(ErrorSubdomain::UserLogin),
                ClientError::INVALID_CREDENTIALS,
            ),
            Self::PasswordEncryptionFailed | Self::NoMatchingSessionUserFound => ErrorContext::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDomain::System,
                Some(ErrorSubdomain::UserLogin),
                ClientError::UNEXPECTED_SERVICE,
            ),
            Self::InternalServer => ErrorContext::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDomain::System,
                None,
                ClientError::UNEXPECTED_SERVICE,
            ),
            Self::MissingEnvironmentVariable => ErrorContext::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDomain::System,
                None,
                ClientError::UNEXPECTED_SERVICE,
            ),
            Self::ModelFailure(context)
            | Self::ValidatorFailure(context)
            | Self::UnknownFailure(context) => ErrorContext::new(
                context.code(),
                context.domain(),
                context.subdomain(),
                context.client_error(),
            ),
        }
    }
}

impl ClientError {
    pub fn errors(&self) -> Option<ErrorMap> {
        match self {
            Self::VALIDATION_ERROR { errors, .. } => errors.to_owned(),
            _ => None,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::INVALID_CREDENTIALS => __("errors.auth.invalidCredentials"),
            Self::NOT_AUTHENTICATED => __("errors.auth.notAuthenticated"),
            Self::INVALID_PASSWORD_FORMAT => __("errors.user.invalidPasswordFormat"),
            Self::PASSWORD_MISMATCH => __("errors.user.passwordMismatch"),
            Self::RESOURCE_NOT_FOUND { .. } => __("errors.general.resourceNotFound"),
            Self::UNEXPECTED_SERVICE => __("errors.unexpectedServiceError"),
            Self::VALIDATION_ERROR { message, .. } => message.clone(),
        }
    }
}
