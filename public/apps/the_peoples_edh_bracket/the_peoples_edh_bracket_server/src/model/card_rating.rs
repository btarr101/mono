use bigdecimal::BigDecimal;
use serde::Serialize;

use crate::constants::TS_RS_EXPORT_TO;

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[derive(sqlx::FromRow, Serialize)]
pub struct CardRating {
    pub uuid: uuid::Uuid,
    pub card_oracle_id: uuid::Uuid,
    pub rater_person_uuid: uuid::Uuid,
    pub points: BigDecimal,
    pub reason: Option<String>,
}
