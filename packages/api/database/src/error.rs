#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database pool creation failure: {0}")]
    DatabasePoolCreationFailure(#[from] sqlx::Error),
}
