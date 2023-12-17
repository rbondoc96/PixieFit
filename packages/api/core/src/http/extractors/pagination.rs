use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Pagination {
    page: i64,
    per_page: i64,
}

impl Pagination {
    pub fn limit(&self) -> i64 {
        self.per_page
    }

    pub fn offset(&self) -> i64 {
        self.per_page * (self.page - 1)
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
impl<S> FromRequestParts<S> for Pagination
where
    S: Send + Sync,
{
    type Rejection = crate::http::Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> core::result::Result<Self, Self::Rejection> {
        let Query(pagination) = Query::<Pagination>::from_request_parts(parts, state)
            .await
            .unwrap_or_default();

        Ok(pagination)
    }
}
