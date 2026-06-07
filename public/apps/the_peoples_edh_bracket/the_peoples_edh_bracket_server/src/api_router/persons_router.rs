use std::num::NonZeroUsize;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use axum_anyhow::{ApiResult, OptionExt};
use names::Generator;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use sqlx::{Pool, Postgres};

use crate::{constants::TS_RS_EXPORT_TO, model::person::Person, state::AppState, util::parse_pagination};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_persons).post(post_debug_person))
        .route("/{uuid}", get(get_person))
}

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[serde_inline_default]
#[derive(Deserialize)]
struct GetPersonsParams {
    #[serde_inline_default(None)]
    q: Option<String>,
    #[serde_inline_default(NonZeroUsize::MIN)]
    page: NonZeroUsize,
    #[serde_inline_default(const { NonZeroUsize::new(100).expect("100 > 0") })]
    page_size: NonZeroUsize,
}

async fn get_persons(
    State(pg_pool): State<Pool<Postgres>>,
    Query(GetPersonsParams { q, page, page_size }): Query<GetPersonsParams>,
) -> ApiResult<Json<Vec<Person>>> {
    let (limit, offset) = parse_pagination(page, page_size);

    let persons = sqlx::query_as!(
        Person,
        "SELECT * FROM person
        WHERE ($1::text IS NULL OR $1 <% username)
        ORDER BY
            CASE WHEN $1 IS NOT NULL THEN word_similarity($1, username) END DESC NULLS LAST,
            username
        LIMIT $2 OFFSET $3",
        q,
        limit,
        offset
    )
    .fetch_all(&pg_pool)
    .await?;

    Ok(Json(persons))
}

async fn post_debug_person(State(pg_pool): State<Pool<Postgres>>) -> ApiResult<(StatusCode, Json<Person>)> {
    let username = Generator::with_naming(names::Name::Numbered)
        .next()
        .context_internal("failed to generate name?")?;

    let person = sqlx::query_as!(
        Person,
        "INSERT INTO person (username) VALUES ($1)
        RETURNING *",
        username
    )
    .fetch_one(&pg_pool)
    .await?;

    Ok((StatusCode::CREATED, Json(person)))
}

async fn get_person(State(pg_pool): State<Pool<Postgres>>, Path(uuid): Path<uuid::Uuid>) -> ApiResult<Json<Person>> {
    let person = sqlx::query_as!(Person, "SELECT * FROM person WHERE uuid = $1 LIMIT 1", uuid)
        .fetch_optional(&pg_pool)
        .await?
        .context_not_found("person could not be found")?;

    Ok(Json(person))
}
