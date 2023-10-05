use actix_web::{get, web::{ServiceConfig, scope}};
use shuttle_actix_web::ShuttleActixWeb;
use crate::organisation::factory::organisation_factory;

mod organisation;
mod project;

#[get("")]
async fn hello_world() -> &'static str {
    "Hello World! It's your Shield"
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(scope("/").service(hello_world))
            .service(scope("/organisations").service(organisation_factory()));
    };

    Ok(config.into())
}
