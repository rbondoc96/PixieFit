use crate::{
    data::{CreateUserData, CreateUserProfileData},
    enums::Role,
    models::{CreateUserProfileData as NewUserProfile, NewUser, User, Profile},
    sys::DatabaseManager,
    utils::{crypt, validators},
    Error, Result, __,
};

pub async fn create_user(data: CreateUserData<'_>, database: &DatabaseManager) -> Result<User> {
    if User::exists(data.email, database).await? {
        return Err(Error::UserCreationFailed(__(
            "errors.user.emailAlreadyExists",
        )));
    }

    if data.password != data.password_confirm {
        return Err(Error::PasswordMismatch);
    }

    validators::validate_password(data.password)?;

    let hash = crypt::encrypt(data.password)?;

    User::create(
        NewUser {
            email: data.email.to_string(),
            first_name: data.first_name.to_string(),
            last_name: data.last_name.to_string(),
            role: data.role.unwrap_or(Role::User),
            password: hash,
        },
        database,
    )
    .await
    .map_err(|error| {
        error.into_database_error()
            .map(|db_error| Error::UserCreationFailed(db_error.message().to_string()))
            .unwrap_or(Error::InternalServer)
    })
}

pub async fn create_user_profile(data: CreateUserProfileData, database: &DatabaseManager) -> Result<Profile> {
    Profile::create(
        NewUserProfile {
            user_id: data.user_id,
            birthday: data.birthday,
            gender: data.gender,
        },
        database,
    )
    .await
    .map_err(|error| {
        error.into_database_error()
            .map(|db_error| Error::UserCreationFailed(db_error.message().to_string()))
            .unwrap_or(Error::InternalServer)
    })
}