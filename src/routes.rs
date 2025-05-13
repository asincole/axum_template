use crate::api_config::AppState;
use crate::routes::config::{configure_cors, configure_swagger};
use crate::utils::api_error::ApiError;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{sync::Arc, time::Duration};
use tower_http::classify::ServerErrorsFailureClass;
use tracing::Span;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme}, Modify,
    OpenApi,
};
use utoipa_axum::router::OpenApiRouter;

mod config;
mod users_handler;

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    security(
           // ("jwt_token" = ["read:items", "edit:items"]),
           ("jwt_token" = [])
       )
)]
struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components = Some(
            utoipa::openapi::ComponentsBuilder::new()
                .security_scheme(
                    "jwt_token",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(HttpAuthScheme::Bearer)
                            .bearer_format("JWT")
                            .build(),
                    ),
                )
                .build(),
        );
    }
}

pub async fn index() -> Result<impl IntoResponse, ApiError> {
    Ok(Html(include_str!("../holla.txt").to_string()))
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = configure_cors();

    let (router, protected_routes) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/users", users_handler::router())
        .layer(cors)
        .with_state(state)
        .split_for_parts();

    let swagger_handler = configure_swagger(protected_routes.clone());

    router
        .merge(swagger_handler)
        .route("/", get(index))
        .layer(tower_http::trace::TraceLayer::new_for_http().on_failure(
            |error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                tracing::error!(
                    error = ?error,
                    latency = ?latency,
                );
            },
        ))
        .layer(tower_http::compression::CompressionLayer::new())
}
