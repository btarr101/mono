use axum::extract::FromRef;
use sqlx::{Pool, Postgres};

use crate::scryfall::client::ScryfallClient;

#[derive(Clone)]
pub struct AppState {
    pub scryfall_client: ScryfallClient<'static>,
    pub pg_pool: Pool<Postgres>,
}

impl FromRef<AppState> for ScryfallClient<'static> {
    fn from_ref(state: &AppState) -> Self { state.scryfall_client.clone() }
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Self { state.pg_pool.clone() }
}
