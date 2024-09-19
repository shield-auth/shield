use std::sync::Arc;

use tokio::sync::Notify;
use tracing::info;

pub async fn shutdown_signal_handler(shutdown_signal: Arc<Notify>) {
    shutdown_signal.notified().await;
    info!("Handling graceful shutdown");
    info!("Close resources, drain and shutdown event handler... etc");
}
