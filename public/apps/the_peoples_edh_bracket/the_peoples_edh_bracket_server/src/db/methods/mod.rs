use std::num::NonZeroUsize;

use bigdecimal::BigDecimalRef;
use sqlx::{Executor, Postgres};

use crate::db::{
    constants::PG_UNIQUE_VIOLATION,
    model::{Card, CardRating, Person},
};

pub async fn insert_person<'e, E>(executor: E) -> anyhow::Result<Person>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as!(Person, "INSERT INTO person DEFAULT VALUES RETURNING *")
        .fetch_one(executor)
        .await
        .map_err(anyhow::Error::new)
}

pub struct UpsertCardParams<'a> {
    pub oracle_id: &'a uuid::Uuid,
    pub name: &'a str,
    pub image_uri: Option<&'a str>,
}

pub async fn upsert_card<'e, E>(executor: E, params: UpsertCardParams<'_>) -> anyhow::Result<Card>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as!(
        Card,
        "INSERT INTO card (oracle_id, name, image_uri) VALUES ($1, $2, $3)
         ON CONFLICT (oracle_id) DO UPDATE SET name = EXCLUDED.name, image_uri = EXCLUDED.image_uri
         RETURNING *",
        params.oracle_id,
        params.name,
        params.image_uri,
    )
    .fetch_one(executor)
    .await
    .map_err(anyhow::Error::new)
}

pub async fn check_card_exists<'e, E>(executor: E, card_oracle_id: uuid::Uuid) -> anyhow::Result<bool>
where
    E: Executor<'e, Database = Postgres>,
{
    let record = sqlx::query!("SELECT oracle_id FROM card WHERE oracle_id = $1", card_oracle_id)
        .fetch_optional(executor)
        .await?;
    Ok(record.is_some())
}

pub struct ListCardsParams {
    pub page: NonZeroUsize,
    pub page_size: NonZeroUsize,
}

pub async fn list_cards<'e, E>(executor: E, params: ListCardsParams) -> anyhow::Result<Vec<Card>>
where
    E: Executor<'e, Database = Postgres>,
{
    let limit = params.page_size.get() as i64;
    let offset = ((params.page.get() - 1) * params.page_size.get()) as i64;
    sqlx::query_as!(Card, "SELECT * FROM card ORDER BY name LIMIT $1 OFFSET $2", limit, offset)
        .fetch_all(executor)
        .await
        .map_err(anyhow::Error::new)
}

pub struct InsertCardRatingParams<'a> {
    pub card_oracle_id: &'a uuid::Uuid,
    pub rater_person_uuid: &'a uuid::Uuid,
    pub points: BigDecimalRef<'a>,
    pub reason: Option<&'a str>,
}

#[derive(thiserror::Error, Debug)]
pub enum InsertRatingError {
    #[error("a rating for this card by this person already exists")]
    Conflict,
    #[error(transparent)]
    Other(#[from] sqlx::Error),
}

pub async fn insert_rating<'e, E>(executor: E, params: InsertCardRatingParams<'_>) -> Result<CardRating, InsertRatingError>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as!(
        CardRating,
        "INSERT INTO card_rating (card_oracle_id, rater_person_uuid, points, reason) VALUES ($1, $2, $3, $4)
         RETURNING *",
        params.card_oracle_id,
        params.rater_person_uuid,
        params.points.to_owned(),
        params.reason,
    )
    .fetch_one(executor)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some(PG_UNIQUE_VIOLATION) => InsertRatingError::Conflict,
        _ => InsertRatingError::Other(e),
    })
}
