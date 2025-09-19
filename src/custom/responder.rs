use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub status: String,
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: String, data: T) -> Self {
        ApiResponse {
            status: "success".to_string(),
            code: 200,
            message,
            data: Some(data),
        }
    }
    pub fn error(code: StatusCode, message: String) -> Self {
        ApiResponse {
            status: "error".to_string(),
            code: code.as_u16(),
            message,
            data: None,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = match StatusCode::from_u16(self.code) {
            Ok(code) => code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(self)).into_response()
    }
}
