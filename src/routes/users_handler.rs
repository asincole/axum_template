use crate::dtos::{CreateUserDto, UserDto};
use crate::services::UsersService;
use crate::{api_config::AppState, utils::api_error::ApiError};
use axum::extract::Path;
use axum::{extract::State, Json};
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

#[cfg(test)]
mod user_tests;

pub const USERS_TAG: &str = "users";

pub(crate) fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(routes!(create_user, get_user))
}

#[utoipa::path(
    post,
    path = "/",
    tag = USERS_TAG,
    responses(
        (status = 201,  body = UserDto),
        ApiError
    )
)]
#[tracing::instrument(name = "users::create_user", skip(state, body))]
async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateUserDto>,
) -> Result<Json<UserDto>, ApiError> {
    let new_user = UsersService::create_user(&state.db.pool, body).await?;
    Ok(Json(new_user))
}

#[utoipa::path(
    get,
    path = "/{user_id}",
    tag = USERS_TAG,
    responses(
        (status = 200,  body = UserDto),
        ApiError
    )
)]
#[tracing::instrument(name = "users::get_user", skip(state))]
async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> anyhow::Result<Json<UserDto>, ApiError> {
    let user: UserDto = UsersService::get_user_by_id(&state.db.pool, user_id).await?;

    Ok(Json(user))
}
