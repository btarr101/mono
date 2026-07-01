use axum::{Json, extract::State};
use serde::Serialize;

use crate::{config::Config, constants::TS_RS_EXPORT_TO};

#[derive(ts_rs::TS, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
pub struct ClientConfig {
    google_oauth_client_id: String,
}

/// NOTE: Everything returned by this handler is public.
#[axum::debug_handler]
pub async fn get_config(State(config): State<Config>) -> Json<ClientConfig> {
    Json(ClientConfig {
        google_oauth_client_id: config.google_oauth_client_id,
    })
}
