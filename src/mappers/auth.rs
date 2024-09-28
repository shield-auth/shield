use std::collections::HashMap;

use serde::Deserialize;

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
