use std::sync::Arc;

use anyhow::Context;
use api_config::AppState;
use routes::create_router;
use tokio::net::TcpListener;
use tracing::info;
use utils::tracing_setup::setup_subscriber;

mod api_config;
mod dtos;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = setup_subscriber();
    tracing::subscriber::set_global_default(subscriber).expect("Tracing subscriber setup failed");

    let app_state: Arc<AppState> = Arc::new(
        AppState::new()
            .await
            .context("Failed to create global app state")?,
    );
    let app = create_router(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
