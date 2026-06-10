use std::num::NonZeroUsize;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use axum_anyhow::{ApiResult, OptionExt};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use sqlx::PgPool;

use crate::{
    constants::TS_RS_EXPORT_TO,
    model::card::{Card, CardLegality},
    state::AppState,
    util::parse_pagination,
};

pub fn get_router() -> Router<AppState> { Router::new().route("/", get(get_cards)).route("/{oracle_id}", get(get_card)) }

#[derive(ts_rs::TS, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct CardWithGlobalPoints {
    #[serde(flatten)]
    card: Card,
    global_points: BigDecimal,
}

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum GetCardsParamsSort {
    HighestRated,
    LowestRated,
}

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde_inline_default]
#[derive(Deserialize)]
struct GetCardsParams {
    #[serde_inline_default(None)]
    q: Option<String>,
    #[serde_inline_default(None)]
    sort: Option<GetCardsParamsSort>,
    #[serde_inline_default(NonZeroUsize::MIN)]
    page: NonZeroUsize,
    #[serde_inline_default(const { NonZeroUsize::new(100).expect("100 > 0") })]
    page_size: NonZeroUsize,
}

async fn get_cards(
    State(pg_pool): State<PgPool>,
    Query(GetCardsParams {
        q,
        sort,
        page,
        page_size,
    }): Query<GetCardsParams>,
) -> ApiResult<Json<Vec<CardWithGlobalPoints>>> {
    let (limit, offset) = parse_pagination(page, page_size);
    let desc = match sort {
        None => None,
        Some(GetCardsParamsSort::HighestRated) => Some(true),
        Some(GetCardsParamsSort::LowestRated) => Some(false),
    };

    let rows = sqlx::query!(
        "SELECT
            c.oracle_id,
            c.name,
            c.image_uri,
            c.legality as \"legality: CardLegality\",
            COALESCE(crc.average_global_points, 5.0) as \"global_points!\"
        FROM card c
        LEFT JOIN card_ratings_cache crc ON crc.card_oracle_id = c.oracle_id
        WHERE ($1::text IS NULL OR lower(c.name) LIKE lower($1) || '%')
        ORDER BY
            CASE WHEN $4 = true THEN crc.average_global_points END DESC NULLS LAST,
            CASE WHEN $4 = false THEN crc.average_global_points END ASC NULLS LAST,
            c.name
        LIMIT $2 OFFSET $3",
        q,
        limit,
        offset,
        desc
    )
    .fetch_all(&pg_pool)
    .await?;

    let cards = rows
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
        .collect();

    Ok(Json(cards))
}

async fn get_card(
    State(pg_pool): State<PgPool>,
    Path(oracle_id): Path<uuid::Uuid>,
) -> ApiResult<Json<CardWithGlobalPoints>> {
    let row = sqlx::query!(
        "SELECT
            c.oracle_id,
            c.name,
            c.image_uri,
            c.legality as \"legality: CardLegality\",
            COALESCE(crc.average_global_points, 5.0) as \"global_points!\"
        FROM card c
        LEFT JOIN card_ratings_cache crc ON crc.card_oracle_id = c.oracle_id
        WHERE c.oracle_id = $1
        LIMIT 1",
        oracle_id
    )
    .fetch_optional(&pg_pool)
    .await?
    .context_not_found("card could not be found")?;

    let card = CardWithGlobalPoints {
        card: Card {
            oracle_id: row.oracle_id,
            name: row.name,
            image_uri: row.image_uri,
            legality: row.legality,
        },
        global_points: row.global_points,
    };

    Ok(Json(card))
}
