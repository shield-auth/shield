use axum::{routing::get, Router};

use crate::handlers::client::{get_client, get_clients};

use super::auth;

pub fn create_routes() -> Router {
    Router::new().route("/", get(get_clients)).nest(
        "/:client_id",
        Router::new().route("/", get(get_client)).nest("/auth", auth::create_routes()),
    )
}
