use axum::extract::Path;
use sea_orm::prelude::Uuid;

pub async fn get_users(Path(realm_id): Path<Uuid>) -> String {
    format!("Hi from users of {realm_id}")
}

pub async fn get_user(Path((realm_id, user_id)): Path<(Uuid, Uuid)>) -> String {
    println!("This is user Name: {} - {}", &realm_id, &user_id);
    format!("user is - {} - {}", realm_id, user_id)
}
