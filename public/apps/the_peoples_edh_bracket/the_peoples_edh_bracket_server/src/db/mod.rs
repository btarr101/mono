use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub mod constants;

pub async fn setup_pg_pool(database_url: &str) -> anyhow::Result<Pool<Postgres>> {
    let cores = std::thread::available_parallelism().map(|cores| cores.get()).unwrap_or(2);

    let pool = PgPoolOptions::new()
        .max_connections(cores as u32)
        .connect(database_url)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
