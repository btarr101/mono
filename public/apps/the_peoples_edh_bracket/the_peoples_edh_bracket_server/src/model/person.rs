use serde::Serialize;

use crate::constants::TS_RS_EXPORT_TO;

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[derive(sqlx::FromRow, Serialize)]
pub struct Person {
    pub uuid: uuid::Uuid,
    pub username: String,
}
