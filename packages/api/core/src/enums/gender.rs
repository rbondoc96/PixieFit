use database::impl_bindable;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::Display;

#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
#[strum(serialize_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
    NonBinary,
    Other,
}

impl_bindable!(Gender);

impl Default for Gender {
    fn default() -> Self {
        Self::Other
    }
}
