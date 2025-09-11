use figment::{Figment, providers::Env};
use serde::{Deserialize, Serialize};

use crate::errors::app_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresServerConfig {
    pub postgres_database_url: String,
}

impl PostgresServerConfig {
    fn figment() -> Figment {
        Figment::new().merge(Env::prefixed(""))
    }
    pub fn init() -> AppResult<Self> {
        Ok(PostgresServerConfig::figment().extract()?)
    }
}
