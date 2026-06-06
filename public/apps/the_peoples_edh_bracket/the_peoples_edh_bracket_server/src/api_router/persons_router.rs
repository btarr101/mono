use axum::{Json, Router, extract::State, routing::post};
use axum_anyhow::ApiResult;
use reqwest::StatusCode;
use sqlx::{Pool, Postgres};

use crate::{model::person::Person, state::AppState};

pub fn get_router() -> Router<AppState> { Router::new().route("/", post(post_person)) }

pub async fn post_person(State(pg_pool): State<Pool<Postgres>>) -> ApiResult<(StatusCode, Json<Person>)> {
    let person = sqlx::query_as!(Person, "INSERT INTO person DEFAULT VALUES RETURNING *")
        .fetch_one(&pg_pool)
        .await?;

    Ok((StatusCode::CREATED, Json(person)))
}
