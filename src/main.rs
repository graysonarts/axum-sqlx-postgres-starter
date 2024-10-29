use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use infra::{AppState, LogRequest};
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod infra;
mod models;
mod queries;
mod routes;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    let config = infra::Config::from_env()?;
    {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();
        info!("{NAME} v{VERSION}");

        let asset_svc = ServeDir::new(config.fallback_dir.to_owned());
        let app_state = AppState::from_config(&config).await?;
        let app_state = Arc::new(Mutex::new(app_state));

        let app = Router::new()
            // .nest("/api", routes::api::api_routes())
            .layer(CorsLayer::permissive()) // TODO: Replace with your CORS policy
            .layer(LogRequest::layer())
            .with_state(app_state)
            .fallback_service(asset_svc);

        let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
        let listener = TcpListener::bind(&addr).await?;
        tracing::info!("Server listening on http://{}", addr);
        axum::serve(listener, app.into_make_service()).await?;
    }

    Ok(())
}
