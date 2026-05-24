use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::led_repo::{Color, LedRepo};

pub async fn ws_rx_handler(mut rx: SplitStream<WebSocket>, tx: Arc<Mutex<SplitSink<WebSocket, Message>>>, leds: Arc<LedRepo>) {
    loop {
        match rx.next().await {
            Some(Ok(message)) => {
                let Some(parsed_message) = parse_ws_message(message) else {
                    continue;
                };

                info!(parsed_message = ?parsed_message, "Received message");

                match parsed_message {
                    ParsedWsMessage::Close => break,
                    ParsedWsMessage::Ping => {
                        let mut tx = tx.lock().await;
                        let _ = tx.send(Message::Text("pong".into())).await;
                    }
                    ParsedWsMessage::SetColor { id, color } => {
                        let _ = leds.set(id, color).await;
                    }
                }
            }
            Some(Err(error)) => {
                error!(error = %error, "Error when receiving message. Closing connection...");
                break;
            }
            None => {
                error!("Connection closed unexpectedly");
                break;
            }
        }
    }

    info!("Connection closed");
}

#[derive(Debug)]
enum ParsedWsMessage {
    Close,
    Ping,
    SetColor { id: usize, color: Color },
}

fn parse_ws_message(message: Message) -> Option<ParsedWsMessage> {
    match message {
        Message::Binary(bytes) if bytes.len() >= 5 => {
            let id = usize::from_be_bytes([0, 0, 0, 0, 0, 0, bytes[0], bytes[1]]);
            let red = bytes[2];
            let green = bytes[3];
            let blue = bytes[4];

            Some(ParsedWsMessage::SetColor {
                id,
                color: Color { red, green, blue },
            })
        }
        Message::Text(utf8) => {
            let text = utf8.to_string();
            (text == "ping").then_some(ParsedWsMessage::Ping)
        }
        Message::Close(_) => Some(ParsedWsMessage::Close),
        _ => None,
    }
}
