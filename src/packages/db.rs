use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use super::settings::SETTINGS;

pub struct AppState {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

pub fn get_connection_pool() -> AppState {
    let url = &SETTINGS.database.uri;
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool.");

    AppState { pool }
}
