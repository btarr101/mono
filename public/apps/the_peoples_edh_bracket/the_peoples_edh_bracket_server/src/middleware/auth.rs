use std::convert::Infallible;

use axum::{
    extract::{FromRef, FromRequestParts, Request, State},
    http::{HeaderMap, request::Parts},
    middleware::Next,
    response::Response,
};
use oidc_jwt_validator::{ValidationSettings, Validator, cache::Strategy};
use reqwest::StatusCode;
use reqwest011::Client as Reqwest011Client;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{controller::persons::upsert_person, model::person::Person};

pub struct AuthMiddlewareParams<'a> {
    pub google_client_id: &'a str,
    pub pg_pool: PgPool,
}

#[derive(Clone)]
pub struct AuthMiddlewareState {
    google_oidc_validator: Validator,
    pg_pool: PgPool,
}

impl FromRef<AuthMiddlewareState> for Validator {
    fn from_ref(state: &AuthMiddlewareState) -> Self { state.google_oidc_validator.clone() }
}

impl FromRef<AuthMiddlewareState> for PgPool {
    fn from_ref(state: &AuthMiddlewareState) -> Self { state.pg_pool.clone() }
}

impl AuthMiddlewareState {
    pub async fn new(
        AuthMiddlewareParams {
            google_client_id,
            pg_pool,
        }: AuthMiddlewareParams<'_>,
    ) -> anyhow::Result<Self> {
        let mut settings = ValidationSettings::new();
        settings.set_audience(&[google_client_id]);

        Ok(Self {
            google_oidc_validator: Validator::new(
                "https://accounts.google.com",
                Reqwest011Client::new(),
                Strategy::Automatic,
                settings,
            )
            .await?,
            pg_pool,
        })
    }
}

const GOOGLE_SUB_NAMESPACE: uuid::Uuid = uuid::uuid!("01f1c8fd-bff4-4f6b-a075-0bbd921f03c1");

#[derive(Debug, Deserialize)]
struct GoogleClaims {
    sub: String,
    email: String,
    picture: Option<String>,
}

#[derive(Clone, Copy)]
pub struct Auth {
    pub person_uuid: uuid::Uuid,
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

#[derive(Clone, Copy)]
pub struct OptionalAuth {
    pub person_uuid: Option<uuid::Uuid>,
}

impl<S> FromRequestParts<S> for OptionalAuth
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        Ok(Self {
            person_uuid: parts.extensions.get::<Auth>().copied().map(|auth| auth.person_uuid),
        })
    }
}

pub async fn auth_middleware(
    State(google_oidc_validator): State<Validator>,
    State(pg_pool): State<PgPool>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    let authorization = headers.get("Authorization").and_then(|value| value.to_str().ok());

    match authorization.and_then(|auth| auth.split_once(" ")) {
        // VERY IMPORTANT - WE ONLY WANT THIS DEBUG AUTH DURING DEV
        #[cfg(debug_assertions)]
        Some(("Debug", raw_person_uuid)) => {
            if let Ok(person_uuid) = raw_person_uuid.try_into() {
                request.extensions_mut().insert(Auth { person_uuid });
            }
        }
        Some(("Bearer", token)) => {
            if let Ok(token_data) = google_oidc_validator.validate::<GoogleClaims>(token).await {
                let uuid = uuid::Uuid::new_v5(&GOOGLE_SUB_NAMESPACE, token_data.claims.sub.as_bytes());
                let username = token_data.claims.email;
                let picture = token_data.claims.picture;

                match upsert_person(&uuid, &username, &pg_pool, picture.as_deref()).await {
                    Ok(Person { uuid, .. }) => {
                        request.extensions_mut().insert(Auth { person_uuid: uuid });
                    }
                    Err(error) => tracing::error!("Error creating google user {:?}: {:?}", username, error),
                }
            }
        }
        _ => {}
    }

    next.run(request).await
}
