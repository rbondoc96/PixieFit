#[derive(Clone, Debug, strum_macros::Display)]
pub enum Domain {
    Database,
    SystemUtilities,
    UserAuthentication,
    UserRegistration,
}
