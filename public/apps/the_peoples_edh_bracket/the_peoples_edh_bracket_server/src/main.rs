use std::{env, path::Path};

use clap::{Parser, ValueEnum};
use serde_envfile::from_env;
use the_peoples_edh_bracket_server::{
    config::Config,
    db::setup_pg_pool,
    scripts::{seed::seed, sync_cards::sync_cards},
    scryfall::client::ScryfallClient,
    server::server,
    state::AppState,
    tracing::setup_tracing,
};

#[derive(Clone, Copy, ValueEnum, Debug, Default)]
pub enum RunMode {
    #[default]
    Server,
    SyncCards,
    Seed,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_enum, default_value_t = RunMode::default())]
    mode: RunMode,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if cfg!(debug_assertions) {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
        let env_path = Path::new(&manifest_dir).join(".env");
        let _ = dotenvy::from_filename(&env_path);
    }

    let _ = dotenvy::dotenv();
    let config: Config = from_env()?;

    let _span = setup_tracing(&config.stage);

    let pg_pool = setup_pg_pool(&config.database_url).await?;
    let state = AppState {
        config,
        scryfall_client: ScryfallClient::new(),
        pg_pool,
    };

    match args.mode {
        RunMode::Server => server(state).await?,
        RunMode::SyncCards => sync_cards(state).await?,
        RunMode::Seed => seed(state).await?,
    }

    Ok(())
}
