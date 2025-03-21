use axum::http::Method;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tower_http::cors::{Any, CorsLayer};
use utoipa::openapi::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

pub fn configure_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .allow_origin(Any)
}

pub fn configure_swagger(protected_routes: OpenApi) -> SwaggerUi {
    let swagger_config = Config::new(["/api-docs/openapi.json"]).persist_authorization(true);
    let swagger = SwaggerUi::new("/swagger")
        .url("/api-docs/openapi.json", protected_routes)
        .config(swagger_config);
    swagger
}
