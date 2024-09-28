use serde::Serialize;

pub mod auth;
pub mod client;
pub mod realm;
pub mod user;

#[derive(Serialize)]
pub struct DeleteResponse {
    pub ok: bool,
}
