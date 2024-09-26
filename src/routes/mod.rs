use axum::Router;

pub mod auth;
pub mod client;
pub mod health;
pub mod realm;
pub mod user;

pub fn create_routes() -> Router {
    Router::new()
        .nest("/health", health::create_routes())
        .nest("/realms", realm::create_routes())
}
