use super::{Error, Result};
use std::sync::OnceLock;

fn env(key: &'static str) -> Result<String> {
    std::env::var(key).map_err(|_| Error::MissingEnvironmentVariable(key))
}

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();

    CONFIG.get_or_init(|| {
        dotenvy::dotenv().ok();
        Config::load().expect("Failed to load config")
    })
}

pub struct Config {
    auth: AuthenticationConfig,
    database: DatabaseConfig,
    server: ServerConfig,
}

pub struct AuthenticationConfig {
    admin_password: String,
    admin_username: String,
    session_cookie_max_age_seconds: i64,
    session_cookie_name: String,
    session_database_table_name: String,
    session_secret: String,
}

pub struct DatabaseConfig {
    max_pool: u32,
    min_pool: u32,
    url: String,
}

pub struct ServerConfig {
    port: u16,
    should_sync_exercises: bool,
}

impl AuthenticationConfig {
    pub fn new(
        admin_password: String,
        admin_username: String,
        session_cookie_max_age_seconds: String,
        session_cookie_name: String,
        session_database_table_name: String,
        session_secret: String,
    ) -> Result<Self> {
        Ok(Self {
            admin_password,
            admin_username,
            session_cookie_max_age_seconds: session_cookie_max_age_seconds.parse::<i64>().map_err(
                |_| Error::StringParseFailure {
                    parse_type: "i64",
                    value: session_cookie_max_age_seconds,
                },
            )?,
            session_cookie_name,
            session_database_table_name,
            session_secret,
        })
    }

    pub fn admin_password(&self) -> String {
        self.admin_password.clone()
    }

    pub fn admin_username(&self) -> String {
        self.admin_username.clone()
    }

    pub fn session_cookie_max_age_seconds(&self) -> i64 {
        self.session_cookie_max_age_seconds
    }

    pub fn session_cookie_name(&self) -> &str {
        self.session_cookie_name.as_ref()
    }

    pub fn session_database_table_name(&self) -> &str {
        self.session_database_table_name.as_ref()
    }

    pub fn session_secret(&self) -> &str {
        self.session_secret.as_ref()
    }
}

impl DatabaseConfig {
    pub fn new(url: String, max_pool: String, min_pool: String) -> Result<Self> {
        Ok(Self {
            url,
            max_pool: max_pool
                .parse::<u32>()
                .map_err(|_| Error::StringParseFailure {
                    parse_type: "u32",
                    value: max_pool,
                })?,
            min_pool: min_pool
                .parse::<u32>()
                .map_err(|_| Error::StringParseFailure {
                    parse_type: "u32",
                    value: min_pool,
                })?,
        })
    }

    pub fn url(&self) -> &str {
        self.url.as_ref()
    }

    pub fn max_connections(&self) -> u32 {
        self.max_pool
    }

    pub fn min_connections(&self) -> u32 {
        self.min_pool
    }
}

impl ServerConfig {
    pub fn new(port: String, should_sync_exercises: String) -> Result<Self> {
        Ok(Self {
            port: port.parse::<u16>().map_err(|_| Error::StringParseFailure {
                parse_type: "u16",
                value: port,
            })?,
            should_sync_exercises: should_sync_exercises.to_lowercase() == "true"
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn should_sync_exercises(&self) -> bool {
        self.should_sync_exercises
    }
}

impl Config {
    fn load() -> Result<Self> {
        Ok(Self {
            auth: AuthenticationConfig::new(
                env("ADMIN_PASSWORD")?,
                env("ADMIN_USERNAME")?,
                env("SESSION_COOKIE_MAX_AGE_SECONDS")?,
                env("SESSION_COOKIE_NAME")?,
                env("SESSION_DATABASE_TABLE_NAME")?,
                env("SESSION_SECRET")?,
            )?,
            database: DatabaseConfig::new(
                env("DATABASE_URL")?,
                env("DATABASE_MAX_POOL")?,
                env("DATABASE_MIN_POOL")?,
            )?,
            server: ServerConfig::new(
                env("SERVER_PORT")?,
                env("SHOULD_SYNC_EXERCISES")?,
            )?,
        })
    }

    pub fn auth(&self) -> &AuthenticationConfig {
        &self.auth
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
}
