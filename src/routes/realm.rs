use axum::{routing::get, Router};

use crate::handlers::realm::{get_realm, get_realms};

use super::client;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(get_realms))
        .route("/:realm", get(get_realm))
        .merge(client::create_routes())
}
