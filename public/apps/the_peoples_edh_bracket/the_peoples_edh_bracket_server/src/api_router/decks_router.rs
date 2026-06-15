use axum::{Json, Router, extract::State, routing::post};
use axum_anyhow::{ApiError, ApiResult};
use bigdecimal::BigDecimal;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, prelude::FromRow};

use crate::{
    constants::TS_RS_EXPORT_TO,
    model::card::{Card, CardLegality},
    state::AppState,
};

pub fn get_router() -> Router<AppState> { Router::new().route("/analyze", post(post_analyze)) }

#[derive(ts_rs::TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct Decklist {
    commanders: Vec<String>,
    maindeck: Vec<String>,
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
struct Deck {
    commanders: Vec<CardWithGlobalPoints>,
    maindeck: Vec<CardWithGlobalPoints>,
}

#[derive(ts_rs::TS, Serialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct AnalyzedDeck {
    source: PostAnalyzeBody,
    deck: Deck,
    total_points: BigDecimal,
}

#[axum::debug_handler]
async fn post_analyze(State(pg_pool): State<PgPool>, Json(body): Json<PostAnalyzeBody>) -> ApiResult<Json<AnalyzedDeck>> {
    let ((commanders, invalid_commanders), (maindeck, invalid_maindeck)) = match &body {
        PostAnalyzeBody::Url(_) => unimplemented!("Deck urls are not implemented yet!"),
        PostAnalyzeBody::Decklist(decklist) => (
            find_cards_by_names(decklist.commanders.as_slice(), &pg_pool).await?,
            find_cards_by_names(decklist.maindeck.as_slice(), &pg_pool).await?,
        ),
    };

    if !invalid_commanders.is_empty() || !invalid_maindeck.is_empty() {
        Err(ApiError::builder()
            .status(StatusCode::BAD_REQUEST)
            .title("Bad Request")
            .detail("The below cards could not be found")
            .meta(json!({
                "commanders": invalid_commanders,
                "maindeck": invalid_maindeck
            }))
            .build())?;
    }

    let total_points = commanders
        .iter()
        .chain(maindeck.iter())
        .map(|card| card.global_points.clone())
        .reduce(|a, b| a + b)
        .unwrap_or_default();

    Ok(Json(AnalyzedDeck {
        source: body,
        deck: Deck { commanders, maindeck },
        total_points,
    }))
}

async fn find_cards_by_names<'a>(
    cards_names: &'a [String],
    pg_pool: &PgPool,
) -> anyhow::Result<(Vec<CardWithGlobalPoints>, Vec<&'a String>)> {
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

    dbg!(&cards);

    let invalid_card_names = cards_names
        .iter()
        .filter(|name| !cards.iter().find(|card| card.card.name == **name).is_some())
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
            maindeck: vec!["Force of Will".to_string(), "Negate".to_string()],
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
