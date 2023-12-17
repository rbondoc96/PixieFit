use super::{ModelResource, ResourceResult};
use crate::enums::Gender;
use crate::models::Profile;
use async_trait::async_trait;
use database::{DatabaseManager, HasRouteKey, Model};
use serde::Serialize;

#[derive(Serialize)]
pub struct ProfileResource {
    id: String,
    gender: Gender,
    birthday: chrono::NaiveDate,
}

#[async_trait]
impl ModelResource for ProfileResource {
    type Model = Profile;

    async fn default(profile: Profile, database: &DatabaseManager) -> ResourceResult<Self> {
        Ok(Self {
            id: profile.route_key(),
            gender: profile.gender,
            birthday: profile.birthday,
        })
    }

    async fn simple(profile: Profile, database: &DatabaseManager) -> ResourceResult<Self> {
        Ok(Self {
            id: profile.route_key(),
            gender: profile.gender,
            birthday: profile.birthday,
        })
    }
}
