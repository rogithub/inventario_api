use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::{Result, config::Config};

pub struct App;

impl App {
    pub async fn run() -> Result<()> {
        let config = Config::load()?;

        let router = Router::new().route("/hello", get(|| async { "Hello World!" }));

        let listener = TcpListener::bind(config.server().address()).await?;

        axum::serve(listener, router).await.map_err(Into::into)
    }
}

