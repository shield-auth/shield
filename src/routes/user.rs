use axum::{routing::get, Router};

use crate::handlers::user::{get_user, get_users};

pub fn create_routes() -> Router {
    Router::new().route("/", get(get_users)).route("/:user_id", get(get_user))
}
