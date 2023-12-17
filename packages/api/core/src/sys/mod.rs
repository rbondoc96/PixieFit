mod config;
mod errors;

pub use config::config;
pub(self) use errors::Error;
pub(self) type Result<TValue> = ::core::result::Result<TValue, Error>;
