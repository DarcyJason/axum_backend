use serde_json::{Value, json};

use crate::{dtos::api_response::ApiResponse, errors::app_error::AppResult};

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health Checker Endpoint",
    responses(
        (
            status = 200,
            description = "The server is up and running",
            body = ApiResponse<Value>,
            example = r#"{
                "status": "success",
                "code": 200,
                "message": "The server is up and running",
                "data": {}
            }"#
        )
    )
)]
pub async fn health_handler() -> AppResult<ApiResponse<Value>> {
    Ok(ApiResponse::success(
        "The server is up and running",
        json!({}),
    ))
}
