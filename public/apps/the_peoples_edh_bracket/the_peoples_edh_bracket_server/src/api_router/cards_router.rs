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
use sqlx::{PgPool, prelude::FromRow};

use crate::{
    constants::TS_RS_EXPORT_TO,
    model::card::{Card, CardLegality},
    state::AppState,
    util::parse_pagination,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_cards))
        .route("/{oracle_id}", get(get_card))
        .route("/{oracle_id}/pts", get(get_card_metrics))
}

#[derive(ts_rs::TS, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct CardMetrics {
    global_points: BigDecimal,
    #[ts(type = "number")]
    total_ratings: i64,
    #[ts(type = "number")]
    card_rank: i64,
}

#[derive(ts_rs::TS, Serialize, FromRow)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct CardWithMetrics {
    #[serde(flatten)]
    card: Card,
    #[serde(flatten)]
    metrics: CardMetrics,
}

#[derive(ts_rs::TS, Serialize, Deserialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde(rename_all = "snake_case")]
enum GetCardsParamsSort {
    HighestRated,
    LowestRated,
    MostControversial,
    MostRated,
    LeastRated,
    Trending,
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
) -> ApiResult<Json<Vec<CardWithMetrics>>> {
    let (limit, offset) = parse_pagination(page, page_size);
    let sort = sort.and_then(|sort| {
        serde_json::to_value(sort)
            .ok()
            .and_then(|value| value.as_str().map(ToOwned::to_owned))
    });

    let rows = sqlx::query!(
        "WITH card_rating_agg AS (
            SELECT
                cr.card_oracle_id,
                COUNT(*) AS total_ratings,
                COUNT(*) FILTER (WHERE cr.points > 0) AS likes_count,
                COUNT(*) FILTER (WHERE cr.points < 0) AS dislikes_count,
                COALESCE(
                    SUM(
                        EXP(
                            -LN(2) * (EXTRACT(EPOCH FROM (NOW() - cr.created_at)) / 3600.0) / 72.0
                        )
                    ),
                    0
                ) AS trending_score
            FROM card_rating cr
            GROUP BY cr.card_oracle_id
        ),
        card_ratings AS (
            SELECT
                cr.card_oracle_id,
                AVG(cr.points) AS average_global_points,
                DENSE_RANK() OVER (ORDER BY AVG(cr.points) DESC) AS card_rank
            FROM card_rating cr
            GROUP BY cr.card_oracle_id
        ),
        unrated_rank AS (
            SELECT
                1 + COUNT(DISTINCT cr.average_global_points)::INT AS unrated_card_rank
            FROM card_ratings cr
            WHERE cr.average_global_points > 0.0
        )
        SELECT
            c.oracle_id,
            c.name,
            c.image_uri,
            c.legality as \"legality: CardLegality\",
            COALESCE(crs.average_global_points, 0.0) as \"global_points!\",
            COALESCE(cra.total_ratings, 0) as \"total_ratings!\",
            COALESCE(crs.card_rank, urr.unrated_card_rank) as \"card_rank!\"
        FROM card c
        LEFT JOIN card_ratings crs ON crs.card_oracle_id = c.oracle_id
        LEFT JOIN card_rating_agg cra ON cra.card_oracle_id = c.oracle_id
        CROSS JOIN unrated_rank urr
        WHERE ($1::text IS NULL OR lower(c.name) LIKE lower($1) || '%')
        ORDER BY
            CASE WHEN $4::text = 'highest_rated' THEN COALESCE(crs.average_global_points, 0.0) END DESC,
            CASE WHEN $4::text = 'lowest_rated' THEN COALESCE(crs.average_global_points, 0.0) END ASC,
            CASE
                WHEN $4::text = 'most_controversial'
                THEN ABS(COALESCE(cra.likes_count, 0) - COALESCE(cra.dislikes_count, 0))
            END DESC,
            CASE WHEN $4::text = 'most_rated' THEN COALESCE(cra.total_ratings, 0) END DESC,
            CASE WHEN $4::text = 'least_rated' THEN COALESCE(cra.total_ratings, 0) END ASC,
            CASE WHEN $4::text = 'trending' THEN COALESCE(cra.trending_score, 0) END DESC,
            c.name
        LIMIT $2 OFFSET $3",
        q,
        limit,
        offset,
        sort
    )
    .fetch_all(&pg_pool)
    .await?;

    let cards = rows
        .into_iter()
        .map(|row| CardWithMetrics {
            card: Card {
                oracle_id: row.oracle_id,
                name: row.name,
                image_uri: row.image_uri,
                legality: row.legality,
            },
            metrics: CardMetrics {
                global_points: row.global_points,
                total_ratings: row.total_ratings,
                card_rank: row.card_rank,
            },
        })
        .collect();

    Ok(Json(cards))
}

async fn get_card(State(pg_pool): State<PgPool>, Path(oracle_id): Path<uuid::Uuid>) -> ApiResult<Json<CardWithMetrics>> {
    let row = sqlx::query!(
        "WITH card_ratings AS (
            SELECT
                cr.card_oracle_id,
                AVG(cr.points) AS average_global_points,
                DENSE_RANK() OVER (ORDER BY AVG(cr.points) DESC) AS card_rank
            FROM card_rating cr
            GROUP BY cr.card_oracle_id
        ),
        unrated_rank AS (
            SELECT
                1 + COUNT(DISTINCT cr.average_global_points)::INT AS unrated_card_rank
            FROM card_ratings cr
            WHERE cr.average_global_points > 0.0
        )
        SELECT
            c.oracle_id,
            c.name,
            c.image_uri,
            c.legality as \"legality: CardLegality\",
            COALESCE(crs.average_global_points, 0.0) as \"global_points!\",
            COUNT(cr.uuid) as \"total_ratings!\",
            COALESCE(crs.card_rank, urr.unrated_card_rank) as \"card_rank!\"
        FROM card c
        LEFT JOIN card_ratings crs ON crs.card_oracle_id = c.oracle_id
        LEFT JOIN card_rating cr ON cr.card_oracle_id = c.oracle_id
        CROSS JOIN unrated_rank urr
        WHERE c.oracle_id = $1
        GROUP BY c.oracle_id, c.name, c.image_uri, c.legality, crs.average_global_points, crs.card_rank, urr.unrated_card_rank
        LIMIT 1",
        oracle_id
    )
    .fetch_optional(&pg_pool)
    .await?
    .context_not_found("card could not be found")?;

    Ok(Json(CardWithMetrics {
        card: Card {
            oracle_id: row.oracle_id,
            name: row.name,
            image_uri: row.image_uri,
            legality: row.legality,
        },
        metrics: CardMetrics {
            global_points: row.global_points,
            total_ratings: row.total_ratings,
            card_rank: row.card_rank,
        },
    }))
}

async fn get_card_metrics(State(pg_pool): State<PgPool>, Path(oracle_id): Path<uuid::Uuid>) -> ApiResult<Json<CardMetrics>> {
    let metrics = sqlx::query_as!(
        CardMetrics,
        "WITH card_ratings AS (
            SELECT
                cr.card_oracle_id,
                AVG(cr.points) AS average_global_points,
                DENSE_RANK() OVER (ORDER BY AVG(cr.points) DESC) AS card_rank
            FROM card_rating cr
            GROUP BY cr.card_oracle_id
        ),
        unrated_rank AS (
            SELECT
                1 + COUNT(DISTINCT cr.average_global_points)::INT AS unrated_card_rank
            FROM card_ratings cr
            WHERE cr.average_global_points > 0.0
        )
        SELECT
            COALESCE(crs.average_global_points, 0.0) as \"global_points!\",
            COUNT(cr.uuid) as \"total_ratings!\",
            COALESCE(crs.card_rank, urr.unrated_card_rank) as \"card_rank!\"
        FROM card c
        LEFT JOIN card_ratings crs ON crs.card_oracle_id = c.oracle_id
        LEFT JOIN card_rating cr ON cr.card_oracle_id = c.oracle_id
        CROSS JOIN unrated_rank urr
        WHERE c.oracle_id = $1
        GROUP BY c.oracle_id, crs.average_global_points, crs.card_rank, urr.unrated_card_rank
        LIMIT 1",
        oracle_id
    )
    .fetch_optional(&pg_pool)
    .await?
    .context_not_found("card metrics could not be found")?;

    Ok(Json(metrics))
}
