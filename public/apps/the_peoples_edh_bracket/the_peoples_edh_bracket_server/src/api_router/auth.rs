use axum::{
    extract::{FromRequestParts, Request},
    http::{HeaderMap, request::Parts},
    middleware::Next,
    response::Response,
};
use reqwest::StatusCode;

#[derive(Clone, Copy)]
pub struct Auth {
    pub person_uuid: uuid::Uuid,
}

pub async fn auth_middleware(headers: HeaderMap, mut request: Request, next: Next) -> Response {
    let authorization = headers.get("Authorization").and_then(|value| value.to_str().ok());

    #[allow(clippy::single_match)]
    match authorization.and_then(|auth| auth.split_once(" ")) {
        // VERY IMPORTANT - WE ONLY WANT THIS DEBUG AUTH DURING DEV
        #[cfg(debug_assertions)]
        Some(("Debug", raw_person_uuid)) => {
            let person_uuid = raw_person_uuid.try_into().unwrap();
            request.extensions_mut().insert(Auth { person_uuid });
        }
        _ => {}
    }

    next.run(request).await
}

impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<Auth>().copied().ok_or(StatusCode::UNAUTHORIZED)
    }
}
