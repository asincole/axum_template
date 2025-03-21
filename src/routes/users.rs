use crate::{
    api_config::AppState,
    utils::{app_error::AppError, models::User},
};
use axum::Json;
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

#[cfg(test)]
mod user_tests;

pub const USERS_TAG: &str = "users";

pub(crate) fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(routes!(get_user))
}

#[utoipa::path(
    get,
    path = "/user",
    tag = USERS_TAG,
    responses(
        (status = 200,  body = User)
    )
)]
#[tracing::instrument(name = "users::get_user")]
async fn get_user() -> anyhow::Result<Json<User>, AppError> {
    Ok(Json(User::default()))
}
