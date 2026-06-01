use bigdecimal::BigDecimal;
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct Person {
    pub uuid: uuid::Uuid,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Card {
    pub oracle_id: uuid::Uuid,
    pub name: String,
    pub image_uri: Option<String>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct CardRating {
    pub uuid: uuid::Uuid,
    pub card_oracle_id: uuid::Uuid,
    pub rater_person_uuid: uuid::Uuid,
    pub points: BigDecimal,
    pub reason: Option<String>,
}
