use std::{io::IsTerminal, sync::Arc};
use axum::{Router, routing::get};
use color_eyre::config::{HookBuilder, Theme};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{Result, config::Config, context::AppContext, middleware::trace,};

pub struct App;

impl App {
    pub async fn run() -> Result<()> {
        
        HookBuilder::default().theme(if std::io::stderr().is_terminal() {
            Theme::dark()
        } else {
            Theme::new()
        });

        let config = Config::load()?;

        config.log().setup()?;


        let ctx = Arc::new(AppContext::try_from(&config)?);

        let router = Router::new()
            .route("/hello", get(|| async { "Hello World!" }))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::make_span_with)
                    .on_request(trace::on_request)
                    .on_response(trace::on_response)
                    .on_failure(trace::on_failure),
            );


        let listener = TcpListener::bind(config.server().address()).await?;
        tracing::info!("Listening on {}", config.server().url());

        axum::serve(listener, router).await.map_err(Into::into)
    }
}

