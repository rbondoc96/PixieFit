mod config;
pub mod database;
mod errors;

pub use config::config;
pub use database::DatabaseManager;
pub use errors::Error;
pub(self) type Result<TValue> = ::core::result::Result<TValue, Error>;
