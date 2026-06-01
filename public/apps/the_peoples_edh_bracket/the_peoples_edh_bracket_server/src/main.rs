use serde_envfile::from_env;
use the_peoples_edh_bracket_server::{
    api_router, config::Config, db::setup_pg_pool, scryfall::client::ScryfallClient, state::AppState, tracing::setup_tracing,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: Config = from_env()?;

    let _span = setup_tracing(&config.stage);
    let state = AppState {
        scryfall_client: ScryfallClient::new(),
        pg_pool: setup_pg_pool().await?,
    };

    let app = axum::Router::new().nest("/api", api_router::get_router()).with_state(state);

    info!("Starting server at http://{}", config.bind_address);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
