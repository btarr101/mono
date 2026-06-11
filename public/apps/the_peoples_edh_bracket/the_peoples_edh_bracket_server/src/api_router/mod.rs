use axum::{Router, middleware::from_fn};

use crate::{api_router::auth::auth_middleware, state::AppState};

mod auth;
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
        .layer(from_fn(auth_middleware))
}
