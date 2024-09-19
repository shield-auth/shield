use axum::extract::Path;

pub async fn get_clients(Path(realm): Path<String>) -> String {
    format!("Hi from clients of {realm}")
}

pub async fn get_client(Path((realm, client_id)): Path<(String, String)>) -> String {
    println!("This is client Name: {} - {}", &realm, &client_id);
    format!("client is - {} - {}", realm, client_id)
}
