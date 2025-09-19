use crate::handlers::auth::{__path_register_handler, register_handler};
use crate::handlers::health::{__path_health_handler, health_handler};
use crate::state::AppState;
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn public_routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(health_handler))
        .routes(routes!(register_handler))
}
