pub mod log;
pub mod db;
pub mod auth;

use serde::Deserialize;
use crate::Result;
use crate::config::log::Logger;
use crate::config::db::{DatabaseConfig, RedisConfig};
use crate::config::auth::AuthConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn url(&self) -> String {
        format!("{}://{}", self.protocol, self.address())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    server: ServerConfig,
    log: Logger,
    database: DatabaseConfig,
    redis: RedisConfig,
    auth: AuthConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        let env = Environment::current();
        Self::from_env(&env)
    }

    /// Load configuration from a specific environment
    ///
    /// If environment variables are set with prefix APP_, it will also read them
    /// e.g APP_CONFIG__PORT=8080
    pub fn from_env(env: &Environment) -> Result<Self> {
        let base_dir = std::env::current_dir()?;
        let config_dir = base_dir.join("config");

        let file_name = format!("{}.yaml", env);

        let settings = config::Config::builder()
            .add_source(config::File::from(config_dir.join(file_name)))
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("__")
                    .prefix_separator("_")
                    .try_parsing(true),
            )
            .build()?;

        settings.try_deserialize().map_err(Into::into)
    }  
    
    pub fn log(&self) -> &Logger {
        &self.log
    }

    pub fn redis(&self) -> &RedisConfig {
        &self.redis
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }  

    pub fn auth(&self) -> &auth::AuthConfig {
        &self.auth
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub enum Environment {
    #[default]
    Development,
    Production,
    Testing,
    Other(String),
}

impl Environment {
    pub fn current() -> Self {
        std::env::var("APP_ENVIRONMENT")
            .or_else(|_| std::env::var("APP_ENV"))
            .map(|s| Self::from(s.as_str()))
            .unwrap_or_default()
    }
}

impl From<&str> for Environment {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Environment::Development,
            "production" | "prod" => Environment::Production,
            "testing" | "test" => Environment::Testing,
            other => Environment::Other(other.to_string()),
        }
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Environment::Development => "development",
            Environment::Production => "production",
            Environment::Testing => "testing",
            Environment::Other(other) => other.as_str(),
        };
        write!(f, "{}", s)
    }
}