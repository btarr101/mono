use axum::extract::FromRef;
use sqlx::{Pool, Postgres};

use crate::{config::Config, moxfield::client::MoxfieldClient, scryfall::client::ScryfallClient};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub scryfall_client: ScryfallClient,
    pub moxfield_client: MoxfieldClient,
    pub pg_pool: Pool<Postgres>,
}

impl FromRef<AppState> for ScryfallClient {
    fn from_ref(state: &AppState) -> Self { state.scryfall_client.clone() }
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Self { state.pg_pool.clone() }
}

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self { state.config.clone() }
}

impl FromRef<AppState> for MoxfieldClient {
    fn from_ref(state: &AppState) -> Self { state.moxfield_client.clone() }
}
