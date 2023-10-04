mod routes;
mod organisation;
mod project;

use routes::{get_routes, RouteKey};
use organisation::{index as org_index};
use project::{index as project_index};

#[macro_use]
extern crate rocket;

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount(&get_path(&RouteKey::Organisations), routes![org_index])
        .mount(&get_path(&RouteKey::Projects), routes![project_index]);
    Ok(rocket.into())
}

#[get("/")]
fn index() -> &'static str {
    "Hello World! It's your Shield"
}

fn get_path(route: &RouteKey) -> String {
    let routes_map = get_routes();
    routes_map.get(route).map(|s|s.to_string()).expect("Route not found")
}
