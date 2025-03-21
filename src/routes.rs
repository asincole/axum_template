use axum::{
    Router,
    http::{
        Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    response::{Html, IntoResponse},
    routing::get,
};
use std::{sync::Arc, time::Duration};
use tower_http::{
    classify::ServerErrorsFailureClass,
    cors::{Any, CorsLayer},
};
use tracing::Span;

use crate::{api_config::AppState, utils::app_error::AppError};

pub async fn index() -> Result<impl IntoResponse, AppError> {
    Ok(Html(include_str!("../holla.txt").to_string()))
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .allow_origin(Any);

    let router = Router::new().route("/", get(index));

    let protected_routes = Router::new().layer(cors).with_state(state);

    router
        .merge(protected_routes)
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
