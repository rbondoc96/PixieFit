use super::controllers::{
    AuthController,
    Controller,
    DevController,
    ExerciseController,
    ExerciseEquipmentController,
    LinkController,
    MuscleController,
    MuscleGroupController,
};
use crate::{
    actions,
    data::CreateUserData,
    enums::Role,
    sys::{config, DatabaseManager},
};
use axum::{
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    http::{HeaderValue, Method},
    middleware, Router,
};
use axum_session::{
    Key, SecurityMode, SessionConfig, SessionLayer, SessionPgPool, SessionPgSession,
    SessionPgSessionStore,
};
use chrono::{Duration, NaiveDate};
use tower_http::cors::{Any, CorsLayer};

type Result<TValue> = ::core::result::Result<TValue, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub async fn create_admin_user(database: &DatabaseManager) -> () {
    let auth = config().auth();
    let username = auth.admin_username();
    let password = auth.admin_password();

    let admin_data = CreateUserData {
        email: username.as_str(),
        first_name: "Admin",
        last_name: "User",
        role: Some(Role::Admin),
        password: password.as_str(),
        password_confirm: password.as_str(),
    };

    match actions::create_user(admin_data, database).await {
        Ok(_) => println!("Created admin user"),
        Err(err) => {
            println!("Admin user not created: {}", err);
        }
    }
}

pub async fn init() -> Result<Router> {
    let server = config().server();

    let db_manager = DatabaseManager::new().await.unwrap_or_else(|_| {
        panic!("Could not initialize database manager");
    });

    let cors = cors()?;
    let session = session(db_manager.clone()).await?;

    create_admin_user(&db_manager).await;

    if server.should_sync_exercises() {
        actions::services::init(&db_manager).await;
    }

    // Note: `.layer()` calls are executed from bottom-to-top
    Ok(Router::new()
        .merge(DevController::router(db_manager.clone()))
        .nest(
            "/api/exercises",
            ExerciseController::router(db_manager.clone())
                .route_layer(middleware::from_fn(crate::http::middleware::require_auth)),
        )
        .nest(
            "/api/exercise-equipment",
            ExerciseEquipmentController::router(db_manager.clone())
                .route_layer(middleware::from_fn(crate::http::middleware::require_auth)),
        )
        .nest(
            "/api/links",
            LinkController::router(db_manager.clone())
                .route_layer(middleware::from_fn(crate::http::middleware::require_auth)),
        )
        .nest(
            "/api/muscle-groups",
            MuscleGroupController::router(db_manager.clone())
                .route_layer(middleware::from_fn(crate::http::middleware::require_auth)),
        )
        .nest(
            "/api/muscles",
            MuscleController::router(db_manager.clone())
                .route_layer(middleware::from_fn(crate::http::middleware::require_auth)),
        )
        .nest("/api/auth", AuthController::router(db_manager.clone()))
        // .layer(middleware::map_response(
        //     crate::http::middleware::response_mapper,
        // ))
        .layer(middleware::from_fn_with_state(
            db_manager.clone(),
            crate::http::middleware::context_resolver,
        ))
        .layer(session)
        .layer(cors))
}

fn cors() -> Result<CorsLayer> {
    Ok(CorsLayer::new()
        .allow_credentials(true)
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE])
        .allow_methods([
            Method::DELETE,
            Method::GET,
            Method::OPTIONS,
            Method::PATCH,
            Method::POST,
            Method::PUT,
        ])
        .allow_origin("http://localhost:3000".parse::<HeaderValue>()?))
}

async fn session(database: DatabaseManager) -> Result<SessionLayer<SessionPgPool>> {
    let auth_config = config().auth();

    /**
     * Lifetime is the amount of time session data is valid for in Memory
     *
     * If I log in, the session data is only valid for X amount of lifetime,
     * even if I perform actions.
     *
     * Once the lifetime expires, the session data is removed from memory and
     * the database. This does not delete the session from the database.
     *
     * If the lifetime hasn't expired, and the session has been deleted from the database,
     * actions hidden behind authentication will still succeed, because the session
     * data is still in Memory. Therefore, the user_id in Context can be used to grab
     * a user from the database.
     *
     * (The above could be solved by checking the database for the session ID, how expensive is that?)
     *
     * Correct configs should be used to make sure that the session is deleted from the DB
     * after or at the same time that the lifetime is set to expire.
     *
     * In the session table:
     * - `expires` is the time that the session expires, related to the lifetime
     * - `autoremove` is the time that the session is removed from the database, related to cookie_age
     */
    let session_config = SessionConfig::default()
        .with_cookie_name(auth_config.session_cookie_name())
        .with_lifetime(Duration::seconds(auth_config.session_cookie_max_age_seconds()))
        .with_max_age(Some(Duration::seconds(
            auth_config.session_cookie_max_age_seconds(),
        )))
        .with_table_name(auth_config.session_database_table_name())
        // TODO: Keep commented so we know which session ID is which in the database
        // .with_key(Key::from(config::session::SESSION_SECRET.as_bytes()))
        .with_security_mode(SecurityMode::Simple);

    // Initialize database tables
    let session_store =
        SessionPgSessionStore::new(Some(database.connection().clone().into()), session_config)
            .await?;

    Ok(SessionLayer::new(session_store))
}
