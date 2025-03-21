use super::*;
use axum::Json;
use crate::utils::models::User;

#[tokio::test]
async fn test_get_user_returns_default_user() {
    let result = get_user().await;
    assert!(result.is_ok(), "Expected Ok result, got an error");

    let Json(user) = result.unwrap();
    assert_eq!(user, User::default(), "User returned is not the default one");
}
