use axum::{routing::get, Json, Router};
use serde::Serialize;
use tracing::debug;

use crate::packages::errors::Error;

pub fn create_routes() -> Router {
    Router::new().route("/", get(get_health))
}

async fn get_health() -> Result<Json<Health>, Error> {
    debug!("Returning health");
    Ok(Json(Health { ok: true }))
}

#[derive(Serialize, Debug)]
struct Health {
    ok: bool,
}
