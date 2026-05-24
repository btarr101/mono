use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, stream::SplitSink};
use tokio::{sync::Mutex, time::Duration};

use crate::led_repo::LedRepo;

pub struct WsTxHandlerOptions {
    pub colors_only: bool,
    pub snapshot_interval: Duration,
}

impl Default for WsTxHandlerOptions {
    fn default() -> Self {
        Self {
            colors_only: true,
            snapshot_interval: Duration::from_millis(1000),
        }
    }
}

pub async fn ws_tx_handler(tx: Arc<Mutex<SplitSink<WebSocket, Message>>>, leds: Arc<LedRepo>, options: WsTxHandlerOptions) {
    let mut latest_generation = None;
    loop {
        if latest_generation < Some(leds.generation()) {
            let snapshot = leds.snapshot().await;
            latest_generation = Some(snapshot.generation);

            let bytes = match options.colors_only {
                true => snapshot.leds.iter().flat_map(|led| <[u8; 3]>::from(led.color)).collect(),
                false => snapshot.leds.iter().flat_map(|led| <[u8; 11]>::from(*led)).collect(),
            };

            let _ = tx.lock().await.send(Message::Binary(bytes)).await;
        }

        tokio::time::sleep(options.snapshot_interval).await;
    }
}
