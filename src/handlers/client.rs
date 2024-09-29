use std::sync::Arc;

use crate::{
    database::client::Model,
    mappers::{
        client::{CreateClientRequest, UpdateClientRequest},
        DeleteResponse,
    },
    packages::{
        db::AppState,
        errors::{AuthenticateError, Error},
        token::TokenUser,
    },
    services::client::{delete_client_by_id, get_all_clients, get_client_by_id, insert_client, update_client_by_id},
    utils::{
        default_resource_checker::is_default_client,
        role_checker::{is_current_realm_admin, is_master_realm_admin},
    },
};
use axum::{extract::Path, Extension, Json};
use sea_orm::prelude::Uuid;

pub async fn get_clients(user: TokenUser, Extension(state): Extension<Arc<AppState>>, Path(realm_id): Path<Uuid>) -> Result<Json<Vec<Model>>, Error> {
    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        let clients = get_all_clients(&state.db, realm_id).await?;
        if clients.is_empty() {
            return Err(Error::not_found());
        }
        Ok(Json(clients))
    } else {
        Err(Error::Authenticate(AuthenticateError::NoResource))
    }
}

pub async fn get_client(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Model>, Error> {
    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        let client = get_client_by_id(&state.db, client_id).await?;
        match client {
            Some(client) => Ok(Json(client)),
            None => return Err(Error::Authenticate(AuthenticateError::NoResource)),
        }
    } else {
        Err(Error::Authenticate(AuthenticateError::ActionForbidden))
    }
}

pub async fn create_client(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(realm_id): Path<Uuid>,
    Json(payload): Json<CreateClientRequest>,
) -> Result<Json<Model>, Error> {
    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        let client = insert_client(&state.db, payload).await?;
        Ok(Json(client))
    } else {
        Err(Error::Authenticate(AuthenticateError::NoResource))
    }
}

pub async fn update_client(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateClientRequest>,
) -> Result<Json<Model>, Error> {
    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        let client = update_client_by_id(&state.db, client_id, payload).await?;
        Ok(Json(client))
    } else {
        Err(Error::Authenticate(AuthenticateError::NoResource))
    }
}

pub async fn delete_client(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<DeleteResponse>, Error> {
    if is_default_client(client_id) {
        return Err(Error::cannot_perform_operation("Cannot delete the default client"));
    }

    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        let client = delete_client_by_id(&state.db, client_id).await?;
        Ok(Json(DeleteResponse {
            ok: client.rows_affected == 1,
        }))
    } else {
        Err(Error::Authenticate(AuthenticateError::NoResource))
    }
}
