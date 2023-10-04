use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub enum RouteKey {
    Organisations,
    Projects
}

pub fn get_routes() -> HashMap<RouteKey, String> {
    let mut routes = HashMap::new();
    routes.insert(RouteKey::Organisations, "/organisations".to_string());
    routes.insert(RouteKey::Projects, "/projects".to_string());

    routes
}