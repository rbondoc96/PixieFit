mod errors;
pub mod services;

use crate::data::{CreateUserData, CreateUserProfileData};
use crate::enums::Role;
use crate::models::{Exercise, Profile, User};
use crate::utils::{crypt, validators};
use database::DatabaseManager;

pub(self) use errors::Error;
pub(self) type Result<TValue> = core::result::Result<TValue, crate::http::Error>;

pub async fn create_user(data: CreateUserData<'_>, database: &DatabaseManager) -> Result<User> {
    use validators::ValidatorResult;

    if User::exists_with_email(data.email, database).await? {
        return Err(Error::UserWithEmailAlreadyExists)?;
    }

    if data.password != data.password_confirm {
        return Err(Error::PasswordMismatch)?;
    }

    if let ValidatorResult::Invalid(messages) = validators::password(data.password) {
        return Err(Error::InvalidPasswordFormat(messages))?;
    }

    let hash = crypt::encrypt(data.password)?;

    let user = User::new()
        .name(data.first_name, data.last_name)
        .email(data.email)
        .role(data.role.unwrap_or_default())
        .password(hash)
        .create(&database)
        .await?;

    Ok(user)
}

pub async fn create_user_profile(data: CreateUserProfileData, database: &DatabaseManager) -> Result<Profile> {
    let profile = Profile::new()
        .user_id(data.user_id)
        .birthday(data.birthday)
        .gender(data.gender)
        .create(&database)
        .await?;

    Ok(profile)
}
