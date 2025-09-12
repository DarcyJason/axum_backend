use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::dtos::api_response::ApiResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(Box<figment::Error>),
    #[error("SurrealDB error: {0}")]
    SurrealDBError(Box<surrealdb::Error>),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Other error: {0}")]
    OtherError(Box<anyhow::Error>),
}

impl From<figment::Error> for AppError {
    fn from(err: figment::Error) -> Self {
        AppError::ConfigError(Box::new(err))
    }
}

impl From<surrealdb::Error> for AppError {
    fn from(err: surrealdb::Error) -> Self {
        AppError::SurrealDBError(Box::new(err))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::OtherError(Box::new(err))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::ConfigError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, 1001, err.to_string())
            }
            AppError::SurrealDBError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, 1002, err.to_string())
            }
            AppError::IOError(err) => (StatusCode::INTERNAL_SERVER_ERROR, 1003, err.to_string()),
            AppError::OtherError(err) => (StatusCode::INTERNAL_SERVER_ERROR, 1999, err.to_string()),
        };

        let body = Json(ApiResponse {
            status: "error".to_string(),
            code,
            message,
            data: None::<()>,
        });

        (status, body).into_response()
    }
}

pub type AppResult<T> = std::result::Result<T, AppError>;
