use super::{ModelResource, NameResource, ProfileResource};
use crate::prelude::*;
use crate::{
    enums::Role,
    models::{Profile, User},
};
use async_trait::async_trait;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResource {
    name: NameResource,
    email: String,
    role: Role,

    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<ProfileResource>,

    last_logged_in_at: Option<ISO8601DateTimeUTC>,
    created_at: ISO8601DateTimeUTC,
    updated_at: ISO8601DateTimeUTC,
}

#[async_trait]
impl ModelResource for UserResource {
    type Model = User;

    async fn default(user: User) -> Self {
        match user.profile().await {
            Ok(profile) => {
                Self {
                    name: NameResource::new(user.first_name(), user.last_name()),
                    email: user.email(),
                    role: user.role(),
                    profile: Some(ProfileResource::default(profile).await),
                    last_logged_in_at: user.last_logged_in_at(),
                    created_at: user.created_at(),
                    updated_at: user.updated_at(),
                }
            }
            Err(_) => {
                Self::simple(user).await
            }
        }
    }

    async fn simple(user: User) -> Self {
        Self {
            name: NameResource::new(user.first_name(), user.last_name()),
            email: user.email(),
            role: user.role(),
            profile: None,
            last_logged_in_at: user.last_logged_in_at(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }
}
