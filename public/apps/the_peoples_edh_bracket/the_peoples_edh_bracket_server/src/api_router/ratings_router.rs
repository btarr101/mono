use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use bigdecimal::BigDecimal;
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::{
    db::methods::{self, InsertCardRatingParams, UpsertCardParams},
    scryfall::client::{
        ScryfallClient,
        cards::{ScryfallCardsCollectionEntry, ScryfallCardsCollectionParams},
    },
    state::AppState,
};

pub fn get_router() -> Router<AppState> { Router::new().route("/", post(post_rating)) }

#[derive(Deserialize)]
struct PostRatingBody {
    card_oracle_id: uuid::Uuid,
    points: BigDecimal,
    reason: Option<String>,
    // TEMP - we need to derive this from auth
    user_uuid: uuid::Uuid,
}

async fn post_rating(
    State(pool): State<Pool<Postgres>>,
    State(scryfall): State<ScryfallClient<'static>>,
    Json(body): Json<PostRatingBody>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut tx = pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !methods::check_card_exists(&mut *tx, body.card_oracle_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let scryfall_card = scryfall
            .cards()
            .collection(ScryfallCardsCollectionParams {
                identifiers: &[ScryfallCardsCollectionEntry {
                    oracle_id: &body.card_oracle_id,
                }],
            })
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .data
            .into_iter()
            .next()
            .ok_or(StatusCode::NOT_FOUND)?;

        methods::upsert_card(
            &mut *tx,
            UpsertCardParams {
                oracle_id: &scryfall_card.oracle_id,
                name: &scryfall_card.name,
                image_uri: scryfall_card.image_uris.as_ref().and_then(|u| u.mediumest()),
            },
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    let rating = methods::insert_rating(
        &mut *tx,
        InsertCardRatingParams {
            card_oracle_id: &body.card_oracle_id,
            rater_person_uuid: &body.user_uuid,
            points: body.points.to_ref(),
            reason: body.reason.as_deref(),
        },
    )
    .await
    .map_err(|e| match e {
        methods::InsertRatingError::Conflict => StatusCode::CONFLICT,
        methods::InsertRatingError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(rating)))
}
