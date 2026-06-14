use axum::middleware::from_fn_with_state;
#[cfg(debug_assertions)]
use axum_anyhow::set_expose_errors;
use tower_http::{
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::info;

use crate::{
    api_router,
    config::Config,
    middleware::auth::{AuthMiddlewareParams, AuthMiddlewareState, auth_middleware},
    state::AppState,
};

pub async fn server(state: AppState, config: Config) -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    set_expose_errors(true);

    let router = axum::Router::new()
        .nest("/api", api_router::get_router())
        .layer(from_fn_with_state(
            AuthMiddlewareState::new(AuthMiddlewareParams {
                google_client_id: &config.google_oauth_client_id,
                pg_pool: state.pg_pool.clone(),
            })
            .await?,
            auth_middleware,
        ))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_methods(AllowMethods::any())
                .allow_origin(AllowOrigin::any())
                .allow_headers(AllowHeaders::any()),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true).level(tracing::Level::INFO))
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        );

    info!("Starting server at http://{}", config.bind_address);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
