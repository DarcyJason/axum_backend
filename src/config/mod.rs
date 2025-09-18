use serde::{Deserialize, Serialize};

use crate::config::postgres_server::PostgresServerConfig;
use crate::config::redis_server::RedisServerConfig;
use crate::{
    config::{
        backend_server::BackendServerConfig, frontend_server::FrontendServerConfig,
        mail_server::MailServerConfig,
    },
    errors::app_error::AppResult,
};

pub mod backend_server;
pub mod frontend_server;
pub mod mail_server;
pub mod postgres_server;
pub mod redis_server;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub backend_server: BackendServerConfig,
    pub frontend_server: FrontendServerConfig,
    pub mail_server: MailServerConfig,
    pub postgres_server: PostgresServerConfig,
    pub redis_server: RedisServerConfig,
}

impl Config {
    pub fn init() -> AppResult<Self> {
        Ok(Config {
            backend_server: BackendServerConfig::init()?,
            frontend_server: FrontendServerConfig::init()?,
            mail_server: MailServerConfig::init()?,
            postgres_server: PostgresServerConfig::init()?,
            redis_server: RedisServerConfig::init()?,
        })
    }
}
