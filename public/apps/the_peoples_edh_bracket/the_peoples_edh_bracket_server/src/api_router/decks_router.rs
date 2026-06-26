use std::{collections::HashMap, iter::repeat, num::NonZeroUsize};

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use axum_anyhow::{ApiError, ApiResult, OptionExt};
use axum_valid::Valid;
use bigdecimal::{BigDecimal, ToPrimitive};
use itertools::{Either, Itertools};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use sqlx::{PgPool, QueryBuilder, prelude::FromRow};
use strum::Display;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::{
    api_router::decks_router::PostAnalyzeBody::Url,
    constants::{MAX_TRACKED_DECKS_PER_PERSON, TS_RS_EXPORT_TO},
    middleware::auth::Auth,
    model::{
        card::{Card, CardLegality},
        tracked_deck::{TrackedDeck, TrackedDeckCardType},
    },
    state::AppState,
    types::PointsHistogramBucket,
    util::parse_pagination,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_tracked_decks).post(post_tracked_deck))
        .route("/analyze", post(post_analyze))
        .route("/{uuid}", get(get_tracked_deck).delete(delete_tracked_deck))
}

#[derive(ts_rs::TS, Serialize, Deserialize, Display)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
enum GetTrackedDecksParamsSort {
    Recent,
    MostPoints,
    LeastPoints,
    MostPersonalPoints,
    LeastPersonalPoints,
}

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde_inline_default]
#[derive(Deserialize)]
struct GetTrackedDecksParams {
    #[serde_inline_default(None)]
    tracker_person_uuid: Option<uuid::Uuid>,
    #[serde_inline_default(None)]
    q: Option<String>,
    #[serde_inline_default(None)]
    sort: Option<GetTrackedDecksParamsSort>,
    #[serde_inline_default(NonZeroUsize::MIN)]
    page: NonZeroUsize,
    #[serde_inline_default(const { NonZeroUsize::new(100).expect("100 > 0") })]
    page_size: NonZeroUsize,
}

#[derive(ts_rs::TS, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct TrackedDeckWithTotalPoints {
    #[serde(flatten)]
    tracked_deck: TrackedDeck,
    total_global_points: BigDecimal,
    total_personal_points: BigDecimal,
}

async fn get_tracked_decks(
    State(pg_pool): State<PgPool>,
    Query(GetTrackedDecksParams {
        tracker_person_uuid,
        q,
        sort,
        page,
        page_size,
    }): Query<GetTrackedDecksParams>,
) -> ApiResult<Json<Vec<TrackedDeckWithTotalPoints>>> {
    let (limit, offset) = parse_pagination(page, page_size);
    let sort = sort.and_then(|sort| {
        serde_json::to_value(sort)
            .ok()
            .and_then(|value| value.as_str().map(ToOwned::to_owned))
    });

    let tracked_decks = sqlx::query!(
        "WITH deck_points AS (
            SELECT
                tdc.tracked_deck_uuid,
                COALESCE(SUM(COALESCE(crc.average_global_points, 0.0) * tdc.count), 0.0) AS total_global_points,
                COALESCE(SUM(COALESCE(cr.points, 0.0) * tdc.count), 0.0) AS total_personal_points
            FROM tracked_deck_card tdc
            INNER JOIN tracked_deck td ON td.uuid = tdc.tracked_deck_uuid
            LEFT JOIN card_ratings_cache crc ON crc.card_oracle_id = tdc.card_oracle_id
            LEFT JOIN card_rating cr
                ON cr.card_oracle_id = tdc.card_oracle_id
                AND cr.rater_person_uuid = td.tracker_person_uuid
            GROUP BY tdc.tracked_deck_uuid
        )
        SELECT
            td.uuid,
            td.tracker_person_uuid,
            td.name,
            td.url_source,
            td.created_at,
            td.updated_at,
            COALESCE(dp.total_global_points, 0.0) AS \"total_global_points!\",
            COALESCE(dp.total_personal_points, 0.0) AS \"total_personal_points!\"
        FROM tracked_deck td
        LEFT JOIN deck_points dp ON dp.tracked_deck_uuid = td.uuid
        WHERE
            ($1::uuid IS NULL OR td.tracker_person_uuid = $1)
            AND ($2::text IS NULL OR lower(td.name) LIKE lower($2) || '%')
        ORDER BY
            CASE WHEN $3::text = 'most_points' THEN COALESCE(dp.total_global_points, 0.0) END DESC,
            CASE WHEN $3::text = 'least_points' THEN COALESCE(dp.total_global_points, 0.0) END ASC,
            CASE WHEN $3::text = 'most_personal_points' THEN COALESCE(dp.total_personal_points, 0.0) END DESC,
            CASE WHEN $3::text = 'least_personal_points' THEN COALESCE(dp.total_personal_points, 0.0) END ASC,
            CASE WHEN $3::text = 'recent' THEN td.created_at END DESC,
            CASE WHEN $3::text = 'recent' THEN td.updated_at END DESC NULLS LAST,
            td.name,
            td.uuid
        LIMIT $4 OFFSET $5",
        tracker_person_uuid,
        q,
        sort,
        limit,
        offset,
    )
    .fetch_all(&pg_pool)
    .await?
    .into_iter()
    .map(|row| TrackedDeckWithTotalPoints {
        tracked_deck: TrackedDeck {
            uuid: row.uuid,
            tracker_person_uuid: row.tracker_person_uuid,
            name: row.name,
            url_source: row.url_source,
            created_at: row.created_at,
            updated_at: row.updated_at,
        },
        total_global_points: row.total_global_points,
        total_personal_points: row.total_personal_points,
    })
    .collect::<Vec<_>>();

    Ok(Json(tracked_decks))
}

#[derive(ts_rs::TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct DecklistMaindeckEntry {
    count: NonZeroUsize,
    name: String,
}

#[derive(ts_rs::TS, Serialize, Deserialize, Debug, Validate)]
#[validate(schema(function = "validate_decklist"))]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct Decklist {
    #[validate(length(min = 1, max = 2))]
    commanders: Vec<String>,
    maindeck: Vec<DecklistMaindeckEntry>,
}

fn validate_decklist(decklist: &Decklist) -> Result<(), ValidationError> {
    let total_maindeck = decklist.maindeck.iter().map(|e| e.count.get()).sum::<usize>();
    let total_commanders = decklist.commanders.len();

    let total_cards = total_maindeck + total_commanders;

    if total_cards != 100 {
        return Err(ValidationError::new("invalid count").with_message(
            format!(
                "The total count of a commander deck must add up to exactly 100 (decklist had {} cards)",
                total_cards
            )
            .into(),
        ));
    }

    Ok(())
}

#[derive(ts_rs::TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct DeckUrl {
    url: String,
}

#[derive(ts_rs::TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde(tag = "type", rename_all = "snake_case")]
enum PostAnalyzeBody {
    Url(DeckUrl),
    Decklist(Decklist),
}

impl Validate for PostAnalyzeBody {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            PostAnalyzeBody::Url(_) => Ok(()),
            PostAnalyzeBody::Decklist(decklist) => decklist.validate(),
        }
    }
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
    deck: Deck,
    total_points: BigDecimal,
    histogram: Vec<PointsHistogramBucket>,
}

#[derive(ts_rs::TS, Serialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct AnalyzedDeckWithSource {
    url_source: Option<String>,
    #[serde(flatten)]
    analyzed_deck: AnalyzedDeck,
}

#[derive(ts_rs::TS, Serialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct PostAnalyzeInvalidCards {
    invalid_commanders: Vec<String>,
    invalid_maindeck: Vec<String>,
}

#[derive(ts_rs::TS, Serialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde(tag = "type", rename_all = "snake_case")]
enum PostAnalyzeDeckResponse {
    Valid(AnalyzedDeckWithSource),
    Invalid(PostAnalyzeInvalidCards),
}

async fn post_analyze(
    State(pg_pool): State<PgPool>,
    Valid(Json(body)): Valid<Json<PostAnalyzeBody>>,
) -> ApiResult<Json<PostAnalyzeDeckResponse>> {
    let ((commanders, invalid_commanders), (maindeck, invalid_maindeck)) = match &body {
        PostAnalyzeBody::Url(_) => unimplemented!("Deck urls are not implemented yet!"),
        PostAnalyzeBody::Decklist(decklist) => (
            find_cards_by_names(&decklist.commanders, &pg_pool).await?,
            async {
                let card_names = decklist.maindeck.iter().map(|entry| entry.name.clone()).collect::<Vec<_>>();
                let (valid_cards, invalid_card_names) = find_cards_by_names(&card_names, &pg_pool).await?;

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
        return Ok(Json(PostAnalyzeDeckResponse::Invalid(PostAnalyzeInvalidCards {
            invalid_commanders,
            invalid_maindeck,
        })));
    }

    let all_points = commanders.iter().map(|commander| commander.global_points.clone()).chain(
        maindeck
            .iter()
            .flat_map(|entry| repeat(entry.card.global_points.clone()).take(entry.count.get())),
    );

    let total_points = all_points.clone().reduce(|a, b| a + b).unwrap_or_default();
    let histogram = build_histogram(all_points);

    Ok(Json(PostAnalyzeDeckResponse::Valid(AnalyzedDeckWithSource {
        url_source: match body {
            Url(DeckUrl { url }) => Some(url),
            _ => None,
        },
        analyzed_deck: AnalyzedDeck {
            deck: Deck { commanders, maindeck },
            total_points,
            histogram,
        },
    })))
}

async fn find_cards_by_names(
    cards_names: impl IntoIterator<Item = impl AsRef<str>>,
    pg_pool: &PgPool,
) -> anyhow::Result<(Vec<CardWithGlobalPoints>, Vec<String>)> {
    let input: Vec<String> = cards_names.into_iter().map(|n| n.as_ref().to_string()).collect();
    let lowercased: Vec<String> = input.iter().map(|n| n.to_lowercase()).collect();

    let cards = sqlx::query!(
        "SELECT
            c.oracle_id as \"oracle_id!\",
            c.name as \"name!\",
            c.image_uri,
            c.legality as \"legality!: CardLegality\",
            COALESCE(crc.average_global_points, 0.0) as \"global_points!\"
        FROM card c
        LEFT JOIN card_ratings_cache crc ON c.oracle_id = crc.card_oracle_id
        WHERE LOWER(c.name) = ANY($1)
        ",
        &lowercased
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

    let invalid_card_names = input
        .iter()
        .filter(|name| {
            !cards
                .iter()
                .find(|card| card.card.name.eq_ignore_ascii_case(name.as_str()))
                .is_some()
        })
        .cloned()
        .collect::<Vec<_>>();

    Ok((cards, invalid_card_names))
}

fn build_histogram(all_points: impl Iterator<Item = BigDecimal>) -> Vec<PointsHistogramBucket> {
    let mut buckets = (0..10)
        .map(|index| PointsHistogramBucket {
            lower_inclusive_points_bound: BigDecimal::from(index),
            upper_exclusive_points_bound: BigDecimal::from(index + 1),
            count: 0,
        })
        .collect::<Vec<_>>();

    for points in all_points {
        if let Some(bucket_index) = points
            .with_scale_round(0, bigdecimal::RoundingMode::Floor)
            .to_usize()
            .map(|bucket_index| bucket_index.clamp(0, buckets.len() - 1))
        {
            dbg!((points.to_string(), bucket_index));
            buckets[bucket_index].count += 1;
        }
    }

    buckets
}

#[derive(ts_rs::TS, Deserialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct PostTrackedDeckBodyMaindeckEntry {
    count: NonZeroUsize,
    oracle_id: uuid::Uuid,
}

#[derive(ts_rs::TS, Deserialize, Validate)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[validate(schema(function = "validate_post_tracked_deck_body"))]
struct PostTrackedDeckBody {
    name: String,
    url_source: Option<String>,
    #[validate(length(min = 1, max = 2))]
    commanders: Vec<uuid::Uuid>,
    maindeck: Vec<PostTrackedDeckBodyMaindeckEntry>,
}

fn validate_post_tracked_deck_body(body: &PostTrackedDeckBody) -> Result<(), ValidationError> {
    let total_maindeck = body.maindeck.iter().map(|e| e.count.get()).sum::<usize>();
    let total_commanders = body.commanders.len();

    let total_cards = total_maindeck + total_commanders;

    if total_cards != 100 {
        return Err(ValidationError::new("invalid count").with_message(
            format!(
                "The total count of a commander deck must add up to exactly 100 (decklist had {} cards)",
                total_cards
            )
            .into(),
        ));
    }

    Ok(())
}

async fn post_tracked_deck(
    State(pg_pool): State<PgPool>,
    Auth { person_uuid }: Auth,
    Valid(Json(body)): Valid<Json<PostTrackedDeckBody>>,
) -> ApiResult<Json<TrackedDeck>> {
    let total_tracked_decks = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM tracked_deck WHERE tracker_person_uuid = $1",
        person_uuid,
    )
    .fetch_one(&pg_pool)
    .await?
    .unwrap_or_default();

    if total_tracked_decks as usize >= MAX_TRACKED_DECKS_PER_PERSON {
        let error = ApiError::builder()
            .status(StatusCode::FORBIDDEN)
            .title("Deck limit exceeded")
            .detail(format!("Max limit of `{}` decks reached", MAX_TRACKED_DECKS_PER_PERSON))
            .build();

        return Err(error);
    }

    let mut tx = pg_pool.begin().await?;

    let tracked_deck = sqlx::query_as!(
        TrackedDeck,
        "INSERT INTO tracked_deck (tracker_person_uuid, name, url_source) VALUES ($1, $2, $3) RETURNING *",
        person_uuid,
        body.name,
        body.url_source,
    )
    .fetch_one(&mut *tx)
    .await?;

    let mut qb = QueryBuilder::new(
        "INSERT INTO tracked_deck_card (
			tracked_deck_uuid,
			ty,
			count,
			card_oracle_id
		)",
    );

    let entries = body
        .commanders
        .iter()
        .map(|commander| (TrackedDeckCardType::Commander, 1, *commander))
        .chain(
            body.maindeck
                .iter()
                .map(|entry| (TrackedDeckCardType::Maindeck, entry.count.get() as i64, entry.oracle_id)),
        );

    qb.push_values(entries, |mut row, entry| {
        row.push_bind(tracked_deck.uuid);
        row.push_bind(entry.0);
        row.push_bind(entry.1);
        row.push_bind(entry.2);
    });

    qb.build().execute(&mut *tx).await?;

    tx.commit().await?;

    Ok(Json(tracked_deck))
}

#[derive(ts_rs::TS, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct TrackedDeckWithAnalysis {
    #[serde(flatten)]
    tracked_deck: TrackedDeck,
    #[serde(flatten)]
    analyzed_deck: AnalyzedDeck,
}

async fn get_tracked_deck(
    State(pg_pool): State<PgPool>,
    Path(uuid): Path<uuid::Uuid>,
) -> ApiResult<Json<TrackedDeckWithAnalysis>> {
    let tracked_deck = sqlx::query_as!(TrackedDeck, "SELECT * FROM tracked_deck WHERE uuid = $1", uuid)
        .fetch_optional(&pg_pool)
        .await?
        .context_not_found("deck not found")?;

    let rows = sqlx::query!(
        "SELECT
            tdc.uuid,
            tdc.tracked_deck_uuid,
            tdc.ty as \"ty: TrackedDeckCardType\",
            tdc.count,
            c.oracle_id,
            c.name,
            c.image_uri,
            c.legality as \"legality: CardLegality\",
            COALESCE(crc.average_global_points, 0.0) as \"global_points!\"
        FROM tracked_deck_card tdc
        INNER JOIN card c ON c.oracle_id = tdc.card_oracle_id
        LEFT JOIN card_ratings_cache crc ON crc.card_oracle_id = tdc.card_oracle_id
        WHERE tracked_deck_uuid = $1
        ",
        uuid
    )
    .fetch_all(&pg_pool)
    .await?;

    let (commanders, maindeck): (Vec<_>, Vec<_>) = rows.into_iter().partition_map(|row| match row.ty {
        TrackedDeckCardType::Commander => Either::Left(CardWithGlobalPoints {
            card: Card {
                oracle_id: row.oracle_id,
                name: row.name,
                image_uri: row.image_uri,
                legality: row.legality,
            },
            global_points: row.global_points,
        }),
        TrackedDeckCardType::Maindeck => Either::Right(DeckMaindeckEntry {
            count: NonZeroUsize::try_from(row.count as usize).unwrap_or(NonZeroUsize::MIN),
            card: CardWithGlobalPoints {
                card: Card {
                    oracle_id: row.oracle_id,
                    name: row.name,
                    image_uri: row.image_uri,
                    legality: row.legality,
                },
                global_points: row.global_points,
            },
        }),
    });

    let all_points = commanders.iter().map(|commander| commander.global_points.clone()).chain(
        maindeck
            .iter()
            .flat_map(|entry| repeat(entry.card.global_points.clone()).take(entry.count.get())),
    );
    let total_points = all_points.clone().reduce(|a, b| a + b).unwrap_or_default();
    let histogram = build_histogram(all_points);

    let tracked_deck_with_cards = TrackedDeckWithAnalysis {
        tracked_deck,
        analyzed_deck: AnalyzedDeck {
            deck: Deck { commanders, maindeck },
            total_points,
            histogram,
        },
    };

    Ok(Json(tracked_deck_with_cards))
}

async fn delete_tracked_deck(
    State(pg_pool): State<PgPool>,
    Auth { person_uuid }: Auth,
    Path(uuid): Path<uuid::Uuid>,
) -> ApiResult<Json<TrackedDeck>> {
    let deleted_tracked_deck = sqlx::query_as!(
        TrackedDeck,
        "
        DELETE FROM tracked_deck
        WHERE
            uuid = $1
            AND tracker_person_uuid = $2
            RETURNING *",
        uuid,
        person_uuid
    )
    .fetch_optional(&pg_pool)
    .await?
    .context_not_found("deck not found")?;

    Ok(Json(deleted_tracked_deck))
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

        let analyzation = post_analyze(State(pg_pool), Valid(Json(PostAnalyzeBody::Decklist(decklist)))).await;

        match analyzation {
            Ok(json) => {
                let _ = dbg!(json.0);
            }
            Err(err) => {
                dbg!(err);
            }
        }

        Ok(())
    }
}
