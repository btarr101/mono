use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use reqwest::StatusCode;
use sqlx::{Pool, Postgres};

use crate::{db::methods, state::AppState};

pub fn get_router() -> Router<AppState> { Router::new().route("/", post(post_person)) }

pub async fn post_person(State(pg_pool): State<Pool<Postgres>>) -> Result<impl IntoResponse, StatusCode> {
    let person = methods::insert_person(&pg_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(person)))
}
