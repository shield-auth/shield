use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateRealmRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateRealmRequest {
    pub name: String,
    pub lock: Option<bool>,
}

#[derive(Serialize)]
pub struct DeleteResponse {
    pub ok: bool,
}
