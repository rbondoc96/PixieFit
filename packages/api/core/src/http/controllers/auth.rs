use super::{Controller, Error, Result};
use crate::prelude::*;
use crate::actions;
use crate::data;
use crate::enums::{Gender, Role};
use crate::http::resources::{ModelResource, UserResource};
use crate::http::{Context, JsonResponse};
use crate::models::{User, Profile};
use crate::utils::{crypt, validators};
use axum::extract::State;
use axum::http::StatusCode;
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
    pub async fn ping(context: Option<Context>) -> Result<JsonResponse> {
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
    ) -> Result<JsonResponse> {
        println!("->> {:<12} - AuthController::index", "AUTH_INDEX");

        let context = context.ok_or(Error::RequestExtensionMissingContext)?;
        let user = context.user();

        Ok(JsonResponse::success(
            Some(UserResource::default(user.clone(), &database).await),
            StatusCode::CREATED,
        ))
    }

    pub async fn login(
        session: Session,
        State(database): State<DatabaseManager>,
        Json(payload): Json<LoginPayload>,
    ) -> Result<JsonResponse> {
        let mut user = User::find_by_email(payload.email, &database).await?;

        if !crypt::decrypt_and_verify(payload.password.as_str(), user.password.as_str())? {
            return Err(Error::UserLoginFailed)?;
        }

        user.update_last_logged_in(&database).await?;
        session.set("user_id", user.id);

        Ok(JsonResponse::success(
            Some(UserResource::default(user, &database).await),
            StatusCode::OK,
        ))
    }

    pub async fn logout(session: Session) -> Result<JsonResponse> {
        session.destroy();
        // Compiler deemed it necessary to add a type annotation
        // Due to the bound that success takes a parameter of Option<impl Serialize>
        Ok(JsonResponse::success(None::<()>, StatusCode::OK))
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

        Ok(JsonResponse::success(
            Some(UserResource::default(user, &database).await),
            StatusCode::CREATED,
        ))
    }
}
