mod controller;

pub use controller::{project, projects};
use rocket::Route;

pub fn project_routes() -> Vec<Route> {
    routes![projects, project]
}
