use std::sync::Arc;

use axum::{http::header, Router};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer, sensitive_headers::SetSensitiveHeadersLayer,
};

use crate::{
    middleware::logger::logger,
    packages::{admin, db::get_db_connection_pool, logger},
    routes,
};

pub async fn create_app() -> Router {
    logger::setup();
    let state = get_db_connection_pool().await.unwrap();

    admin::setup(&state).await.expect("Failed to setup admin account");
    Router::new()
        .with_state(Arc::new(state))
        .merge(routes::create_routes())
        .layer(logger())
        // Mark the `Authorization` request header as sensitive so it doesn't
        // show in logs.
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(header::AUTHORIZATION)))
        // Compress responses
        .layer(CompressionLayer::new())
        // Propagate `X-Request-Id`s from requests to responses
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static("x-request-id")))
        .layer(CorsLayer::permissive()) // TODO: Update is later
}
