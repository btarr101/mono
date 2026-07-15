use std::time::Duration;

use axum::{http::Request, middleware::from_fn_with_state};
#[cfg(debug_assertions)]
use axum_anyhow::set_expose_errors;
use futures_util::TryFutureExt;
use reqwest::header::AUTHORIZATION;
use tokio_cron_scheduler::{Job, JobScheduler};
use tower_http::{
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::info;

use crate::{
    api_router,
    client_assets_handler::client_assets_handler,
    jobs::sync_decks,
    middleware::auth::{AuthMiddlewareParams, AuthMiddlewareState, auth_middleware},
    state::AppState,
};

pub async fn server(state: AppState) -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    set_expose_errors(true);

    let bind_address = state.config.bind_address.clone();
    let router = axum::Router::new()
        .nest("/api", api_router::get_router())
        .layer(from_fn_with_state(
            AuthMiddlewareState::new(AuthMiddlewareParams {
                google_client_id: &state.config.google_oauth_client_id,
                pg_pool: state.pg_pool.clone(),
            })
            .await?,
            auth_middleware,
        ))
        .with_state(state.clone())
        .layer(
            CorsLayer::new()
                .allow_methods(AllowMethods::any())
                .allow_origin(AllowOrigin::any())
                .allow_headers(AllowHeaders::any()),
        )
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

    let scheduler = JobScheduler::new().await?;
    // 10 seconds = 6 tracked decks per minute
    // definitely not scaleable but good enough for now
    let job = Job::new_repeated_async(Duration::from_secs(10), move |_uuid, _l| {
        let state = state.clone();
        Box::pin(async move {
            if let Err(e) = sync_decks::sync_last_synced_deck(state).await {
                tracing::error!("Failed to sync decks: {}", e);
            }
        })
    })?;
    scheduler.add(job).await?;

    info!("Starting server at http://{}", bind_address);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;

    tokio::try_join!(
        async move { axum::serve(listener, router).await }.map_err(anyhow::Error::new),
        scheduler.start().map_err(anyhow::Error::new)
    )?;

    Ok(())
}
