use cookie::SameSite;
use cookie::Cookie;
use axum::{
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
    #[serde(skip_serializing)]
    pub cookies: Vec<Cookie<'static>>,
}

impl<T> ApiResponse<T> {
    pub fn success(msg: &str, data: T) -> Self {
        Self {
            status: "success".to_string(),
            code: StatusCode::OK.as_u16(),
            message: msg.to_string(),
            data: Some(data),
            cookies: Vec::new(),
        }
    }

    pub fn error(status: StatusCode,msg: &str) -> Self {
        Self {
            status: "error".to_string(),
            code: status.as_u16(),
            message: msg.to_string(),
            data: None,
            cookies: Vec::new(),
        }
    }
    pub fn add_cookie(mut self, cookie: Cookie<'static>) -> Self {
        self.cookies.push(cookie);
        self
    }
    pub fn with_tokens(self, access_token: &str, refresh_token: Option<&str>) -> Self {
        let mut cookies = vec![];
        let mut access_cookie = Cookie::build(("access_token", access_token.to_owned()))
            .path("/")
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Strict)
            .max_age(time::Duration::hours(1))
            .build();
        cookies.push(access_cookie);
        if let Some(refresh_token) = refresh_token {
            let refresh_cookie = Cookie::build(("refresh_token", refresh_token.to_owned()))
                .path("/")
                .secure(true)
                .http_only(true)
                .same_site(SameSite::Strict)
                .max_age(time::Duration::days(7))
                .build();
            cookies.push(refresh_cookie);
        }
        Self { cookies, ..self }
    }
    pub fn revoke_tokens(self) -> Self {
        let access_cookie = Cookie::build(("access_token", ""))
            .path("/")
            .max_age(time::Duration::ZERO)
            .build();

        let refresh_cookie = Cookie::build(("refresh_token", ""))
            .path("/")
            .max_age(time::Duration::ZERO)
            .build();

        Self {
            cookies: vec![access_cookie, refresh_cookie],
            ..self
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body =
            serde_json::to_string(&self).unwrap_or_else(|_| "{\"status\":\"error\"}".to_string());
        let mut response = (
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::OK),
            [("content-type", "application/json")],
            body,
        )
            .into_response();
        let headers = response.headers_mut();
        for cookie in self.cookies {
            headers.append(
                axum::http::header::SET_COOKIE,
                cookie.to_string().parse().unwrap(),
            );
        }
        response
    }
}
