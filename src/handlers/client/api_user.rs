use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use entity::api_user;
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

use crate::{
    mappers::client::api_user::CreateApiUserRequest,
    packages::{
        db::AppState,
        errors::{AuthenticateError, Error},
        token_user::TokenUser,
    },
    utils::{
        helpers::generate_random_string::{generate_random_string, Length},
        role_checker::{is_current_realm_admin, is_master_realm_admin},
    },
};

pub async fn get_api_users(user: TokenUser, Path((realm_id, _client_id)): Path<(Uuid, Uuid)>) -> Result<Json<Vec<api_user::Model>>, Error> {
    if !is_master_realm_admin(&user) && !is_current_realm_admin(&user, &realm_id.to_string()) {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }

    todo!()
}

pub async fn create_api_user(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateApiUserRequest>,
) -> Result<Json<api_user::Model>, Error> {
    if !is_master_realm_admin(&user) && !is_current_realm_admin(&user, &realm_id.to_string()) {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }

    let api_secret = generate_random_string(Length::U64);

    let api_user_model = api_user::ActiveModel {
        id: Set(Uuid::now_v7()),
        secret: Set(api_secret),
        name: Set(payload.name),
        description: Set(payload.description),
        realm_id: Set(realm_id),
        client_id: Set(client_id),
        role: Set(payload.role),
        access: Set(payload.access),
        expires: Set(payload.expires.unwrap().to_datetime()),
        created_by: Set(user.sub),
        updated_by: Set(user.sub),
        ..Default::default()
    };

    let api_user = api_user_model.insert(&state.db).await?;
    Ok(Json(api_user))
}

pub async fn update_api_user(user: TokenUser, Path((realm_id, _client_id)): Path<(Uuid, Uuid)>) -> Result<Json<api_user::Model>, Error> {
    if !is_master_realm_admin(&user) && !is_current_realm_admin(&user, &realm_id.to_string()) {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }

    todo!()
}

pub async fn delete_api_user(user: TokenUser, Path((realm_id, _client_id)): Path<(Uuid, Uuid)>) -> Result<Json<api_user::Model>, Error> {
    if !is_master_realm_admin(&user) && !is_current_realm_admin(&user, &realm_id.to_string()) {
        return Err(Error::Authenticate(AuthenticateError::NoResource));
    }

    todo!()
}
