use diesel::{
    dsl::insert_into,
    r2d2::{ConnectionManager, PooledConnection},
    select, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use tracing::{debug, info};

use crate::{
    models::realm::{NewRealm, Realm},
    packages::errors::Error,
    schemas::db::realm::{self, name},
};

use super::db::AppState;

pub async fn setup(state: &AppState) {
    info!("Checking ADMIN availability!");

    let mut conn = state.pool.get().map_err(Error::db_error).unwrap();
    debug!("Connection initialized");
    let is_master_realm_exists = select(diesel::dsl::exists(realm::dsl::realm.filter(name.eq("Master"))))
        .get_result::<bool>(&mut conn)
        .unwrap();
    debug!("find query ran for master realm");

    if is_master_realm_exists {
        info!("Master realm exists");
        info!("Starting the server...");
    } else {
        info!("Master realm does not exist");
        info!("⌛ Initializing the ADMIN...");
        // TODO: 1. Create "Master" realm.
        let res = create_master_realm(&mut conn);
        // TODO: 2. Create admin using the admin_username and admin_password. If missing then throw error.

        println!("{:#?}", res);
        // TODO: 3. Give access of "Master" realm to admin.
    }
}

fn create_master_realm(conn: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Realm {
    let new_realm = NewRealm { name: "Master" };
    let res = insert_into(realm::table)
        .values(&new_realm)
        .get_result(conn)
        .map_err(Error::db_error)
        .unwrap();
    info!("✅ 1/3 Master realm created");
    return res;
}
