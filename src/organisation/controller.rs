use actix_web::{get, HttpResponse, Result};
use actix_web::web::Path;

#[get("")]
pub async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Hello from the organization index!"))
}

#[get("/{organisation_id}")]
pub async fn org_info(path: Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(format!("Your organisation id: {}", path)))
}
