use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::{net::TcpListener, signal};
use tracing::{error, info};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config::Config,
    constants::logo::LOGO,
    database::{client::DBClient, init::init_postgres},
    errors::app_error::AppResult,
    routes::create_routes,
    security::cors::cors,
    state::AppState,
};

pub async fn run() -> AppResult<()> {
    println!("{}", LOGO);

    info!("\n\n=============== NEW SESSION STARTED ===============\n");

    #[derive(OpenApi)]
    #[openapi(
            modifiers(&SecurityAddon),
            tags(
                (name = "Backend", description = "Axum backend API")
            )
        )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "api_key",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("backend_apikey"))),
                )
            }
        }
    }

    let config = Config::init().map_err(|e| {
        error!("Failed to initialize config: {}", e);
        e
    })?;
    let pg_pool = init_postgres(config.clone()).await;
    let db_client = DBClient::new(pg_pool);
    let port = config.clone().backend_server.backend_port;
    let frontend_address = config.clone().frontend_server.frontend_address;

    info!("The backend server is running at http://localhost:{}", port);

    let app_state = Arc::new(AppState::new(config, db_client));

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1/", create_routes())
        .layer(cors(frontend_address))
        .with_state(app_state.clone())
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(Redoc::with_url("/redoc", api.clone()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api));

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = TcpListener::bind(&address).await?;

    info!(
        "The classic API doc is at http://localhost:{}/swagger-ui",
        address.port()
    );
    info!(
        "The brief API doc is at http://localhost:{}/redoc",
        address.port()
    );
    info!(
        "The API doc bases on web component is at http://localhost:{}/rapidoc",
        address.port()
    );
    info!(
        "The modern API doc is at http://localhost:{}/scalar",
        address.port()
    );

    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(async {
            signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
        })
        .await?;
    Ok(())
}
