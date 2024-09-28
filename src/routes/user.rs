use axum::{
    routing::{get, patch},
    Router,
};

use crate::handlers::user::{add_resources, delete_resource, delete_user, get_resources, get_user, get_users, update_resource};

pub fn create_routes() -> Router {
    Router::new().route("/", get(get_users)).nest(
        "/:user_id",
        Router::new().route("/", get(get_user).delete(delete_user)).nest(
            "/resources",
            Router::new()
                .route("/", get(get_resources).post(add_resources))
                .route("/:resource_id", patch(update_resource).delete(delete_resource)),
        ),
    )
}
