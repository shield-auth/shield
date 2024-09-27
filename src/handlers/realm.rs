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

pub async fn get_realms(user: TokenUser, Extension(state): Extension<Arc<AppState>>) -> Result<Json<Vec<Model>>, Error> {
    let resource = match user.resource {
        Some(resource) => resource,
        None => return Err(Error::Authenticate(AuthenticateError::NoResource)),
    };
    let role = resource.identifiers.get("role");
    let realm = resource.identifiers.get("realm");
    if role.is_none() {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }
    if realm.is_none() {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }

    let is_admin = role.unwrap() == "admin";
    let is_master_realm = realm.unwrap() == "master";

    if !is_admin || !is_master_realm {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }

    let realms = Realm::find().all(&state.db).await?;
    Ok(Json(realms))
}

pub async fn get_realm(user: TokenUser, Extension(state): Extension<Arc<AppState>>, Path(realm_id): Path<Uuid>) -> Result<Json<Model>, Error> {
    let resource = match user.resource {
        Some(resource) => resource,
        None => return Err(Error::Authenticate(AuthenticateError::NoResource)),
    };
    let role = resource.identifiers.get("role");
    let realm = resource.identifiers.get("realm");
    if role.is_none() {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }
    if realm.is_none() {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }

    let is_admin = role.unwrap() == "admin";
    let is_master_realm = realm.unwrap() == "master";

    if is_admin && is_master_realm {
        let fetched_realm = Realm::find().one(&state.db).await?;
        match fetched_realm {
            Some(fetched_realm) => Ok(Json(fetched_realm)),
            None => {
                return Err(Error::Authenticate(AuthenticateError::NoResource));
            }
        }
    } else {
        let fetched_realm = Realm::find_by_id(realm_id).one(&state.db).await?;
        match fetched_realm {
            Some(fetched_realm) => {
                if fetched_realm.slug == *realm.unwrap() {
                    return Ok(Json(fetched_realm));
                } else {
                    return Err(Error::Authenticate(AuthenticateError::NoResource));
                }
            }
            None => {
                return Err(Error::Authenticate(AuthenticateError::NoResource));
            }
        }
    }
}
