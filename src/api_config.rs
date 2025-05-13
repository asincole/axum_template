pub use db_config::{DbConfig};
use anyhow::Context;
use dotenvy::var;
use std::time::Duration;

mod db_config;

/// `AppConfig` holds all configuration parameters for the application.
///
/// This struct centralizes all environment-based configurations in one place,
/// making it easier to manage application settings. This is also run very early in 
/// the server startup and fails early if a necessary config is missing. For this template project,
/// it contains just a sample configuration value, but in real applications,
/// it would contain database URLs, API keys, feature flags, etc.
#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
}

impl AppConfig {
    /// Creates an `AppConfig` instance by loading values from environment variables.
    ///
    /// This method uses dotenvy to read configuration from environment variables,
    /// which enables 12-factor app style configuration. This approach allows for
    /// easy configuration changes between different environments (dev, staging, prod)
    /// without code changes.
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = var("DATABASE_URL").context("DATABASE_URL must be set")?;

        Ok(AppConfig { database_url })
    }
}

/// `AppState` represents the shared application state that will be available to all request handlers.
///
/// This struct is a core component of Axum applications, as it provides a way to share
/// resources like database connections, HTTP clients, and configuration across different
/// request handlers. Axum will clone this for each request, so all members should be
/// inexpensive to clone (e.g. Arc-wrapped resources or other lightweight handles).
#[derive(Clone)]
pub struct AppState {
    pub http_client: reqwest::Client,
    pub config: AppConfig,
    pub db: DbConfig,
}

impl AppState {
    /// Creates a new instance of the application state with all required resources.
    ///
    /// This initializes all shared resources needed by route handlers, such as
    /// - HTTP clients for external API calls
    /// - Configuration settings
    ///
    /// In a more complex application, this would also initialize database connections,
    /// caches, message queues, etc.
    pub async fn new() -> anyhow::Result<Self> {
        let config = AppConfig::from_env()?;
        let db = DbConfig::new(&config.database_url).await?;
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create the HTTP client");
        Ok(Self {
            config,
            http_client,
            db,
        })
    }
}
