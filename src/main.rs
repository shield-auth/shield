use std::net::SocketAddr;

use app::create_app;
use packages::settings::SETTINGS;
use tokio::net::TcpListener;
use tracing::info;

mod app;
mod handlers;
mod mappers;
mod middleware;
mod packages;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = SETTINGS.read().server.port;
    let address = SocketAddr::from(([0, 0, 0, 0], port));

    let app = create_app().await;
    let listener = TcpListener::bind(address).await?;

    info!("Server is listening on {}", &address);

    axum::serve(listener, app).await
}
