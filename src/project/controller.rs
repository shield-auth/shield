use actix_web::{get, HttpResponse, Result};
use actix_web::web::Path;

#[get("")]
pub async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Hello from Shield, this is Projects Route"))
}

#[get("/{project_id}")]
pub async fn project_info(path: Path<(String, String)>) -> Result<HttpResponse> {
    let (organisation_id, project_id) = path.into_inner();
    Ok(HttpResponse::Ok().body(format!("Your project id is: {} belongs to: {}", project_id, organisation_id)))
}
