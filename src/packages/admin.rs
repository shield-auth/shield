use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use sea_orm::{prelude::Uuid, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::info;

use crate::{
    database::{
        client,
        prelude::{Realm, User},
        realm, resource, resource_group, user,
    },
    packages::settings::SETTINGS,
    utils::hash::generate_password_hash,
};

use super::{db::AppState, errors::Error};

#[allow(dead_code)]
#[derive(Debug)]
pub struct DefaultCred {
    realm_id: Uuid,
    client_id: Uuid,
}

pub async fn setup(state: &AppState) -> Result<(), Error> {
    info!("Checking ADMIN availability!");
    let is_admin_user_exists = User::find().filter(user::Column::Email.eq(&SETTINGS.admin.email)).one(&state.db).await?;

    if is_admin_user_exists.is_some() {
        info!("DB has been already initialized!");
        info!("Starting the server...");
    } else {
        info!("DB has not been initialized!");
        info!("⌛ Initializing the DB...");

        initialize_db(&state.db).await?;
        info!("Admin initialization complete.");
    }

    Ok(())
}

async fn initialize_db(conn: &DatabaseConnection) -> Result<(), Error> {
    let realm = create_master_realm(conn).await?;
    let result = (|| async {
        let client = create_default_client(conn, realm.id).await?;
        let user = create_admin_user(conn, realm.id).await?;
        assign_resource_to_admin(conn, realm.id, client.id, user.id).await?;
        let default_cred = DefaultCred {
            realm_id: realm.id,
            client_id: client.id,
        };
        info!("🗝️ Please note these credentials!");
        println!("{:?}", default_cred);

        let file_path = "./logs/default_cred.txt"; // Use relative path for better portability
        let path = Path::new(file_path);
        if let Some(parent_dir) = path.parent() {
            create_dir_all(parent_dir).expect("Failed to create directory");
        } else {
            panic!("Invalid file path");
        }
        let mut file = File::create(file_path).expect("Failed to create file");
        let content = format!("{:#?}", default_cred);
        file.write_all(content.as_bytes()).expect("Failed to write to file");

        info!("📝 However above credentials have been '/logs/default_cred.txt' file.");
        Ok(())
    })()
    .await;

    if result.is_err() {
        Realm::delete_by_id(realm.id).exec(conn).await?;
        Err(result.unwrap_err())
    } else {
        Ok(())
    }
}

async fn create_master_realm(conn: &DatabaseConnection) -> Result<realm::Model, Error> {
    let new_realm = realm::ActiveModel {
        name: Set("Master".to_owned()),
        ..Default::default()
    };
    let inserted_realm = new_realm.insert(conn).await?;
    info!("✅ 1/5: Master realm created");

    Ok(inserted_realm)
}

async fn create_default_client(conn: &DatabaseConnection, realm_id: Uuid) -> Result<client::Model, Error> {
    let new_client = client::ActiveModel {
        name: Set("client".to_owned()),
        realm_id: Set(realm_id),
        ..Default::default()
    };
    let inserted_client = new_client.insert(conn).await?;
    info!("✅ 2/5: Default client created");

    Ok(inserted_client)
}

async fn create_admin_user(conn: &DatabaseConnection, realm_id: Uuid) -> Result<user::Model, Error> {
    let pw_hash = generate_password_hash(&SETTINGS.admin.password).await?;
    let new_user = user::ActiveModel {
        email: Set(SETTINGS.admin.email.to_owned()),
        password_hash: Set(Some(pw_hash)),
        realm_id: Set(realm_id),
        first_name: Set(SETTINGS.admin.email.to_owned()),
        is_temp_password: Set(Some(false)),
        ..Default::default()
    };
    let inserted_user = new_user.insert(conn).await?;
    info!("✅ 3/5: Admin user created");

    Ok(inserted_user)
}

async fn assign_resource_to_admin(conn: &DatabaseConnection, realm_id: Uuid, client_id: Uuid, user_id: Uuid) -> Result<resource::Model, Error> {
    let new_resource_group = resource_group::ActiveModel {
        client_id: Set(client_id),
        realm_id: Set(realm_id),
        user_id: Set(user_id),
        name: Set("default_resource_group".to_owned()),
        description: Set(Some(
            "This resource group has been created at the time of system initialization.".to_owned(),
        )),
        ..Default::default()
    };
    let inserted_resource_group = new_resource_group.insert(conn).await?;
    info!("✅ 4/5: Default resource group created");

    let new_resource = resource::ActiveModel {
        group_id: Set(inserted_resource_group.id),
        name: Set("role".to_owned()),
        value: Set("admin".to_owned()),
        description: Set(Some("This role has been created at the time of initialization.".to_owned())),
        ..Default::default()
    };
    let inserted_resource = new_resource.insert(conn).await?;

    let new_resource_2 = resource::ActiveModel {
        group_id: Set(inserted_resource_group.id),
        name: Set("realm".to_owned()),
        value: Set("master".to_owned()),
        description: Set(Some("This role has been created at the time of initialization.".to_owned())),
        ..Default::default()
    };
    let _inserted_resource_2 = new_resource_2.insert(conn).await?;
    info!("✅ 5/5: Default resource created");
    Ok(inserted_resource)
}
