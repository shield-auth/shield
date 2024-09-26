use axum::extract::Path;
use sea_orm::prelude::Uuid;

pub async fn get_clients(Path(realm_id): Path<Uuid>) -> String {
    format!("Hi from clients of {realm_id}")
}

pub async fn get_client(Path((realm_id, client_id)): Path<(Uuid, Uuid)>) -> String {
    println!("This is client Name: {} - {}", &realm_id, &client_id);
    format!("client is - {} - {}", realm_id, client_id)
}
