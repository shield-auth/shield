use serde::Serialize;

pub mod client;
pub mod realm;

#[derive(Serialize)]
pub struct DeleteResponse {
    pub ok: bool,
}
