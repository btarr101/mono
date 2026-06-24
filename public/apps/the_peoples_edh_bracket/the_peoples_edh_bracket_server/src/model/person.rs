use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::constants::TS_RS_EXPORT_TO;

#[derive(ts_rs::TS)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Person {
    pub uuid: uuid::Uuid,
    pub username: String,
    pub picture_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
