use sea_orm::prelude::Uuid;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateClientRequest {
    pub name: String,
    pub realm_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateClientRequest {
    pub name: String,
    pub lock: Option<bool>,
}
