#[get("/")]
pub fn index() -> &'static str {
    "Hello from Shield, this is Organisations Route"
}