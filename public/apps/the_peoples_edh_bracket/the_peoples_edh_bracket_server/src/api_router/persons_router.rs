use std::num::NonZeroUsize;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use axum_anyhow::{ApiResult, OptionExt};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use sqlx::{Pool, Postgres};
use strum::Display;

use crate::{
    constants::TS_RS_EXPORT_TO, controller::persons::create_debug_person, middleware::auth::Auth, model::person::Person,
    state::AppState, util::parse_pagination,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_persons).post(post_debug_person))
        .route("/me", get(get_me))
        .route("/{uuid}", get(get_person))
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

#[derive(ts_rs::TS, Deserialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde_inline_default]
struct GetPersonsParams {
    #[serde_inline_default(None)]
    q: Option<String>,
    #[serde_inline_default(None)]
    sort: Option<GetPersonsParamsSort>,
    #[serde_inline_default(NonZeroUsize::MIN)]
    page: NonZeroUsize,
    #[serde_inline_default(const { NonZeroUsize::new(100).expect("100 > 0") })]
    page_size: NonZeroUsize,
}

#[derive(ts_rs::TS, Deserialize, Serialize)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
struct PersonWithMetrics {
    #[serde(flatten)]
    person: Person,
    followers: i64,
    following: i64,
    likes: i64,
    dislikes: i64,
    cards_rated: i64,
}

async fn get_persons(
    State(pg_pool): State<Pool<Postgres>>,
    Query(GetPersonsParams {
        q,
        sort,
        page,
        page_size,
    }): Query<GetPersonsParams>,
) -> ApiResult<Json<Vec<PersonWithMetrics>>> {
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
            COALESCE(cr.cards_rated, 0) AS \"cards_rated!\"
        FROM person
        LEFT JOIN person_likes_dislikes_cache pldc ON person.uuid = pldc.person_uuid
        LEFT JOIN (
            SELECT
                rater_person_uuid,
                COUNT(*) AS cards_rated
            FROM card_rating
            GROUP BY rater_person_uuid
        ) cr ON person.uuid = cr.rater_person_uuid
        WHERE ($1::text IS NULL OR lower(username) LIKE lower($1) || '%')
        ORDER BY
            CASE WHEN $2::TEXT = 'likes' THEN COALESCE(likes, 0) END DESC,
            CASE WHEN $2::TEXT = 'likes' THEN COALESCE(dislikes, 0) END ASC,
            CASE WHEN $2::TEXT = 'cards_rated' THEN COALESCE(cr.cards_rated, 0) END DESC,
            username
        LIMIT $3 OFFSET $4",
        q,
        sort,
        limit,
        offset,
    )
    .fetch_all(&pg_pool)
    .await?
    .into_iter()
    .map(|row| PersonWithMetrics {
        person: Person {
            uuid: row.uuid,
            username: row.username,
            picture_url: row.picture_url,
            created_at: row.created_at,
            updated_at: row.updated_at,
        },
        followers: 0,
        following: 0,
        likes: row.likes,
        dislikes: row.dislikes,
        cards_rated: row.cards_rated,
    })
    .collect::<Vec<_>>();

    Ok(Json(persons))
}

async fn post_debug_person(State(pg_pool): State<Pool<Postgres>>) -> ApiResult<(StatusCode, Json<Person>)> {
    let person = create_debug_person(&pg_pool).await?;

    Ok((StatusCode::CREATED, Json(person)))
}

async fn get_person(State(pg_pool): State<Pool<Postgres>>, Path(uuid): Path<uuid::Uuid>) -> ApiResult<Json<Person>> {
    let person = sqlx::query_as!(Person, "SELECT * FROM person WHERE uuid = $1 LIMIT 1", uuid)
        .fetch_optional(&pg_pool)
        .await?
        .context_not_found("person could not be found")?;

    Ok(Json(person))
}

async fn get_me(State(pg_pool): State<Pool<Postgres>>, Auth { person_uuid }: Auth) -> ApiResult<Json<Person>> {
    let person = sqlx::query_as!(Person, "SELECT * FROM person WHERE uuid = $1 LIMIT 1", person_uuid)
        .fetch_optional(&pg_pool)
        .await?
        .context_not_found("person could not be found")?;

    Ok(Json(person))
}
