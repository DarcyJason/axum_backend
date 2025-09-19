use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Email is invalid")]
    InvalidEmail,
    #[error("Username is invalid, it can only contain letters, numbers, and underscores")]
    InvalidUsername,
    #[error("Field '{0}' cannot be empty")]
    EmptyField(String),
}
