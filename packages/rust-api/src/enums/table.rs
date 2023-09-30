use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "snake_case")]
pub enum Table {
    Links,
    Muscles,
    Users,
    UserProfiles,
}
