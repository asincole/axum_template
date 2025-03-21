use std::time::Duration;

use anyhow::Context;
use dotenvy::var;

/// `AppConfig` holds all configuration parameters for the application.
///
/// This struct centralizes all environment-based configuration in one place,
/// making it easier to manage application settings. This is also run very early in,
/// the server startup and fails early if a needed config is missing. For this template project,
/// it contains just a sample configuration value, but in real applications,
/// it would contain database URLs, API keys, feature flags, etc.
#[derive(Clone)]
pub struct AppConfig {
    pub example_config: String,
}

impl AppConfig {
    /// Creates an `AppConfig` instance by loading values from environment variables.
    ///
    /// This method uses dotenvy to read configuration from environment variables,
    /// which enables 12-factor app style configuration. This approach allows for
    /// easy configuration changes between different environments (dev, staging, prod)
    /// without code changes.
    pub fn from_env() -> anyhow::Result<Self> {
        let example_config = var("EXAMPLE_CONFIG").context("EXAMPLE_CONFIG must be set")?;

        Ok(AppConfig { example_config })
    }
}

/// `AppState` represents the shared application state that will be available to all request handlers.
///
/// This struct is a core component of Axum applications, as it provides a way to share
/// resources like database connections, HTTP clients, and configuration across different
/// request handlers. Axum will clone this for each request, so all members should be
/// inexpensive to clone (e.g., Arc-wrapped resources or other lightweight handles).
#[derive(Clone)]
pub struct AppState {
    pub http_client: reqwest::Client,
    pub config: AppConfig,
}

impl AppState {
    /// Creates a new instance of the application state with all required resources.
    ///
    /// This initializes all shared resources needed by route handlers, such as:
    /// - HTTP clients for external API calls
    /// - Configuration settings
    ///
    /// In a more complex application, this would also initialize database connections,
    /// caches, message queues, etc.
    pub async fn new() -> anyhow::Result<Self> {
        let config = AppConfig::from_env()?;
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");
        Ok(Self {
            config,
            http_client,
        })
    }
}
