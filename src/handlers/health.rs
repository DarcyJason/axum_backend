use serde_json::{Value, json};

use crate::{errors::app_error::AppResult, response::ApiResponse};

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
                "status": success,
                "message": "The server is up and running",
                "code": 200,
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
