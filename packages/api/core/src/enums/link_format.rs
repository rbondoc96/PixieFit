use database::impl_bindable;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::Display;

#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
#[strum(serialize_all = "snake_case")]
pub enum LinkFormat {
    Gif,
    Html,
    Jpeg,
    Mov,
    Mp3,
    Mp4,
    Png,
    Svg,
}

impl_bindable!(LinkFormat);

impl Default for LinkFormat {
    fn default() -> Self {
        Self::Jpeg
    }
}
