use std::num::NonZeroUsize;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use axum_anyhow::{ApiResult, OptionExt};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use sqlx::{Pool, Postgres};

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
        WHERE ($1::text IS NULL OR lower(username) LIKE lower($1) || '%')
        ORDER BY username
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
