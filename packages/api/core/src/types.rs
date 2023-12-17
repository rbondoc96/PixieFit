pub type BoxDynError = Box<DynError>;
pub type DynError = dyn std::error::Error + Send + Sync + 'static;
pub type ErrorMap = std::collections::HashMap<String, Vec<String>>;
pub type ISO8601DateTimeUTC = chrono::DateTime<chrono::Utc>;
