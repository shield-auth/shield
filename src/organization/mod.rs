mod controller;
mod project;

use controller::{index, organization};
use project::project_routes;

use rocket::Route;

pub fn org_routes() -> Vec<Route> {
    let mut _org_routes = routes![index, organization];
    let mut _project_routes = project_routes();

    _org_routes.append(&mut _project_routes);

    _org_routes
}
