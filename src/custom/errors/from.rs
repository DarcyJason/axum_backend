use crate::custom::errors::app_error::AppError;

impl From<figment::Error> for AppError {
    fn from(err: figment::Error) -> Self {
        AppError::ConfigError(Box::new(err))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::PostgresError(Box::new(err))
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::RedisError(Box::new(err))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::OtherError(Box::new(err))
    }
}
