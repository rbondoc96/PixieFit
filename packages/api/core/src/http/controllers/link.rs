use super::{Controller, Result};
use crate::prelude::*;
use crate::enums::{LinkFormat, LinkType, Table};
use crate::http::resources::{LinkResource, ModelResource};
use crate::http::response::JsonResponse;
use crate::models::Link;
use axum::extract::State;
use axum::response::Json;
use axum::routing::{get, post, Router};
use database::{DatabaseManager, Model};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateLinkPayload {
    model_name: Table,
    model_id: i16,
    #[serde(rename = "type")]
    link_type: LinkType,
    format: LinkFormat,
    label: String,
    description: Option<String>,
    src: String,
}

pub struct LinkController;

impl Controller for LinkController {
    fn router(state: DatabaseManager) -> Router {
        Router::new()
            .route("/", get(Self::list).post(Self::create))
            .with_state(state)
    }
}

type LinkList = Vec<LinkResource>;

impl LinkController {
    pub async fn list(State(database): State<DatabaseManager>) -> Result<JsonResponse> {
        println!("->> {:<12} - LinkController::list", "Link_LIST");

        let links = Link::all(&database).await?;

        Ok(JsonResponse::ok()
            .with_data(LinkResource::list(links, &database).await?)
        )
    }

    pub async fn create(
        State(database): State<DatabaseManager>,
        Json(payload): Json<CreateLinkPayload>,
    ) -> Result<JsonResponse> {
        let link = Link::new()
            .model(payload.model_name, payload.model_id)
            .link_type(payload.link_type)
            .format(payload.format)
            .label(payload.label)
            .description(payload.description)
            .src(payload.src)
            .create(&database)
            .await?;

        Ok(JsonResponse::created()
            .with_data(LinkResource::default(link, &database).await?)
        )
    }
}
