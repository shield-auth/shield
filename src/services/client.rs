use chrono::Utc;
use sea_orm::{prelude::Uuid, ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, QueryFilter, Set};

use crate::{
    database::{
        client::{ActiveModel, Column, Model},
        prelude::Client,
    },
    mappers::client::{CreateClientRequest, UpdateClientRequest},
    packages::errors::{AuthenticateError, Error},
    utils::default_resource_checker::is_default_client,
};

pub async fn get_all_clients(db: &DatabaseConnection, realm_id: Uuid) -> Result<Vec<Model>, Error> {
    Ok(Client::find().filter(Column::RealmId.eq(realm_id)).all(db).await?)
}

pub async fn get_client_by_id(db: &DatabaseConnection, client_id: Uuid) -> Result<Option<Model>, Error> {
    Ok(Client::find_by_id(client_id).one(db).await?)
}

pub async fn insert_client(db: &DatabaseConnection, payload: CreateClientRequest) -> Result<Model, Error> {
    let client = ActiveModel {
        name: Set(payload.name.to_owned()),
        realm_id: Set(payload.realm_id),
        ..Default::default()
    };
    Ok(client.insert(db).await?)
}

pub async fn update_client_by_id(db: &DatabaseConnection, client_id: Uuid, payload: UpdateClientRequest) -> Result<Model, Error> {
    if is_default_client(client_id) && payload.lock == Some(true) {
        return Err(Error::cannot_perform_operation("Cannot lock the default client"));
    }

    let client = get_client_by_id(db, client_id).await?;
    match client {
        Some(client) => {
            let locked_at = match payload.lock {
                Some(true) => Some(client.locked_at.unwrap_or_else(|| Utc::now().naive_utc())),
                Some(false) => None,
                None => client.locked_at,
            };

            let updated_client = ActiveModel {
                id: Set(client.id),
                name: Set(payload.name),
                locked_at: Set(locked_at),
                ..Default::default()
            };
            let updated_client = updated_client.update(db).await?;
            Ok(updated_client)
        }
        None => {
            return Err(Error::Authenticate(AuthenticateError::NoResource));
        }
    }
}

pub async fn delete_client_by_id(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, Error> {
    Ok(Client::delete_by_id(id).exec(db).await?)
}
