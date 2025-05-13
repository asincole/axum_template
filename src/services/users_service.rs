use crate::dtos::{CreateUserDto, UserDto};
use crate::utils::api_error::ApiError;
use sqlx::{query_as, Pool, Postgres};
use tracing::error;
use uuid::Uuid;

#[cfg_attr(test, faux::create)]
#[derive(Debug)]
pub struct UsersService;

#[cfg_attr(test, faux::methods)]
impl UsersService {
    #[tracing::instrument(name = "users_service::create_user")]
    pub async fn create_user(
        pool: &Pool<Postgres>,
        user_data: CreateUserDto,
    ) -> Result<UserDto, ApiError> {
        query_as!(
            UserDto,
            r#"
        INSERT INTO "users" (first_name, last_name, username, email)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
            user_data.first_name,
            user_data.last_name,
            user_data.username,
            user_data.email,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!("Failed to create user: {}", e);
            match e {
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_username_key") => {
                    ApiError::AlreadyExists(format!(
                        "Username '{}' is already registered",
                        user_data.username
                    ))
                }
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_email_key") => {
                    ApiError::AlreadyExists(format!(
                        "Email '{}' is already registered",
                        user_data.email
                    ))
                }
                _ => ApiError::from(e),
            }
        })
    }

    #[tracing::instrument(name = "users_service::get_user_by_id")]
    pub async fn get_user_by_id(
        pool: &Pool<Postgres>,
        user_id: Uuid,
    ) -> Result<UserDto, sqlx::Error> {
        query_as!(UserDto, "select * from  users  where user_id = $1", user_id)
            .fetch_one(pool)
            .await
    }
}
