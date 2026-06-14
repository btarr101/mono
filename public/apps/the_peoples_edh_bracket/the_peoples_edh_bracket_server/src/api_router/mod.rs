use axum::Router;

use crate::state::AppState;

mod cards_router;
mod home_route;
mod persons_router;
mod ratings_router;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .nest("/home", home_route::get_router())
        .nest("/persons", persons_router::get_router())
        .nest("/cards", cards_router::get_router())
        .nest("/ratings", ratings_router::get_router())
}
