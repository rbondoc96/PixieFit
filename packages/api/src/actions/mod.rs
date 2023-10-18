mod errors;

use crate::{
    data::{CreateUserData, CreateUserProfileData},
    enums::Role,
    models::{
        CreateExerciseData,
        CreateUserProfileData as NewUserProfile,
        Exercise,
        NewUser,
        User,
        Profile,
    },
    sys::DatabaseManager,
    utils::{crypt, validators},
};

pub(self) use errors::Error;
pub(self) type Result<TValue> = core::result::Result<TValue, crate::error::Error>;

pub async fn create_user(data: CreateUserData<'_>, database: &DatabaseManager) -> Result<User> {
    if User::exists(data.email, database).await? {
        return Err(Error::UserWithEmailAlreadyExists)?;
    }

    if data.password != data.password_confirm {
        return Err(Error::PasswordMismatch)?;
    }

    validators::validate_password(data.password)?;

    let hash = crypt::encrypt(data.password)?;

    let user = User::create(
        NewUser {
            email: data.email.to_string(),
            first_name: data.first_name.to_string(),
            last_name: data.last_name.to_string(),
            role: data.role.unwrap_or(Role::User),
            password: hash,
        },
        database,
    )
    .await?;

    Ok(user)
}

pub async fn create_user_profile(data: CreateUserProfileData, database: &DatabaseManager) -> Result<Profile> {
    let profile = Profile::create(
        NewUserProfile {
            user_id: data.user_id,
            birthday: data.birthday,
            gender: data.gender,
        },
        database,
    )
    .await?;

    Ok(profile)
}
