use serde_json::Value;

use crate::custom::responder::ApiResponse;

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
                "message": "The server is healthy",
                "data": null
            }"#
        )
    )
)]
pub async fn health_handler() -> ApiResponse<()> {
    ApiResponse::success("The server is healthy".to_string(), ())
}
