use axum::{routing::get, Router};

use crate::handlers::client::{create_client, delete_client, get_client, get_clients, update_client};

use super::auth;

pub fn create_routes() -> Router {
    Router::new().route("/", get(get_clients).post(create_client)).nest(
        "/:client_id",
        Router::new()
            .route("/", get(get_client).patch(update_client).delete(delete_client))
            .nest("/auth", auth::create_routes()),
    )
}
