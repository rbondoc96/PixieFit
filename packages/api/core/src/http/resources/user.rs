use super::{ModelResource, NameResource, ProfileResource, ResourceResult};
use crate::prelude::*;
use crate::enums::Role;
use crate::models::{Profile, User};
use async_trait::async_trait;
use database::DatabaseManager;
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

    async fn default(user: User, database: &DatabaseManager) -> ResourceResult<Self> {
        match user.role {
            Role::User => {
                let profile = ProfileResource::default(
                    user.profile(database).await?,
                    database,
                ).await?;

                Ok(Self {
                    name: NameResource::new(user.first_name, user.last_name),
                    email: user.email,
                    role: user.role,
                    profile: Some(profile),
                    last_logged_in_at: user.last_logged_in_at,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                })
            }
            Role::Admin => {
                Self::simple(user, database).await
            }
        }
    }

    async fn simple(user: User, database: &DatabaseManager) -> ResourceResult<Self> {
        Ok(Self {
            name: NameResource::new(user.first_name, user.last_name),
            email: user.email,
            role: user.role,
            profile: None,
            last_logged_in_at: user.last_logged_in_at,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }
}
