use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tracing::error;

use crate::errors::password::PasswordError;
use crate::{dtos::api_response::ApiResponse, errors::validation::ValidationError};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] Box<figment::Error>),
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationError),
    #[error("Password error: {0}")]
    PasswordError(#[from] PasswordError),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Other error: {0}")]
    OtherError(#[from] Box<anyhow::Error>),
}

impl From<figment::Error> for AppError {
    fn from(err: figment::Error) -> Self {
        AppError::ConfigError(Box::new(err))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::ConfigError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, 1001, err.to_string())
            }
            AppError::ValidationError(err) => match err {
                ValidationError::InvalidEmail => (StatusCode::BAD_REQUEST, 1101, err.to_string()),
                ValidationError::InvalidUsername => {
                    (StatusCode::BAD_REQUEST, 1102, err.to_string())
                }
                ValidationError::EmptyField(_) => (StatusCode::BAD_REQUEST, 1103, err.to_string()),
            },
            AppError::PasswordError(err) => match err {
                PasswordError::PasswordTooShort => (StatusCode::BAD_REQUEST, 2001, err.to_string()),
                PasswordError::PasswordTooLong => (StatusCode::BAD_REQUEST, 2002, err.to_string()),
                PasswordError::ConfirmPasswordTooShort => {
                    (StatusCode::BAD_REQUEST, 2003, err.to_string())
                }
                PasswordError::ConfirmPasswordTooLong => {
                    (StatusCode::BAD_REQUEST, 2004, err.to_string())
                }
                PasswordError::PasswordAndConfirmPasswordAreNotMatch => {
                    (StatusCode::BAD_REQUEST, 2005, err.to_string())
                }
                PasswordError::PasswordHashingError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, 2006, err.to_string())
                }
            },
            AppError::IOError(err) => (StatusCode::INTERNAL_SERVER_ERROR, 3001, err.to_string()),
            AppError::OtherError(err) => (StatusCode::INTERNAL_SERVER_ERROR, 9999, err.to_string()),
        };

        if status.is_server_error() {
            error!(error = ?self, "An internal server error occurred");
        }

        let body = Json(ApiResponse {
            status: "error".to_string(),
            code,
            message: message.to_string(),
            data: None::<()>,
            cookies: Vec::new()
        });

        (status, body).into_response()
    }
}

pub type AppResult<T> = std::result::Result<T, AppError>;
