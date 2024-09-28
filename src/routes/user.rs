use axum::{routing::get, Router};

use crate::handlers::user::{delete_user, get_resources, get_user, get_users};

pub fn create_routes() -> Router {
    Router::new().route("/", get(get_users)).nest(
        "/:user_id",
        Router::new()
            .route("/", get(get_user).delete(delete_user))
            .route("/resources", get(get_resources)),
    )
}
