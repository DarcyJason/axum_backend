use redis::Client;
use crate::config::redis_server::RedisServerConfig;

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(redis_server_config: RedisServerConfig) -> Self {
        let client = redis::Client::open(redis_server_config.redis_address).unwrap();
        RedisClient { client }
    }
}
