use axum::Json;
use serde_json::Value;

use crate::{
    custom::{
        errors::{app_error::AppError, validation::ValidationError},
        responder::ApiResponse,
        result::AppResult,
    },
    dtos::requests::register::RegisterRequest,
};

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Authentication Endpoint",
    request_body = RegisterRequest,
    responses(
        (
            status = 200,
            description = "Registration successful",
            body = ApiResponse<Value>,
            example = r#"{
                "status": "success",
                "code": 200,
                "message": "Registration successful.",
                "data": null
            }"#
        ),
        (
            status = 400,
            description = "Validation error (e.g., empty field, invalid email)",
            body = ApiResponse<Value>,
            example = r#"{
                "status": "error",
                "code": 400,
                "message": "Field 'name' cannot be empty",
                "data": null
            }"#
        )
    )
)]
pub async fn register_handler(Json(payload): Json<RegisterRequest>) -> AppResult<ApiResponse<()>> {
    if payload.name.is_empty() {
        return Err(AppError::ValidationError(ValidationError::EmptyField(
            "name".to_string(),
        )));
    }
    if payload.email.is_empty() {
        return Err(AppError::ValidationError(ValidationError::EmptyField(
            "email".to_string(),
        )));
    }
    if payload.password.is_empty() {
        return Err(AppError::ValidationError(ValidationError::EmptyField(
            "password".to_string(),
        )));
    }
    if payload.confirm_password.is_empty() {
        return Err(AppError::ValidationError(ValidationError::EmptyField(
            "confirm_password".to_string(),
        )));
    }
    Ok(ApiResponse::success(
        "Registration successful.".to_string(),
        (),
    ))
}
