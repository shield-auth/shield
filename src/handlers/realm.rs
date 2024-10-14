use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use entity::realm;
use sea_orm::prelude::Uuid;

use crate::{
    mappers::{
        realm::{CreateRealmRequest, UpdateRealmRequest},
        DeleteResponse,
    },
    packages::{
        db::AppState,
        errors::{AuthenticateError, Error},
        jwt_token::JwtUser,
    },
    services::realm::{delete_realm_by_id, get_all_realms, get_realm_by_id, insert_realm, update_realm_by_id},
    utils::{
        default_resource_checker::is_default_realm,
        role_checker::{is_current_realm_admin, is_master_realm_admin},
    },
};

pub async fn get_realms(user: JwtUser, Extension(state): Extension<Arc<AppState>>) -> Result<Json<Vec<realm::Model>>, Error> {
    if is_master_realm_admin(&user) {
        let realms = get_all_realms(&state.db).await?;
        if realms.is_empty() {
            return Err(Error::not_found());
        }
        return Ok(Json(realms));
    }

    Err(Error::Authenticate(AuthenticateError::NoResource))
}

pub async fn get_realm(user: JwtUser, Extension(state): Extension<Arc<AppState>>, Path(realm_id): Path<Uuid>) -> Result<Json<realm::Model>, Error> {
    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        let fetched_realm = get_realm_by_id(&state.db, realm_id).await?;
        match fetched_realm {
            Some(fetched_realm) => Ok(Json(fetched_realm)),
            None => Err(Error::not_found()),
        }
    } else {
        Err(Error::Authenticate(AuthenticateError::NoResource))
    }
}

pub async fn create_realm(
    user: JwtUser,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<CreateRealmRequest>,
) -> Result<Json<realm::Model>, Error> {
    if is_master_realm_admin(&user) {
        let realm = insert_realm(&state.db, payload.name).await?;
        Ok(Json(realm))
    } else {
        Err(Error::Authenticate(AuthenticateError::NoResource))
    }
}

pub async fn update_realm(
    user: JwtUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(realm_id): Path<Uuid>,
    Json(payload): Json<UpdateRealmRequest>,
) -> Result<Json<realm::Model>, Error> {
    if is_master_realm_admin(&user) {
        let realm = update_realm_by_id(&state.db, realm_id, payload).await?;
        Ok(Json(realm))
    } else {
        Err(Error::Authenticate(AuthenticateError::NoResource))
    }
}

pub async fn delete_realm(
    user: JwtUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(realm_id): Path<Uuid>,
) -> Result<Json<DeleteResponse>, Error> {
    if is_master_realm_admin(&user) {
        if is_default_realm(realm_id) {
            return Err(Error::cannot_perform_operation("Cannot delete the default realm"));
        }
        let result = delete_realm_by_id(&state.db, realm_id).await?;
        Ok(Json(DeleteResponse {
            ok: result.rows_affected == 1,
        }))
    } else {
        Err(Error::Authenticate(AuthenticateError::NoResource))
    }
}
