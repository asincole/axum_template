use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, PartialEq, ToSchema, Serialize, Deserialize, Default, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserDto {
    #[validate(length(
        min = 1,
        max = 50,
        message = "The first name must be between 1 and 50 characters"
    ))]
    pub first_name: String,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Last name must be between 1 and 50 characters"
    ))]
    pub last_name: String,

    #[validate(length(
        min = 4,
        max = 30,
        message = "Username must be between 4 and 30 characters"
    ))]
    pub username: String,

    #[validate(email(message = "Invalid email address"))]
    #[validate(length(max = 100, message = "Email must not exceed 100 characters"))]
    pub email: String,
}

#[derive(Debug, PartialEq, ToSchema, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    // #[validate(email)]
    pub email: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub version: i32,
}
