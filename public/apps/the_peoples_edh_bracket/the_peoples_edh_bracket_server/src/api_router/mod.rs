use axum::Router;

use crate::state::AppState;

mod cards_router;
mod persons_router;
mod ratings_router;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .nest("/persons", persons_router::get_router())
        .nest("/cards", cards_router::get_router())
        .nest("/ratings", ratings_router::get_router())
}
