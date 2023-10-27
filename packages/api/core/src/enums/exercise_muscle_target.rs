use database::impl_bindable;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use sqlx::postgres::{PgArguments, Postgres};
use sqlx::query::Query;
use strum_macros::Display;

#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
#[strum(serialize_all = "snake_case")]
pub enum ExerciseMuscleTarget {
    Primary,
    Secondary,
    Tertiary,
}

impl_bindable!(ExerciseMuscleTarget);

impl Default for ExerciseMuscleTarget {
    fn default() -> Self {
        Self::Primary
    }
}
