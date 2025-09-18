use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Password is too short, minimum length is 8 characters")]
    PasswordTooShort,
    #[error("Password is too long, maximum length is 64 characters")]
    PasswordTooLong,
    #[error("Confirm password is too short, minimum length is 8 characters")]
    ConfirmPasswordTooShort,
    #[error("Confirm password is too long, maximum length is 64 characters")]
    ConfirmPasswordTooLong,
    #[error("Password and confirm password are not match")]
    PasswordAndConfirmPasswordAreNotMatch,
    #[error("Password hashing error: {0}")]
    PasswordHashingError(argon2::password_hash::Error),
}
