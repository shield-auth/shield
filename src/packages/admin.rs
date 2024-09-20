use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::info;

use crate::database::{
    prelude::*,
    realm::{self, ActiveModel},
};

use super::db::AppState;

pub async fn setup(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    info!("Checking ADMIN availability!");
    let is_master_realm_exists = Realm::find().filter(realm::Column::Name.eq("Master")).one(&state.db).await?;

    if is_master_realm_exists.is_some() {
        info!("Master realm exists");
        info!("Starting the server...");
    } else {
        info!("Master realm does not exist");
        info!("⌛ Initializing the ADMIN...");

        // Step 1: Create "Master" realm.
        let master_realm = create_master_realm(&state.db).await?;
        info!("Master realm created: {:#?}", master_realm);

        // Step 2: Create admin user (implement logic here).
        // Example: create_admin(&state.db, admin_username, admin_password).await?;

        // Step 3: Give admin access to "Master" realm.
        // Example: grant_access_to_admin(&state.db, master_realm.id, admin_user.id).await?;

        info!("Admin initialization complete.");
    }

    Ok(())
}

async fn create_master_realm(conn: &DatabaseConnection) -> Result<realm::Model, Box<dyn std::error::Error>> {
    let new_realm = ActiveModel {
        name: Set("Master".to_owned()),
        ..Default::default()
    };
    let inserted_realm = new_realm.insert(conn).await?;
    info!("✅ 1/3 Master realm created");

    Ok(inserted_realm)
}

// async fn create_default_client(conn: &DatabaseConnection, realm_id: uu)
