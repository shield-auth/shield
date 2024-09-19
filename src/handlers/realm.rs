use axum::extract::Path;

pub async fn get_realms() -> String {
    "Hi from MASTER REALM".to_owned()
}

pub async fn get_realm(Path(realm): Path<String>) -> String {
    println!("This is realm Name: {}", &realm);
    format!("Realm is - {}", realm)
}
