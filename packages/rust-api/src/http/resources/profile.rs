use super::ModelResource;
use crate::{
    enums::Gender,
    models::Profile,
};
use async_trait::async_trait;
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

    async fn default(profile: Profile) -> Self {
        Self {
            id: profile.route_key(),
            gender: profile.gender(),
            birthday: profile.birthday(),
        }
    }

    async fn simple(profile: Profile) -> Self {
        Self {
            id: profile.route_key(),
            gender: profile.gender(),
            birthday: profile.birthday(),
        }
    }
}
