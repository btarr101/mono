use std::num::NonZeroUsize;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, put},
};
use axum_anyhow::{ApiResult, IntoApiError, OptionExt};
use bigdecimal::{BigDecimal, Signed};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use sqlx::{PgPool, prelude::FromRow};
use strum::Display;
use validator::Validate;

use crate::{
    constants::TS_RS_EXPORT_TO,
    controller::ratings::{RateCardError, RateCardParams, ReviewRatingError, ReviewRatingParams, rate_card, review_rating},
    middleware::auth::{Auth, OptionalAuth},
    model::card_rating::CardRating,
    state::AppState,
    util::parse_pagination,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_ratings).put(put_rating))
        .route("/histogram/card/{oracle_id}", get(get_rating_histogram_for_card))
        .route("/{uuid}", get(get_rating).put(put_rating))
        .route("/{uuid}/review", put(put_rating_review))
}

#[derive(ts_rs::TS, Serialize, FromRow)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct CardRatingReviews {
    person_review: Option<bool>,
    #[ts(type = "number")]
    likes: i64,
    #[ts(type = "number")]
    dislikes: i64,
}

#[derive(ts_rs::TS, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct CardRatingWithTotalPoints {
    #[serde(flatten)]
    card_rating: CardRating,
    total_points: BigDecimal,
}

#[derive(ts_rs::TS, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct CardRatingWithReviewsAndGlobalPoints {
    #[serde(flatten)]
    card_rating: CardRatingWithTotalPoints,
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
            COALESCE(prc.total_personal_points, 0) as \"total_points!\",
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
        LEFT JOIN person_ratings_cache prc ON prc.person_uuid = cr.rater_person_uuid
        LEFT JOIN review_counts rc ON rc.card_rating_uuid = cr.uuid
        WHERE
            ($1::uuid IS NULL OR cr.card_oracle_id = $1)
            AND ($2::uuid IS NULL OR cr.rater_person_uuid = $2)
        ORDER BY
            CASE WHEN $3::text = 'liked' THEN COALESCE(rc.likes, 0) END DESC,
            CASE WHEN $3::text = 'liked' THEN COALESCE(rc.dislikes, 0) END ASC,
            CASE WHEN $3::text = 'disliked' THEN COALESCE(rc.dislikes, 0) END DESC,
            CASE WHEN $3::text = 'disliked' THEN COALESCE(rc.likes, 0) END ASC,
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
            card_rating: CardRatingWithTotalPoints {
                card_rating: CardRating {
                    uuid: row.uuid,
                    card_oracle_id: row.card_oracle_id,
                    rater_person_uuid: row.rater_person_uuid,
                    points: row.points,
                    reason: row.reason,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                },
                total_points: row.total_points,
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

fn validate_big_decimal_not_negative(value: &BigDecimal) -> Result<(), validator::ValidationError> {
    if value.is_negative() {
        return Err(validator::ValidationError::new("value cannot be negative"));
    }

    Ok(())
}

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[derive(Deserialize, Validate)]
struct PutRatingBody {
    card_oracle_id: uuid::Uuid,
    #[validate(custom(function = validate_big_decimal_not_negative))]
    points: BigDecimal,
    #[validate(length(max = 400))]
    reason: Option<String>,
}

async fn put_rating(
    State(pg_pool): State<PgPool>,
    Auth { person_uuid }: Auth,
    Json(PutRatingBody {
        card_oracle_id,
        points,
        reason,
    }): Json<PutRatingBody>,
) -> ApiResult<Json<CardRating>> {
    let card_rating = rate_card(
        RateCardParams {
            card_oracle_id,
            person_uuid,
            points,
            reason,
        },
        &pg_pool,
    )
    .await
    .map_err(|error| match error {
        RateCardError::CardDoesNotExist => {
            error.context_bad_request(format!("Could not find card with oracle_id '{}'", card_oracle_id))
        }
        RateCardError::Other(error) => error.into(),
    })?;

    Ok(Json(card_rating))
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
            COALESCE(prc.total_personal_points, 0) as \"total_points!\",
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
        LEFT JOIN person_ratings_cache prc ON prc.person_uuid = cr.rater_person_uuid
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
        card_rating: CardRatingWithTotalPoints {
            card_rating: CardRating {
                uuid: row.uuid,
                card_oracle_id: row.card_oracle_id,
                rater_person_uuid: row.rater_person_uuid,
                points: row.points,
                reason: row.reason,
                created_at: row.created_at,
                updated_at: row.updated_at,
            },
            total_points: row.total_points,
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
struct PutRatingReviewBody {
    like: Option<bool>,
}

async fn put_rating_review(
    State(pg_pool): State<PgPool>,
    Auth { person_uuid }: Auth,
    Path(uuid): Path<uuid::Uuid>,
    Json(PutRatingReviewBody { like }): Json<PutRatingReviewBody>,
) -> ApiResult<()> {
    review_rating(
        ReviewRatingParams {
            card_rating_uuid: uuid,
            person_uuid,
            like,
        },
        &pg_pool,
    )
    .await
    .map_err(|error| match error {
        ReviewRatingError::RatingDoesNotExist => error.context_not_found("Card rating not found"),
        ReviewRatingError::CannotReviewOwnRating => error.context_forbidden("Person cannot review their own rating"),
        ReviewRatingError::Other(error) => error.into(),
    })
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
