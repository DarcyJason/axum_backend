use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::{config::postgres_server::PostgresServerConfig, custom::result::AppResult};

pub struct PostgresClient {
    pub conn: Pool<Postgres>,
}

impl PostgresClient {
    pub async fn new(postgres_server_config: PostgresServerConfig) -> AppResult<Self> {
        let conn = PgPoolOptions::new()
            .max_connections(10)
            .connect(&postgres_server_config.postgres_address)
            .await?;
        Ok(PostgresClient { conn })
    }
}
