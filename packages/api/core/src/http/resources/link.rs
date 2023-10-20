use super::ModelResource;
use crate::prelude::*;
use crate::{
    enums::{LinkFormat, LinkType},
    models::Link,
};
use async_trait::async_trait;
use serde::Serialize;

#[derive(Serialize)]
pub struct LinkResource {
    id: String,
    #[serde(rename = "type")]
    link_type: LinkType,
    format: LinkFormat,
    label: String,
    description: Option<String>,
    src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<ISO8601DateTimeUTC>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<ISO8601DateTimeUTC>,
}

#[async_trait]
impl ModelResource for LinkResource {
    type Model = Link;

    async fn default(link: Link) -> Self {
        Self {
            id: link.route_key(),
            link_type: link.link_type(),
            format: link.format(),
            label: link.label(),
            description: link.description(),
            src: link.src(),
            created_at: Some(link.created_at()),
            updated_at: Some(link.updated_at()),
        }
    }

    async fn simple(link: Link) -> Self {
        Self {
            id: link.route_key(),
            link_type: link.link_type(),
            format: link.format(),
            label: link.label(),
            description: link.description(),
            src: link.src(),
            created_at: None,
            updated_at: None,
        }
    }
}
