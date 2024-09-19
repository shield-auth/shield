use axum::extract::Path;

pub async fn get_users(Path(realm): Path<String>) -> String {
    format!("Hi from users of {realm}")
}

pub async fn get_user(Path((realm, user_id)): Path<(String, String)>) -> String {
    println!("This is user Name: {} - {}", &realm, &user_id);
    format!("user is - {} - {}", realm, user_id)
}
