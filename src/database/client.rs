use crate::config::Config;
use crate::custom::result::AppResult;
use crate::database::{postgres::client::PostgresClient, redis::client::RedisClient};

pub struct DBClient {
    pub postgres_client: PostgresClient,
    pub redis_client: RedisClient,
}

impl DBClient {
    pub async fn new(config: Config) -> AppResult<Self> {
        let postgres_client = PostgresClient::new(config.postgres_server).await?;
        let redis_client = RedisClient::new(config.redis_server).await?;
        Ok(DBClient {
            postgres_client,
            redis_client,
        })
    }
}
