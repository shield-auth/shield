use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use sea_orm::{prelude::Uuid, EntityTrait};

use crate::{
    database::{prelude::Realm, realm::Model},
    packages::{
        db::AppState,
        errors::{AuthenticateError, Error},
        token::TokenUser,
    },
};

pub async fn get_realm() -> String {
    "Hi from MASTER REALM".to_owned()
}

pub async fn get_realms(user: TokenUser, Extension(state): Extension<Arc<AppState>>) -> Result<Json<Vec<Model>>, Error> {
    let resource = match user.resource {
        Some(resource) => resource,
        None => return Err(Error::Authenticate(AuthenticateError::NoResource)),
    };
    let is_admin = resource.identifiers.get("role");
    if is_admin.is_none() {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }
    let is_admin = is_admin.unwrap() == "admin";

    if !is_admin {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }

    // let realm = match realm {
    //     Some(realm) => realm,
    //     None => return Err(Error::Authenticate(AuthenticateError::NoResource)),
    // };

    // if realm.locked_at.is_some() {
    //     return Err(Error::Authenticate(AuthenticateError::Locked));
    // }

    // if realm.slug != "master" {
    //     return Err(Error::Authenticate(AuthenticateError::NoResource));
    // }

    let realms = Realm::find().all(&state.db).await?;
    Ok(Json(realms))
}
