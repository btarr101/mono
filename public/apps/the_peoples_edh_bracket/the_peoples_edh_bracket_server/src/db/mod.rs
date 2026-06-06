use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub mod constants;
pub async fn setup_pg_pool() -> anyhow::Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        // TODO - These should come from a config
        .max_connections(5)
        .connect("postgres://admin:root@localhost:5432/db")
        .await?;
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
