use super::{Controller, Error, Result};
use crate::prelude::*;
use crate::{
    actions,
    data,
    enums::{Gender, Role},
    http::resources::{ModelResource, UserResource},
    http::{Context, JsonResponse},
    models::{CreateUserProfileData, NewUser, User, Profile},
    sys::DatabaseManager,
    utils::{crypt, validators},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post, Router},
};
use axum_session::SessionPgSession as Session;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub birthday: chrono::NaiveDate,
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
    type State = DatabaseManager;

    fn router(state: Self::State) -> Router {
        Router::new()
            .route("/register", post(Self::register))
            .route("/", get(Self::index).post(Self::login).delete(Self::logout))
            .route("/ping", get(Self::ping))
            .with_state(state)
    }
}

impl AuthController {
    pub async fn ping(context: Option<Context>) -> Result<JsonResponse<String>> {
        if context.is_none() {
            return Err(Error::RequestExtensionMissingContext.into());
        }

        Ok(JsonResponse::success(
            Some("ping-pong".to_string()),
            StatusCode::OK,
        ))
    }

    pub async fn index(
        context: Option<Context>,
        State(database): State<DatabaseManager>,
    ) -> Result<JsonResponse<UserResource>> {
        println!("->> {:<12} - AuthController::index", "AUTH_INDEX");

        let context = context.ok_or(Error::RequestExtensionMissingContext)?;
        let user = context.user();

        Ok(JsonResponse::success(
            Some(UserResource::default(user.clone()).await),
            StatusCode::CREATED,
        ))
    }

    pub async fn login(
        session: Session,
        State(database): State<DatabaseManager>,
        Json(payload): Json<LoginPayload>,
    ) -> Result<JsonResponse<UserResource>> {
        let mut user = User::find_by_email(payload.email, &database).await?;

        if !crypt::decrypt_and_verify(payload.password.as_str(), user.password().as_str())? {
            return Err(Error::UserLoginFailed)?;
        }

        user.update_last_logged_in(&database).await?;
        session.set("user_id", user.id());

        Ok(JsonResponse::success(
            Some(UserResource::default(user).await),
            StatusCode::OK,
        ))
    }

    pub async fn logout(session: Session) -> Result<JsonResponse<()>> {
        session.destroy();
        Ok(JsonResponse::success(None, StatusCode::OK))
    }

    pub async fn register(
        State(database): State<DatabaseManager>,
        Json(payload): Json<RegisterPayload>,
    ) -> Result<JsonResponse<UserResource>> {
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
                user_id: user.id(),
                birthday: payload.birthday,
                gender: payload.gender,
            },
            &database,
        ).await?;

        Ok(JsonResponse::success(
            Some(UserResource::default(user).await),
            StatusCode::CREATED,
        ))
    }
}
