use axum::{Json, Router, extract::State, routing::get};
use axum_anyhow::ApiResult;
use serde::Serialize;
use sqlx::PgPool;

use crate::{constants::TS_RS_EXPORT_TO, state::AppState};

pub fn get_router() -> Router<AppState> { Router::new().route("/metrics", get(get_home_metrics)) }

#[derive(ts_rs::TS, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct HomeMetrics {
    #[ts(type = "number")]
    total_cards_rated: i64,
    #[ts(type = "number")]
    total_persons: i64,
    #[ts(type = "number")]
    total_ratings: i64,
}

async fn get_home_metrics(State(pg_pool): State<PgPool>) -> ApiResult<Json<HomeMetrics>> {
    let metrics = sqlx::query_as!(
        HomeMetrics,
        "WITH rating_agg AS (
            SELECT
                COUNT(*) as total_ratings,
                COUNT(DISTINCT card_oracle_id) AS total_cards_rated
            FROM card_rating
        ),
        person_agg AS (
            SELECT COUNT(*) AS total
            FROM person
        )
        SELECT
            rating_agg.total_cards_rated AS \"total_cards_rated!\",
            person_agg.total AS \"total_persons!\",
            rating_agg.total_ratings AS \"total_ratings!\"
        FROM person_agg, rating_agg"
    )
    .fetch_one(&pg_pool)
    .await?;

    Ok(Json(metrics))
}
