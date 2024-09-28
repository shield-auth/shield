use std::sync::Arc;

use crate::{
    database::{client, prelude::Client},
    packages::{
        db::AppState,
        errors::{AuthenticateError, Error},
        token::TokenUser,
    },
    utils::role_checker::{is_any_realm_admin, is_master_realm_admin},
};
use axum::{extract::Path, Extension, Json};
use sea_orm::{prelude::Uuid, ColumnTrait, EntityTrait, QueryFilter};

pub async fn get_clients(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(realm_id): Path<Uuid>,
) -> Result<Json<Vec<client::Model>>, Error> {
    if is_master_realm_admin(&user) {
        let clients = Client::find().filter(client::Column::RealmId.eq(realm_id)).all(&state.db).await?;
        Ok(Json(clients))
    } else {
        if is_any_realm_admin(&user) {
            let resource = user.resource.unwrap();
            let clients = Client::find().filter(client::Column::RealmId.eq(realm_id)).all(&state.db).await?;
            let client = clients.iter().find(|&client| client.id == resource.client_id);
            match client {
                Some(_) => return Ok(Json(clients)),
                None => return Err(Error::Authenticate(AuthenticateError::NoResource)),
            };
        } else {
            Err(Error::Authenticate(AuthenticateError::NoResource))
        }
    }
}

pub async fn get_client(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<client::Model>, Error> {
    if is_master_realm_admin(&user) {
        let client = Client::find_by_id(client_id).one(&state.db).await?;
        match client {
            Some(client) => Ok(Json(client)),
            None => return Err(Error::Authenticate(AuthenticateError::NoResource)),
        }
    } else {
        if is_any_realm_admin(&user) {
            let client = Client::find_by_id(client_id)
                .filter(client::Column::RealmId.eq(realm_id))
                .one(&state.db)
                .await?;
            match client {
                Some(client) => return Ok(Json(client)),
                None => return Err(Error::Authenticate(AuthenticateError::NoResource)),
            };
        } else {
            if user.resource.as_ref().unwrap().client_id == client_id {
                let client = Client::find_by_id(client_id).one(&state.db).await?;
                match client {
                    Some(client) => return Ok(Json(client)),
                    None => return Err(Error::Authenticate(AuthenticateError::NoResource)),
                };
            } else {
                return Err(Error::Authenticate(AuthenticateError::ActionForbidden));
            }
        }
    }
}
