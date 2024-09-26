use axum::{routing::post, Router};

use crate::handlers::auth::{login, register, verify};

pub fn create_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/verify", post(verify))
}
