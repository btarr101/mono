use std::sync::Arc;

use axum::{
    Router,
    extract::{Query, State, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use futures::StreamExt;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use tokio::sync::Mutex;
use tracing::{Instrument, info_span};
use ws_rx_handler::ws_rx_handler;

use crate::{
    api_router::ws_tx_handler::{WsTxHandlerOptions, ws_tx_handler},
    led_repo::LedRepo,
};

mod ws_rx_handler;
mod ws_tx_handler;

pub fn get_router() -> Router<Arc<LedRepo>> { Router::new().route("/leds/ws", get(get_ws)).fallback(handler_404) }

#[serde_inline_default]
#[derive(Deserialize)]
struct GetWsParams {
    #[serde_inline_default(false)]
    colors_only: bool,
    #[serde_inline_default(100)]
    snapshot_interval: u64,
}

async fn get_ws(
    State(leds): State<Arc<LedRepo>>,
    Query(GetWsParams {
        colors_only,
        snapshot_interval,
    }): Query<GetWsParams>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |ws| {
        Box::pin(async move {
            let ws_client_id = uuid::Uuid::new_v4();

            let (tx, rx) = ws.split();
            let tx = Arc::new(Mutex::new(tx));

            let rx_span = info_span!("rx", ws_client_id = ws_client_id.to_string(),);
            let mut rx_task = tokio::spawn(ws_rx_handler(rx, tx.clone(), leds.clone()).instrument(rx_span));

            let tx_span = info_span!("tx", ws_client_id = ws_client_id.to_string(), colors_only, snapshot_interval);
            let mut tx_task = tokio::spawn(ws_tx_handler(tx, leds, WsTxHandlerOptions::default()).instrument(tx_span));

            tokio::select! {
                _ = &mut rx_task => tx_task.abort(),
                _ = &mut tx_task => rx_task.abort(),
            };
        })
    })
}

async fn handler_404() -> StatusCode { StatusCode::NOT_FOUND }
