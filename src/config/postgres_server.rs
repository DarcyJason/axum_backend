use figment::Figment;
use figment::providers::Env;
use serde::{Deserialize, Serialize};

use crate::custom::result::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresServerConfig {
    pub postgres_address: String,
}

impl PostgresServerConfig {
    fn figment() -> Figment {
        Figment::new().merge(Env::prefixed(""))
    }
    pub fn init() -> AppResult<Self> {
        Ok(PostgresServerConfig::figment().extract()?)
    }
}
