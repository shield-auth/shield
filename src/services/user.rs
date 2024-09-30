use crate::{mappers::auth::CreateUserRequest, packages::errors::Error, utils::hash::generate_password_hash};
use entity::{resource, resource_group, user};
use futures::future::join_all;
use sea_orm::{prelude::Uuid, ActiveModelTrait, DatabaseConnection, Set};

pub async fn insert_user(db: &DatabaseConnection, realm_id: Uuid, client_id: Uuid, payload: CreateUserRequest) -> Result<user::Model, Error> {
    let password_hash = generate_password_hash(payload.password).await?;
    let user = user::ActiveModel {
        realm_id: Set(realm_id),
        email: Set(payload.email),
        password_hash: Set(Some(password_hash)),
        first_name: Set(payload.first_name),
        last_name: Set(payload.last_name),
        phone: Set(payload.phone),
        image: Set(payload.image),
        ..Default::default()
    };

    let user = user.insert(db).await?;

    let resource_group = resource_group::ActiveModel {
        realm_id: Set(user.realm_id),
        client_id: Set(client_id),
        user_id: Set(user.id),
        name: Set(payload.resource.group_name),
        ..Default::default()
    };
    let resource_group = resource_group.insert(db).await?;

    let futures: Vec<_> = payload
        .resource
        .identifiers
        .iter()
        .map(|(name, value)| {
            let resource = resource::ActiveModel {
                group_id: Set(resource_group.id),
                name: Set(name.to_string()),
                value: Set(value.to_string()),
                ..Default::default()
            };
            resource.insert(db)
        })
        .collect();

    join_all(futures).await;

    Ok(user)
}
