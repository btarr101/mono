use clap::{Parser, ValueEnum};
use the_peoples_edh_bracket_server::{
    config::Config,
    db::setup_pg_pool,
    moxfield::client::{MoxfieldClient, MoxfieldClientConfig},
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

    let config: Config = Config::from_env_with_dotenv()?;

    let _span = setup_tracing(&config.stage);

    let pg_pool = setup_pg_pool(&config.database_url).await?;
    let moxfield_client = MoxfieldClient::new(MoxfieldClientConfig {
        user_agent: &config.moxfield_user_agent.as_str(),
    });

    let state = AppState {
        config,
        scryfall_client: ScryfallClient::new(),
        moxfield_client,
        pg_pool,
    };

    match args.mode {
        RunMode::Server => server(state).await?,
        RunMode::SyncCards => sync_cards(state).await?,
        RunMode::Seed => seed(state).await?,
    }

    Ok(())
}
