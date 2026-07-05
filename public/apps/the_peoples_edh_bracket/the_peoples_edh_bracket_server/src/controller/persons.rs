use anyhow::anyhow;
use names::Generator;
use sqlx::PgPool;

use crate::model::person::Person;

pub async fn create_debug_person(pg_pool: &PgPool) -> anyhow::Result<Person> {
    let username = Generator::with_naming(names::Name::Numbered)
        .next()
        .ok_or(anyhow!("Failed to generate name?"))?;

    let person = sqlx::query_as!(
        Person,
        "INSERT INTO person (username) VALUES ($1)
        RETURNING *",
        username
    )
    .fetch_one(pg_pool)
    .await?;

    Ok(person)
}

pub async fn upsert_person(
    uuid: &uuid::Uuid,
    username: &str,
    pg_pool: &PgPool,
    picture_url: Option<&str>,
) -> anyhow::Result<Person> {
    let person = sqlx::query_as!(
        Person,
        "INSERT INTO person (uuid, username, picture_url) VALUES ($1, $2, $3)
        ON CONFLICT (uuid)
        DO UPDATE SET
            username = EXCLUDED.username,
            picture_url = EXCLUDED.picture_url
        RETURNING *",
        uuid,
        username,
        picture_url
    )
    .fetch_one(pg_pool)
    .await?;

    Ok(person)
}
