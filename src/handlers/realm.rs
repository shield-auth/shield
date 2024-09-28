use std::{fs, sync::Arc};

use axum::{extract::Path, Extension, Json};
use chrono::Utc;
use sea_orm::{prelude::Uuid, ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::{
    database::{
        prelude::Realm,
        realm::{ActiveModel, Model},
    },
    packages::{
        db::AppState,
        errors::{AuthenticateError, Error},
        token::TokenUser,
    },
    utils::{
        default_resource_checker::is_default_realm,
        helpers::default_cred::DefaultCred,
        role_checker::{is_master_realm_admin, is_realm_admin},
    },
};

pub async fn get_realms(user: TokenUser, Extension(state): Extension<Arc<AppState>>) -> Result<Json<Vec<Model>>, Error> {
    if is_master_realm_admin(&user) {
        let realms = Realm::find().all(&state.db).await?;
        return Ok(Json(realms));
    }

    return Err(Error::Authenticate(AuthenticateError::NoResource));
}

pub async fn get_realm(user: TokenUser, Extension(state): Extension<Arc<AppState>>, Path(realm_id): Path<Uuid>) -> Result<Json<Model>, Error> {
    if is_master_realm_admin(&user) {
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
                if is_realm_admin(&user)
                    && user
                        .resource
                        .is_some_and(|x| x.identifiers.get("realm").is_some_and(|y| y == &fetched_realm.slug))
                {
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

#[derive(Deserialize)]
pub struct CreateRealmRequest {
    name: String,
}

pub async fn create_realm(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<CreateRealmRequest>,
) -> Result<Json<Model>, Error> {
    if is_master_realm_admin(&user) {
        let realm = ActiveModel {
            name: Set(payload.name),
            ..Default::default()
        };
        let realm = realm.insert(&state.db).await?;
        Ok(Json(realm))
    } else {
        Err(Error::Authenticate(AuthenticateError::NoResource))
    }
}

#[derive(Deserialize)]
pub struct UpdateRealmRequest {
    name: String,
    lock: Option<bool>,
}

pub async fn update_realm(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(realm_id): Path<Uuid>,
    Json(payload): Json<UpdateRealmRequest>,
) -> Result<Json<Model>, Error> {
    if is_master_realm_admin(&user) || is_realm_admin(&user) {
        let realm = Realm::find_by_id(realm_id).one(&state.db).await?;
        match realm {
            Some(realm) => {
                if is_default_realm(realm.id) && payload.lock == Some(true) {
                    return Err(Error::cannot_perform_operation("Cannot lock the default realm"));
                }

                let locked_at = match payload.lock {
                    Some(true) => Some(realm.locked_at.unwrap_or_else(|| Utc::now().naive_utc())),
                    Some(false) => None,
                    None => realm.locked_at,
                };

                let updated_realm = ActiveModel {
                    id: Set(realm.id),
                    name: Set(payload.name),
                    locked_at: Set(locked_at),
                    ..Default::default()
                };
                let updated_realm = updated_realm.update(&state.db).await?;
                Ok(Json(updated_realm))
            }
            None => {
                return Err(Error::Authenticate(AuthenticateError::NoResource));
            }
        }
    } else {
        Err(Error::Authenticate(AuthenticateError::NoResource))
    }
}

#[derive(Serialize)]
pub struct DeleteResponse {
    ok: bool,
}

pub async fn delete_realm(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(realm_id): Path<Uuid>,
) -> Result<Json<DeleteResponse>, Error> {
    if is_master_realm_admin(&user) {
        if is_default_realm(realm_id) {
            return Err(Error::cannot_perform_operation("Cannot delete the default realm"));
        }
        Realm::delete_by_id(realm_id).exec(&state.db).await?;
        Ok(Json(DeleteResponse { ok: true }))
    } else {
        Err(Error::Authenticate(AuthenticateError::NoResource))
    }
}
