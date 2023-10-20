use crate::models::User;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

pub(self) use crate::http::errors::Error;
pub(self) type Result<TValue> = ::core::result::Result<TValue, Error>;

#[derive(Clone, Debug)]
pub struct Context {
    user: User,
}

impl Context {
    pub fn new(user: User) -> Self {
        Self { user }
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}

/**
 * Takes information from the request headers.
 */
#[async_trait]
impl<TState> FromRequestParts<TState> for Context
where
    TState: Send + Sync,
{
    type Rejection = crate::error::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &TState) -> core::result::Result<Self, Self::Rejection> {
        println!(
            "->> {:>12} -- Context::from_request_parts",
            "CTX_FROM_REQ_PARTS"
        );

        parts
            .extensions
            .get::<Self>()
            .ok_or(Error::RequestExtensionMissingContext.into())
            .cloned()
    }
}
