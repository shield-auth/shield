use std::collections::HashMap;

use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ResourceSubset {
    pub group_name: String,
    pub identifiers: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub image: Option<String>,
    pub resource: ResourceSubset,
}

#[derive(Serialize)]
pub struct LogoutResponse {
    pub ok: bool,
    pub user_id: Uuid,
}
