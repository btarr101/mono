use std::{collections::HashMap, num::NonZeroUsize};

use axum::{Json, Router, extract::State, routing::post};
use axum_anyhow::ApiResult;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};

use crate::{
    constants::TS_RS_EXPORT_TO,
    model::card::{Card, CardLegality},
    state::AppState,
};

pub fn get_router() -> Router<AppState> { Router::new().route("/analyze", post(post_analyze)) }

#[derive(ts_rs::TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct DecklistMaindeckEntry {
    count: NonZeroUsize,
    name: String,
}

#[derive(ts_rs::TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct Decklist {
    commanders: Vec<String>,
    maindeck: Vec<DecklistMaindeckEntry>,
}

#[derive(ts_rs::TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct DeckUrl {
    url: String,
}

#[derive(ts_rs::TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum PostAnalyzeBody {
    Url(DeckUrl),
    Decklist(Decklist),
}

#[derive(ts_rs::TS, Serialize, FromRow, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct CardWithGlobalPoints {
    #[serde(flatten)]
    card: Card,
    global_points: BigDecimal,
}

#[derive(ts_rs::TS, Serialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct DeckMaindeckEntry {
    count: NonZeroUsize,
    card: CardWithGlobalPoints,
}

#[derive(ts_rs::TS, Serialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct Deck {
    commanders: Vec<CardWithGlobalPoints>,
    maindeck: Vec<DeckMaindeckEntry>,
}

#[derive(ts_rs::TS, Serialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct AnalyzedDeck {
    source: PostAnalyzeBody,
    deck: Deck,
    total_points: BigDecimal,
}

#[derive(ts_rs::TS, Serialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct PostAnalyzeInvalidCardsResponse {
    invalid_commanders: Vec<String>,
    invalid_maindeck: Vec<String>,
}

async fn post_analyze(
    State(pg_pool): State<PgPool>,
    Json(body): Json<PostAnalyzeBody>,
) -> ApiResult<Json<Result<AnalyzedDeck, PostAnalyzeInvalidCardsResponse>>> {
    let ((commanders, invalid_commanders), (maindeck, invalid_maindeck)) = match &body {
        PostAnalyzeBody::Url(_) => unimplemented!("Deck urls are not implemented yet!"),
        PostAnalyzeBody::Decklist(decklist) => (
            find_cards_by_names(decklist.commanders.as_slice(), &pg_pool).await?,
            async {
                let card_names = decklist.maindeck.iter().map(|entry| entry.name.clone()).collect::<Vec<_>>();
                let (valid_cards, invalid_card_names) = find_cards_by_names(card_names.as_slice(), &pg_pool).await?;

                let card_counts = decklist
                    .maindeck
                    .iter()
                    .map(|entry| (entry.name.to_lowercase(), entry.count))
                    .collect::<HashMap<_, _>>();

                let valid_cards = valid_cards
                    .into_iter()
                    .map(|card| DeckMaindeckEntry {
                        count: card_counts
                            .get(&card.card.name.to_lowercase())
                            .cloned()
                            .unwrap_or(NonZeroUsize::MIN),
                        card,
                    })
                    .collect::<Vec<_>>();

                anyhow::Ok((valid_cards, invalid_card_names))
            }
            .await?,
        ),
    };

    if !invalid_commanders.is_empty() || !invalid_maindeck.is_empty() {
        return Ok(Json(Err(PostAnalyzeInvalidCardsResponse {
            invalid_commanders,
            invalid_maindeck,
        })));
    }

    let total_points = commanders
        .iter()
        .map(|commander| commander.global_points.clone())
        .chain(maindeck.iter().map(|entry| {
            entry.card.global_points.clone() * BigDecimal::from(u128::try_from(entry.count.get()).unwrap_or(u128::MIN))
        }))
        .reduce(|a, b| a + b)
        .unwrap_or_default();

    Ok(Json(Ok(AnalyzedDeck {
        source: body,
        deck: Deck { commanders, maindeck },
        total_points,
    })))
}

async fn find_cards_by_names(
    cards_names: &[String],
    pg_pool: &PgPool,
) -> anyhow::Result<(Vec<CardWithGlobalPoints>, Vec<String>)> {
    let cards = sqlx::query!(
        "SELECT
            c.oracle_id,
            c.name,
            c.image_uri,
            c.legality as \"legality: CardLegality\",
            COALESCE(crc.average_global_points, 0.0) as \"global_points!\"
        FROM card c
        LEFT JOIN card_ratings_cache crc ON crc.card_oracle_id = c.oracle_id
        WHERE c.name ILIKE ANY($1)
        ",
        cards_names
    )
    .fetch_all(pg_pool)
    .await?
    .into_iter()
    .map(|row| CardWithGlobalPoints {
        card: Card {
            oracle_id: row.oracle_id,
            name: row.name,
            image_uri: row.image_uri,
            legality: row.legality,
        },
        global_points: row.global_points,
    })
    .collect::<Vec<_>>();

    let invalid_card_names = cards_names
        .iter()
        .filter(|name| !cards.iter().find(|card| card.card.name == **name).is_some())
        .cloned()
        .collect::<Vec<_>>();

    Ok((cards, invalid_card_names))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::setup_pg_pool;

    #[tokio::test]
    async fn test_analyze_by_decklist() -> anyhow::Result<()> {
        let decklist = Decklist {
            commanders: vec!["Storm Crow".to_string()],
            maindeck: vec![
                DecklistMaindeckEntry {
                    count: NonZeroUsize::MIN,
                    name: "Force of Will".into(),
                },
                DecklistMaindeckEntry {
                    count: NonZeroUsize::MIN,
                    name: "Negate".into(),
                },
            ],
        };
        let pg_pool = setup_pg_pool("postgres://admin:root@localhost:5432/db").await?;

        let analyzation = post_analyze(State(pg_pool), Json(PostAnalyzeBody::Decklist(decklist))).await;

        match analyzation {
            Ok(json) => {
                dbg!(json.0);
            }
            Err(err) => {
                dbg!(err);
            }
        }

        Ok(())
    }
}
