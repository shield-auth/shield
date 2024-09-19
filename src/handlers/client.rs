use axum::extract::Path;

pub async fn get_clients() -> String {
    "Hi from clients".to_owned()
}

pub async fn get_client(Path((realm, client_id)): Path<(String, String)>) -> String {
    println!("This is client Name: {} - {}", &realm, &client_id);
    format!("client is - {} - {}", realm, client_id)
}
