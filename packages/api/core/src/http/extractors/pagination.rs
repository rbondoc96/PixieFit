use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Pagination {
    page: u16,
    per_page: u8,
}

impl Pagination {
    pub fn page(&self) -> u16 {
        self.page
    }

    pub fn per_page(&self) -> u8 {
        self.per_page
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 10,
        }
    }
}

#[async_trait]
impl<TState> FromRequestParts<TState> for Pagination
where
    TState: Send + Sync,
{
    type Rejection = crate::error::Error;

    async fn from_request_parts(parts: &mut Parts, state: &TState) -> core::result::Result<Self, Self::Rejection> {
        let Query(pagination) = Query::<Pagination>::from_request_parts(parts, state)
            .await
            .unwrap_or_default();

        Ok(pagination)
    }
}
