use sea_orm::prelude::Uuid;

use crate::packages::token::TokenUser;

use super::default_resource_checker::is_default_realm;

pub fn is_master_realm_admin(user: &TokenUser) -> bool {
    let resource = match &user.resource {
        Some(resource) => resource,
        None => return false,
    };
    let role = resource.identifiers.get("role");
    let realm = resource.identifiers.get("realm");
    if role.is_none() || realm.is_none() {
        return false;
    }

    let is_admin = role.unwrap() == "admin";
    let is_master_realm = is_default_realm(resource.identifiers.get("realm").unwrap().parse::<Uuid>().unwrap());

    is_admin && is_master_realm
}

pub fn is_current_realm_admin(user: &TokenUser, realm_id: &str) -> bool {
    user.resource.as_ref().is_some_and(|x| {
        x.identifiers
            .get("realm")
            .is_some_and(|y| y == realm_id && x.identifiers.get("role").is_some_and(|z| z == "admin"))
    })
}
