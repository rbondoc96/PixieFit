use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::models::User;
use crate::{Error, Result};

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
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &TState) -> Result<Self> {
        println!(
            "->> {:>12} -- Context::from_request_parts",
            "CTX_FROM_REQ_PARTS"
        );

        parts
            .extensions
            .get::<Self>()
            .ok_or(Error::RequestExtMissingContext)
            .cloned()
    }
}
