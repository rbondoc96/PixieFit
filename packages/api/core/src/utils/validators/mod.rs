mod password;

pub use password::password;

pub enum ValidatorResult {
    Valid,
    Invalid(Vec<String>),
}
