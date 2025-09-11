use crate::handlers::health::__path_health_handler;
use crate::handlers::health::health_handler;
use crate::state::AppState;
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn public_routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(routes!(health_handler))
}
