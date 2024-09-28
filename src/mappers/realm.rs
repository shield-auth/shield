use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateRealmRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateRealmRequest {
    pub name: String,
    pub lock: Option<bool>,
}
