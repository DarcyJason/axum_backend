use crate::routes::protected::protected_routes;
use crate::routes::public::public_routes;
use crate::state::AppState;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

pub mod protected;
pub mod public;

pub fn create_routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .merge(public_routes())
        .merge(protected_routes())
}
