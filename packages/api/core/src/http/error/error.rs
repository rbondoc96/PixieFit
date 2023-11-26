use super::JsonResponse;
use crate::error::{ClientError, Domain};
use crate::types::{DynError, ErrorMap};
use crate::utils::__;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub struct Error {
    client: ClientError,
    code: StatusCode,
    domain: Domain,
    message: String,
    messages: Option<ErrorMap>,
    source: Option<Box<DynError>>,
}

impl Error {
    // region Private Static Methods

    fn new(
        client: ClientError,
        code: StatusCode,
        domain: Domain,
        message: impl ToString,
        messages: Option<ErrorMap>,
        source: Option<Box<DynError>>,
    ) -> Self {
        Self {
            client,
            code,
            domain,
            message: message.to_string(),
            messages,
            source,
        }
    }

    // endregion

    // region Static Instantiation Methods

    /// HTTP 400
    pub fn bad_request(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::BAD_REQUEST,
            domain,
            __("errors.generic.http.400"),
            None,
            None,
        )
    }

    /// HTTP 401
    pub fn not_authenticated(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::UNAUTHORIZED,
            domain,
            __("errors.generic.http.401"),
            None,
            None,
        )
    }

    /// HTTP 403
    pub fn forbidden(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::FORBIDDEN,
            domain,
            __("errors.generic.http.401"),
            None,
            None,
        )
    }

    /// HTTP 404
    pub fn not_found(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::NOT_FOUND,
            domain,
            __("errors.generic.http.404"),
            None,
            None,
        )
    }

    /// HTTP 409
    pub fn conflict(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::CONFLICT,
            domain,
            __("errors.generic.http.409"),
            None,
            None,
        )
    }

    /// HTTP 413
    pub fn payload_too_large(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::PAYLOAD_TOO_LARGE,
            domain,
            __("errors.generic.http.413"),
            None,
            None,
        )
    }

    /// HTTP 418
    pub fn im_a_teapot(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::IM_A_TEAPOT,
            domain,
            __("errors.generic.http.418"),
            None,
            None,
        )
    }

    /// HTTP 422
    pub fn unprocessable(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::UNPROCESSABLE_ENTITY,
            domain,
            __("errors.generic.http.422"),
            None,
            None,
        )
    }

    /// HTTP 429
    pub fn too_many_requests(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::TOO_MANY_REQUESTS,
            domain,
            __("errors.generic.http.429"),
            None,
            None,
        )
    }

    /// HTTP 500
    pub fn internal_error(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::INTERNAL_SERVER_ERROR,
            domain,
            __("errors.generic.http.500"),
            None,
            None,
        )
    }

    /// HTTP 502
    pub fn bad_gateway(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::BAD_GATEWAY,
            domain,
            __("errors.generic.http.502"),
            None,
            None,
        )
    }

    /// HTTP 503
    pub fn service_unavailable(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::SERVICE_UNAVAILABLE,
            domain,
            __("errors.generic.http.503"),
            None,
            None,
        )
    }

    /// HTTP 504
    pub fn gateway_timeout(client: ClientError, domain: Domain) -> Self {
        Self::new(
            client,
            StatusCode::GATEWAY_TIMEOUT,
            domain,
            __("errors.generic.http.504"),
            None,
            None,
        )
    }

    // endregion

    // region Instance Accessor Methods

    pub fn client(&self) -> ClientError {
        self.client.clone()
    }

    pub fn code(&self) -> StatusCode {
        self.code
    }

    pub fn domain(&self) -> Domain {
        self.domain.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn messages(&self) -> Option<ErrorMap> {
        self.messages.clone()
    }

    pub fn name(&self) -> String {
        self.client.to_string()
    }

    // endregion

    // region Instance Mutator Methods

    pub fn with_message(mut self, message: impl ToString) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn with_messages(mut self, messages: ErrorMap) -> Self {
        self.messages = Some(messages);
        self
    }

    pub fn with_source(mut self, source: Box<DynError>) -> Self {
        self.source = Some(source);
        self
    }

    // endregion
}

// region Trait Implementations

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::debug!("{:?}", &self);

        JsonResponse::error(self).into_response()
    }
}

impl ::core::fmt::Display for Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error> {
        write!(
            f,
            "HttpError {{ code: {}, message: {}, name: {} }}",
            self.code, self.message, self.client.to_string(),
        )
    }
}

impl From<database::Error> for Error {
    fn from(error: database::Error) -> Self {
        let err_pointer = &error;
        let message = err_pointer.to_string();

        Self::internal_error(
            ClientError::Unknown,
            Domain::Database,
        )
            .with_message(message)
            .with_source(Box::new(error))
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        let err_pointer = &error;
        let message = err_pointer.to_string();

        Self::internal_error(
            ClientError::Unknown,
            Domain::Database,
        )
            .with_message(message)
            .with_source(Box::new(error))
    }
}

// endregion
