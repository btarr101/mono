use std::num::NonZeroUsize;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use axum_anyhow::{ApiResult, IntoApiError, OptionExt, forbidden};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use sqlx::{PgPool, prelude::FromRow};
use strum::Display;
use validator::Validate;

use crate::{
    api_router::auth::{Auth, OptionalAuth},
    constants::TS_RS_EXPORT_TO,
    db::constants::PG_UNIQUE_VIOLATION,
    model::card_rating::CardRating,
    state::AppState,
    util::parse_pagination,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_ratings).post(post_rating))
        .route("/histogram/card/{oracle_id}", get(get_rating_histogram_for_card))
        .route("/{uuid}", get(get_rating).patch(patch_rating))
        .route("/{uuid}/review", post(post_review_rating))
}

#[derive(ts_rs::TS, Serialize, FromRow)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct CardRatingReviews {
    person_review: Option<bool>,
    likes: i64,
    dislikes: i64,
}

#[derive(ts_rs::TS, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct CardRatingWithReviewsAndGlobalPoints {
    #[serde(flatten)]
    card_rating: CardRating,
    global_points: BigDecimal,
    reviews: CardRatingReviews,
}

#[derive(ts_rs::TS, Serialize, Deserialize, Display)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
enum GetRatingsParamsSort {
    Liked,
    Disliked,
    Controversial,
    Recent,
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
    #[serde_inline_default(None)]
    sort: Option<GetRatingsParamsSort>,
    #[serde_inline_default(NonZeroUsize::MIN)]
    page: NonZeroUsize,
    #[serde_inline_default(const { NonZeroUsize::new(100).expect("100 > 0") })]
    page_size: NonZeroUsize,
}

async fn get_ratings(
    State(pg_pool): State<PgPool>,
    OptionalAuth { person_uuid }: OptionalAuth,
    Query(GetRatingsParams {
        card_oracle_id,
        rater_person_uuid,
        sort,
        page,
        page_size,
    }): Query<GetRatingsParams>,
) -> ApiResult<Json<Vec<CardRatingWithReviewsAndGlobalPoints>>> {
    let (limit, offset) = parse_pagination(page, page_size);
    let sort = sort.and_then(|sort| {
        serde_json::to_value(sort)
            .ok()
            .and_then(|value| value.as_str().map(ToOwned::to_owned))
    });

    let rows = sqlx::query!(
        "WITH review_counts AS (
            SELECT
                crr.reviewed_card_rating_uuid AS card_rating_uuid,
                COUNT(*) FILTER (WHERE crr.liked) AS likes,
                COUNT(*) FILTER (WHERE NOT crr.liked) AS dislikes
            FROM card_rating_review crr
            GROUP BY crr.reviewed_card_rating_uuid
        )
        SELECT
            cr.uuid,
            cr.card_oracle_id,
            cr.rater_person_uuid,
            cr.points,
            cr.reason,
            crg.global_points,
            (
                SELECT crr_person.liked
                FROM card_rating_review crr_person
                WHERE crr_person.reviewed_card_rating_uuid = cr.uuid
                AND crr_person.reviewer_person_uuid = $4::uuid
                LIMIT 1
            ) AS person_review,
            rc.likes,
            rc.dislikes,
            cr.created_at,
            cr.updated_at
        FROM card_rating cr
        LEFT JOIN card_rating_global crg ON crg.card_rating_uuid = cr.uuid
        LEFT JOIN review_counts rc ON rc.card_rating_uuid = cr.uuid
        WHERE
            ($1::uuid IS NULL OR cr.card_oracle_id = $1)
            AND ($2::uuid IS NULL OR cr.rater_person_uuid = $2)
        ORDER BY
            CASE WHEN $3::text = 'liked' THEN COALESCE(rc.likes, 0) END DESC,
            CASE WHEN $3::text = 'disliked' THEN COALESCE(rc.dislikes, 0) END DESC,
            CASE
                WHEN $3::text = 'controversial'
                THEN LEAST(COALESCE(rc.likes, 0), COALESCE(rc.dislikes, 0))
            END DESC,
            CASE WHEN $3::text = 'recent' THEN cr.created_at END DESC,
            CASE WHEN $3::text = 'recent' THEN cr.updated_at END DESC NULLS LAST,
            cr.uuid
        LIMIT $5 OFFSET $6",
        card_oracle_id,
        rater_person_uuid,
        sort,
        person_uuid,
        limit,
        offset
    )
    .fetch_all(&pg_pool)
    .await?;

    let ratings = rows
        .into_iter()
        .map(|row| CardRatingWithReviewsAndGlobalPoints {
            card_rating: CardRating {
                uuid: row.uuid,
                card_oracle_id: row.card_oracle_id,
                rater_person_uuid: row.rater_person_uuid,
                points: row.points,
                reason: row.reason,
                created_at: row.created_at,
                updated_at: row.updated_at,
            },
            global_points: row.global_points.unwrap_or_default(),
            reviews: CardRatingReviews {
                person_review: row.person_review,
                likes: row.likes.unwrap_or_default(),
                dislikes: row.dislikes.unwrap_or_default(),
            },
        })
        .collect();

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

async fn get_rating(
    State(pg_pool): State<PgPool>,
    OptionalAuth { person_uuid }: OptionalAuth,
    Path(uuid): Path<uuid::Uuid>,
) -> ApiResult<Json<CardRatingWithReviewsAndGlobalPoints>> {
    let row = sqlx::query!(
        "SELECT
            cr.uuid,
            cr.card_oracle_id,
            cr.rater_person_uuid,
            cr.points,
            cr.reason,
            crg.global_points,
            (
                SELECT crr_person.liked
                FROM card_rating_review crr_person
                WHERE crr_person.reviewed_card_rating_uuid = cr.uuid
                  AND crr_person.reviewer_person_uuid = $2
                LIMIT 1
            ) AS person_review,
            rc.likes,
            rc.dislikes,
            cr.created_at,
            cr.updated_at
        FROM card_rating cr
        LEFT JOIN card_rating_global crg ON crg.card_rating_uuid = cr.uuid
        LEFT JOIN (
            SELECT
                crr.reviewed_card_rating_uuid AS card_rating_uuid,
                COUNT(*) FILTER (WHERE crr.liked) AS likes,
                COUNT(*) FILTER (WHERE NOT crr.liked) AS dislikes
            FROM card_rating_review crr
            GROUP BY crr.reviewed_card_rating_uuid
        ) rc ON rc.card_rating_uuid = cr.uuid
        WHERE cr.uuid = $1
        LIMIT 1",
        uuid,
        person_uuid,
    )
    .fetch_optional(&pg_pool)
    .await?
    .context_not_found("rating not found")?;

    let rating = CardRatingWithReviewsAndGlobalPoints {
        card_rating: CardRating {
            uuid: row.uuid,
            card_oracle_id: row.card_oracle_id,
            rater_person_uuid: row.rater_person_uuid,
            points: row.points,
            reason: row.reason,
            created_at: row.created_at,
            updated_at: row.updated_at,
        },
        global_points: row.global_points.unwrap_or_default(),
        reviews: CardRatingReviews {
            person_review: row.person_review,
            likes: row.likes.unwrap_or(0),
            dislikes: row.dislikes.unwrap_or(0),
        },
    };

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

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[derive(Deserialize, Validate)]
struct PostReviewRatingBody {
    like: Option<bool>,
}

async fn post_review_rating(
    State(pg_pool): State<PgPool>,
    Auth { person_uuid }: Auth,
    Path(uuid): Path<uuid::Uuid>,
    Json(PostReviewRatingBody { like }): Json<PostReviewRatingBody>,
) -> ApiResult<()> {
    let mut tx = pg_pool.begin().await?;

    let rater_person_uuid = sqlx::query!("SELECT rater_person_uuid FROM card_rating WHERE uuid = $1", uuid,)
        .fetch_optional(&mut *tx)
        .await?
        .map(|row| row.rater_person_uuid)
        .context_not_found("rating not found")?;

    (rater_person_uuid != person_uuid).ok_or(forbidden("Forbidden", "person cannot review their own rating"))?;

    if let Some(like) = like {
        sqlx::query!(
            "INSERT INTO card_rating_review (reviewer_person_uuid, reviewed_card_rating_uuid, liked)
             VALUES ($1, $2, $3)
             ON CONFLICT (reviewer_person_uuid, reviewed_card_rating_uuid)
             DO UPDATE SET liked = EXCLUDED.liked",
            person_uuid,
            uuid,
            like,
        )
        .execute(&mut *tx)
        .await?;
    } else {
        sqlx::query!(
            "DELETE FROM card_rating_review
            WHERE reviewer_person_uuid = $1 AND reviewed_card_rating_uuid = $2",
            person_uuid,
            uuid
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde_inline_default]
#[derive(Deserialize)]
struct GetRatingHistogramParams {
    #[serde_inline_default(const { NonZeroUsize::new(10).expect("10 > 0") })]
    buckets: NonZeroUsize,
}

#[derive(ts_rs::TS, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct RatingHistogramBucket {
    lower_inclusive_points_bound: BigDecimal,
    upper_exclusive_points_bound: BigDecimal,
    count: usize,
}

async fn get_rating_histogram_for_card(
    State(pg_pool): State<PgPool>,
    Path(oracle_id): Path<uuid::Uuid>,
    Query(GetRatingHistogramParams { buckets }): Query<GetRatingHistogramParams>,
) -> ApiResult<Json<Vec<RatingHistogramBucket>>> {
    let rows = sqlx::query!(
        "WITH params AS (
            SELECT
                $2::int AS fixed_buckets,
                0::numeric AS min_global_points,
                10::numeric AS max_global_points
        ),
        bucket_bounds AS (
            SELECT
                gs.bucket_index,
                p.min_global_points
                    + (p.max_global_points - p.min_global_points)
                    * (gs.bucket_index - 1)
                    / (p.fixed_buckets::numeric)
                    AS lower_inclusive_points_bound,
                CASE
                    WHEN gs.bucket_index = p.fixed_buckets THEN p.max_global_points
                    ELSE p.min_global_points
                        + (p.max_global_points - p.min_global_points)
                        * gs.bucket_index
                        / (p.fixed_buckets::numeric)
                END AS upper_exclusive_points_bound
            FROM params p
            CROSS JOIN LATERAL generate_series(1, p.fixed_buckets) AS gs(bucket_index)
        ),
        bucket_counts AS (
            SELECT
                GREATEST(
                    1,
                    LEAST(
                        width_bucket(
                            crg.global_points,
                            p.min_global_points,
                            p.max_global_points,
                            p.fixed_buckets
                        ),
                        p.fixed_buckets
                    )
                ) AS bucket_index,
                COUNT(*)::bigint AS count
            FROM card_rating_global crg
            CROSS JOIN params p
            WHERE crg.card_oracle_id = $1
            GROUP BY 1
        )
        SELECT
            bb.lower_inclusive_points_bound AS \"lower_inclusive_points_bound!\",
            bb.upper_exclusive_points_bound AS \"upper_exclusive_points_bound!\",
            COALESCE(bc.count, 0)::bigint AS \"count!\"
        FROM bucket_bounds bb
        LEFT JOIN bucket_counts bc ON bc.bucket_index = bb.bucket_index
        ORDER BY bb.bucket_index",
        oracle_id,
        buckets.get() as i32,
    )
    .fetch_all(&pg_pool)
    .await?;

    let buckets = rows
        .into_iter()
        .map(|row| RatingHistogramBucket {
            lower_inclusive_points_bound: row.lower_inclusive_points_bound,
            upper_exclusive_points_bound: row.upper_exclusive_points_bound,
            count: row.count as usize,
        })
        .collect();

    Ok(Json(buckets))
}
