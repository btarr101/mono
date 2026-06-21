use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::constants::TS_RS_EXPORT_TO;

/// The type of card in a tracked deck.
///
/// This is used to distinguish between commander and maindeck cards.
/// (in the future, partners if I ever get around to it)
#[derive(ts_rs::TS, sqlx::Type, Debug, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "text")]
pub enum TrackedDeckCardType {
    /// This card is one of the commander(s)
    Commander,
    /// This card is in the maindeck
    Maindeck,
}

/// A card in a tracked deck.
#[derive(ts_rs::TS, sqlx::FromRow, Serialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
pub struct TrackedDeckCard {
    pub uuid: uuid::Uuid,
    pub tracked_deck_uuid: uuid::Uuid,
    pub ty: TrackedDeckCardType,
    pub count: i32,
    pub card_oracle_id: uuid::Uuid,
}

/// A tracked deck
#[derive(ts_rs::TS, sqlx::FromRow, Serialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
pub struct TrackedDeck {
    pub uuid: uuid::Uuid,
    pub tracker_person_uuid: uuid::Uuid,
    pub name: String,
    pub url_source: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
