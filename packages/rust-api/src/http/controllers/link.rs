use super::{Controller, Result};
use crate::prelude::*;
use crate::{
    http::resources::{LinkResource, ModelResource},
    http::response::JsonResponse,
    models::{CreateLinkData, Link, Model},
    sys::DatabaseManager,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post, Router},
};

pub struct LinkController;

impl Controller for LinkController {
    type State = DatabaseManager;

    fn router(state: Self::State) -> Router {
        Router::new()
            .route("/", get(Self::list).post(Self::create))
            .with_state(state)
    }
}

type LinkList = Vec<LinkResource>;

impl LinkController {
    pub async fn list(State(database): State<DatabaseManager>) -> Result<JsonResponse<LinkList>> {
        println!("->> {:<12} - LinkController::list", "Link_LIST");

        let links = Link::all(&database).await?;

        Ok(JsonResponse::success(
            Some(LinkResource::list(links).await),
            StatusCode::OK,
        ))
    }

    pub async fn create(
        State(database): State<DatabaseManager>,
        Json(payload): Json<CreateLinkData>,
    ) -> Result<JsonResponse<LinkResource>> {
        let link = Link::create(payload, &database).await?;

        Ok(JsonResponse::success(
            Some(LinkResource::default(link).await),
            StatusCode::CREATED,
        ))
    }
}
