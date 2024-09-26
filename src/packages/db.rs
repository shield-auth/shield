use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

use super::settings::SETTINGS;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn get_db_connection_pool() -> Result<AppState, DbErr> {
    let uri = &SETTINGS.database.uri;
    let mut opts = ConnectOptions::new(uri);
    opts.max_connections(20).connect_timeout(Duration::from_secs(5));

    let db = Database::connect(opts).await?;

    Ok(AppState { db })
}
