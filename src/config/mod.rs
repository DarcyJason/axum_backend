use serde::{Deserialize, Serialize};

use crate::{
    config::{
        backend_server::BackendServerConfig, frontend_server::FrontendServerConfig,
        mail_server::MailServerConfig, surrealdb_server::SurrealDBServerConfig,
    },
    errors::app_error::AppResult,
};

pub mod backend_server;
pub mod frontend_server;
pub mod mail_server;
pub mod surrealdb_server;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub backend_server: BackendServerConfig,
    pub frontend_server: FrontendServerConfig,
    pub mail_server: MailServerConfig,
    pub surrealdb_server: SurrealDBServerConfig,
}

impl Config {
    pub fn init() -> AppResult<Self> {
        Ok(Config {
            backend_server: BackendServerConfig::init()?,
            frontend_server: FrontendServerConfig::init()?,
            mail_server: MailServerConfig::init()?,
            surrealdb_server: SurrealDBServerConfig::init()?,
        })
    }
}
