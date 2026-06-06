use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct Person {
    pub uuid: uuid::Uuid,
}
