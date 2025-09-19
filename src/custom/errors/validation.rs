use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Email is invalid")]
    InvalidEmail,
    #[error("Field '{0}' cannot be empty")]
    EmptyField(String),
}
