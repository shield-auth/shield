use axum::{routing::get, Json, Router};
use serde::Serialize;
use tracing::debug;

use crate::packages::{api_token::ApiTokenUser, errors::Error};

pub fn create_routes() -> Router {
    Router::new().route("/", get(get_health))
}

async fn get_health(user: ApiTokenUser) -> Result<Json<Health>, Error> {
    debug!("🚀 Health request received! {:#?}", user);
    debug!("Returning health");
    Ok(Json(Health { ok: true }))
}

#[derive(Serialize, Debug)]
struct Health {
    ok: bool,
}
