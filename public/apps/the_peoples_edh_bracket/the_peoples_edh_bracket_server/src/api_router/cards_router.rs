use std::num::NonZeroUsize;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use axum_anyhow::{ApiResult, OptionExt};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use sqlx::PgPool;

use crate::{
    model::card::{Card, CardLegality},
    state::AppState,
};

pub fn get_router() -> Router<AppState> { Router::new().route("/", get(get_cards)).route("/{oracle_id}", get(get_card)) }

async fn get_card(State(pg_pool): State<PgPool>, Path(oracle_id): Path<uuid::Uuid>) -> ApiResult<Json<Card>> {
    let card = sqlx::query_as!(
        Card,
        "SELECT
            oracle_id,
            name,
            image_uri,
            legality as \"legality: CardLegality\"
        FROM card
        WHERE oracle_id = $1
        LIMIT 1",
        oracle_id
    )
    .fetch_optional(&pg_pool)
    .await?
    .context_not_found("card could not be found")?;

    Ok(Json(card))
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum GetCardsParamsSort {
    HighestRated,
    LowestRated,
}

#[derive(ts_rs::TS)]
#[ts(export)]
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
) -> ApiResult<Json<Vec<Card>>> {
    let limit = page_size.get() as i64;
    let offset = ((page.get() - 1) * page_size.get()) as i64;
    let desc = match sort {
        None => None,
        Some(GetCardsParamsSort::HighestRated) => Some(true),
        Some(GetCardsParamsSort::LowestRated) => Some(false),
    };

    let cards = sqlx::query_as!(
        Card,
        "SELECT
            c.oracle_id,
            c.name,
            c.image_uri,
            c.legality as \"legality: CardLegality\"
        FROM card c
        LEFT JOIN (
            SELECT card_oracle_id, AVG(points) AS avg_points
            FROM card_rating GROUP BY card_oracle_id
        ) r ON c.oracle_id = r.card_oracle_id
        WHERE ($1::text IS NULL OR $1 <% c.name)
        ORDER BY
            CASE WHEN $4 = true THEN r.avg_points END DESC NULLS LAST,
            CASE WHEN $4 = false THEN r.avg_points END ASC NULLS LAST,
            CASE WHEN $1 IS NOT NULL THEN word_similarity($1, c.name) END DESC NULLS LAST,
            c.name
        LIMIT $2 OFFSET $3",
        q,
        limit,
        offset,
        desc
    )
    .fetch_all(&pg_pool)
    .await?;

    Ok(Json(cards))
}
