use chrono::{FixedOffset, Utc};
use sea_orm::{ActiveModelTrait, Set};
use std::sync::Arc;

use crate::{
    database::{
        prelude::{Client, Resource, ResourceGroup, Session, User},
        resource, resource_group, session,
        session::ActiveModel as SessionActiveModel,
        user::{self, Model},
    },
    mappers::auth::{CreateUserRequest, LogoutResponse},
    middleware::session_info_extractor::SessionInfo,
    packages::{
        db::AppState,
        errors::{AuthenticateError, Error},
        settings::SETTINGS,
        token::{create, TokenUser},
    },
    services::user::insert_user,
    utils::role_checker::{is_current_realm_admin, is_master_realm_admin},
};
use axum::{extract::Path, Extension, Json};
use sea_orm::{prelude::Uuid, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
    user: Model,
    realm_id: Uuid,
    client_id: Uuid,
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Extension(session_info): Extension<Arc<SessionInfo>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<Credentials>,
) -> Result<Json<LoginResponse>, Error> {
    debug!("🚀 Login request received! {:#?}", session_info);
    let user_with_resource_groups = User::find()
        .filter(user::Column::Email.eq(payload.email))
        .find_also_related(ResourceGroup)
        .filter(resource_group::Column::RealmId.eq(realm_id))
        .filter(resource_group::Column::ClientId.eq(client_id))
        .one(&state.db)
        .await?;

    if user_with_resource_groups.is_none() {
        debug!("No matching data found");
        return Err(Error::not_found());
    }

    let (user, resource_groups) = user_with_resource_groups.unwrap();
    if !user.verify_password(&payload.password) {
        debug!("Wrong password");
        return Err(Error::Authenticate(AuthenticateError::WrongCredentials));
    }
    if user.locked_at.is_some() {
        debug!("User is locked");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    if resource_groups.is_none() {
        debug!("No matching resource group found");
        return Err(Error::not_found());
    }

    let resource_groups = resource_groups.unwrap();
    if resource_groups.locked_at.is_some() {
        debug!("Resource group is locked");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    // Fetch client separately
    let client = Client::find_by_id(client_id).one(&state.db).await?.ok_or_else(|| {
        debug!("No client found");
        Error::not_found()
    })?;

    if client.locked_at.is_some() {
        debug!("Client is locked");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    let sessions = Session::find()
        .filter(session::Column::ClientId.eq(client.id))
        .filter(session::Column::UserId.eq(user.id))
        .count(&state.db)
        .await?;

    if sessions >= client.max_concurrent_sessions as u64 {
        debug!("Client has reached max concurrent sessions");
        return Err(Error::Authenticate(AuthenticateError::MaxConcurrentSessions));
    }
    // Fetch resources
    let resources = Resource::find()
        .filter(resource::Column::GroupId.eq(resource_groups.id))
        .filter(resource::Column::LockedAt.is_null())
        .all(&state.db)
        .await?;

    if resources.is_empty() {
        debug!("No resources found");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    let session = SessionActiveModel {
        user_id: Set(user.id),
        client_id: Set(client.id),
        ip_address: Set(session_info.ip_address.to_string()),
        user_agent: Set(Some(session_info.user_agent.to_string())),
        browser: Set(Some(session_info.browser.to_string())),
        browser_version: Set(Some(session_info.browser_version.to_string())),
        operating_system: Set(Some(session_info.operating_system.to_string())),
        device_type: Set(Some(session_info.device_type.to_string())),
        country_code: Set(Some(session_info.country_code.to_string())),
        expires: Set((Utc::now() + chrono::Duration::seconds(client.session_lifetime as i64)).into()),
        ..Default::default()
    };
    let session = session.insert(&state.db).await?;

    let access_token = create(
        user.clone(),
        client,
        resource_groups,
        resources,
        session,
        &SETTINGS.read().secrets.signing_key,
    )
    .unwrap();

    Ok(Json(LoginResponse {
        access_token,
        user,
        realm_id,
        client_id,
    }))
}

pub async fn register(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<Model>, Error> {
    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        let user = insert_user(&state.db, realm_id, client_id, payload).await?;
        Ok(Json(user))
    } else {
        return Err(Error::Authenticate(AuthenticateError::ActionForbidden));
    }
}

pub async fn logout(user: TokenUser, Extension(_state): Extension<Arc<AppState>>) -> Result<Json<LogoutResponse>, Error> {
    Ok(Json(LogoutResponse { ok: true, user_id: user.sub }))
}

pub async fn verify() {
    debug!("🚀 Verify request received!");
    todo!();
}
