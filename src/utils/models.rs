use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utoipa::ToSchema;

#[derive(Debug, PartialEq, ToSchema, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Ulid,
    pub first_name: String,
    pub last_name: String,
}
