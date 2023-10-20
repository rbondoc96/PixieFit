use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    DatabasePoolCreationFailure(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
    MissingEnvironmentVariable(&'static str),
    StringParseFailure {
        parse_type: &'static str,
        value: String,
    },
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for Error {}
