use crate::error::{ClientError, Domain};
use bcrypt::BcryptError;

#[derive(Debug, strum_macros::Display, thiserror::Error)]
pub enum Error {
    StringDecryption(BcryptError),
    StringEncryption(BcryptError),
}

impl From<Error> for crate::http::Error {
    fn from(error: Error) -> Self {
        let message = match error {
            Error::StringDecryption(error) => error.to_string(),
            Error::StringEncryption(error) => error.to_string(),
        };

        Self::internal_error(ClientError::Internal, Domain::SystemUtilities)
            .with_message(message)
    }
}
