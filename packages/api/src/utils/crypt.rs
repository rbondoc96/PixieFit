use super::{Error, Result};

pub fn encrypt(value: &str) -> Result<String> {
    encrypt_with_cost(value, bcrypt::DEFAULT_COST)
}

pub fn encrypt_with_cost(value: &str, cost: u32) -> Result<String> {
    bcrypt::hash(value, cost).map_err(Error::StringEncryption)
}

pub fn decrypt_and_verify(value: &str, hash: &str) -> Result<bool> {
    bcrypt::verify(value, hash).map_err(Error::StringDecryption)
}
