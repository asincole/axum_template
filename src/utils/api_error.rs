use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;
use utoipa::ToSchema;

mod swagger_responses;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ApiError {
    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Authorization failed: {0}")]
    Authorization(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("{0}")]
    AlreadyExists(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("an error occurred with the database")]
    #[serde(skip)]
    Sqlx(#[from] sqlx::Error),

    #[error("an internal server error occurred")]
    #[serde(skip)]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    status: &'static str,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Authorization(msg) => (StatusCode::FORBIDDEN, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::AlreadyExists(msg) => (StatusCode::CONFLICT, msg),
            ApiError::Validation(msg) | ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::Sqlx(ref msg) => {
                error!("SQLx error: {:?}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }
            ApiError::Anyhow(ref e) => {
                error!("Generic error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }
        };

        let body = ErrorResponse {
            status: status.canonical_reason().unwrap_or("Unknown"),
            message: error_message,
        };

        (status, axum::Json(body)).into_response()
    }
}
