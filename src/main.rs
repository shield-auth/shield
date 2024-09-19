use std::{net::SocketAddr, sync::Arc};

use app::create_app;
use packages::{settings::SETTINGS, shutdown::shutdown_signal_handler};
use tokio::{net::TcpListener, sync::Notify};
use tracing::info;

mod app;
mod handlers;
mod models;
mod packages;
mod routes;
mod schemas;
mod utils;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = SETTINGS.server.port;
    let address = SocketAddr::from(([127, 0, 0, 1], port));

    let app = create_app().await;
    let listener = TcpListener::bind(address).await?;

    info!("Server is listening on {}", &address);

    let shutdown_signal = Arc::new(Notify::new());
    let shutdown_signal_clone = shutdown_signal.clone();
    ctrlc::set_handler(move || {
        shutdown_signal_clone.notify_one();
    })
    .expect("Error setting Ctrl+C handler");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal_handler(shutdown_signal))
        .await
}
