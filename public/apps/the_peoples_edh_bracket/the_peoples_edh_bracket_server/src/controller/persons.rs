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
