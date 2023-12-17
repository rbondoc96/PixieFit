use database::impl_bindable;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::Display;

#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
#[strum(serialize_all = "snake_case")]
pub enum LinkType {
    Image,
    Video,
    Webpage,
}

impl_bindable!(LinkType);

impl Default for LinkType {
    fn default() -> Self {
        Self::Image
    }
}
