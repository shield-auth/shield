use axum::{routing::get, Router};

use crate::handlers::realm::{get_realm, get_realms};

use super::{client, user};

pub fn create_routes() -> Router {
    Router::new().route("/", get(get_realms)).nest(
        "/:realm_id",
        Router::new()
            .route("/", get(get_realm))
            .nest("/clients", client::create_routes())
            .nest("/users", user::create_routes()),
    )
}
