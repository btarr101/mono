use clap::{Parser, ValueEnum};
use serde_envfile::from_env;
use the_peoples_edh_bracket_server::{
    config::Config, db::setup_pg_pool, scryfall::client::ScryfallClient, server::server, state::AppState, sync_cards::sync_cards,
    tracing::setup_tracing,
};

#[derive(Clone, Copy, ValueEnum, Debug, Default)]
pub enum RunMode {
    #[default]
    Server,
    SyncCards,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    mode: RunMode,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config: Config = from_env()?;

    let _span = setup_tracing(&config.stage);
    let state = AppState {
        scryfall_client: ScryfallClient::new(),
        pg_pool: setup_pg_pool().await?,
    };

    match args.mode {
        RunMode::Server => server(state, config).await?,
        RunMode::SyncCards => sync_cards(state).await?,
    }

    Ok(())
}
