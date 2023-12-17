#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ModelNotCreated(String),
    #[error("No {name} found where {search_key} = {search_value}.")]
    ModelNotFound {
        name: &'static str,
        table: &'static str,
        search_key: String,
        search_value: String,
    },
    #[error("An unknown error has occurred.")]
    Unknown(#[from] sqlx::Error),
}
