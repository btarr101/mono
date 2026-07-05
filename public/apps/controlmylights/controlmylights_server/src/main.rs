use std::sync::Arc;

use axum::http::{Request, header::AUTHORIZATION};
use controlmylights_server::{
    api_router,
    client_assets_handler::client_assets_handler,
    config::Config,
    led_repo::{Color, LedRepo},
};
use serde_envfile::from_env;
use tower_http::{
    cors::{AllowMethods, AllowOrigin, CorsLayer},
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: Config = from_env()?;

    let subscriber = tracing_subscriber::fmt().with_env_filter(
        EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy(),
    );

    if cfg!(debug_assertions) {
        subscriber.compact().init();
    } else {
        subscriber.json().init();
    }

    let _span = tracing::info_span!("root", stage = &config.stage).entered();

    let app = axum::Router::new()
        .nest("/api", api_router::get_router())
        .with_state(Arc::new(LedRepo::new([Color::BLACK; 1152].into_iter())))
        .layer(
            CorsLayer::new()
                .allow_methods(AllowMethods::any())
                .allow_origin(AllowOrigin::any()),
        )
        // We embed the React SPA directly into the server binary, which is sweet!
        .fallback(client_assets_handler)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let mut headers = request.headers().clone();
                    headers.remove(AUTHORIZATION);

                    tracing::info_span!(
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                        headers = ?headers,
                    )
                })
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        );

    info!("Starting server at http://{}", config.bind_address);
    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
