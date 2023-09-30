use serde::Serialize;

#[derive(Serialize)]
pub struct NameResource {
    first: String,
    full: String,
    last: String,
}

impl NameResource {
    pub fn new(first: String, last: String) -> Self {
        let full = format!("{} {}", first, last);

        Self { first, full, last }
    }
}
