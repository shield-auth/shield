use actix_web::{Scope, web};
use crate::organisation::controller::{index, org_info};
use crate::project::factory::project_factory;

pub fn organisation_factory () -> Scope {
    web::scope("")
        .service(index)
        .service(org_info)
        .service(web::scope("/{organisation_id}/projects").service(project_factory()))
}