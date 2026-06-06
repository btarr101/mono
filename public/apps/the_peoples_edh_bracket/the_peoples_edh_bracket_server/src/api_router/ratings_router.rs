use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use axum_anyhow::{ApiResult, IntoApiError, OptionExt};
use bigdecimal::BigDecimal;
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::{db::constants::PG_UNIQUE_VIOLATION, model::card_rating::CardRating, state::AppState};

pub fn get_router() -> Router<AppState> { Router::new().route("/", post(post_rating)) }

#[derive(Deserialize)]
struct PostRatingBody {
    card_oracle_id: uuid::Uuid,
    points: BigDecimal,
    reason: Option<String>,
    // TEMP - we need to derive this from auth
    user_uuid: uuid::Uuid,
}

async fn post_rating(
    State(pg_pool): State<Pool<Postgres>>,
    Json(PostRatingBody {
        card_oracle_id,
        points,
        reason,
        user_uuid,
    }): Json<PostRatingBody>,
) -> ApiResult<impl IntoResponse> {
    let tx = pg_pool.begin().await?;

    sqlx::query!("SELECT oracle_id FROM card WHERE oracle_id = $1", card_oracle_id)
        .fetch_optional(&pg_pool)
        .await?
        .context_not_found("Could not find card")?;

    let rating = sqlx::query_as!(
        CardRating,
        "INSERT INTO card_rating (card_oracle_id, rater_person_uuid, points, reason) VALUES ($1, $2, $3, $4)
         RETURNING *",
        card_oracle_id,
        user_uuid,
        points.to_owned(),
        reason,
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some(PG_UNIQUE_VIOLATION) => {
            e.context_conflict("User has already rated this card")
        }
        _ => e.into(),
    })?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(rating)))
}
