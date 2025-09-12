use axum_backend::{observability::log::init_log, run};
use dotenvy::dotenv;
use tracing::error;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let _guard = init_log();

    if let Err(e) = run().await {
        error!("Application failed to start: {}", e);
        std::process::exit(1);
    }
}
