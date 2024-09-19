use axum::{routing::get, Router};

use crate::handlers::client::{get_client, get_clients};

pub fn create_routes() -> Router {
    Router::new().route("/", get(get_clients)).route("/:client_id", get(get_client))
}
