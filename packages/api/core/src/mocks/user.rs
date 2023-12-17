use crate::models::{Profile, User};
use crate::prelude::*;

pub struct MockUser {
    password: &'static str,
    user: User,
}

impl MockUser {
    pub async fn create_admin(database: &DatabaseManager) -> Self {
        let user = User::fake()
            .admin()
            .password("#Password1234")
            .create(database)
            .await
            .unwrap();

        Self {
            password: "#Password1234",
            user,
        }
    }

    pub async fn create(database: &DatabaseManager) -> Self {
        let user = User::fake()
            .password("#Password1234")
            .create(database)
            .await
            .unwrap();

        let profile = Profile::fake()
            .user(&user)
            .create(database)
            .await
            .unwrap();

        Self {
            password: "#Password1234",
            user,
        }
    }

    pub fn email(&self) -> &str {
        self.user.email.as_str()
    }

    pub fn password(&self) -> &'static str {
        self.password
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}
