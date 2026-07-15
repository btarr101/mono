use bigdecimal::BigDecimal;
use serde::Serialize;

use crate::constants::TS_RS_EXPORT_TO;

/// The legality of a Magic: The Gathering card.
///
/// Restricted is very unlikely - probably impossible.
/// Non legal cards will not be listed.
#[derive(ts_rs::TS, Debug, Serialize, sqlx::Type)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "text")]
pub enum CardLegality {
    /// The card is legal
    Legal,
    /// The card is restricted
    Restricted,
    /// The card is banned
    Banned,
}

/// A Magic: The Gathering card.
#[derive(ts_rs::TS, sqlx::FromRow, Serialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
pub struct Card {
    /// A unique identifier for the mechanics / behavior of the card
    pub oracle_id: uuid::Uuid,
    /// The name of the card
    pub name: String,
    /// An image uri for the card
    pub image_uri: Option<String>,
    /// The legality of the card
    pub legality: CardLegality,
}

#[derive(ts_rs::TS, Serialize, sqlx::FromRow, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
pub struct CardWithGlobalPoints {
    #[serde(flatten)]
    pub card: Card,
    pub global_points: BigDecimal,
}
