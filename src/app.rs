use std::sync::Arc;

use axum::{http::header, Router};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer, sensitive_headers::SetSensitiveHeadersLayer, trace,
};

use crate::{
    packages::{admin, db::get_connection_pool, logger},
    routes,
};

pub async fn create_app() -> Router {
    logger::setup();
    let state = get_connection_pool();
    admin::setup(&state).await;

    Router::new()
        .with_state(Arc::new(state))
        .merge(routes::create_routes())
        .layer(
            // High level logging of requests and responses
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(false))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        // Mark the `Authorization` request header as sensitive so it doesn't
        // show in logs.
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(header::AUTHORIZATION)))
        // Compress responses
        .layer(CompressionLayer::new())
        // Propagate `X-Request-Id`s from requests to responses
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static("x-request-id")))
        .layer(CorsLayer::permissive()) // TODO: Update is later
}
