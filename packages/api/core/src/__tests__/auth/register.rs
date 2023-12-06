use crate::__tests__::actions::auth as actions;
use crate::enums::Gender;
use crate::models::{Profile, User};
use crate::prelude::*;

#[sqlx::test]
async fn success(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let database = server.database();

    // Act
    let payload = json!({
        "birthday": "2000-01-01",
        "email": "test_user@example.com",
        "first_name": "MyFirstName",
        "last_name": "MyLastName",
        "gender": Gender::Male,
        "password": "#TestPassword1234",
        "password_confirm": "#TestPassword1234",
    });

    let response = actions::register(&server, payload).await;

    let user = User::find_by_email("test_user@example.com", database).await;
    let user_count = User::count(database).await;
    let profile_count = Profile::count(database).await;

    // Assert
    response.assert_created();
    assert_ok(&user);
    assert_eq!(User::count(server.database()).await?, 1);
    assert_eq!(Profile::count(server.database()).await?, 1);
    assert_eq!(Profile::find_by_user(user?.id, server.database()).await?
        .user(server.database()).await?.email,
        "test_user@example.com",
    );

    Ok(())
}

#[sqlx::test]
async fn fails_with_duplicate_email(pool: PgPool) -> Result<()> {
    // Arrange
    let server = MockServer::init(pool).await;
    let user = User::fake()
        .email("test_user@example.com")
        .create(server.database())
        .await?;

    // Act
    let payload = json!({
        "birthday": "2000-01-01",
        "email": "test_user@example.com",
        "first_name": "MyFirstName",
        "last_name": "MyLastName",
        "gender": Gender::Male,
        "password": "#TestPassword1234",
        "password_confirm": "#TestPassword1234",
    });

    let response = actions::register(&server, payload).await;

    // Assert
    response.assert_unprocessable();
    assert_eq!(User::count(server.database()).await?, 1);

    Ok(())
}

macro_rules! password_format_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[sqlx::test]
            async fn $name(pool: PgPool) -> Result<()> {
                // Arrange
                let server = MockServer::init(pool).await;

                // Act
                let payload = json!({
                    "birthday": "2000-01-01",
                    "email": "test_user@example.com",
                    "first_name": "MyFirstName",
                    "last_name": "MyLastName",
                    "gender": Gender::Male,
                    "password": $value,
                    "password_confirm": "#TestPassword1234",
                });

                let response = actions::register(&server, payload).await;

                // Assert
                response.assert_unprocessable();
                assert_eq!(User::count(server.database()).await?, 0);

                Ok(())
            }
        )*
    }
}

password_format_tests! {
    fails_with_mismatched_passwords: "#TestPassword9876",
    fails_if_less_than_8_characters: "IHave_7",
    fails_if_more_than_32_characters: "I______________Have____________33",
    fails_if_no_lowercase_character: "#I_HAVE_NO_1234_LOWERCASE",
    fails_if_no_uppercase_character: "#i_have_no_1234_uppercase",
    fails_if_no_numeric_character: "#i_have_NO_numbers",
    fails_if_no_special_character: "IHaveNo1234SpecialCharacters",
    fails_if_has_spaces: "#I_Have_A Space Somewhere",
}
