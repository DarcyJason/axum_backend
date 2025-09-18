use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use crate::config::postgres_server::PostgresServerConfig;

pub struct PostgresClient {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl PostgresClient {
    pub fn new(postgres_server_config: PostgresServerConfig) -> Self {
        let connection_information = format!("host={} user={}", postgres_server_config.postgres_host, postgres_server_config.postgres_user);
        let manager = PostgresConnectionManager::new(
            connection_information.parse().unwrap(),
            NoTls,
        );
        let pool = Pool::builder().build(manager).unwrap();
        PostgresClient { pool }
    }
}
