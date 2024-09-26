use axum::extract::Path;
use sea_orm::prelude::Uuid;

pub async fn get_realms() -> String {
    "Hi from MASTER REALM".to_owned()
}

pub async fn get_realm(Path(realm_id): Path<Uuid>) -> String {
    println!("This is realm Name: {}", &realm_id);
    format!("Realm is - {}", realm_id)
}
