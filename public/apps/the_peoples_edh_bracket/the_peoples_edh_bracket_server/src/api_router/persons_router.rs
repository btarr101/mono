use std::num::NonZeroUsize;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use axum_anyhow::{ApiResult, OptionExt};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use sqlx::PgPool;
use strum::Display;

use crate::{
    constants::TS_RS_EXPORT_TO,
    controller::persons::create_debug_person,
    middleware::auth::{Auth, OptionalAuth},
    model::person::Person,
    state::AppState,
    util::parse_pagination,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_persons).post(post_debug_person))
        .route("/me", get(get_me))
        .route("/{uuid}", get(get_person))
        .route("/{uuid}/follow", post(post_follow_person))
        .route("/{uuid}/unfollow", post(post_unfollow_person))
}

#[derive(ts_rs::TS, Serialize, Deserialize, Display)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
enum GetPersonsParamsSort {
    Likes,
    Followers,
    CardsRated,
}

/// Query parameters used for getting persons
#[derive(ts_rs::TS, Deserialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde_inline_default]
struct GetPersonsParams {
    /// Gets all persons who are following a specific person
    #[serde_inline_default(None)]
    person_following: Option<uuid::Uuid>,
    /// Gets all persons who are followed by a specific person
    #[serde_inline_default(None)]
    person_followee: Option<uuid::Uuid>,
    /// Queries people by their username
    #[serde_inline_default(None)]
    q: Option<String>,
    /// How to sort the result
    #[serde_inline_default(None)]
    sort: Option<GetPersonsParamsSort>,
    /// Pagination parameter
    #[serde_inline_default(NonZeroUsize::MIN)]
    page: NonZeroUsize,
    /// Size of the page
    #[serde_inline_default(const { NonZeroUsize::new(100).expect("100 > 0") })]
    page_size: NonZeroUsize,
}

/// A base person enriched with metrics and follow information
#[derive(ts_rs::TS, Deserialize, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct PersonEnriched {
    #[serde(flatten)]
    person: Person,
    /// How many followers this person has
    #[ts(type = "number")]
    followers: i64,
    /// How many people this person is following
    #[ts(type = "number")]
    following: i64,
    /// How many likes across all ratings this person has received
    #[ts(type = "number")]
    likes: i64,
    /// How many dislikes across all ratings this person has received
    #[ts(type = "number")]
    dislikes: i64,
    /// How many cards this person has rated
    #[ts(type = "number")]
    cards_rated: i64,
    /// How many decks this person is tracking
    #[ts(type = "number")]
    tracked_decks: i64,
    /// If authenticated, if the current user is following this person
    am_following: Option<bool>,
    /// If the `person_following` was passed, this is the date this person has started following that person
    started_following: Option<DateTime<Utc>>,
    /// If the `person_followee` was passed, this is the date this person was followed by that person
    followed_on: Option<DateTime<Utc>>,
}

async fn get_persons(
    State(pg_pool): State<PgPool>,
    OptionalAuth { person_uuid }: OptionalAuth,
    Query(GetPersonsParams {
        person_following,
        person_followee,
        q,
        sort,
        page,
        page_size,
    }): Query<GetPersonsParams>,
) -> ApiResult<Json<Vec<PersonEnriched>>> {
    let (limit, offset) = parse_pagination(page, page_size);
    let sort = sort.and_then(|sort| {
        serde_json::to_value(sort)
            .ok()
            .and_then(|value| value.as_str().map(ToOwned::to_owned))
    });

    let persons = sqlx::query!(
        "SELECT
            person.*,
            COALESCE(pldc.likes, 0) AS \"likes!\",
            COALESCE(pldc.dislikes, 0) AS \"dislikes!\",
            COALESCE(cr.cards_rated, 0) AS \"cards_rated!\",
            COALESCE(td.tracked_decks, 0) AS \"tracked_decks!\",
            COALESCE(fc.followers, 0) AS \"followers!\",
            COALESCE(fg.following, 0) AS \"following!\",
            CASE
                WHEN $3::uuid is NULL THEN NULL
                ELSE EXISTS (
                    SELECT 1
                    FROM follower
                    WHERE
                        follower_person_uuid = $3
                            AND followed_person_uuid = person.uuid
                )
            END AS am_following,
            fr.created_at AS \"started_following?\",
            fb.created_at AS \"followed_on?\"
        FROM person
        LEFT JOIN person_likes_dislikes_cache pldc ON person.uuid = pldc.person_uuid
        LEFT JOIN (
            SELECT
                rater_person_uuid,
                COUNT(*) AS cards_rated
            FROM card_rating
            GROUP BY rater_person_uuid
        ) cr ON person.uuid = cr.rater_person_uuid
        LEFT JOIN (
            SELECT tracker_person_uuid AS person_uuid, COUNT(*) AS tracked_decks
            FROM tracked_deck
            GROUP BY tracker_person_uuid
        ) td ON person.uuid = td.person_uuid
        LEFT JOIN (
            SELECT followed_person_uuid AS person_uuid, COUNT(*) AS followers
            FROM follower
            GROUP BY followed_person_uuid
        ) fc ON person.uuid = fc.person_uuid
        LEFT JOIN (
            SELECT follower_person_uuid AS person_uuid, COUNT(*) AS following
            FROM follower
            GROUP BY follower_person_uuid
        ) fg ON person.uuid = fg.person_uuid
        LEFT JOIN follower AS fr
            ON $1::uuid IS NOT NULL
            AND person.uuid = fr.follower_person_uuid
            AND fr.followed_person_uuid = $1
        LEFT JOIN follower AS fb
            ON $2::uuid IS NOT NULL
            AND person.uuid = fb.followed_person_uuid
            AND fb.follower_person_uuid = $2
        WHERE
            ($4::text IS NULL OR lower(username) LIKE lower($4) || '%')
            AND ($1 IS NULL OR fr.uuid IS NOT NULL)
            AND ($2 IS NULL OR fb.uuid IS NOT NULL)
        ORDER BY
            CASE WHEN $5::TEXT = 'likes' THEN COALESCE(likes, 0) END DESC,
            CASE WHEN $5::TEXT = 'likes' THEN COALESCE(dislikes, 0) END ASC,
            CASE WHEN $5::TEXT = 'cards_rated' THEN COALESCE(cr.cards_rated, 0) END DESC,
            username
        LIMIT $6 OFFSET $7",
        person_following,
        person_followee,
        person_uuid,
        q,
        sort,
        limit,
        offset,
    )
    .fetch_all(&pg_pool)
    .await?
    .into_iter()
    .map(|row| PersonEnriched {
        person: Person {
            uuid: row.uuid,
            username: row.username,
            picture_url: row.picture_url,
            created_at: row.created_at,
            updated_at: row.updated_at,
        },
        am_following: row.am_following,
        followers: row.followers,
        following: row.following,
        likes: row.likes,
        dislikes: row.dislikes,
        cards_rated: row.cards_rated,
        tracked_decks: row.tracked_decks,
        followed_on: row.followed_on,
        started_following: row.started_following,
    })
    .collect::<Vec<_>>();

    Ok(Json(persons))
}

async fn post_debug_person(State(pg_pool): State<PgPool>) -> ApiResult<(StatusCode, Json<Person>)> {
    let person = create_debug_person(&pg_pool).await?;

    Ok((StatusCode::CREATED, Json(person)))
}

async fn get_person(
    State(pg_pool): State<PgPool>,
    OptionalAuth { person_uuid }: OptionalAuth,
    Path(uuid): Path<uuid::Uuid>,
) -> ApiResult<Json<PersonEnriched>> {
    let row = sqlx::query!(
        "SELECT
            person.*,
            COALESCE(pldc.likes, 0) AS \"likes!\",
            COALESCE(pldc.dislikes, 0) AS \"dislikes!\",
            COALESCE(cr.cards_rated, 0) AS \"cards_rated!\",
            COALESCE(td.tracked_decks, 0) AS \"tracked_decks!\",
            COALESCE(fc.followers, 0) AS \"followers!\",
            COALESCE(fg.following, 0) AS \"following!\",
            CASE
                WHEN $2::uuid IS NULL THEN NULL
                ELSE EXISTS (
                    SELECT 1
                    FROM follower f
                    WHERE
                        f.follower_person_uuid = $2
                        AND f.followed_person_uuid = person.uuid
                )
            END AS am_following
        FROM person
        LEFT JOIN person_likes_dislikes_cache pldc ON person.uuid = pldc.person_uuid
        LEFT JOIN (
            SELECT
                rater_person_uuid,
                COUNT(*) AS cards_rated
            FROM card_rating
            GROUP BY rater_person_uuid
        ) cr ON person.uuid = cr.rater_person_uuid
        LEFT JOIN (
            SELECT tracker_person_uuid AS person_uuid, COUNT(*) AS tracked_decks
            FROM tracked_deck
            GROUP BY tracker_person_uuid
        ) td ON person.uuid = td.person_uuid
        LEFT JOIN (
            SELECT followed_person_uuid AS person_uuid, COUNT(*) AS followers
            FROM follower
            GROUP BY followed_person_uuid
        ) fc ON person.uuid = fc.person_uuid
        LEFT JOIN (
            SELECT follower_person_uuid AS person_uuid, COUNT(*) AS following
            FROM follower
            GROUP BY follower_person_uuid
        ) fg ON person.uuid = fg.person_uuid
        WHERE person.uuid = $1
        LIMIT 1",
        uuid,
        person_uuid
    )
    .fetch_optional(&pg_pool)
    .await?
    .context_not_found("person could not be found")?;

    Ok(Json(PersonEnriched {
        person: Person {
            uuid: row.uuid,
            username: row.username,
            picture_url: row.picture_url,
            created_at: row.created_at,
            updated_at: row.updated_at,
        },
        followers: row.followers,
        following: row.following,
        likes: row.likes,
        dislikes: row.dislikes,
        cards_rated: row.cards_rated,
        tracked_decks: row.tracked_decks,
        am_following: row.am_following,
        followed_on: None,
        started_following: None,
    }))
}

async fn get_me(State(pg_pool): State<PgPool>, Auth { person_uuid }: Auth) -> ApiResult<Json<Person>> {
    let person = sqlx::query_as!(
        Person,
        "SELECT
            person.*
        FROM person
        WHERE person.uuid = $1
        LIMIT 1",
        person_uuid
    )
    .fetch_optional(&pg_pool)
    .await?
    .context_not_found("person could not be found")?;

    Ok(Json(person))
}

async fn post_follow_person(
    State(pg_pool): State<PgPool>,
    Auth { person_uuid }: Auth,
    Path(uuid): Path<uuid::Uuid>,
) -> ApiResult<()> {
    sqlx::query!(
        "INSERT INTO follower (follower_person_uuid, followed_person_uuid)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING",
        person_uuid,
        uuid
    )
    .execute(&pg_pool)
    .await?;

    Ok(())
}

async fn post_unfollow_person(
    State(pg_pool): State<PgPool>,
    Auth { person_uuid }: Auth,
    Path(uuid): Path<uuid::Uuid>,
) -> ApiResult<()> {
    sqlx::query!(
        "DELETE FROM follower
        WHERE follower_person_uuid = $1 AND followed_person_uuid = $2",
        person_uuid,
        uuid
    )
    .execute(&pg_pool)
    .await?;

    Ok(())
}
