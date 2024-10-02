pub mod api_user;
use sea_orm::prelude::Uuid;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateClientRequest {
    pub name: String,
    pub realm_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateClientRequest {
    pub name: Option<String>,
    pub lock: Option<bool>,
    pub max_concurrent_sessions: Option<i32>,
    pub session_lifetime: Option<i32>,       // in seconds
    pub refresh_token_lifetime: Option<i32>, // in seconds
    pub refresh_token_reuse_limit: Option<i32>,
}
