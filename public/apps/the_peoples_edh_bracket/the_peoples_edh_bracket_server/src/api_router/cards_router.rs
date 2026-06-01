use std::num::NonZeroUsize;

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;
use serde_inline_default::serde_inline_default;

use crate::{
    db::methods::{self, ListCardsParams},
    state::AppState,
};

pub fn get_router() -> Router<AppState> { Router::new().route("/", get(get_cards)) }

#[serde_inline_default]
#[derive(Deserialize)]
struct GetCardsParams {
    #[serde_inline_default(NonZeroUsize::MIN)]
    page: NonZeroUsize,
    #[serde_inline_default(const { NonZeroUsize::new(100).expect("100 > 0") })]
    page_size: NonZeroUsize,
}

async fn get_cards(
    State(app_state): State<AppState>,
    Query(GetCardsParams { page, page_size }): Query<GetCardsParams>,
) -> Result<impl IntoResponse, StatusCode> {
    let cards = methods::list_cards(&app_state.pg_pool, ListCardsParams { page, page_size })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(cards))
}
