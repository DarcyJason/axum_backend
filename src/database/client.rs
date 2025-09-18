use crate::config::Config;
use crate::database::{postgres::client::PostgresClient, redis::client::RedisClient};

pub struct DBClient {
    postgres_client: PostgresClient,
    redis_client: RedisClient,
}

impl DBClient {
    pub fn new(config: Config) -> Self {
        let postgres_client = PostgresClient::new(config.postgres_server);
        let redis_client = RedisClient::new(config.redis_server);
        DBClient {
            postgres_client,
            redis_client,
        }
    }
}
