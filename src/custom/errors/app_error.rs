use axum::http::StatusCode;
use thiserror::Error;

use crate::custom::errors::password::PasswordError;
use crate::custom::errors::validation::ValidationError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] Box<figment::Error>),
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationError),
    #[error("Password error: {0}")]
    PasswordError(#[from] PasswordError),
    #[error("Postgres error: {0}")]
    PostgresError(#[from] Box<sqlx::Error>),
    #[error("Redis error: {0}")]
    RedisError(#[from] Box<redis::RedisError>),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Other error: {0}")]
    OtherError(#[from] Box<anyhow::Error>),
}

impl AppError {
    pub fn status_code(&self) -> (StatusCode, String) {
        match self {
            AppError::ConfigError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::ValidationError(err) => match err {
                ValidationError::InvalidEmail => (StatusCode::BAD_REQUEST, err.to_string()),
                ValidationError::InvalidUsername => (StatusCode::BAD_REQUEST, err.to_string()),
                ValidationError::EmptyField(_) => (StatusCode::BAD_REQUEST, err.to_string()),
            },
            AppError::PasswordError(err) => match err {
                PasswordError::PasswordTooShort => (StatusCode::BAD_REQUEST, err.to_string()),
                PasswordError::PasswordTooLong => (StatusCode::BAD_REQUEST, err.to_string()),
                PasswordError::ConfirmPasswordTooShort => {
                    (StatusCode::BAD_REQUEST, err.to_string())
                }
                PasswordError::ConfirmPasswordTooLong => (StatusCode::BAD_REQUEST, err.to_string()),
                PasswordError::PasswordAndConfirmPasswordAreNotMatch => {
                    (StatusCode::BAD_REQUEST, err.to_string())
                }
                PasswordError::PasswordHashingError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            },
            AppError::PostgresError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::RedisError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::IOError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::OtherError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        }
    }
}
