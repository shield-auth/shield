#[get("/")]
pub fn index() -> &'static str {
    "Hello from Shield, this is Organizations Route"
}

#[get("/<organization_id>")]
pub fn organization(organization_id: &str) -> String {
    format!(
        "Hello from Shield, this is Organizations Route for org_id: {}",
        organization_id
    )
}
