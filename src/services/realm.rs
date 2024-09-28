use chrono::Utc;
use sea_orm::{prelude::Uuid, ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, Set};

use crate::{
    database::{
        prelude::Realm,
        realm::{ActiveModel, Model},
    },
    mappers::realm::UpdateRealmRequest,
    packages::errors::{AuthenticateError, Error},
    utils::default_resource_checker::is_default_realm,
};

pub async fn get_all_realms(db: &DatabaseConnection) -> Result<Vec<Model>, Error> {
    Ok(Realm::find().all(db).await?)
}

pub async fn get_realm_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<Model>, Error> {
    Ok(Realm::find_by_id(id).one(db).await?)
}

pub async fn insert_realm(db: &DatabaseConnection, name: String) -> Result<Model, Error> {
    let realm = ActiveModel {
        name: Set(name),
        ..Default::default()
    };
    Ok(realm.insert(db).await?)
}

pub async fn update_realm_by_id(db: &DatabaseConnection, id: Uuid, payload: UpdateRealmRequest) -> Result<Model, Error> {
    let realm = get_realm_by_id(db, id).await?;
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
            let updated_realm = updated_realm.update(db).await?;
            Ok(updated_realm)
        }
        None => {
            return Err(Error::Authenticate(AuthenticateError::NoResource));
        }
    }
}

pub async fn delete_realm_by_id(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, Error> {
    Ok(Realm::delete_by_id(id).exec(db).await?)
}
