use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    handlers::auth::{login, logout, register, verify},
    middleware::session_info_extractor::session_info_middleware,
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/register", post(register))
        .route("/verify", post(verify))
        .layer(middleware::from_fn(session_info_middleware))
}
