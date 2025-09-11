use crate::config::Config;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::error;

pub async fn init_postgres(config: Config) -> Pool<Postgres> {
    let pg_address = &config.clone().postgres_server.postgres_database_url;
    match PgPoolOptions::new()
        .max_connections(10)
        .connect(pg_address)
        .await
    {
        Ok(pool) => pool,
        Err(err) => {
            error!("Failed to connect to PostgreSQL: {}", err);
            std::process::exit(1)
        }
    }
}
