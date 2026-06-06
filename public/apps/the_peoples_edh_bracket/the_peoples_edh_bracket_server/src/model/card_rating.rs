use bigdecimal::BigDecimal;
use serde::Serialize;

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(sqlx::FromRow, Serialize)]
pub struct CardRating {
    pub uuid: uuid::Uuid,
    pub card_oracle_id: uuid::Uuid,
    pub rater_person_uuid: uuid::Uuid,
    pub points: BigDecimal,
    pub reason: Option<String>,
}
