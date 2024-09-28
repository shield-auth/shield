use crate::packages::token::TokenUser;

pub fn is_master_realm_admin(user: &TokenUser) -> bool {
    let resource = match &user.resource {
        Some(resource) => resource,
        None => return false,
    };
    let role = resource.identifiers.get("role");
    let realm = resource.identifiers.get("realm");
    if role.is_none() {
        return false;
    }
    if realm.is_none() {
        return false;
    }

    let is_admin = role.unwrap() == "admin";
    let is_master_realm = realm.unwrap() == "master";

    is_admin && is_master_realm
}

pub fn is_any_realm_admin(user: &TokenUser) -> bool {
    user.resource
        .as_ref()
        .is_some_and(|x| x.client_name == "client" && x.identifiers.get("role").is_some_and(|y| y == "admin"))
}

pub fn is_current_realm_admin(user: &TokenUser, realm_name: &str) -> bool {
    user.resource.as_ref().is_some_and(|x| {
        x.identifiers
            .get("realm")
            .is_some_and(|y| y == realm_name && x.client_name == "client" && x.identifiers.get("role").is_some_and(|z| z == "admin"))
    })
}
