use actix_web::{Scope, web};
use crate::project::controller::{index, project_info};

pub fn project_factory () -> Scope {
    web::scope("")
        .service(index)
        .service(project_info)
}