use std::fmt::{ self, Display };

#[derive(Debug)]
pub struct Report(pub color_eyre::Report);

impl <E> From<E> for Report
where
    E: Into<color_eyre::Report>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub type Result<T, E = Report> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Axum(#[from] axum::Error),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    DirectiveParseError(#[from] tracing_subscriber::filter::ParseError),
    #[error(transparent)]
    EnvFilter(#[from] std::env::VarError),
    #[error(transparent)]
    FromEnv(#[from] tracing_subscriber::filter::FromEnvError),
    #[error(transparent)]
    TryInit(#[from] tracing_subscriber::util::TryInitError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    Redis(#[from] redis::RedisError),

    #[error("Error occured when signing or verifying token")]
    TokenError,
    #[error(transparent)]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
}