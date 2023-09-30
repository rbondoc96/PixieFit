pub type DatabasePool = sqlx::postgres::PgPool;
pub type DynError = dyn std::error::Error + Send + Sync;
pub type ErrorMap = std::collections::HashMap<String, Vec<String>>;
pub type ISO8601DateTimeUTC = chrono::DateTime<chrono::Utc>;
