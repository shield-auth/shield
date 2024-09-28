use serde::Serialize;

pub mod auth;
pub mod client;
pub mod realm;

#[derive(Serialize)]
pub struct DeleteResponse {
    pub ok: bool,
}
