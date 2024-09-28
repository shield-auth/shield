use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::auth::{login, logout, register, verify};

pub fn create_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/register", post(register))
        .route("/verify", post(verify))
}
