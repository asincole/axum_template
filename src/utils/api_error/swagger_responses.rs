use crate::utils::api_error::ApiError;
use serde_json::json;
use std::collections::BTreeMap;
use utoipa::openapi::ContentBuilder;
use utoipa::openapi::{RefOr, ResponseBuilder, ResponsesBuilder};

impl utoipa::IntoResponses for ApiError {
    fn responses() -> BTreeMap<String, RefOr<utoipa::openapi::Response>> {
        ResponsesBuilder::new()
            .response(
                "401",
                ResponseBuilder::new()
                    .description("Authentication Error")
                    .content("application/json",
                             ContentBuilder::new()
                                 .example(Some(json!({
                                "status": "Unauthorized",
                                "message": "Authentication failed: Invalid credentials"
                            })))
                                 .build(),
                    ),
            )
            .response(
                "403",
                ResponseBuilder::new()
                    .description("Authorization Error")
                    .content("application/json",
                             ContentBuilder::new()
                                 .example(Some(json!({
                                "status": "Forbidden",
                                "message": "Authorization failed: Insufficient permissions"
                            })))
                                 .build(),
                    ),
            )
            .response(
                "404",
                ResponseBuilder::new()
                    .description("Resource Not Found")
                    .content("application/json",
                             ContentBuilder::new()
                                 .example(Some(json!({
                                "status": "Not Found",
                                "message": "Resource not found: User with ID '123e4567-e89b-12d3-a456-426614174000' not found"
                            })))
                                 .build(),
                    ),
            )
            .response(
                "409",
                ResponseBuilder::new()
                    .description("Resource Conflict")
                    .content("application/json",
                             ContentBuilder::new()
                                 .example(Some(json!({
                                "status": "Conflict",
                                "message": "Resource already exists"
                            })))
                                 .build(),
                    ),
            )
            .response(
                "400",
                ResponseBuilder::new()
                    .description("Bad Request")
                    .content("application/json",
                             ContentBuilder::new()
                                 .example(Some(json!({
                                "status": "Bad Request",
                                "message": "Validation error: Email format is invalid"
                            })))
                                 .build(),
                    ),
            )
            .response(
                "500",
                ResponseBuilder::new()
                    .description("Internal Server Error")
                    .content("application/json",
                             ContentBuilder::new()
                                 .example(Some(json!({
                                "status": "Internal Server Error",
                                "message": "Internal server error: Unexpected error occurred"
                            })))
                                 .build(),
                    ),
            )
            .build()
            .into()
    }
}
