use std::num::NonZeroUsize;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use axum_anyhow::{ApiResult, IntoApiError, OptionExt, forbidden};
use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use sqlx::PgPool;
use validator::Validate;

use crate::{
    api_router::auth::Auth, constants::TS_RS_EXPORT_TO, db::constants::PG_UNIQUE_VIOLATION, model::card_rating::CardRating,
    state::AppState, util::parse_pagination,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_ratings).post(post_rating))
        .route("/{uuid}", get(get_rating).patch(patch_rating))
}

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde_inline_default]
#[derive(Deserialize)]
struct GetRatingsParams {
    #[serde_inline_default(None)]
    card_oracle_id: Option<uuid::Uuid>,
    #[serde_inline_default(None)]
    rater_person_uuid: Option<uuid::Uuid>,
    #[serde_inline_default(NonZeroUsize::MIN)]
    page: NonZeroUsize,
    #[serde_inline_default(const { NonZeroUsize::new(100).expect("100 > 0") })]
    page_size: NonZeroUsize,
}

async fn get_ratings(
    State(pg_pool): State<PgPool>,
    Query(GetRatingsParams {
        card_oracle_id,
        rater_person_uuid,
        page,
        page_size,
    }): Query<GetRatingsParams>,
) -> ApiResult<Json<Vec<CardRating>>> {
    let (limit, offset) = parse_pagination(page, page_size);

    let ratings = sqlx::query_as!(
        CardRating,
        "SELECT *
        FROM card_rating
        WHERE
            ($1::uuid IS NULL OR card_oracle_id = $1)
            AND ($2::uuid IS NULL OR rater_person_uuid = $2)
        LIMIT $3 OFFSET $4
        ",
        card_oracle_id,
        rater_person_uuid,
        limit,
        offset
    )
    .fetch_all(&pg_pool)
    .await?;

    Ok(Json(ratings))
}

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[derive(Deserialize, Validate)]
struct PostRatingBody {
    card_oracle_id: uuid::Uuid,
    points: BigDecimal,
    #[validate(length(max = 4000))]
    reason: Option<String>,
}

async fn post_rating(
    State(pg_pool): State<PgPool>,
    Auth { person_uuid }: Auth,
    Json(PostRatingBody {
        card_oracle_id,
        points,
        reason,
    }): Json<PostRatingBody>,
) -> ApiResult<(StatusCode, Json<CardRating>)> {
    let mut tx = pg_pool.begin().await?;

    sqlx::query!("SELECT oracle_id FROM card WHERE oracle_id = $1", card_oracle_id)
        .fetch_optional(&pg_pool)
        .await?
        .context_bad_request(format!("Could not find card with oracle_id '{}'", card_oracle_id))?;

    let rating = sqlx::query_as!(
        CardRating,
        "INSERT INTO card_rating (card_oracle_id, rater_person_uuid, points, reason) VALUES ($1, $2, $3, $4)
         RETURNING *",
        card_oracle_id,
        person_uuid,
        points.to_owned(),
        reason,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some(PG_UNIQUE_VIOLATION) => {
            e.context_conflict("person has already rated this card")
        }
        _ => e.into(),
    })?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(rating)))
}

async fn get_rating(State(pg_pool): State<PgPool>, Path(uuid): Path<uuid::Uuid>) -> ApiResult<Json<CardRating>> {
    let rating = sqlx::query_as!(CardRating, "SELECT * from card_rating WHERE uuid = $1 LIMIT 1", uuid)
        .fetch_optional(&pg_pool)
        .await?
        .context_not_found("rating not found")?;

    Ok(Json(rating))
}

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[derive(Deserialize, Validate)]
struct PatchRatingBody {
    points: Option<BigDecimal>,
    #[ts(optional, type = "string | null")]
    #[validate(length(max = 4000))]
    reason: Option<Option<String>>,
}

async fn patch_rating(
    State(pg_pool): State<PgPool>,
    Auth { person_uuid }: Auth,
    Path(uuid): Path<uuid::Uuid>,
    Json(PatchRatingBody { points, reason }): Json<PatchRatingBody>,
) -> ApiResult<(StatusCode, Json<CardRating>)> {
    let mut tx = pg_pool.begin().await?;

    let rater_person_uuid = sqlx::query!("SELECT rater_person_uuid FROM card_rating WHERE uuid = $1", uuid,)
        .fetch_optional(&mut *tx)
        .await?
        .map(|row| row.rater_person_uuid)
        .context_not_found("rating not found")?;

    (rater_person_uuid == person_uuid).ok_or(forbidden("Forbidden", "cannot patch another person's rating"))?;

    let should_update_reason = reason.is_some();
    let new_reason = reason.flatten();

    let rating = sqlx::query_as!(
        CardRating,
        "UPDATE card_rating
         SET points = COALESCE($3, points),
             reason = CASE WHEN $4 THEN $5 ELSE reason END
         WHERE uuid = $1 AND rater_person_uuid = $2
         RETURNING *",
        uuid,
        person_uuid,
        points,
        should_update_reason,
        new_reason.as_deref(),
    )
    .fetch_optional(&mut *tx)
    .await?
    .context_not_found("rating not found")?;

    tx.commit().await?;

    Ok((StatusCode::OK, Json(rating)))
}
