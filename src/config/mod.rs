use serde::{Deserialize, Serialize};

use crate::{
    config::{
        backend_server::BackendServerConfig, frontend_server::FrontendServerConfig,
        mail_server::MailServerConfig, postgres_server::PostgresServerConfig,
    },
    errors::app_error::AppResult,
};

pub mod backend_server;
pub mod frontend_server;
pub mod mail_server;
pub mod postgres_server;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub backend_server: BackendServerConfig,
    pub frontend_server: FrontendServerConfig,
    pub postgres_server: PostgresServerConfig,
    pub mail_server: MailServerConfig,
}

impl Config {
    pub fn init() -> AppResult<Self> {
        Ok(Config {
            backend_server: BackendServerConfig::init()?,
            frontend_server: FrontendServerConfig::init()?,
            postgres_server: PostgresServerConfig::init()?,
            mail_server: MailServerConfig::init()?,
        })
    }
}
