use super::{Controller, Error, Result};
use crate::actions;
use crate::data;
use crate::enums::{Gender, Role};
use crate::http::resources::{ModelResource, UserResource};
use crate::http::{Context, JsonResponse};
use crate::models::{User, Profile};
use crate::prelude::*;
use crate::utils::{crypt, validators};
use axum::extract::State;
use axum::response::Json;
use axum::routing::{get, post, Router};
use axum_session::SessionPgSession as Session;
use chrono::NaiveDate;
use database::{DatabaseManager, Model};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub birthday: NaiveDate,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
    pub role: Option<Role>,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub struct AuthController;

impl Controller for AuthController {
    fn router(state: DatabaseManager) -> Router {
        Router::new()
            .route("/register", post(Self::register))
            .route("/", get(Self::index).post(Self::login).delete(Self::logout))
            .route("/admin", get(Self::index).post(Self::admin_login).delete(Self::logout))
            .route("/ping", get(Self::ping))
            .with_state(state)
    }
}

impl AuthController {
    pub async fn ping(context: Option<Context>) -> Result<JsonResponse> {
        let result = context.ok_or(Error::RequestExtensionMissingContext)
            .map(|_| {
                JsonResponse::ok()
                    .with_data("pong")
            })?;

        Ok(result)
    }

    pub async fn index(
        session: Session,
        context: Option<Context>,
        State(database): State<DatabaseManager>,
    ) -> Result<JsonResponse> {
        tracing::debug!("->> {:<12} - AuthController::index", "AUTH_INDEX");

        let user = context.ok_or(Error::RequestExtensionMissingContext)
            .map(|context| {
                session.renew();
                context.user().clone()
            })?;

        Ok(JsonResponse::ok()
            .with_data(UserResource::default(user, &database).await?)
        )
    }

    pub async fn login(
        session: Session,
        State(database): State<DatabaseManager>,
        Json(payload): Json<LoginPayload>,
    ) -> Result<JsonResponse> {
        let mut user = User::find_by_email(payload.email, &database)
            .await
            .map_err(|_| Error::NoMatchingCredentialsFound)?;

        if !crypt::decrypt_and_verify(payload.password.as_str(), user.password.as_str())? {
            return Err(Error::NoMatchingCredentialsFound)?;
        }

        if user.role == Role::Admin {
            return Err(Error::UnauthorizedUserRole)?;
        }

        user.update_last_logged_in(&database).await?;
        session.set("user_id", user.id);

        Ok(JsonResponse::ok()
            .with_data(UserResource::default(user, &database).await?)
        )
    }

    pub async fn admin_login(
        session: Session,
        State(database): State<DatabaseManager>,
        Json(payload): Json<LoginPayload>,
    ) -> Result<JsonResponse> {
        let mut user = User::find_by_email(&payload.email, &database)
            .await
            .map_err(|_| Error::NoMatchingCredentialsFound)?;

        if !crypt::decrypt_and_verify(payload.password.as_str(), user.password.as_str())? {
            return Err(Error::NoMatchingCredentialsFound)?;
        }

        if user.role != Role::Admin {
            return Err(Error::UnauthorizedUserRole)?;
        }

        user.update_last_logged_in(&database).await?;
        session.set("user_id", user.id);

        Ok(JsonResponse::ok()
            .with_data(UserResource::default(user, &database).await?)
        )
    }

    pub async fn logout(session: Session, context: Option<Context>) -> Result<JsonResponse> {
        context.ok_or(Error::RequestExtensionMissingContext)?;

        session.destroy();
        Ok(JsonResponse::ok())
    }

    pub async fn register(
        State(database): State<DatabaseManager>,
        Json(payload): Json<RegisterPayload>,
    ) -> Result<JsonResponse> {
        let user = actions::create_user(
            data::CreateUserData {
                email: payload.email.as_str(),
                first_name: payload.first_name.as_str(),
                last_name: payload.last_name.as_str(),
                role: payload.role,
                password: payload.password.as_str(),
                password_confirm: payload.password_confirm.as_str(),
            },
            &database,
        ).await?;

        let _profile = actions::create_user_profile(
            data::CreateUserProfileData {
                user_id: user.id,
                birthday: payload.birthday,
                gender: payload.gender,
            },
            &database,
        ).await?;

        Ok(JsonResponse::created()
            .with_data(UserResource::default(user, &database).await?)
        )
    }
}
